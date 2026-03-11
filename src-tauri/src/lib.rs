// lib.rs
//
// Purpose: Gridforge audio application Tauri backend
//
// Entry point for the Tauri application with project command handlers.

mod commands;
mod dto;
mod engine;
mod plugin;
mod project;

use commands::{
    channel_set_source, get_source_params, list_sources, note_preview, pattern_update,
    project_load, project_new, project_save, set_source_param, transport_play,
    transport_set_tempo, transport_stop, ProjectState,
};
use engine::{Engine, DEFAULT_BUFFER_SIZE, DEFAULT_SAMPLE_RATE};
use std::sync::Arc;

struct AppState {
    engine: Engine,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = Engine::new(DEFAULT_SAMPLE_RATE, DEFAULT_BUFFER_SIZE);

    if let Err(e) = engine.start() {
        log::warn!("Failed to start audio engine: {}", e);
    } else {
        log::info!("Audio engine started successfully");
    }

    let app_state = AppState { engine };
    let project_state = ProjectState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .manage(engine)
        .manage(project_state)
        .invoke_handler(tauri::generate_handler![
            project_new,
            project_save,
            project_load,
            note_preview,
            pattern_update,
            transport_play,
            transport_stop,
            transport_set_tempo,
            list_sources,
            get_source_params,
            channel_set_source,
            set_source_param,
        ])
        .setup(|app| {
            log::info!("Gridforge application started");

            // Start the event emission thread for playhead and levels
            engine.start_event_thread(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
