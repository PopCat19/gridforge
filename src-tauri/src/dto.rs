// dto.rs
//
// Purpose: Backend DTOs for Tauri command payloads
//
// Request/response structs for project commands.

use serde::{Deserialize, Serialize};

use crate::project::{Note, Pattern, Project};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNewRequest {
    pub name: Option<String>,
}

impl Default for ProjectNewRequest {
    fn default() -> Self {
        Self { name: None }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectNewResponse {
    pub project: Project,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSaveRequest {
    pub project: Project,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSaveResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectLoadRequest {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectLoadResponse {
    pub project: Project,
}

// Pattern Update DTOs

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteDto {
    pub pitch: u8,
    pub velocity: u8,
    pub start: u16,
    pub duration: u16,
}

impl From<Note> for NoteDto {
    fn from(note: Note) -> Self {
        Self {
            pitch: note.pitch,
            velocity: note.velocity,
            start: note.start,
            duration: note.duration,
        }
    }
}

impl From<NoteDto> for Note {
    fn from(dto: NoteDto) -> Self {
        Self {
            pitch: dto.pitch,
            velocity: dto.velocity,
            start: dto.start,
            duration: dto.duration,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternUpdateRequest {
    pub channel: usize,
    pub pattern_index: usize,
    pub cell_row: usize,
    pub cell_col: usize,
    pub note: Option<NoteDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternUpdateResponse {
    pub success: bool,
}

// Transport DTOs

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportPlayRequest {
    pub position: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportPlayResponse {
    pub playing: bool,
    pub position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportStopResponse {
    pub playing: bool,
    pub position: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportSetTempoRequest {
    pub tempo: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportSetTempoResponse {
    pub tempo: u16,
}

// Event Payloads for frontend synchronization

/// Playhead position event - emitted on each step change during playback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayheadEvent {
    /// Current bar number (0-indexed)
    pub bar: u32,
    /// Current beat within the bar (0-3)
    pub beat: u32,
    /// Current step within the beat (0-15, 16th notes)
    pub step: u32,
    /// Current tick within the step (0-23)
    pub tick: u32,
    /// Whether playback is active
    pub is_playing: bool,
}

/// Audio levels event - peak levels per channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelsEvent {
    /// Peak level for each channel (0.0 to 1.0+)
    pub channels: Vec<f32>,
    /// Master peak level
    pub master: f32,
}

// Source Registry DTOs

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceInfo {
    pub id: String,
    pub name: String,
    pub category: String,
}

impl From<crate::plugin::SourceDescriptor> for SourceInfo {
    fn from(desc: crate::plugin::SourceDescriptor) -> Self {
        Self {
            id: desc.id,
            name: desc.name,
            category: desc.category.as_str().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSourcesResponse {
    pub sources: Vec<SourceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelSetSourceRequest {
    pub channel: usize,
    pub source_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelSetSourceResponse {
    pub channel: usize,
    pub source_id: String,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSourceParamsRequest {
    pub source_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamInfo {
    pub id: String,
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub default: f32,
    pub unit: String,
}

impl From<crate::plugin::ParamDescriptor> for ParamInfo {
    fn from(param: crate::plugin::ParamDescriptor) -> Self {
        Self {
            id: param.id.0,
            name: param.name,
            min: param.min,
            max: param.max,
            default: param.default,
            unit: param.unit,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSourceParamsResponse {
    pub source_id: String,
    pub params: Vec<ParamInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSourceParamRequest {
    pub channel: usize,
    pub param_id: String,
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSourceParamResponse {
    pub channel: usize,
    pub param_id: String,
    pub value: f32,
    pub success: bool,
}
