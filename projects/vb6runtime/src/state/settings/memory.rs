//! In-memory settings backend.
//!
//! Stores settings entirely in memory with no persistence. Used for:
//!
//! - **WASM**: The JS playground syncs to/from `localStorage`
//! - **Tests**: Avoids filesystem/registry side effects

use std::collections::HashMap;
use std::sync::Mutex;

use super::backend::SettingsBackend;
use super::{Entry, IndexKey, PathCase};

/// In-memory settings backend.
///
/// All data is stored in a `HashMap` behind a `Mutex`. This backend has no
/// persistence; for WASM hosts, the JS layer is responsible for syncing
/// settings to `localStorage`.
pub struct MemoryBackend {
    values: Mutex<HashMap<IndexKey, Entry>>,
}

impl MemoryBackend {
    /// Create a new empty in-memory backend.
    pub fn new() -> Self {
        Self {
            values: Mutex::new(HashMap::new()),
        }
    }

    /// Create a pre-populated in-memory backend from existing entries.
    pub fn from_entries(entries: HashMap<IndexKey, Entry>) -> Self {
        Self {
            values: Mutex::new(entries),
        }
    }
}

impl Default for MemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsBackend for MemoryBackend {
    fn get(&self, appname: &str, section: &str, key: &str) -> Option<String> {
        let index = super::index_key(appname, section, key);
        self.values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&index)
            .map(|entry| entry.value.clone())
    }

    fn set(&self, appname: &str, section: &str, key: &str, value: &str) -> std::io::Result<()> {
        let index = super::index_key(appname, section, key);
        let entry = Entry {
            path: PathCase {
                appname: appname.to_string(),
                section: section.to_string(),
                key: key.to_string(),
            },
            value: value.to_string(),
        };
        self.values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(index, entry);
        Ok(())
    }

    fn remove_key(&self, appname: &str, section: &str, key: &str) -> std::io::Result<()> {
        let index = super::index_key(appname, section, key);
        self.values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&index);
        Ok(())
    }

    fn remove_section(&self, appname: &str, section: &str) -> std::io::Result<()> {
        let app = super::normalize(appname);
        let sec = super::normalize(section);
        self.values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|(a, s, _), _| !(a == &app && s == &sec));
        Ok(())
    }

    fn remove_appname(&self, appname: &str) -> std::io::Result<()> {
        let app = super::normalize(appname);
        self.values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|(a, _, _), _| a != &app);
        Ok(())
    }

    fn get_all(&self, appname: &str, section: &str) -> Vec<(String, String)> {
        let (app, sec) = (super::normalize(appname), super::normalize(section));
        let mut out: Vec<(String, String)> = self
            .values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .filter(|((a, s, _), _)| *a == app && *s == sec)
            .map(|(_, entry)| (entry.path.key.clone(), entry.value.clone()))
            .collect();
        out.sort();
        out
    }

    fn entries(&self) -> Vec<(String, String, String, String)> {
        let mut out: Vec<(String, String, String, String)> = self
            .values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
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

    fn load_all(&self) -> HashMap<IndexKey, Entry> {
        self.values
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_returns_none_for_missing_setting() {
        let backend = MemoryBackend::new();
        assert_eq!(backend.get("MyApp", "Section", "Key"), None);
    }

    #[test]
    fn set_then_get_roundtrips() {
        let backend = MemoryBackend::new();
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        assert_eq!(
            backend.get("MyApp", "Startup", "Left").as_deref(),
            Some("150")
        );
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let backend = MemoryBackend::new();
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
    fn set_reuses_the_original_case_of_an_existing_entry() {
        let backend = MemoryBackend::new();
        backend.set("MyApp", "Window", "Width", "600").unwrap();
        backend.set("myapp", "window", "width", "800").unwrap();
        assert_eq!(
            backend.get("MyApp", "Window", "Width").as_deref(),
            Some("800")
        );
        // In-memory, only one entry exists
        assert_eq!(backend.load_all().len(), 1);
    }

    #[test]
    fn get_all_returns_section_pairs_sorted_by_key() {
        let backend = MemoryBackend::new();
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
    fn remove_key_deletes_only_that_setting() {
        let backend = MemoryBackend::new();
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
        let backend = MemoryBackend::new();
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        backend.set("MyApp", "Startup", "Top", "40").unwrap();
        backend.remove_section("myapp", "startup").unwrap();
        assert!(backend.get_all("MyApp", "Startup").is_empty());
    }

    #[test]
    fn remove_appname_deletes_the_whole_application() {
        let backend = MemoryBackend::new();
        backend.set("MyApp", "Startup", "Left", "150").unwrap();
        backend.set("MyApp", "Other", "Top", "40").unwrap();
        backend.remove_appname("myapp").unwrap();
        assert!(backend.get_all("MyApp", "Startup").is_empty());
        assert!(backend.get_all("MyApp", "Other").is_empty());
    }

    #[test]
    fn entries_returns_every_setting_sorted() {
        let backend = MemoryBackend::new();
        backend.set("MyApp", "Startup", "Top", "40").unwrap();
        backend.set("myapp", "Startup", "left", "150").unwrap();
        backend.set("OtherApp", "Startup", "Left", "1").unwrap();
        assert_eq!(
            backend.entries(),
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
    fn from_entries_prepopulates_the_backend() {
        let mut entries = HashMap::new();
        entries.insert(
            (
                "myapp".to_string(),
                "startup".to_string(),
                "left".to_string(),
            ),
            Entry {
                path: PathCase {
                    appname: "MyApp".to_string(),
                    section: "Startup".to_string(),
                    key: "Left".to_string(),
                },
                value: "150".to_string(),
            },
        );
        let backend = MemoryBackend::from_entries(entries);
        assert_eq!(
            backend.get("MyApp", "Startup", "Left").as_deref(),
            Some("150")
        );
    }
}
