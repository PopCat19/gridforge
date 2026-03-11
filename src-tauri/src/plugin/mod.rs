// plugin/mod.rs
//
// Purpose: Plugin system module for audio sources
//
// This module:
// - Re-exports SoundSource trait and utilities
// - Provides chip synth implementation
// - Manages source registry for dynamic discovery

pub mod chip_synth;
pub mod traits;

pub use chip_synth::ChipSynth;
pub use traits::{
    freq_to_note, note_to_freq, AudioBuffer, Note, ParamDescriptor, ParamId, ParamValue,
    SoundSource, SourceCategory, SourceDescriptor, Velocity,
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Factory function type for creating sound sources
type SourceFactory = Box<dyn Fn(u32) -> Box<dyn SoundSource> + Send + Sync>;

/// Source registry for dynamic source discovery and instantiation
pub struct SourceRegistry {
    sources: HashMap<String, SourceEntry>,
}

struct SourceEntry {
    descriptor: SourceDescriptor,
    factory: SourceFactory,
}

impl SourceRegistry {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    /// Register a sound source with the registry
    pub fn register<F>(mut self, id: &str, name: &str, category: SourceCategory, factory: F) -> Self
    where
        F: Fn(u32) -> Box<dyn SoundSource> + Send + Sync + 'static,
    {
        // Create a temporary instance to get the params
        let temp = factory(48000);
        let params = temp.params();

        let descriptor = SourceDescriptor::new(id, name, category).with_params(params);

        self.sources.insert(
            id.to_string(),
            SourceEntry {
                descriptor,
                factory: Box::new(factory),
            },
        );

        self
    }

    /// List all available sources
    pub fn list_sources(&self) -> Vec<SourceDescriptor> {
        self.sources
            .values()
            .map(|entry| entry.descriptor.clone())
            .collect()
    }

    /// Get descriptor for a specific source
    pub fn get_descriptor(&self, id: &str) -> Option<SourceDescriptor> {
        self.sources.get(id).map(|e| e.descriptor.clone())
    }

    /// Create a source instance by ID
    pub fn create_source(&self, id: &str, sample_rate: u32) -> Option<Box<dyn SoundSource>> {
        self.sources
            .get(id)
            .map(|entry| (entry.factory)(sample_rate))
    }

    /// Check if a source exists
    pub fn has_source(&self, id: &str) -> bool {
        self.sources.contains_key(id)
    }
}

impl Default for SourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global source registry instance
pub fn create_default_registry() -> SourceRegistry {
    SourceRegistry::new().register("chip_synth", "Chip Synth", SourceCategory::Synth, |sr| {
        Box::new(ChipSynth::new(sr)) as Box<dyn SoundSource>
    })
}

/// Thread-safe wrapper for source registry
pub struct SourceRegistryHandle {
    registry: Arc<Mutex<SourceRegistry>>,
}

impl SourceRegistryHandle {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(create_default_registry())),
        }
    }

    pub fn with_registry(registry: SourceRegistry) -> Self {
        Self {
            registry: Arc::new(Mutex::new(registry)),
        }
    }

    pub fn list_sources(&self) -> Vec<SourceDescriptor> {
        if let Ok(registry) = self.registry.lock() {
            registry.list_sources()
        } else {
            Vec::new()
        }
    }

    pub fn get_descriptor(&self, id: &str) -> Option<SourceDescriptor> {
        if let Ok(registry) = self.registry.lock() {
            registry.get_descriptor(id)
        } else {
            None
        }
    }

    pub fn create_source(&self, id: &str, sample_rate: u32) -> Option<Box<dyn SoundSource>> {
        if let Ok(registry) = self.registry.lock() {
            registry.create_source(id, sample_rate)
        } else {
            None
        }
    }

    pub fn has_source(&self, id: &str) -> bool {
        if let Ok(registry) = self.registry.lock() {
            registry.has_source(id)
        } else {
            false
        }
    }
}

impl Default for SourceRegistryHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_registry_list_sources() {
        let registry = create_default_registry();
        let sources = registry.list_sources();

        // Default registry should have chip_synth
        assert!(!sources.is_empty());
        assert!(sources.iter().any(|s| s.id == "chip_synth"));
    }

    #[test]
    fn test_source_registry_create_source() {
        let registry = create_default_registry();

        let source = registry.create_source("chip_synth", 48000);
        assert!(source.is_some());

        let source = source.unwrap();
        assert_eq!(source.name(), "ChipSynth");
    }

    #[test]
    fn test_source_registry_has_source() {
        let registry = create_default_registry();

        assert!(registry.has_source("chip_synth"));
        assert!(!registry.has_source("nonexistent"));
    }

    #[test]
    fn test_source_registry_get_descriptor() {
        let registry = create_default_registry();

        let descriptor = registry.get_descriptor("chip_synth");
        assert!(descriptor.is_some());

        let desc = descriptor.unwrap();
        assert_eq!(desc.id, "chip_synth");
        assert_eq!(desc.name, "ChipSynth");
        assert_eq!(desc.category, SourceCategory::Synth);
    }

    #[test]
    fn test_source_registry_handle() {
        let handle = SourceRegistryHandle::new();

        let sources = handle.list_sources();
        assert!(!sources.is_empty());

        let source = handle.create_source("chip_synth", 48000);
        assert!(source.is_some());
    }
}
