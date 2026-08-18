//! File-based settings backend.
//!
//! Stores settings as files in a directory tree, with one file per key:
//!
//! ```text
//! <root>/<appname>/<section>/<key>
//! ```
//!
//! The file content is the setting's value. This mirrors the Windows registry
//! hierarchy on the filesystem.
//!
//! Default locations by platform:
//! - **Windows**: `%APPDATA%\vb6\settings`
//! - **macOS**: `~/Library/Application Support/vb6/settings`
//! - **Linux/Unix**: `$XDG_CONFIG_HOME/vb6/settings` (or `~/.config/vb6/settings`)

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use super::backend::SettingsBackend;
use super::{Entry, IndexKey, PathCase};

/// File-based settings backend.
///
/// Settings are stored as individual files in a directory tree that mirrors
/// the Windows registry structure.
pub struct FileBackend {
    root: PathBuf,
}

impl FileBackend {
    /// Create a new file backend rooted at `root`.
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// The root directory of this backend.
    pub fn root(&self) -> &Path {
        &self.root
    }
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

/// Recursively load `path` into the state map.
///
/// `depth` is the number of components already collected (0 = appname,
/// 1 = section); a directory at depth 2 is a key, whose file content is the
/// setting's value. Stray files and unreadable entries are skipped.
fn load_dir(
    state: &mut HashMap<IndexKey, Entry>,
    path: &Path,
    depth: usize,
    components: &mut Vec<String>,
) {
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
                let key = super::index_key(&path.appname, &path.section, &path.key);
                state.insert(key, Entry { path, value });
                components.pop();
            }
        }
    }
}

impl SettingsBackend for FileBackend {
    fn get(&self, appname: &str, section: &str, key: &str) -> Option<String> {
        // Case-insensitive lookup: find the actual directory/file names
        let app_dir = find_child(&self.root, appname)?;
        let section_dir = find_child(&app_dir, section)?;
        let key_file = find_child(&section_dir, key)?;
        fs::read_to_string(key_file).ok()
    }

    fn set(&self, appname: &str, section: &str, key: &str, value: &str) -> io::Result<()> {
        for component in [appname, section, key] {
            if !valid_component(component) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid setting path component: {component:?}"),
                ));
            }
        }
        let file = self.root.join(appname).join(section).join(key);
        if let Some(parent) = file.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file, value)
    }

    fn remove_key(&self, appname: &str, section: &str, key: &str) -> io::Result<()> {
        // Use case-insensitive lookup to find the actual file
        if let Some(app_dir) = find_child(&self.root, appname) {
            if let Some(section_dir) = find_child(&app_dir, section) {
                if let Some(key_file) = find_child(&section_dir, key) {
                    let _ = fs::remove_file(&key_file);
                    // Best-effort removal of now-empty section and appname directories.
                    let _ = fs::remove_dir(&section_dir);
                    let _ = fs::remove_dir(&app_dir);
                }
            }
        }
        Ok(())
    }

    fn remove_section(&self, appname: &str, section: &str) -> io::Result<()> {
        if let Some(app_dir) = find_child(&self.root, appname) {
            if let Some(section_dir) = find_child(&app_dir, section) {
                if section_dir.is_dir() {
                    fs::remove_dir_all(&section_dir)?;
                }
            }
            let _ = fs::remove_dir(app_dir);
        }
        Ok(())
    }

    fn remove_appname(&self, appname: &str) -> io::Result<()> {
        if let Some(app_dir) = find_child(&self.root, appname) {
            if app_dir.is_dir() {
                fs::remove_dir_all(&app_dir)?;
            }
        }
        Ok(())
    }

    fn get_all(&self, appname: &str, section: &str) -> Vec<(String, String)> {
        let mut out = Vec::new();

        // Use case-insensitive lookup
        let Some(app_dir) = find_child(&self.root, appname) else {
            return out;
        };
        let Some(section_dir) = find_child(&app_dir, section) else {
            return out;
        };

        if let Ok(entries) = fs::read_dir(&section_dir) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if valid_component(&name) {
                        if let Ok(value) = fs::read_to_string(entry.path()) {
                            out.push((name, value));
                        }
                    }
                }
            }
        }

        out.sort();
        out
    }

    fn entries(&self) -> Vec<(String, String, String, String)> {
        let mut out = Vec::new();

        let Ok(app_entries) = fs::read_dir(&self.root) else {
            return out;
        };

        for app_entry in app_entries.flatten() {
            if !app_entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                continue;
            }
            let appname = app_entry.file_name().to_string_lossy().into_owned();
            if !valid_component(&appname) {
                continue;
            }

            let Ok(section_entries) = fs::read_dir(app_entry.path()) else {
                continue;
            };

            for section_entry in section_entries.flatten() {
                if !section_entry
                    .file_type()
                    .map(|t| t.is_dir())
                    .unwrap_or(false)
                {
                    continue;
                }
                let section = section_entry.file_name().to_string_lossy().into_owned();
                if !valid_component(&section) {
                    continue;
                }

                let Ok(key_entries) = fs::read_dir(section_entry.path()) else {
                    continue;
                };

                for key_entry in key_entries.flatten() {
                    if !key_entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                        continue;
                    }
                    let key = key_entry.file_name().to_string_lossy().into_owned();
                    if !valid_component(&key) {
                        continue;
                    }

                    if let Ok(value) = fs::read_to_string(key_entry.path()) {
                        out.push((appname.clone(), section.clone(), key, value));
                    }
                }
            }
        }

        out.sort();
        out
    }

    fn load_all(&self) -> HashMap<IndexKey, Entry> {
        let mut state = HashMap::new();
        load_dir(&mut state, &self.root, 0, &mut Vec::new());
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_test_backend(dir: &Path) -> FileBackend {
        fs::create_dir_all(dir).unwrap();
        FileBackend::new(dir.to_path_buf())
    }

    #[test]
    fn get_returns_none_for_missing_setting() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        assert_eq!(backend.get("MyApp", "Section", "Key"), None);
    }

    #[test]
    fn set_then_get_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        assert_eq!(
            backend.get("MyApp", "Startup", "Left").as_deref(),
            Some("150")
        );
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        backend.set("MyApp", "Window", "Width", "600").unwrap();
        assert_eq!(
            backend.get("myapp", "window", "width").as_deref(),
            Some("600")
        );
        assert_eq!(
            backend.get("MYAPP", "WINDOW", "WIDTH").as_deref(),
            Some("600")
        );
    }

    #[test]
    fn set_rejects_path_traversal() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        for bad in ["../escape", "a/b", "a\\b", "", ".", ".."] {
            let err = backend.set("MyApp", bad, "Key", "value").unwrap_err();
            assert_eq!(err.kind(), io::ErrorKind::InvalidInput, "for {bad:?}");
        }
    }

    #[test]
    fn remove_key_deletes_only_that_setting() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        backend.set("MyApp", "Startup", "Top", "40").unwrap();
        backend.remove_key("MyApp", "Startup", "left").unwrap();
        assert_eq!(backend.get("MyApp", "Startup", "Left"), None);
        assert_eq!(
            backend.get("MyApp", "Startup", "Top").as_deref(),
            Some("40")
        );
    }

    #[test]
    fn remove_section_deletes_all_keys_under_it() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        backend.set("MyApp", "Startup", "Top", "40").unwrap();
        backend.remove_section("myapp", "startup").unwrap();
        assert!(backend.get_all("MyApp", "Startup").is_empty());
        assert_eq!(backend.get("MyApp", "Startup", "Left"), None);
    }

    #[test]
    fn remove_appname_deletes_the_whole_application() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        backend.set("MyApp", "Other", "Top", "40").unwrap();
        backend.remove_appname("myapp").unwrap();
        assert!(backend.get_all("MyApp", "Startup").is_empty());
        assert!(backend.get_all("MyApp", "Other").is_empty());
    }

    #[test]
    fn get_all_returns_section_pairs_sorted_by_key() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        backend.set("MyApp", "Startup", "Top", "40").unwrap();
        backend.set("MyApp", "Other", "Left", "10").unwrap();
        backend.set("OtherApp", "Startup", "Left", "1").unwrap();
        assert_eq!(
            backend.get_all("MyApp", "Startup"),
            vec![
                ("Left".to_string(), "150".to_string()),
                ("Top".to_string(), "40".to_string()),
            ]
        );
        assert_eq!(
            backend.get_all("myapp", "startup"),
            backend.get_all("MyApp", "Startup")
        );
    }

    #[test]
    fn entries_returns_every_setting_sorted_and_in_original_case() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        backend.set("MyApp", "Startup", "Top", "40").unwrap();
        backend.set("myapp", "Startup", "left", "150").unwrap();
        backend.set("OtherApp", "Startup", "Left", "1").unwrap();
        let mut entries = backend.entries();
        entries.sort();
        assert_eq!(
            entries,
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
    }

    #[test]
    fn load_all_returns_all_settings() {
        let dir = tempfile::tempdir().unwrap();
        let backend = create_test_backend(dir.path());
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        backend.set("MyApp", "Startup", "Top", "40").unwrap();

        let loaded = backend.load_all();
        assert_eq!(loaded.len(), 2);
        assert_eq!(
            loaded
                .get(&(
                    "myapp".to_string(),
                    "startup".to_string(),
                    "left".to_string()
                ))
                .map(|e| e.value.as_str()),
            Some("150")
        );
        assert_eq!(
            loaded
                .get(&(
                    "myapp".to_string(),
                    "startup".to_string(),
                    "top".to_string()
                ))
                .map(|e| e.value.as_str()),
            Some("40")
        );
    }
}
