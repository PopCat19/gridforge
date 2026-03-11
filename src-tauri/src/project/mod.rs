// mod.rs
//
// Purpose: Project module entry point
//
// Re-exports project format types and provides helper functions.

pub mod format;

pub use format::*;

pub fn create_default_project() -> Project {
    Project::new()
}
