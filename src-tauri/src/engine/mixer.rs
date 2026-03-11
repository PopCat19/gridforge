// mixer.rs
//
// Purpose: Multi-channel audio mixer with per-channel volume and mute/solo
//
// This module:
// - Mixes multiple channel synths to stereo output
// - Handles per-channel volume and pan
// - Supports mute/solo functionality
// - Stubs noise synthesis for future implementation

use crate::plugin::chip_synth::{ChipSynth, Waveform};
use crate::plugin::traits::{Note, SoundSource, Velocity, ParamId, ParamValue, ParamDescriptor};
use crate::project::ChannelType;

/// Maximum number of channels supported
pub const MAX_CHANNELS: usize = 4;

/// Single channel in the mixer
pub struct MixerChannel {
    /// The synth for this channel
    pub synth: ChipSynth,
    /// Channel volume (0.0 to 1.0+)
    pub volume: f32,
    /// Pan (-1.0 = left, 0.0 = center, 1.0 = right)
    pub pan: f32,
    /// Mute state
    pub mute: bool,
    /// Solo state - if any channel is soloed, only soloed channels are heard
    pub solo: bool,
    /// Channel type (pitch or noise)
    pub channel_type: ChannelType,
}

impl MixerChannel {
    pub fn new(sample_rate: u32, channel_type: ChannelType) -> Self {
        let waveform = match channel_type {
            ChannelType::Pitch => Waveform::Square,
            ChannelType::Noise => Waveform::Noise,
        };

        let mut synth = ChipSynth::with_name(
            &format!("Channel_{}", channel_type.as_str()),
            sample_rate,
        );
        synth.set_param(&ParamId::new("waveform"), ParamValue::Int(waveform.to_index()));

        Self {
            synth,
            volume: 1.0,
            pan: 0.0,
            mute: false,
            solo: false,
            channel_type,
        }
    }

    /// Check if this channel should be heard
    pub fn is_active(&self, any_solo: bool) -> bool {
        if self.mute {
            return false;
        }
        if any_solo && !self.solo {
            return false;
        }
        true
    }

    /// Get the effective volume (considering mute/solo)
    pub fn effective_volume(&self, any_solo: bool) -> f32 {
        if !self.is_active(any_solo) {
            return 0.0;
        }
        self.volume
    }
}

impl ChannelType {
    pub fn as_str(&self) -> &str {
        match self {
            ChannelType::Pitch => "pitch",
            ChannelType::Noise => "noise",
        }
    }
}

/// Multi-channel mixer
pub struct Mixer {
    /// Individual channel mixers
    pub channels: Vec<MixerChannel>,
    /// Master volume
    pub master_volume: f32,
    /// Sample rate
    sample_rate: u32,
    /// Peak level for each channel (0.0 to 1.0)
    channel_peaks: Vec<f32>,
    /// Master peak level
    master_peak: f32,
}

impl Mixer {
    pub fn new(sample_rate: u32, num_channels: usize) -> Self {
        let num_channels = num_channels.min(MAX_CHANNELS);
        let channels = (0..num_channels)
            .map(|i| {
                let channel_type = if i == 3 {
                    ChannelType::Noise
                } else {
                    ChannelType::Pitch
                };
                MixerChannel::new(sample_rate, channel_type)
            })
            .collect();

        Self {
            channels,
            master_volume: 1.0,
            sample_rate,
            channel_peaks: vec![0.0; num_channels],
            master_peak: 0.0,
        }
    }

    /// Process audio - mix all channels to stereo output
    pub fn process(&mut self, buffer: &mut [f32], sample_rate: u32) {
        // Check if any channel is soloed
        let any_solo = self.channels.iter().any(|ch| ch.solo);

        // Generate temporary buffers for each channel
        let mut channel_buffers: Vec<Vec<f32>> = self
            .channels
            .iter_mut()
            .map(|ch| vec![0.0f32; buffer.len()])
            .collect();

        // Process each channel
        for (i, channel) in self.channels.iter_mut().enumerate() {
            let effective_vol = channel.effective_volume(any_solo);
            if effective_vol > 0.0 {
                channel.synth.process(&mut channel_buffers[i], sample_rate);
            }
        }

        // Mix channels together with volume and pan
        let samples = buffer.len() / 2; // stereo
        let mut max_left = 0.0f32;
        let mut max_right = 0.0f32;

        for i in 0..samples {
            let mut left = 0.0f32;
            let mut right = 0.0f32;

            for (ch_idx, channel) in self.channels.iter().enumerate() {
                let effective_vol = channel.effective_volume(any_solo);
                if effective_vol > 0.0 {
                    // Get sample from channel buffer
                    let sample = channel_buffers[ch_idx][i * 2];

                    // Apply pan
                    // pan: -1 (left) to 1 (right)
                    // left gain = (1 - pan) / 2 for full left, 0 for full right
                    // right gain = (1 + pan) / 2 for full right, 0 for full left
                    let pan = channel.pan.clamp(-1.0, 1.0);
                    let left_gain = ((1.0 - pan) / 2.0).sqrt();
                    let right_gain = ((1.0 + pan) / 2.0).sqrt();

                    left += sample * effective_vol * left_gain;
                    right += sample * effective_vol * right_gain;

                    // Track channel peaks (mono mix of left/right)
                    let mono_sample = (sample * effective_vol).abs();
                    self.channel_peaks[ch_idx] = self.channel_peaks[ch_idx].max(mono_sample);
                }
            }

            // Apply master volume
            left *= self.master_volume;
            right *= self.master_volume;

            // Track master peaks
            max_left = max_left.max(left.abs());
            max_right = max_right.max(right.abs());

            // Soft clip for pleasant distortion
            buffer[i * 2] = left.tanh();
            buffer[i * 2 + 1] = right.tanh();
        }

        // Update master peak (average of L/R)
        self.master_peak = (max_left + max_right) / 2.0;
    }

    /// Trigger a note on a specific channel
    pub fn note_on(&mut self, channel: usize, note: Note, velocity: Velocity) {
        if channel < self.channels.len() {
            self.channels[channel].synth.note_on(note, velocity);
        }
    }

    /// Stop a note on a specific channel
    pub fn note_off(&mut self, channel: usize, note: Note) {
        if channel < self.channels.len() {
            self.channels[channel].synth.note_off(note);
        }
    }

    /// Stop all notes on all channels
    pub fn all_notes_off(&mut self) {
        for channel in &mut self.channels {
            channel.synth.all_notes_off();
        }
    }

    /// Set volume for a specific channel
    pub fn set_channel_volume(&mut self, channel: usize, volume: f32) {
        if channel < self.channels.len() {
            self.channels[channel].volume = volume.clamp(0.0, 2.0);
        }
    }

    /// Set pan for a specific channel
    pub fn set_channel_pan(&mut self, channel: usize, pan: f32) {
        if channel < self.channels.len() {
            self.channels[channel].pan = pan.clamp(-1.0, 1.0);
        }
    }

    /// Set mute state for a specific channel
    pub fn set_channel_mute(&mut self, channel: usize, mute: bool) {
        if channel < self.channels.len() {
            self.channels[channel].mute = mute;
        }
    }

    /// Set solo state for a specific channel
    pub fn set_channel_solo(&mut self, channel: usize, solo: bool) {
        if channel < self.channels.len() {
            self.channels[channel].solo = solo;
        }
    }

    /// Get the number of channels
    pub fn num_channels(&self) -> usize {
        self.channels.len()
    }

    /// Get channel info
    pub fn get_channel(&self, index: usize) -> Option<&MixerChannel> {
        self.channels.get(index)
    }

    /// Get current peak levels and decay the internal peaks
    /// Returns (channel_peaks, master_peak)
    pub fn get_levels(&mut self) -> (Vec<f32>, f32) {
        let channels = self.channel_peaks.clone();
        let master = self.master_peak;

        // Decay peaks for next frame (smooth decay)
        const DECAY_RATE: f32 = 0.9;
        for peak in &mut self.channel_peaks {
            *peak *= DECAY_RATE;
        }
        self.master_peak *= DECAY_RATE;

        (channels, master)
    }
}

impl SoundSource for Mixer {
    fn process(&mut self, buffer: &mut [f32], sample_rate: u32) {
        Mixer::process(self, buffer, sample_rate);
    }

    fn note_on(&mut self, note: Note, velocity: Velocity) {
        // Route to first channel by default for backwards compatibility
        Mixer::note_on(self, 0, note, velocity);
    }

    fn note_off(&mut self, note: Note) {
        Mixer::note_off(self, 0, note);
    }

    fn set_param(&mut self, param_id: &ParamId, value: ParamValue) {
        // Forward to first channel
        if let Some(channel) = self.channels.get_mut(0) {
            channel.synth.set_param(param_id, value);
        }
    }

    fn params(&self) -> Vec<ParamDescriptor> {
        if let Some(channel) = self.channels.get(0) {
            channel.synth.params()
        } else {
            Vec::new()
        }
    }

    fn name(&self) -> &str {
        "Mixer"
    }

    fn get_param(&self, param_id: &ParamId) -> Option<ParamValue> {
        if let Some(channel) = self.channels.get(0) {
            channel.synth.get_param(param_id)
        } else {
            None
        }
    }

    fn all_notes_off(&mut self) {
        Mixer::all_notes_off(self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Thread-safe wrapper for Mixer (for use with audio callbacks)
pub struct MixerHandle {
    mixer: std::sync::Arc<std::sync::Mutex<Mixer>>,
}

impl MixerHandle {
    pub fn new(sample_rate: u32, num_channels: usize) -> Self {
        Self {
            mixer: std::sync::Arc::new(std::sync::Mutex::new(Mixer::new(sample_rate, num_channels))),
        }
    }

    pub fn inner(&self) -> std::sync::Arc<std::sync::Mutex<Mixer>> {
        std::sync::Arc::clone(&self.mixer)
    }

    pub fn process(&self, buffer: &mut [f32], sample_rate: u32) {
        if let Ok(mut mixer) = self.mixer.lock() {
            mixer.process(buffer, sample_rate);
        }
    }

    pub fn note_on(&self, channel: usize, note: Note, velocity: Velocity) {
        if let Ok(mut mixer) = self.mixer.lock() {
            mixer.note_on(channel, note, velocity);
        }
    }

    pub fn note_off(&self, channel: usize, note: Note) {
        if let Ok(mut mixer) = self.mixer.lock() {
            mixer.note_off(channel, note);
        }
    }

    pub fn all_notes_off(&self) {
        if let Ok(mut mixer) = self.mixer.lock() {
            mixer.all_notes_off();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixer_creation() {
        let mixer = Mixer::new(48000, 4);
        assert_eq!(mixer.num_channels(), 4);
    }

    #[test]
    fn test_mixer_mute() {
        let mut mixer = Mixer::new(48000, 2);
        mixer.note_on(0, 60, 100);

        // Mute channel 0
        mixer.set_channel_mute(0, true);

        let any_solo = false;
        assert!(!mixer.channels[0].is_active(any_solo));
    }

    #[test]
    fn test_mixer_solo() {
        let mut mixer = Mixer::new(48000, 2);
        mixer.set_channel_solo(0, true);

        let any_solo = true;
        assert!(mixer.channels[0].is_active(any_solo));
        assert!(!mixer.channels[1].is_active(any_solo));
    }

    #[test]
    fn test_channel_type_str() {
        assert_eq!(ChannelType::Pitch.as_str(), "pitch");
        assert_eq!(ChannelType::Noise.as_str(), "noise");
    }
}
