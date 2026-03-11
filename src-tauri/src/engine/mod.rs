// engine/mod.rs
//
// Purpose: Audio engine core state and lifecycle management
//
// This module:
// - Manages shared audio engine state
// - Handles engine start/stop operations
// - Provides project loading interface

mod output;
mod sequencer;
mod mixer;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub use output::AudioOutput;
pub use sequencer::{SequencerState, DEFAULT_STEPS, get_notes_for_step, TICKS_PER_BEAT};
pub use mixer::{Mixer, MixerHandle, MAX_CHANNELS};

use crate::dto::{LevelsEvent, PlayheadEvent};
use crate::plugin::ChipSynth;
use crate::project::Project;
use mixer::Mixer;

pub const DEFAULT_SAMPLE_RATE: u32 = 48000;
pub const DEFAULT_BUFFER_SIZE: u32 = 256;

pub struct EngineState {
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub is_running: bool,
    pub output: Option<AudioOutput>,
    pub mixer: Mixer,
    pub sequencer: SequencerState,
    pub current_project: Option<Project>,
    pub active_pattern: usize,
    pub active_channel: usize,
    /// Last emitted step (for change detection)
    last_emitted_step: u32,
}

impl EngineState {
    pub fn new(sample_rate: u32, buffer_size: u32) -> Self {
        Self {
            sample_rate,
            buffer_size,
            is_running: false,
            output: None,
            mixer: Mixer::new(sample_rate, MAX_CHANNELS),
            sequencer: SequencerState::new(sample_rate, 120),
            current_project: None,
            active_pattern: 0,
            active_channel: 0,
            last_emitted_step: 0,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.is_running {
            return Ok(());
        }

        let mixer = Arc::new(Mutex::new(&mut self.mixer));
        let output = AudioOutput::new(self.sample_rate, self.buffer_size, mixer)?;
        output.start()?;

        self.output = Some(output);
        self.is_running = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if !self.is_running {
            return Ok(());
        }

        if let Some(output) = self.output.take() {
            output.stop();
        }

        self.is_running = false;
        Ok(())
    }

    pub fn load_project(&self, _path: &str) -> Result<(), String> {
        log::info!("Loading project from: {}", _path);
        Ok(())
    }

    /// Trigger a note preview (fire-and-forget, auto-releases after ~300ms)
    pub fn trigger_preview(&self, channel: u8, note: u8, velocity: u8) {
        if note > 127 {
            log::warn!("Invalid note {} for preview", note);
            return;
        }
        if velocity > 127 {
            log::warn!("Invalid velocity {} for preview", velocity);
            return;
        }

        let mixer = Arc::new(Mutex::new(&mut self.mixer));

        // Fire-and-forget: spawn thread to handle note on/off
        thread::spawn(move || {
            // Note on
            if let Ok(mut mixer) = mixer.lock() {
                mixer.note_on(channel as usize, note, velocity);
            }

            // Wait ~300ms then note off
            thread::sleep(Duration::from_millis(300));

            if let Ok(mut mixer) = mixer.lock() {
                mixer.note_off(channel as usize, note);
            }
        });
    }

    /// Transport play - start sequencer playback
    pub fn transport_play(&mut self) {
        self.sequencer.play();
        log::info!("Transport play");
    }

    /// Transport stop - stop sequencer playback
    pub fn transport_stop(&mut self) {
        self.sequencer.stop();
        // Also stop all notes on all channels
        self.mixer.all_notes_off();
        log::info!("Transport stop");
    }

    /// Transport set tempo
    pub fn transport_set_tempo(&mut self, tempo: u16) {
        self.sequencer.set_tempo(tempo);
        log::info!("Transport tempo set to: {}", tempo);
    }

    /// Get current sequencer state
    pub fn get_sequencer_state(&self) -> &SequencerState {
        &self.sequencer
    }

    /// Check if step changed and needs event emission
    pub fn poll_playhead_event(&mut self) -> Option<PlayheadEvent> {
        let current_step = self.sequencer.current_step;

        if current_step != self.last_emitted_step && self.sequencer.is_playing {
            self.last_emitted_step = current_step;

            Some(PlayheadEvent {
                bar: self.sequencer.current_bar(),
                beat: self.sequencer.current_beat_in_bar(),
                step: current_step,
                tick: self.sequencer.current_tick,
                is_playing: self.sequencer.is_playing,
            })
        } else {
            None
        }
    }

    /// Get current levels
    pub fn poll_levels_event(&mut self) -> LevelsEvent {
        let (channels, master) = self.mixer.get_levels();

        LevelsEvent {
            channels,
            master,
        }
    }
}

pub struct Engine {
    state: Arc<Mutex<EngineState>>,
    event_sender: std::sync::mpsc::Sender<EngineCommand>,
}

enum EngineCommand {
    Stop,
}

impl Engine {
    pub fn new(sample_rate: u32, buffer_size: u32) -> Self {
        let state = Arc::new(Mutex::new(EngineState::new(sample_rate, buffer_size)));
        let (event_sender, _) = std::sync::mpsc::channel();

        Self {
            state,
            event_sender,
        }
    }

    /// Start the event emission thread with a Tauri app handle
    pub fn start_event_thread(&self, app_handle: tauri::AppHandle) {
        let state = Arc::clone(&self.state);

        thread::spawn(move || {
            let mut last_levels_time = std::time::Instant::now();
            const LEVELS_INTERVAL_MS: u64 = 50;

            loop {
                // Check if we should stop
                if let Ok(state) = state.lock() {
                    if !state.is_running {
                        break;
                    }
                }

                // Poll and emit playhead event
                if let Ok(mut state) = state.lock() {
                    if let Some(playhead) = state.poll_playhead_event() {
                        let _ = app_handle.emit("playhead", playhead);
                    }
                }

                // Emit levels at regular intervals
                let now = std::time::Instant::now();
                if now.duration_since(last_levels_time).as_millis() as u64 >= LEVELS_INTERVAL_MS {
                    last_levels_time = now;
                    if let Ok(mut state) = state.lock() {
                        let levels = state.poll_levels_event();
                        let _ = app_handle.emit("levels", levels);
                    }
                }

                // Sleep a small amount to avoid busy-waiting
                thread::sleep(Duration::from_millis(10));
            }

            log::info!("Event emission thread stopped");
        });
    }

    pub fn start(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.start()
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.stop()
    }

    pub fn load_project(&self, path: &str) -> Result<(), String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        state.load_project(path)
    }

    pub fn is_running(&self) -> bool {
        self.state.lock().map(|s| s.is_running).unwrap_or(false)
    }

    pub fn state(&self) -> Arc<Mutex<EngineState>> {
        Arc::clone(&self.state)
    }

    /// Trigger a note preview (fire-and-forget, auto-releases after ~300ms)
    pub fn trigger_preview(&self, channel: u8, note: u8, velocity: u8) {
        let state = self.state.lock().map_err(|e| e.to_string()).ok();
        if let Some(state) = state {
            state.trigger_preview(channel, note, velocity);
        }
    }

    /// Transport play - start sequencer playback
    pub fn transport_play(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.transport_play();
        Ok(())
    }

    /// Transport stop - stop sequencer playback
    pub fn transport_stop(&self) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.transport_stop();
        Ok(())
    }

    /// Transport set tempo
    pub fn transport_set_tempo(&self, tempo: u16) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.transport_set_tempo(tempo);
        Ok(())
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if let Ok(mut state) = self.state.lock() {
            let _ = state.stop();
        }
    }
}
