//! Persistent store for VB6 application settings.
//!
//! VB6 keeps application settings in the Windows registry under
//! `HKEY_CURRENT_USER\Software\VB and VBA Program Settings\appname\section\key`,
//! written by `SaveSetting` and read by `GetSetting`/`GetAllSettings`. This
//! module provides the same behavior across platforms using pluggable backends:
//!
//! - **Windows**: Uses the actual Windows registry (default)
//! - **Linux/macOS**: Uses a file-based directory tree (default)
//! - **WASM**: Uses in-memory storage (synced to localStorage by JS host)
//!
//! The backend can be switched at runtime with [`set_backend`], or you can
//! use [`set_store_root`] to point the file backend at a custom directory.
//!
//! # In-Memory Snapshot
//!
//! An in-memory snapshot is loaded from the active backend on first access
//! and can be reloaded with [`reset`]. Writes update both the snapshot and
//! the backend, so settings survive across runs. When switching backends,
//! the snapshot is reloaded from the new backend.
//!
//! # Case Insensitivity
//!
//! As in the Windows registry, `appname`, `section`, and `key` are matched
//! case-insensitively. Each component must be a single path segment (no path
//! separators), so a setting can never escape the store.

pub mod backend;
pub mod file;
pub mod memory;
pub mod registry;

use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use backend::SettingsBackend;

/// A setting's path, preserving the case the components were stored under.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathCase {
    /// The application name.
    pub appname: String,
    /// The section name.
    pub section: String,
    /// The key name.
    pub key: String,
}

/// One stored setting.
#[derive(Clone, Debug)]
pub struct Entry {
    /// The path components in the case they were stored under.
    pub path: PathCase,
    /// The setting's value.
    pub value: String,
}

/// Lowercased `(appname, section, key)` used as the in-memory index key.
pub type IndexKey = (String, String, String);

/// Lowercase a path component for case-insensitive matching.
pub(crate) fn normalize(component: &str) -> String {
    component.to_ascii_lowercase()
}

/// Build the case-insensitive index key for a setting path.
pub(crate) fn index_key(appname: &str, section: &str, key: &str) -> IndexKey {
    (normalize(appname), normalize(section), normalize(key))
}

/// Validate a single path component of a setting's registry-style path.
fn valid_component(component: &str) -> bool {
    !component.is_empty()
        && component != "."
        && component != ".."
        && !component.contains(['/', '\\', '\0', ':', '*', '?', '"', '<', '>', '|'])
}

/// The default store root for the current platform.
///
/// `None` when the platform has no config location (for example a
/// webassembly host, which has no environment variables).
fn default_store_root() -> Option<PathBuf> {
    let base: Option<PathBuf> = if cfg!(windows) {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|home| {
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
        })
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config"))
            })
    };
    base.map(|base| base.join("vb6").join("settings"))
}

/// Create the default backend for the current platform.
fn default_backend() -> Box<dyn SettingsBackend> {
    if cfg!(target_arch = "wasm32") {
        Box::new(memory::MemoryBackend::new())
    } else if cfg!(windows) {
        Box::new(registry::RegistryBackend::new())
    } else {
        let root = default_store_root().expect("platform has no config location");
        Box::new(file::FileBackend::new(root))
    }
}

/// The active backend.
static BACKEND: OnceLock<Mutex<Box<dyn SettingsBackend>>> = OnceLock::new();

/// In-memory snapshot of all settings.
static SNAPSHOT: OnceLock<Mutex<Option<Snapshot>>> = OnceLock::new();

/// The in-memory snapshot, loaded from the active backend.
struct Snapshot {
    /// Index of every stored setting, keyed case-insensitively.
    values: HashMap<IndexKey, Entry>,
}

/// Get the active backend, initializing with default if needed.
fn backend() -> &'static Mutex<Box<dyn SettingsBackend>> {
    BACKEND.get_or_init(|| Mutex::new(default_backend()))
}

/// Access the shared snapshot, loading it from the backend first.
fn snapshot() -> &'static Mutex<Option<Snapshot>> {
    SNAPSHOT.get_or_init(|| Mutex::new(None))
}

/// Lock the snapshot, returning a guard that dereferences to the snapshot.
///
/// Panics if the snapshot has not been initialized.
fn lock() -> impl std::ops::DerefMut<Target = Snapshot> {
    let guard = snapshot().lock().unwrap_or_else(|e| e.into_inner());
    // We use a small wrapper to handle the Option.
    // If the snapshot is None, we need to initialize it first.
    drop(guard);

    // Initialize if needed
    {
        let mut snap = snapshot().lock().unwrap_or_else(|e| e.into_inner());
        if snap.is_none() {
            let backend_guard = backend().lock().unwrap_or_else(|e| e.into_inner());
            let values = backend_guard.load_all();
            *snap = Some(Snapshot { values });
        }
    }

    // Now get the guard
    // We need a custom wrapper since MutexGuard doesn't work with Option<Snapshot>
    SnapshotGuard(snapshot().lock().unwrap_or_else(|e| e.into_inner()))
}

/// Wrapper to provide DerefMut to Snapshot through Option<Snapshot>.
struct SnapshotGuard(std::sync::MutexGuard<'static, Option<Snapshot>>);

impl std::ops::Deref for SnapshotGuard {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().expect("snapshot must be initialized")
    }
}

impl std::ops::DerefMut for SnapshotGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().expect("snapshot must be initialized")
    }
}

/// Reload the snapshot from the active backend, discarding any in-memory-only
/// values. Useful when a host wants a fresh baseline or after the backend
/// changes.
pub fn reset() {
    let backend_guard = backend().lock().unwrap_or_else(|e| e.into_inner());
    let values = backend_guard.load_all();
    let mut snap = snapshot().lock().unwrap_or_else(|e| e.into_inner());
    *snap = Some(Snapshot { values });
}

/// Set the active backend and reload the snapshot from it.
///
/// This is the primary way to switch storage backends at runtime. After
/// switching, all in-memory state is replaced with data from the new backend.
///
/// # Examples
///
/// ```ignore
/// // Switch to an in-memory backend
/// use vb6runtime::state::settings::memory::MemoryBackend;
/// vb6runtime::state::settings::set_backend(Box::new(MemoryBackend::new()));
///
/// // Switch to a file backend with a custom root
/// use vb6runtime::state::settings::file::FileBackend;
/// vb6runtime::state::settings::set_backend(Box::new(FileBackend::new("/tmp/myapp".into())));
/// ```
pub fn set_backend(new_backend: Box<dyn SettingsBackend>) {
    // Load from new backend first (while holding the backend lock briefly)
    let values = {
        let mut backend_guard = backend().lock().unwrap_or_else(|e| e.into_inner());
        *backend_guard = new_backend;
        backend_guard.load_all()
    };

    // Update snapshot
    let mut snap = snapshot().lock().unwrap_or_else(|e| e.into_inner());
    *snap = Some(Snapshot { values });
}

/// Override the store root and reload the snapshot from it.
///
/// This is a convenience wrapper around [`set_backend`] that creates a
/// [`FileBackend`](file::FileBackend) pointing at `root`. Hosts (interpreters,
/// test harnesses, unit tests) use this to point the store at a controlled
/// directory instead of the user's config directory.
pub fn set_store_root(root: impl Into<PathBuf>) {
    set_backend(Box::new(file::FileBackend::new(root.into())));
}

/// Reset to the default backend for the current platform.
///
/// This undoes any call to [`set_backend`] or [`set_store_root`] and reloads
/// from the platform's default location.
pub fn reset_backend() {
    set_backend(default_backend());
}

/// Reset to the default backend for the current platform.
///
/// Alias for [`reset_backend`] for backwards compatibility.
pub fn reset_store_root() {
    reset_backend();
}

/// The value stored for `(appname, section, key)`, matched case-insensitively.
pub fn get(appname: &str, section: &str, key: &str) -> Option<String> {
    let index = index_key(appname, section, key);
    lock().values.get(&index).map(|entry| entry.value.clone())
}

/// Every `(key, value)` pair stored under `(appname, section)`, sorted by key.
///
/// Keys are returned in the case they were stored under. This is the data
/// behind `GetAllSettings`.
pub fn get_all(appname: &str, section: &str) -> Vec<(String, String)> {
    let (app, section) = (normalize(appname), normalize(section));
    let mut out: Vec<(String, String)> = lock()
        .values
        .iter()
        .filter(|((a, s, _), _)| *a == app && *s == section)
        .map(|(_, entry)| (entry.path.key.clone(), entry.value.clone()))
        .collect();
    out.sort();
    out
}

/// Every `(appname, section, key, value)` stored in the whole store, sorted
/// by appname, then section, then key.
///
/// Components are returned in the case they were stored under. This is the
/// complete contents of the store, for hosts that need to persist the whole
/// snapshot (for example a webassembly host mirroring it into `localStorage`).
pub fn entries() -> Vec<(String, String, String, String)> {
    let mut out: Vec<(String, String, String, String)> = lock()
        .values
        .values()
        .map(|entry| {
            (
                entry.path.appname.clone(),
                entry.path.section.clone(),
                entry.path.key.clone(),
                entry.value.clone(),
            )
        })
        .collect();
    out.sort();
    out
}

/// Set `(appname, section, key)` to `value`, persisting it to the backend.
///
/// The components must be valid single path segments; an existing entry keeps
/// the case it was originally stored under so no duplicate-case entries appear.
/// Returns an `io` error when a component is invalid or a backend write fails.
pub fn set(appname: &str, section: &str, key: &str, value: &str) -> io::Result<()> {
    for component in [appname, section, key] {
        if !valid_component(component) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid setting path component: {component:?}"),
            ));
        }
    }

    // Persist to backend
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .set(appname, section, key, value)?;

    // Update snapshot
    let index = index_key(appname, section, key);
    let mut state = lock();
    let path = match state.values.get(&index) {
        Some(existing) => existing.path.clone(),
        None => PathCase {
            appname: appname.to_string(),
            section: section.to_string(),
            key: key.to_string(),
        },
    };
    state.values.insert(
        index,
        Entry {
            path,
            value: value.to_string(),
        },
    );

    Ok(())
}

/// Remove the setting `(appname, section, key)` from the store.
pub fn remove_key(appname: &str, section: &str, key: &str) -> io::Result<()> {
    // Remove from backend
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove_key(appname, section, key)?;

    // Update snapshot
    let index = index_key(appname, section, key);
    lock().values.remove(&index);

    Ok(())
}

/// Remove every setting under `(appname, section)`, including the section.
pub fn remove_section(appname: &str, section: &str) -> io::Result<()> {
    // Remove from backend
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove_section(appname, section)?;

    // Update snapshot
    let (app, sec) = (normalize(appname), normalize(section));
    lock()
        .values
        .retain(|(a, s, _), _| !(a == &app && s == &sec));

    Ok(())
}

/// Remove every setting under `appname`, including the application directory.
pub fn remove_appname(appname: &str) -> io::Result<()> {
    // Remove from backend
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove_appname(appname)?;

    // Update snapshot
    let app = normalize(appname);
    lock().values.retain(|(a, _, _), _| a != &app);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::test_support::with_temp_settings_store;

    #[test]
    fn get_returns_none_for_a_missing_setting() {
        with_temp_settings_store(|_| {
            assert_eq!(get("MyApp", "Section", "Key"), None);
        });
    }

    #[test]
    fn set_then_get_roundtrips() {
        with_temp_settings_store(|_| {
            set("MyApp", "Startup", "Left", "150").unwrap();
            assert_eq!(get("MyApp", "Startup", "Left").as_deref(), Some("150"));
        });
    }

    #[test]
    fn lookup_is_case_insensitive() {
        with_temp_settings_store(|_| {
            set("MyApp", "Window", "Width", "600").unwrap();
            assert_eq!(get("myapp", "window", "width").as_deref(), Some("600"));
            assert_eq!(get("MYAPP", "WINDOW", "WIDTH").as_deref(), Some("600"));
        });
    }

    #[test]
    fn set_reuses_the_original_case_of_an_existing_entry() {
        with_temp_settings_store(|_| {
            set("MyApp", "Window", "Width", "600").unwrap();
            set("myapp", "window", "width", "800").unwrap();
            assert_eq!(get("MyApp", "Window", "Width").as_deref(), Some("800"));
        });
    }

    #[test]
    fn values_survive_a_reload_from_disk() {
        with_temp_settings_store(|_| {
            set("MyApp", "Startup", "Left", "150").unwrap();
            reset();
            assert_eq!(get("MyApp", "Startup", "Left").as_deref(), Some("150"));
        });
    }

    #[test]
    fn get_all_returns_section_pairs_sorted_by_key() {
        with_temp_settings_store(|_| {
            set("MyApp", "Startup", "Left", "150").unwrap();
            set("MyApp", "Startup", "Top", "40").unwrap();
            set("MyApp", "Other", "Left", "10").unwrap();
            set("OtherApp", "Startup", "Left", "1").unwrap();
            assert_eq!(
                get_all("MyApp", "Startup"),
                vec![
                    ("Left".to_string(), "150".to_string()),
                    ("Top".to_string(), "40".to_string()),
                ]
            );
            assert_eq!(get_all("myapp", "startup"), get_all("MyApp", "Startup"));
        });
    }

    #[test]
    fn entries_returns_every_setting_sorted_and_in_original_case() {
        with_temp_settings_store(|_| {
            set("MyApp", "Startup", "Top", "40").unwrap();
            set("myapp", "Startup", "left", "150").unwrap();
            set("OtherApp", "Startup", "Left", "1").unwrap();
            assert_eq!(
                entries(),
                vec![
                    (
                        "MyApp".to_string(),
                        "Startup".to_string(),
                        "Top".to_string(),
                        "40".to_string()
                    ),
                    (
                        "OtherApp".to_string(),
                        "Startup".to_string(),
                        "Left".to_string(),
                        "1".to_string()
                    ),
                    (
                        "myapp".to_string(),
                        "Startup".to_string(),
                        "left".to_string(),
                        "150".to_string()
                    ),
                ]
            );
        });
    }

    #[test]
    fn set_rejects_path_traversal() {
        with_temp_settings_store(|_| {
            for bad in ["../escape", "a/b", "a\\b", "", ".", ".."] {
                let err = set("MyApp", bad, "Key", "value").unwrap_err();
                assert_eq!(err.kind(), io::ErrorKind::InvalidInput, "for {bad:?}");
            }
        });
    }

    #[test]
    fn remove_key_deletes_only_that_setting() {
        with_temp_settings_store(|_| {
            set("MyApp", "Startup", "Left", "150").unwrap();
            set("MyApp", "Startup", "Top", "40").unwrap();
            remove_key("MyApp", "Startup", "left").unwrap();
            assert_eq!(get("MyApp", "Startup", "Left"), None);
            assert_eq!(get("MyApp", "Startup", "Top").as_deref(), Some("40"));
        });
    }

    #[test]
    fn remove_section_deletes_all_keys_under_it() {
        with_temp_settings_store(|_| {
            set("MyApp", "Startup", "Left", "150").unwrap();
            set("MyApp", "Startup", "Top", "40").unwrap();
            remove_section("myapp", "startup").unwrap();
            assert!(get_all("MyApp", "Startup").is_empty());
            assert_eq!(get("MyApp", "Startup", "Left"), None);
        });
    }

    #[test]
    fn remove_appname_deletes_the_whole_application() {
        with_temp_settings_store(|_| {
            set("MyApp", "Startup", "Left", "150").unwrap();
            set("MyApp", "Other", "Top", "40").unwrap();
            remove_appname("myapp").unwrap();
            assert!(get_all("MyApp", "Startup").is_empty());
            assert!(get_all("MyApp", "Other").is_empty());
        });
    }

    #[test]
    fn set_backend_switches_to_new_backend() {
        use memory::MemoryBackend;

        let _guard = crate::state::test_support::TEST_LOCK.lock().unwrap();

        // Start with a file backend
        let dir = tempfile::tempdir().unwrap();
        set_store_root(dir.path());

        // Set some values
        set("MyApp", "Section", "Key", "FileValue").unwrap();
        assert_eq!(get("MyApp", "Section", "Key").as_deref(), Some("FileValue"));

        // Switch to memory backend
        let mem = MemoryBackend::new();
        set_backend(Box::new(mem));

        // Old values should be gone
        assert_eq!(get("MyApp", "Section", "Key"), None);

        // New values should work
        set("MyApp", "Section", "Key", "MemValue").unwrap();
        assert_eq!(get("MyApp", "Section", "Key").as_deref(), Some("MemValue"));

        // Reset to default
        reset_backend();
    }
}
