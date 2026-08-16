//! Persistent store for VB6 application settings.
//!
//! VB6 keeps application settings in the Windows registry under
//! `HKEY_CURRENT_USER\Software\VB and VBA Program Settings\appname\section\key`,
//! written by `SaveSetting` and read by `GetSetting`/`GetAllSettings`. Because
//! the registry is Windows-only, this runtime backs the same hierarchy with a
//! per-user directory tree on every platform, so the settings functions behave
//! the same way on Linux, macOS, and Windows.
//!
//! The store lives under the user's config directory, in a subdirectory that
//! mirrors the registry root:
//!
//! - **Windows**: `%APPDATA%\vb6\settings`
//! - **macOS**: `~/Library/Application Support/vb6/settings`
//! - **Linux and other Unix**: `$XDG_CONFIG_HOME/vb6/settings`, falling back
//!   to `~/.config/vb6/settings` when `XDG_CONFIG_HOME` is unset
//!
//! Settings are stored one file per key, mirroring the registry path:
//!
//! ```text
//! <root>/<appname>/<section>/<key>
//! ```
//!
//! with the file content being the setting's value. As in the Windows
//! registry, `appname`, `section`, and `key` are matched case-insensitively.
//! Each component must be a single path segment (no path separators), so a
//! setting can never escape the store root.
//!
//! The in-memory snapshot is loaded from the store root on first access and
//! can be reloaded with [`reset`]. Writes go through to the store root, so
//! settings survive across runs of the program. When no config location is
//! available (for example in a webassembly host, which has neither a
//! filesystem nor environment variables), the store stays empty and in
//! memory; a host can install a baseline with [`set`] or point the store at a
//! directory with [`set_store_root`].

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

/// A setting's path, preserving the case the components were stored under.
#[derive(Clone, Debug, PartialEq, Eq)]
struct PathCase {
    appname: String,
    section: String,
    key: String,
}

/// One stored setting.
#[derive(Clone, Debug)]
struct Entry {
    /// The path components in the case they were stored under.
    path: PathCase,
    /// The setting's value.
    value: String,
}

/// Lowercased `(appname, section, key)` used as the in-memory index key.
type IndexKey = (String, String, String);

struct SettingsState {
    /// Index of every stored setting, keyed case-insensitively.
    values: HashMap<IndexKey, Entry>,
}

/// Lowercase a path component for case-insensitive matching.
fn normalize(component: &str) -> String {
    component.to_ascii_lowercase()
}

/// Build the case-insensitive index key for a setting path.
fn index_key(appname: &str, section: &str, key: &str) -> IndexKey {
    (normalize(appname), normalize(section), normalize(key))
}

/// The directory backing the settings store, when one is available.
static ROOT_OVERRIDE: Mutex<Option<PathBuf>> = Mutex::new(None);

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
            .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
    };
    base.map(|base| base.join("vb6").join("settings"))
}

/// The store root, honoring a host-installed override.
pub fn store_root() -> Option<PathBuf> {
    ROOT_OVERRIDE
        .lock()
        .ok()
        .and_then(|guard| guard.clone())
        .or_else(default_store_root)
}

/// Validate a single path component of a setting's registry-style path.
///
/// Rejects anything that could escape the store root (`..`), that cannot be a
/// single directory or file name (`/`, `\`, NUL, empty), or that is not a
/// legal file name on Windows (`:`, `*`, `?`, `"`, `<`, `>`, `|`). Restricting
/// the shared set keeps a store portable to Windows and case-insensitive
/// matching unambiguous.
fn valid_component(component: &str) -> bool {
    !component.is_empty()
        && component != "."
        && component != ".."
        && !component.contains(['/', '\\', '\0', ':', '*', '?', '"', '<', '>', '|'])
}

/// Find the child of `parent` whose name matches `name` case-insensitively.
fn find_child(parent: &Path, name: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(parent).ok()?.flatten() {
        if entry
            .file_name()
            .to_string_lossy()
            .eq_ignore_ascii_case(name)
        {
            return Some(entry.path());
        }
    }
    None
}

/// Recursively load `path` into `state`.
///
/// `depth` is the number of components already collected (0 = appname,
/// 1 = section); a directory at depth 2 is a key, whose file content is the
/// setting's value. Stray files and unreadable entries are skipped.
fn load_dir(state: &mut SettingsState, path: &Path, depth: usize, components: &mut Vec<String>) {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !valid_component(&name) {
            continue;
        }
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or(false);
        if depth < 2 && is_dir {
            components.push(name);
            load_dir(state, &entry.path(), depth + 1, components);
            components.pop();
        } else if depth == 2 && is_file {
            if let Ok(value) = fs::read_to_string(entry.path()) {
                components.push(name);
                let path = PathCase {
                    appname: components[0].clone(),
                    section: components[1].clone(),
                    key: components[2].clone(),
                };
                let key = index_key(&path.appname, &path.section, &path.key);
                state.values.insert(key, Entry { path, value });
                components.pop();
            }
        }
    }
}

/// Load the store from `root`, or an empty store when no root is available.
fn load_from(root: Option<PathBuf>) -> SettingsState {
    let mut state = SettingsState {
        values: HashMap::new(),
    };
    if let Some(root) = root {
        load_dir(&mut state, &root, 0, &mut Vec::new());
    }
    state
}

static STATE: OnceLock<Mutex<SettingsState>> = OnceLock::new();

/// Access the shared snapshot, loading it from the store root first.
fn snapshot() -> &'static Mutex<SettingsState> {
    STATE.get_or_init(|| Mutex::new(load_from(store_root())))
}

/// Lock the snapshot, recovering from a poisoned mutex.
fn lock() -> std::sync::MutexGuard<'static, SettingsState> {
    snapshot()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Reload the snapshot from the store root, discarding any in-memory-only
/// values. Useful when a host wants a fresh baseline or after files changed on
/// disk.
pub fn reset() {
    *lock() = load_from(store_root());
}

/// Override the store root and reload the snapshot from it.
///
/// Hosts (interpreters, test harnesses, unit tests) use this to point the
/// store at a controlled directory instead of the user's config directory.
pub fn set_store_root(root: impl Into<PathBuf>) {
    *ROOT_OVERRIDE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(root.into());
    reset();
}

/// Drop a store-root override installed with [`set_store_root`] and reload
/// from the default location.
pub fn reset_store_root() {
    *ROOT_OVERRIDE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    reset();
}

/// An `io` error for when no store location is available.
fn no_store_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::NotFound,
        "no settings store location available",
    )
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

/// Set `(appname, section, key)` to `value`, persisting it to the store root.
///
/// The components must be valid single path segments; an existing entry keeps
/// the case it was originally stored under so no duplicate-case files appear.
/// Returns an `io` error when there is no store location or the write fails.
pub fn set(appname: &str, section: &str, key: &str, value: &str) -> io::Result<()> {
    for component in [appname, section, key] {
        if !valid_component(component) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid setting path component: {component:?}"),
            ));
        }
    }
    let root = store_root().ok_or_else(no_store_error)?;
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
    let file = root.join(&path.appname).join(&path.section).join(&path.key);
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&file, value)?;
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
    let root = store_root().ok_or_else(no_store_error)?;
    let index = index_key(appname, section, key);
    let mut state = lock();
    let path = match state.values.remove(&index) {
        Some(entry) => entry.path,
        None => PathCase {
            appname: appname.to_string(),
            section: section.to_string(),
            key: key.to_string(),
        },
    };
    let file = root.join(&path.appname).join(&path.section).join(&path.key);
    let _ = fs::remove_file(&file);
    // Best-effort removal of now-empty section and appname directories.
    let _ = fs::remove_dir(file.parent().unwrap_or(&root));
    let _ = fs::remove_dir(file.parent().and_then(Path::parent).unwrap_or(&root));
    Ok(())
}

/// Remove every setting under `(appname, section)`, including the section.
pub fn remove_section(appname: &str, section: &str) -> io::Result<()> {
    let root = store_root().ok_or_else(no_store_error)?;
    let mut state = lock();
    state.values.retain(|(a, s, _), _| {
        !a.eq_ignore_ascii_case(appname) || !s.eq_ignore_ascii_case(section)
    });
    if let Some(app_dir) = find_child(&root, appname) {
        if let Some(section_dir) = find_child(&app_dir, section) {
            if section_dir.is_dir() {
                fs::remove_dir_all(&section_dir)?;
            }
        }
        let _ = fs::remove_dir(app_dir);
    }
    Ok(())
}

/// Remove every setting under `appname`, including the application directory.
pub fn remove_appname(appname: &str) -> io::Result<()> {
    let root = store_root().ok_or_else(no_store_error)?;
    let mut state = lock();
    state
        .values
        .retain(|(a, _, _), _| !a.eq_ignore_ascii_case(appname));
    if let Some(app_dir) = find_child(&root, appname) {
        if app_dir.is_dir() {
            fs::remove_dir_all(&app_dir)?;
        }
    }
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
        with_temp_settings_store(|root| {
            set("MyApp", "Window", "Width", "600").unwrap();
            set("myapp", "window", "width", "800").unwrap();
            assert_eq!(get("MyApp", "Window", "Width").as_deref(), Some("800"));
            // The rewrite must not leave a second case-variant file behind.
            let mut files: Vec<String> = Vec::new();
            collect_files(root, &mut files);
            assert_eq!(files.len(), 1);
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

    /// Collect every regular file path under `root`, for asserting the on-disk
    /// layout.
    fn collect_files(dir: &Path, out: &mut Vec<String>) {
        for entry in fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, out);
            } else {
                out.push(path.file_name().unwrap().to_string_lossy().into_owned());
            }
        }
    }
}
