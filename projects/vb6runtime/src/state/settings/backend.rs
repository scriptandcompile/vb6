//! Trait abstracting over different settings storage backends.
//!
//! VB6 stores application settings in the Windows registry under
//! `HKEY_CURRENT_USER\Software\VB and VBA Program Settings\appname\section\key`.
//! This runtime supports multiple storage backends to provide the same behavior
//! across platforms:
//!
//! - **Windows**: [`RegistryBackend`](super::registry::RegistryBackend) uses the actual Windows registry
//! - **Linux/macOS**: [`FileBackend`](super::file::FileBackend) uses a directory tree
//! - **WASM**: [`MemoryBackend`](super::memory::MemoryBackend) stores in memory (synced to localStorage by JS host)
//!
//! All backends maintain case-insensitive matching of path components,
//! matching the Windows registry behavior.

use std::collections::HashMap;
use std::io;

use super::{Entry, IndexKey};

/// Abstraction over different settings storage backends.
///
/// Implementations must be `Send + Sync` so the backend can be shared across
/// threads via a `Mutex<Box<dyn SettingsBackend>>`.
pub trait SettingsBackend: Send + Sync {
    /// Get a single setting value.
    ///
    /// The three path components are matched case-insensitively.
    /// Returns `None` if the setting does not exist.
    fn get(&self, appname: &str, section: &str, key: &str) -> Option<String>;

    /// Set a single setting value, creating it if it doesn't exist.
    ///
    /// The backend must persist the value so it survives process restarts
    /// (except for the in-memory backend used in WASM/testing).
    fn set(&self, appname: &str, section: &str, key: &str, value: &str) -> io::Result<()>;

    /// Remove a single key from the store.
    ///
    /// If the key doesn't exist, this is a no-op (returns `Ok(())`).
    fn remove_key(&self, appname: &str, section: &str, key: &str) -> io::Result<()>;

    /// Remove an entire section and all its keys.
    ///
    /// If the section doesn't exist, this is a no-op (returns `Ok(())`).
    fn remove_section(&self, appname: &str, section: &str) -> io::Result<()>;

    /// Remove an entire application and all its sections.
    ///
    /// If the application doesn't exist, this is a no-op (returns `Ok(())`).
    fn remove_appname(&self, appname: &str) -> io::Result<()>;

    /// Get all `(key, value)` pairs in a section.
    ///
    /// Keys are returned in their original case as stored.
    /// Results are sorted by key.
    fn get_all(&self, appname: &str, section: &str) -> Vec<(String, String)>;

    /// Get all settings in the store.
    ///
    /// Returns `(appname, section, key, value)` tuples sorted by
    /// appname, then section, then key. Components are in their original case.
    fn entries(&self) -> Vec<(String, String, String, String)>;

    /// Load all settings into a HashMap for snapshot initialization.
    ///
    /// Returns a map from lowercased index keys to entries with original-case
    /// paths and values. This is called once when the snapshot is first
    /// accessed, and again whenever the backend is switched.
    fn load_all(&self) -> HashMap<IndexKey, Entry>;
}
