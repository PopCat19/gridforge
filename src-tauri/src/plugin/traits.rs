// plugin/traits.rs
//
// Purpose: Audio plugin trait definitions
//
// This module:
// - Defines SoundSource trait for audio generators
// - Provides parameter descriptor for runtime parameter access

use std::any::Any;

/// MIDI note number (0-127)
pub type Note = u8;

/// MIDI velocity (0-127)
pub type Velocity = u8;

/// Parameter identifier
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParamId(pub String);

impl ParamId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

/// Parameter value type
#[derive(Clone, Debug, PartialEq)]
pub enum ParamValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    String(String),
}

impl ParamValue {
    pub fn as_float(&self) -> Option<f32> {
        match self {
            ParamValue::Float(v) => Some(*v),
            ParamValue::Int(v) => Some(*v as f32),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            ParamValue::Int(v) => Some(*v),
            ParamValue::Float(v) => Some(*v as i32),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ParamValue::Bool(v) => Some(*v),
            _ => None,
        }
    }
}

/// Parameter metadata descriptor
#[derive(Clone, Debug)]
pub struct ParamDescriptor {
    pub id: ParamId,
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub default: f32,
    pub unit: String,
}

impl ParamDescriptor {
    pub fn new(id: ParamId, name: &str, min: f32, max: f32, default: f32, unit: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            min,
            max,
            default,
            unit: unit.to_string(),
        }
    }
}

/// Source category for classification
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SourceCategory {
    Synth,
    Sampler,
    Effect,
    Drum,
    Other,
}

impl SourceCategory {
    pub fn as_str(&self) -> &str {
        match self {
            SourceCategory::Synth => "synth",
            SourceCategory::Sampler => "sampler",
            SourceCategory::Effect => "effect",
            SourceCategory::Drum => "drum",
            SourceCategory::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "synth" => SourceCategory::Synth,
            "sampler" => SourceCategory::Sampler,
            "effect" => SourceCategory::Effect,
            "drum" => SourceCategory::Drum,
            _ => SourceCategory::Other,
        }
    }
}

/// Source descriptor - metadata for dynamic source discovery
#[derive(Clone, Debug)]
pub struct SourceDescriptor {
    pub id: String,
    pub name: String,
    pub category: SourceCategory,
    pub params: Vec<ParamDescriptor>,
}

impl SourceDescriptor {
    pub fn new(id: &str, name: &str, category: SourceCategory) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category,
            params: Vec::new(),
        }
    }

    pub fn with_params(mut self, params: Vec<ParamDescriptor>) -> Self {
        self.params = params;
        self
    }
}

/// Audio buffer type - interleaved stereo f32 samples
pub type AudioBuffer = Vec<f32>;

/// Core trait for audio sources (instruments, effects)
///
/// Minimal and backend-focused: implementors provide sample data
/// via process(), handle note events, and expose runtime parameters.
pub trait SoundSource: Send + Sync {
    /// Fill buffer with audio samples
    ///
    /// Buffer layout: [L0, R0, L1, R1, L2, R2, ...] for stereo
    /// Buffer layout: [L0, L1, L2, ...] for mono (if supported)
    ///
    /// # Arguments
    /// * `buffer` - Mutable slice to fill with samples
    /// * `sample_rate` - Current sample rate for time-based calculations
    fn process(&mut self, buffer: &mut [f32], sample_rate: u32);

    /// Start a note at given pitch and velocity
    fn note_on(&mut self, note: Note, velocity: Velocity);

    /// Stop a note
    fn note_off(&mut self, note: Note);

    /// Set a parameter value by ID
    fn set_param(&mut self, param_id: &ParamId, value: ParamValue);

    /// Get parameter descriptor list
    fn params(&self) -> Vec<ParamDescriptor>;

    /// Get source name for identification
    fn name(&self) -> &str;

    /// Get source category for classification
    fn category(&self) -> SourceCategory {
        SourceCategory::Other
    }

    /// Optional: Get parameter current value
    fn get_param(&self, _param_id: &ParamId) -> Option<ParamValue> {
        None
    }

    /// Optional: Stop all active notes
    fn all_notes_off(&mut self) {
        // Default: do nothing, override in implementation
    }

    /// Optional: Downcast for specific type access
    fn as_any(&self) -> &dyn Any {
        unimplemented!("as_any not implemented")
    }
}

/// Convert MIDI note to frequency in Hz
///
/// A4 (MIDI note 69) = 440 Hz
/// Formula: f = 440 * 2^((n-69)/12)
pub fn note_to_freq(note: Note) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}

/// Convert frequency to MIDI note (with detune)
pub fn freq_to_note(freq: f32) -> (Note, f32) {
    let note = (69.0 + 12.0 * (freq / 440.0).log2()).round() as i32;
    let note = note.clamp(0, 127) as Note;
    let exact_freq = note_to_freq(note);
    let detune = 1200.0 * (freq / exact_freq).log2();
    (note, detune)
}
