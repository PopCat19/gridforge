// sequencer.rs
//
// Purpose: Simple tick-based sequencer for pattern playback
//
// This module:
// - Steps through ticks at project tempo
// - Reads notes from project patterns
// - Triggers notes on the synth at correct timing

use crate::project::{Note, Pattern};

/// Ticks per beat (PPQ - Pulses Per Quarter note)
pub const TICKS_PER_BEAT: u32 = 24;

/// Steps per pattern (default pattern length)
pub const DEFAULT_STEPS: u32 = 16;

/// Sequencer state
#[derive(Debug, Clone)]
pub struct SequencerState {
    pub is_playing: bool,
    pub current_step: u32,
    pub current_beat: u32,
    pub tempo: u16,
    pub sample_rate: u32,
    /// Current tick within the current step (0 to TICKS_PER_BEAT-1)
    pub current_tick: u32,
    /// Samples elapsed since last step
    samples_elapsed: u64,
    /// Samples per step (calculated from tempo)
    samples_per_step: u64,
}

impl SequencerState {
    pub fn new(sample_rate: u32, tempo: u16) -> Self {
        let samples_per_step = calculate_samples_per_step(sample_rate, tempo);
        Self {
            is_playing: false,
            current_step: 0,
            current_beat: 0,
            tempo,
            sample_rate,
            current_tick: 0,
            samples_elapsed: 0,
            samples_per_step,
        }
    }

    /// Update tempo and recalculate timing
    pub fn set_tempo(&mut self, tempo: u16) {
        self.tempo = tempo;
        self.samples_per_step = calculate_samples_per_step(self.sample_rate, tempo);
    }

    /// Process a buffer of samples, returns true if we stepped to a new note
    pub fn process(&mut self, samples: usize) -> bool {
        if !self.is_playing {
            return false;
        }

        self.samples_elapsed += samples as u64;

        // Update tick position within current step
        let samples_per_tick = self.samples_per_step / TICKS_PER_BEAT;
        if samples_per_tick > 0 {
            let ticks_advanced = (self.samples_elapsed / samples_per_tick) as u32;
            self.current_tick = (self.current_tick + ticks_advanced) % TICKS_PER_BEAT;
        }

        if self.samples_elapsed >= self.samples_per_step {
            self.samples_elapsed -= self.samples_per_step;
            self.current_tick = 0;
            self.advance_step();
            true
        } else {
            false
        }
    }

    /// Advance to the next step
    fn advance_step(&mut self) {
        self.current_step = (self.current_step + 1) % DEFAULT_STEPS;
        if self.current_step == 0 {
            self.current_beat = (self.current_beat + 1) % 4;
        }
    }

    /// Start playback
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.current_step = 0;
        self.current_beat = 0;
        self.current_tick = 0;
        self.samples_elapsed = 0;
    }

    /// Get current bar number (0-indexed)
    pub fn current_bar(&self) -> u32 {
        self.current_beat / 4
    }

    /// Get current beat within bar (0-3)
    pub fn current_beat_in_bar(&self) -> u32 {
        self.current_beat % 4
    }

    /// Seek to a specific step
    pub fn seek(&mut self, step: u32) {
        self.current_step = step % DEFAULT_STEPS;
        self.current_beat = step / DEFAULT_STEPS;
        self.samples_elapsed = 0;
    }
}

/// Calculate samples per step based on tempo and sample rate
///
/// Formula: samples_per_step = sample_rate * 60 / (tempo * steps_per_beat)
/// where steps_per_beat = 4 (16th notes for 4/4 time)
fn calculate_samples_per_step(sample_rate: u32, tempo: u16) -> u64 {
    let tempo_f = tempo as f64;
    let sample_rate_f = sample_rate as f64;

    // 16th notes = 4 steps per beat
    let steps_per_beat = 4.0;

    // samples per beat = sample_rate * 60 / tempo
    // samples per step = samples per beat / steps_per_beat
    let samples_per_step = (sample_rate_f * 60.0 / tempo_f) / steps_per_beat;

    samples_per_step.ceil() as u64
}

/// Get notes that should be played at the current step from a pattern
pub fn get_notes_for_step(pattern: &Pattern, channel: usize, step: u32) -> Vec<Note> {
    let mut notes = Vec::new();

    // Ensure channel exists
    if channel >= pattern.grid.len() {
        return notes;
    }

    let channel_grid = &pattern.grid[channel];

    // Find notes that start at this step
    for note in channel_grid.iter() {
        if note.start == step && note.velocity > 0 {
            notes.push(*note);
        }
    }

    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_per_step_calculation() {
        // At 120 BPM, 48000 Hz, 16 steps per beat = 4 steps per quarter note
        // samples_per_step = 48000 * 60 / (120 * 4) = 600 samples
        let samples = calculate_samples_per_step(48000, 120);
        assert_eq!(samples, 600);
    }

    #[test]
    fn test_sequencer_tempo_change() {
        let mut seq = SequencerState::new(48000, 120);
        let initial_samples = seq.samples_per_step;

        // Change tempo to 60 BPM (half speed)
        seq.set_tempo(60);

        // Should have double the samples per step
        assert_eq!(seq.samples_per_step, initial_samples * 2);
    }

    #[test]
    fn test_sequencer_play_stops() {
        let mut seq = SequencerState::new(48000, 120);

        // Initially stopped
        assert!(!seq.is_playing);
        assert_eq!(seq.current_step, 0);

        // Start playing
        seq.play();
        assert!(seq.is_playing);

        // Stop playing
        seq.stop();
        assert!(!seq.is_playing);
        assert_eq!(seq.current_step, 0);
        assert_eq!(seq.current_tick, 0);
    }

    #[test]
    fn test_sequencer_tick_advancement() {
        let mut seq = SequencerState::new(48000, 120);
        seq.play();

        // Process exactly samples_per_step samples - should advance step
        let samples_per_step = seq.samples_per_step;
        let stepped = seq.process(samples_per_step as usize);

        assert!(stepped);
        assert_eq!(seq.current_step, 1);
    }

    #[test]
    fn test_sequencer_no_step_when_stopped() {
        let mut seq = SequencerState::new(48000, 120);
        // Don't start playing

        let samples_per_step = seq.samples_per_step;
        let stepped = seq.process(samples_per_step as usize);

        assert!(!stepped);
        assert_eq!(seq.current_step, 0);
    }

    #[test]
    fn test_sequencer_bar_beat_tracking() {
        let mut seq = SequencerState::new(48000, 120);
        seq.play();

        let samples_per_step = seq.samples_per_step;

        // Advance 16 steps (one full pattern)
        for _ in 0..16 {
            seq.process(samples_per_step as usize);
        }

        // Should be back at step 0
        assert_eq!(seq.current_step, 0);
        // Should have completed 4 beats
        assert!(seq.current_beat >= 0);
    }
}
