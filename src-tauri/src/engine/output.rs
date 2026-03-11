// engine/output.rs
//
// Purpose: cpal audio output stream management
//
// This module:
// - Initializes cpal output stream
// - Creates safe buffer callback path
// - Outputs silence initially
// - Handles device enumeration gracefully

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, OutputCallbackInfo, SampleFormat, Stream, StreamConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::engine::mixer::Mixer;

pub struct AudioOutput {
    stream: Option<Stream>,
    is_active: Arc<AtomicBool>,
}

impl AudioOutput {
    pub fn new(
        sample_rate: u32,
        buffer_size: u32,
        mixer: Arc<Mutex<&mut Mixer>>,
    ) -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| "No audio output device available".to_string())?;

        log::info!(
            "Using audio device: {}",
            device.name().unwrap_or_else(|_| "Unknown".to_string())
        );

        let config = device
            .default_output_config()
            .map_err(|e| format!("Failed to get default output config: {}", e))?;

        log::info!(
            "Audio config: {} channels, {} Hz, {:?}",
            config.channels(),
            config.sample_rate().0,
            config.sample_format()
        );

        let is_active = Arc::new(AtomicBool::new(false));
        let is_active_clone = Arc::clone(&is_active);
        let mixer_clone = Arc::clone(&mixer);

        let stream = match config.sample_format() {
            SampleFormat::F32 => Self::build_stream::<f32>(
                &device,
                &config.into(),
                buffer_size,
                is_active_clone,
                mixer_clone,
            ),
            SampleFormat::I16 => Self::build_stream::<i16>(
                &device,
                &config.into(),
                buffer_size,
                is_active_clone,
                mixer_clone,
            ),
            SampleFormat::U16 => Self::build_stream::<u16>(
                &device,
                &config.into(),
                buffer_size,
                is_active_clone,
                mixer_clone,
            ),
            format => {
                return Err(format!("Unsupported sample format: {:?}", format));
            }
        }?;

        Ok(Self {
            stream: Some(stream),
            is_active,
        })
    }

    fn build_stream<T>(
        device: &Device,
        config: &StreamConfig,
        _buffer_size: u32,
        is_active: Arc<AtomicBool>,
        mixer: Arc<Mutex<&mut Mixer>>,
    ) -> Result<Stream, String>
    where
        T: cpal::Sample + cpal::SizedSample + Send + 'static,
    {
        let channels = config.channels as usize;

        let err_fn = |err| log::error!("Audio stream error: {}", err);

        let stream = device
            .build_output_stream(
                config,
                move |data: &mut [T], _: &OutputCallbackInfo| {
                    if !is_active.load(Ordering::SeqCst) {
                        return;
                    }
                    // Process mixer audio
                    let samples = data.len() / channels;
                    let mut buffer = vec![0.0f32; samples * 2]; // stereo interleaved

                    if let Ok(mut mixer) = mixer.lock() {
                        mixer.process(&mut buffer, 48000);
                    }

                    // Convert f32 to target format
                    for (i, sample) in buffer.iter().enumerate() {
                        if i < data.len() {
                            data[i] = cpal::Sample::from(sample);
                        }
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| format!("Failed to build output stream: {}", e))?;

        Ok(stream)
    }

    pub fn start(&self) -> Result<(), String> {
        if let Some(ref stream) = self.stream {
            stream
                .play()
                .map_err(|e| format!("Failed to start audio stream: {}", e))?;
            self.is_active.store(true, Ordering::SeqCst);
            log::info!("Audio output started");
        }
        Ok(())
    }

    pub fn stop(&self) {
        self.is_active.store(false, Ordering::SeqCst);
        // Give the callback time to see the inactive flag
        std::thread::sleep(std::time::Duration::from_millis(50));
        log::info!("Audio output stopped");
    }
}

impl Drop for AudioOutput {
    fn drop(&mut self) {
        self.stop();
    }
}
