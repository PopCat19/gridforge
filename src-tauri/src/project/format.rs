// format.rs
//
// Purpose: Project serialization format with versioned schema
//
// Defines core project structures for the Gridforge audio application.
// Supports channels, patterns, sequences, and audio-related metadata.

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_serialize_to_json() {
        let project = Project::new();
        let json = serde_json::to_string(&project).expect("Failed to serialize project");

        // Check that JSON contains expected fields
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"tempo\""));
        assert!(json.contains("\"time_signature\""));
        assert!(json.contains("\"channels\""));
        assert!(json.contains("\"patterns\""));
        assert!(json.contains("\"sequences\""));
    }

    #[test]
    fn test_project_deserialize_from_json() {
        let json = r#"{
            "version": "1.0.0",
            "name": "Test Project",
            "tempo": 140,
            "time_signature": { "numerator": 3, "denominator": 4 },
            "channels": [],
            "patterns": [],
            "sequences": []
        }"#;

        let project: Project = serde_json::from_str(json).expect("Failed to deserialize project");

        assert_eq!(project.version, "1.0.0");
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.tempo, 140);
        assert_eq!(project.time_signature.numerator, 3);
        assert_eq!(project.time_signature.denominator, 4);
    }

    #[test]
    fn test_default_project_structure() {
        let project = Project::default();

        // Check default values
        assert_eq!(project.version, PROJECT_VERSION);
        assert_eq!(project.name, "Untitled Project");
        assert_eq!(project.tempo, 120);
        assert_eq!(project.time_signature.numerator, 4);
        assert_eq!(project.time_signature.denominator, 4);
        assert!(project.channels.is_empty());
        assert!(project.patterns.is_empty());
        assert!(project.sequences.is_empty());
    }

    #[test]
    fn test_project_roundtrip() {
        let original = Project {
            version: "1.0.0".to_string(),
            name: "Roundtrip Test".to_string(),
            tempo: 180,
            time_signature: TimeSignature {
                numerator: 6,
                denominator: 8,
            },
            channels: vec![Channel::default()],
            patterns: vec![Pattern::default()],
            sequences: vec![Sequence::default()],
        };

        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let restored: Project = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(original.version, restored.version);
        assert_eq!(original.name, restored.name);
        assert_eq!(original.tempo, restored.tempo);
        assert_eq!(
            original.time_signature.numerator,
            restored.time_signature.numerator
        );
        assert_eq!(original.channels.len(), restored.channels.len());
        assert_eq!(original.patterns.len(), restored.patterns.len());
    }
}

pub const PROJECT_VERSION: &str = "1.0.0";

/// Channel type - determines how notes are synthesized
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelType {
    /// Pitch-based synthesis (square, sawtooth, pulse)
    Pitch,
    /// Noise-based synthesis (white noise, etc.)
    Noise,
}

impl Default for ChannelType {
    fn default() -> Self {
        ChannelType::Pitch
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub version: String,
    pub name: String,
    pub tempo: u16,
    pub time_signature: TimeSignature,
    pub channels: Vec<Channel>,
    pub patterns: Vec<Pattern>,
    pub sequences: Vec<Sequence>,
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

impl Project {
    pub fn new() -> Self {
        Self {
            version: PROJECT_VERSION.to_string(),
            name: "Untitled Project".to_string(),
            tempo: 120,
            time_signature: TimeSignature::default(),
            channels: Vec::new(),
            patterns: Vec::new(),
            sequences: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self {
            numerator: 4,
            denominator: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: u8,
    pub name: String,
    pub channel_type: ChannelType,
    pub volume: f32,
    pub pan: f32,
    pub mute: bool,
    pub solo: bool,
    pub instrument: Option<String>,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Channel 1".to_string(),
            channel_type: ChannelType::default(),
            volume: 1.0,
            pan: 0.0,
            mute: false,
            solo: false,
            instrument: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: u32,
    pub name: String,
    pub length: u16,
    pub grid: Vec<Vec<Note>>,
}

impl Default for Pattern {
    fn default() -> Self {
        Self::new()
    }
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "Pattern 1".to_string(),
            length: 16,
            grid: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Note {
    pub pitch: u8,
    pub velocity: u8,
    pub start: u16,
    pub duration: u16,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            pitch: 60,
            velocity: 100,
            start: 0,
            duration: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    pub id: u32,
    pub name: String,
    pub pattern_indices: Vec<u32>,
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

impl Sequence {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "Sequence 1".to_string(),
            pattern_indices: Vec::new(),
        }
    }
}
