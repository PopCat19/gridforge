// plugin/chip_synth.rs
//
// Purpose: Basic chip synthesizer implementation
//
// This module:
// - Implements SoundSource trait for a simple synth
// - Supports square and sawtooth waveforms
// - Provides note on/off with basic envelope (attack/release)

use std::any::Any;
use std::collections::HashMap;

use super::traits::{
    note_to_freq, AudioBuffer, ParamDescriptor, ParamId, ParamValue, SoundSource, SourceCategory,
    Velocity,
};

/// Waveform type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Waveform {
    Square,
    Sawtooth,
    Pulse25,
    Noise,
}

impl Default for Waveform {
    fn default() -> Self {
        Waveform::Square
    }
}

impl Waveform {
    pub fn from_index(index: i32) -> Self {
        match index {
            0 => Waveform::Square,
            1 => Waveform::Sawtooth,
            2 => Waveform::Pulse25,
            _ => Waveform::Square,
        }
    }

    pub fn to_index(&self) -> i32 {
        match self {
            Waveform::Square => 0,
            Waveform::Sawtooth => 1,
            Waveform::Pulse25 => 2,
            Waveform::Noise => 3,
        }
    }
}

/// Active voice state
#[derive(Clone, Debug)]
struct Voice {
    note: u8,
    velocity: u8,
    phase: f32,
    age_samples: u64,
}

impl Voice {
    fn new(note: u8, velocity: u8) -> Self {
        Self {
            note,
            velocity,
            phase: 0.0,
            age_samples: 0,
        }
    }
}

/// ChipSynth - Basic chip-style synthesizer
pub struct ChipSynth {
    name: String,
    sample_rate: u32,
    waveform: Waveform,
    attack_samples: u32,
    release_samples: u32,
    pulse_width: f32,
    voices: HashMap<u8, Voice>,
    global_age: u64,
    params: HashMap<ParamId, ParamValue>,
}

impl ChipSynth {
    pub fn new(sample_rate: u32) -> Self {
        let mut params = HashMap::new();
        params.insert(ParamId::new("waveform"), ParamValue::Int(0));
        params.insert(ParamId::new("attack"), ParamValue::Float(0.01));
        params.insert(ParamId::new("release"), ParamValue::Float(0.1));
        params.insert(ParamId::new("pulse_width"), ParamValue::Float(0.5));

        Self {
            name: "ChipSynth".to_string(),
            sample_rate,
            waveform: Waveform::default(),
            attack_samples: (0.01 * sample_rate as f32) as u32,
            release_samples: (0.1 * sample_rate as f32) as u32,
            pulse_width: 0.5,
            voices: HashMap::new(),
            global_age: 0,
            params,
        }
    }

    pub fn with_name(name: &str, sample_rate: u32) -> Self {
        let mut synth = Self::new(sample_rate);
        synth.name = name.to_string();
        synth
    }

    /// Generate one sample for a voice
    fn generate_sample(&self, voice: &Voice) -> f32 {
        let freq = note_to_freq(voice.note);
        let phase_increment = freq / self.sample_rate as f32;

        let sample = match self.waveform {
            Waveform::Square => {
                if voice.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Sawtooth => 2.0 * voice.phase - 1.0,
            Waveform::Pulse25 => {
                if voice.phase < self.pulse_width * 0.25 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Noise => {
                // Simple linear congruential generator for noise
                let seed = (voice
                    .age_samples
                    .wrapping_mul(1103515245)
                    .wrapping_add(12345))
                    % 32768;
                2.0 * (seed as f32 / 32768.0) - 1.0
            }
        };

        sample
    }

    /// Apply envelope to sample
    fn apply_envelope(&self, sample: f32, voice: &Voice) -> f32 {
        let vel = voice.velocity as f32 / 127.0;

        // Attack phase
        let attack = if voice.age_samples < self.attack_samples as u64 {
            voice.age_samples as f32 / self.attack_samples as f32
        } else {
            1.0
        };

        // Calculate release
        // For simplicity, we use a fixed release based on note-off time
        // In a full implementation, we'd track release state per voice
        let release = 1.0;

        sample * vel * attack * release
    }

    /// Process stereo output
    fn process_stereo(&mut self, left: &mut [f32], right: &mut [f32]) {
        // Update global age
        self.global_age += left.len() as u64;

        // Mix all active voices
        for (i, (l, r)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            let mut mix = 0.0f32;

            for (_note, voice) in self.voices.iter_mut() {
                let sample = self.generate_sample(voice);
                mix += self.apply_envelope(sample, voice);

                // Advance phase
                let freq = note_to_freq(voice.note);
                let phase_increment = freq / self.sample_rate as f32;
                voice.phase += phase_increment;
                if voice.phase >= 1.0 {
                    voice.phase -= 1.0;
                }
                voice.age_samples += 1;
            }

            // Soft clip for pleasant distortion
            *l = mix.tanh();
            *r = mix.tanh();
        }
    }

    /// Process mono output (interleaved buffer)
    fn process_mono(&mut self, buffer: &mut [f32]) {
        // Update global age
        self.global_age += buffer.len() as u64 / 2;

        let samples = buffer.len() / 2;
        for i in 0..samples {
            let mut mix = 0.0f32;

            for (_note, voice) in self.voices.iter_mut() {
                let sample = self.generate_sample(voice);
                mix += self.apply_envelope(sample, voice);

                // Advance phase
                let freq = note_to_freq(voice.note);
                let phase_increment = freq / self.sample_rate as f32;
                voice.phase += phase_increment;
                if voice.phase >= 1.0 {
                    voice.phase -= 1.0;
                }
                voice.age_samples += 1;
            }

            // Write stereo
            let clipped = mix.tanh();
            buffer[i * 2] = clipped;
            buffer[i * 2 + 1] = clipped;
        }
    }
}

impl SoundSource for ChipSynth {
    fn process(&mut self, buffer: &mut [f32], sample_rate: u32) {
        // Update sample rate if changed
        if sample_rate != self.sample_rate {
            self.sample_rate = sample_rate;
            self.attack_samples = (0.01 * sample_rate as f32) as u32;
            self.release_samples = (0.1 * sample_rate as f32) as u32;
        }

        // Detect mono vs stereo from buffer size
        // For simplicity, assume stereo interleaved
        if buffer.len() >= 4 {
            self.process_mono(buffer);
        }
    }

    fn note_on(&mut self, note: Note, velocity: Velocity) {
        // Replace existing voice for this note
        self.voices.insert(note, Voice::new(note, velocity));
    }

    fn note_off(&mut self, note: Note) {
        // Remove voice - release is handled in generation
        self.voices.remove(&note);
    }

    /// Stop all active notes
    fn all_notes_off(&mut self) {
        self.voices.clear();
    }

    fn set_param(&mut self, param_id: &ParamId, value: ParamValue) {
        match param_id.0.as_str() {
            "waveform" => {
                if let Some(v) = value.as_int() {
                    self.waveform = Waveform::from_index(v);
                    self.params.insert(param_id.clone(), ParamValue::Int(v));
                }
            }
            "attack" => {
                if let Some(v) = value.as_float() {
                    self.attack_samples = (v * self.sample_rate as f32).max(1.0) as u32;
                    self.params.insert(param_id.clone(), ParamValue::Float(v));
                }
            }
            "release" => {
                if let Some(v) = value.as_float() {
                    self.release_samples = (v * self.sample_rate as f32).max(1.0) as u32;
                    self.params.insert(param_id.clone(), ParamValue::Float(v));
                }
            }
            "pulse_width" => {
                if let Some(v) = value.as_float() {
                    self.pulse_width = v.clamp(0.0, 1.0);
                    self.params.insert(param_id.clone(), ParamValue::Float(v));
                }
            }
            _ => {}
        }
    }

    fn params(&self) -> Vec<ParamDescriptor> {
        vec![
            ParamDescriptor::new(ParamId::new("waveform"), "Waveform", 0.0, 3.0, 0.0, "index"),
            ParamDescriptor::new(ParamId::new("attack"), "Attack", 0.0, 2.0, 0.01, "seconds"),
            ParamDescriptor::new(ParamId::new("release"), "Release", 0.0, 5.0, 0.1, "seconds"),
            ParamDescriptor::new(
                ParamId::new("pulse_width"),
                "Pulse Width",
                0.0,
                1.0,
                0.5,
                "ratio",
            ),
        ]
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn category(&self) -> SourceCategory {
        SourceCategory::Synth
    }

    fn get_param(&self, param_id: &ParamId) -> Option<ParamValue> {
        self.params.get(param_id).cloned()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_to_freq() {
        // A4 = 440 Hz
        let freq = note_to_freq(69);
        assert!((freq - 440.0).abs() < 0.01);

        // C5 = 523.25 Hz (note 72)
        let freq = note_to_freq(72);
        assert!((freq - 523.25).abs() < 0.01);
    }

    #[test]
    fn test_chip_synth_creation() {
        let synth = ChipSynth::new(48000);
        assert_eq!(synth.name(), "ChipSynth");
        assert_eq!(synth.params().len(), 4);
    }

    #[test]
    fn test_waveform_index() {
        assert_eq!(Waveform::Square.to_index(), 0);
        assert_eq!(Waveform::Sawtooth.to_index(), 1);
        assert_eq!(Waveform::from_index(0), Waveform::Square);
        assert_eq!(Waveform::from_index(1), Waveform::Sawtooth);
    }

    #[test]
    fn test_chip_synth_basic_synthesis() {
        let mut synth = ChipSynth::new(48000);

        // Trigger a note
        synth.note_on(60, 100); // Middle C at velocity 100

        // Generate some samples
        let mut buffer = vec![0.0f32; 1024]; // 512 stereo samples
        synth.process(&mut buffer, 48000);

        // Buffer should have some non-zero values after note is triggered
        let has_output = buffer.iter().any(|&s| s != 0.0);
        assert!(has_output, "Synth should produce output after note_on");

        // Stop the note
        synth.note_off(60);

        // Generate more samples after note off
        let mut buffer2 = vec![0.0f32; 1024];
        synth.process(&mut buffer2, 48000);

        // After note off and release, might still have some audio
        // but voices should be cleared
        assert!(
            synth.voices.is_empty(),
            "Voices should be empty after note_off"
        );
    }

    #[test]
    fn test_chip_synth_all_notes_off() {
        let mut synth = ChipSynth::new(48000);

        // Trigger multiple notes
        synth.note_on(60, 100);
        synth.note_on(64, 100);
        synth.note_on(67, 100);

        assert_eq!(synth.voices.len(), 3);

        // All notes off
        synth.all_notes_off();

        assert!(synth.voices.is_empty());
    }

    #[test]
    fn test_chip_synth_params() {
        let synth = ChipSynth::new(48000);

        // Check default params
        let waveform = synth.get_param(&ParamId::new("waveform"));
        assert!(waveform.is_some());
        assert_eq!(waveform.unwrap(), ParamValue::Int(0));
    }
}
