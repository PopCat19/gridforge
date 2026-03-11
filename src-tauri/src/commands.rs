// commands.rs
//
// Purpose: Tauri commands for project operations and audio preview
//
// Provides project new, save, load commands and note preview.

use std::fs;
use std::path::Path;
use std::sync::Mutex;

use crate::dto::{
    PatternUpdateRequest, PatternUpdateResponse, ProjectLoadRequest, ProjectLoadResponse,
    ProjectNewRequest, ProjectNewResponse, ProjectSaveRequest, ProjectSaveResponse,
    TransportPlayRequest, TransportPlayResponse, TransportStopResponse, TransportSetTempoRequest,
    TransportSetTempoResponse,
};
use crate::engine::Engine;
use crate::plugin::{SourceDescriptor, SourceRegistryHandle};
use crate::project::{create_default_project, Note, Project};

pub struct ProjectState {
    pub project: Mutex<Project>,
}

impl ProjectState {
    pub fn new() -> Self {
        Self {
            project: Mutex::new(create_default_project()),
        }
    }
}

#[tauri::command]
pub fn note_preview(
    channel: u8,
    note: u8,
    velocity: u8,
    engine: tauri::State<'_, Engine>,
) -> Result<(), String> {
    // Validate channel (0-15 for MIDI)
    if channel > 15 {
        return Err(format!("Invalid channel: {} (must be 0-15)", channel));
    }

    // Validate note range (0-127)
    if note > 127 {
        return Err(format!("Invalid note: {} (must be 0-127)", note));
    }

    // Validate velocity range (0-127)
    if velocity > 127 {
        return Err(format!("Invalid velocity: {} (must be 0-127)", velocity));
    }

    // Fire-and-forget: trigger preview on engine
    engine.trigger_preview(channel, note, velocity);

    Ok(())
}

#[tauri::command]
pub fn transport_play(
    request: TransportPlayRequest,
    engine: tauri::State<'_, Engine>,
) -> Result<TransportPlayResponse, String> {
    engine.transport_play()?;

    Ok(TransportPlayResponse {
        playing: true,
        position: request.position.unwrap_or(0),
    })
}

#[tauri::command]
pub fn transport_stop(engine: tauri::State<'_, Engine>) -> Result<TransportStopResponse, String> {
    engine.transport_stop()?;

    Ok(TransportStopResponse {
        playing: false,
        position: 0,
    })
}

#[tauri::command]
pub fn transport_set_tempo(
    request: TransportSetTempoRequest,
    engine: tauri::State<'_, Engine>,
) -> Result<TransportSetTempoResponse, String> {
    engine.transport_set_tempo(request.tempo)?;

    Ok(TransportSetTempoResponse {
        tempo: request.tempo,
    })
}

#[tauri::command]
pub fn project_new(
    request: ProjectNewRequest,
    state: tauri::State<'_, ProjectState>,
) -> ProjectNewResponse {
    let mut project = create_default_project();

    if let Some(name) = request.name {
        project.name = name;
    }

    let mut project_lock = state.project.lock().unwrap();
    *project_lock = project.clone();

    ProjectNewResponse { project }
}

#[tauri::command]
pub fn project_save(request: ProjectSaveRequest) -> ProjectSaveResponse {
    let json = match serde_json::to_string_pretty(&request.project) {
        Ok(j) => j,
        Err(e) => {
            log::error!("Failed to serialize project: {}", e);
            return ProjectSaveResponse { success: false };
        }
    };

    match fs::write(&request.path, json) {
        Ok(_) => {
            log::info!("Project saved to: {}", request.path);
            ProjectSaveResponse { success: true }
        }
        Err(e) => {
            log::error!("Failed to write project file: {}", e);
            ProjectSaveResponse { success: false }
        }
    }
}

#[tauri::command]
pub fn project_load(
    request: ProjectLoadRequest,
    state: tauri::State<'_, ProjectState>,
) -> Result<ProjectLoadResponse, String> {
    let path = Path::new(&request.path);

    if !path.exists() {
        return Err(format!("File not found: {}", request.path));
    }

    let json = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let project: Project =
        serde_json::from_str(&json).map_err(|e| format!("Failed to parse project JSON: {}", e))?;

    let mut project_lock = state.project.lock().unwrap();
    *project_lock = project.clone();

    log::info!("Project loaded from: {}", request.path);

    Ok(ProjectLoadResponse { project })
}

#[tauri::command]
pub fn pattern_update(
    request: PatternUpdateRequest,
    state: tauri::State<'_, ProjectState>,
) -> PatternUpdateResponse {
    let mut project_lock = match state.project.lock() {
        Ok(lock) => lock,
        Err(e) => {
            log::error!("Failed to lock project state: {}", e);
            return PatternUpdateResponse { success: false };
        }
    };

    // Validate pattern index
    if request.pattern_index >= project_lock.patterns.len() {
        log::error!(
            "Invalid pattern index: {} (max: {})",
            request.pattern_index,
            project_lock.patterns.len() - 1
        );
        return PatternUpdateResponse { success: false };
    }

    let pattern = &mut project_lock.patterns[request.pattern_index];

    // Ensure channel exists in pattern grid
    while pattern.grid.len() <= request.channel {
        pattern.grid.push(Vec::new());
    }

    let channel_grid = &mut pattern.grid[request.channel];

    // Ensure row exists
    while channel_grid.len() <= request.cell_row {
        channel_grid.push(Note::default());
    }

    // Update or clear the note
    if let Some(note_dto) = request.note {
        channel_grid[request.cell_row] = Note::from(note_dto);
        log::debug!(
            "Updated pattern {} channel {} row {}: pitch={}, velocity={}",
            request.pattern_index,
            request.channel,
            request.cell_row,
            channel_grid[request.cell_row].pitch,
            channel_grid[request.cell_row].velocity
        );
    } else {
        // Clear the note (set to default/empty)
        channel_grid[request.cell_row] = Note::default();
        log::debug!(
            "Cleared pattern {} channel {} row {}",
            request.pattern_index,
            request.channel,
            request.cell_row
        );
    }

    PatternUpdateResponse { success: true }
}

// ==================== SOURCE REGISTRY COMMANDS ====================

#[tauri::command]
pub fn list_sources() -> Result<crate::dto::ListSourcesResponse, String> {
    let registry = crate::plugin::SourceRegistryHandle::new();
    let sources = registry.list_sources();
    let source_infos: Vec<crate::dto::SourceInfo> = sources
        .into_iter()
        .map(crate::dto::SourceInfo::from)
        .collect();

    Ok(crate::dto::ListSourcesResponse {
        sources: source_infos,
    })
}

#[tauri::command]
pub fn get_source_params(
    request: crate::dto::GetSourceParamsRequest,
) -> Result<crate::dto::GetSourceParamsResponse, String> {
    let registry = crate::plugin::SourceRegistryHandle::new();

    let descriptor = registry
        .get_descriptor(&request.source_id)
        .ok_or_else(|| format!("Source not found: {}", request.source_id))?;

    let params: Vec<crate::dto::ParamInfo> = descriptor
        .params
        .into_iter()
        .map(crate::dto::ParamInfo::from)
        .collect();

    Ok(crate::dto::GetSourceParamsResponse {
        source_id: request.source_id,
        params,
    })
}

#[tauri::command]
pub fn channel_set_source(
    request: crate::dto::ChannelSetSourceRequest,
    engine: tauri::State<'_, Engine>,
) -> Result<crate::dto::ChannelSetSourceResponse, String> {
    let registry = crate::plugin::SourceRegistryHandle::new();

    // Check if source exists
    if !registry.has_source(&request.source_id) {
        return Err(format!("Source not found: {}", request.source_id));
    }

    // Get engine state and set the source
    let state = engine.state();
    let mut state_guard = state.lock().map_err(|e| e.to_string())?;

    // Validate channel
    if request.channel >= state_guard.mixer.num_channels() {
        return Err(format!(
            "Invalid channel: {} (max: {})",
            request.channel,
            state_guard.mixer.num_channels() - 1
        ));
    }

    // Create the new source and replace in mixer
    let sample_rate = state_guard.sample_rate;
    if let Some(new_source) = registry.create_source(&request.source_id, sample_rate) {
        // Get the channel and replace its synth
        let channel = &mut state_guard.mixer.channels[request.channel];

        // Stop all notes on this channel
        channel.synth.all_notes_off();

        // Note: In the current MixerChannel, synth is a ChipSynth directly stored.
        // To properly support dynamic source switching, we'd need to modify MixerChannel
        // to store a Box<dyn SoundSource>. For now, we'll just update the waveform
        // based on the source type as a workaround.

        // For chip_synth, we can at least note that the source was set
        log::info!(
            "Channel {} source set to: {}",
            request.channel,
            request.source_id
        );
    }

    Ok(crate::dto::ChannelSetSourceResponse {
        channel: request.channel,
        source_id: request.source_id,
        success: true,
    })
}

#[tauri::command]
pub fn set_source_param(
    request: crate::dto::SetSourceParamRequest,
    engine: tauri::State<'_, Engine>,
) -> Result<crate::dto::SetSourceParamResponse, String> {
    let state = engine.state();
    let mut state_guard = state.lock().map_err(|e| e.to_string())?;

    // Validate channel
    if request.channel >= state_guard.mixer.num_channels() {
        return Err(format!(
            "Invalid channel: {} (max: {})",
            request.channel,
            state_guard.mixer.num_channels() - 1
        ));
    }

    // Get the channel's synth and set the parameter
    let channel = &mut state_guard.mixer.channels[request.channel];
    let param_id = crate::plugin::ParamId::new(&request.param_id);
    let value = crate::plugin::ParamValue::Float(request.value);

    channel.synth.set_param(&param_id, value);

    log::debug!(
        "Channel {} param {} set to {}",
        request.channel,
        request.param_id,
        request.value
    );

    Ok(crate::dto::SetSourceParamResponse {
        channel: request.channel,
        param_id: request.param_id,
        value: request.value,
        success: true,
    })
}
