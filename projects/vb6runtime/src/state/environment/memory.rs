//! In-memory environment backend.
//!
//! Seeds the snapshot empty, with no ties to the real process environment.
//! Used for tests and as a way to start from a clean baseline; a host
//! populates the snapshot with [`super::set_env`] afterwards.

use super::backend::EnvironmentBackend;

/// Environment backend that always seeds an empty snapshot.
#[derive(Debug, Default)]
pub struct MemoryBackend;

impl MemoryBackend {
    /// Create a new memory backend.
    pub fn new() -> Self {
        Self
    }
}

impl EnvironmentBackend for MemoryBackend {
    fn load(&self) -> Vec<(String, String)> {
        Vec::new()
    }
}
