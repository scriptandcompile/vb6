//! Configurable snapshot of the operating system environment.
//!
//! `Environ$` reads from this snapshot instead of the live process environment
//! so a host can install a controlled environment before a program runs. The
//! snapshot is seeded from the real process environment on first access, and
//! can be rebuilt with [`reset`] or amended with [`set_env`].
//!
//! The table keeps its entries in environment-table order so the numeric
//! (by-position) form of `Environ$` enumerates deterministically. Variable
//! names are matched case-insensitively, matching VB6 on Windows.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// A `NAME`/`value` entry of the environment table.
type Entry = (String, String);

/// Ordered environment table plus a case-insensitive name index.
struct EnvState {
    /// Entries in environment-table order; `Environ$(n)` is entry `n - 1`.
    entries: Vec<Entry>,
    /// Lowercased name -> position in `entries`, for case-insensitive lookup.
    index: HashMap<String, usize>,
}

impl EnvState {
    /// Snapshot the current process environment.
    fn from_process() -> Self {
        let entries: Vec<Entry> = std::env::vars().collect();
        let index = entries
            .iter()
            .enumerate()
            .map(|(i, (name, _))| (normalize_key(name), i))
            .collect();
        Self { entries, index }
    }

    /// Set or replace the value of `name`, appending it when it is new.
    fn set(&mut self, name: &str, value: &str) {
        let key = normalize_key(name);
        match self.index.get(&key) {
            Some(&i) => self.entries[i].1 = value.to_string(),
            None => {
                self.index.insert(key, self.entries.len());
                self.entries.push((name.to_string(), value.to_string()));
            }
        }
    }

    /// Case-insensitive lookup of `name`, returning its value.
    fn get(&self, name: &str) -> Option<&str> {
        let key = normalize_key(name);
        self.index.get(&key).map(|&i| self.entries[i].1.as_str())
    }

    /// The `NAME=value` string at the 1-based position `position`, if any.
    fn at(&self, position: usize) -> Option<String> {
        self.entries
            .get(position - 1)
            .map(|(name, value)| format!("{name}={value}"))
    }
}

/// Lowercase a variable name for case-insensitive matching.
fn normalize_key(name: &str) -> String {
    name.to_ascii_lowercase()
}

static ENV: OnceLock<Mutex<EnvState>> = OnceLock::new();

/// Access the shared snapshot, seeding it from the process environment first.
fn snapshot() -> &'static Mutex<EnvState> {
    ENV.get_or_init(|| Mutex::new(EnvState::from_process()))
}

/// Lock the snapshot, recovering from a poisoned mutex.
fn lock() -> std::sync::MutexGuard<'static, EnvState> {
    snapshot()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Rebuild the snapshot from the process environment, discarding any values
/// installed with [`set_env`]. Useful when a host wants a fresh baseline;
/// otherwise the snapshot is seeded once on first access and amended by
/// [`set_env`].
pub fn reset() {
    *lock() = EnvState::from_process();
}

/// Set (or replace) the value of environment variable `name` in the snapshot.
pub fn set_env(name: &str, value: &str) {
    lock().set(name, value);
}

/// Remove `name` from the snapshot (case-insensitively).
pub fn remove_env(name: &str) {
    let mut state = lock();
    let key = normalize_key(name);
    if let Some(&i) = state.index.get(&key) {
        state.index.remove(&key);
        state.entries.remove(i);
        for position in state.index.values_mut() {
            if *position > i {
                *position -= 1;
            }
        }
    }
}

/// Case-insensitive lookup of `name`; `None` when the variable is not set.
pub fn get_env(name: &str) -> Option<String> {
    lock().get(name).map(str::to_string)
}

/// The `NAME=value` string at the 1-based position `position`, if any.
pub fn env_at(position: usize) -> Option<String> {
    lock().at(position)
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::Mutex;

    /// Serializes tests that read or write the shared environment snapshot so
    /// parallel test execution cannot interfere with a test's fixed
    /// environment. Shared by every environment test module.
    pub(crate) static TEST_LOCK: Mutex<()> = Mutex::new(());

    /// Find the 1-based position of `name` in the current snapshot.
    pub(crate) fn position_of(name: &str) -> usize {
        let mut i = 1;
        while let Some(entry) = super::env_at(i) {
            if entry.starts_with(&format!("{name}=")) {
                return i;
            }
            i += 1;
        }
        panic!("{name} not found in environment snapshot");
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{position_of, TEST_LOCK};
    use super::*;

    #[test]
    fn set_env_overwrites_in_place() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        set_env("VB6RUNTIME_TEST_ONE", "first");
        let position = position_of("VB6RUNTIME_TEST_ONE");
        set_env("vb6runtime_test_one", "second");
        assert_eq!(get_env("VB6RUNTIME_TEST_ONE").as_deref(), Some("second"));
        assert_eq!(
            env_at(position).as_deref(),
            Some("VB6RUNTIME_TEST_ONE=second")
        );
        // The overwrite must not append a second entry.
        assert_eq!(env_at(position + 1), None);
    }

    #[test]
    fn get_env_is_case_insensitive() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        set_env("VB6RUNTIME_TEST_CASE", "value");
        assert_eq!(get_env("vb6runtIme_test_case").as_deref(), Some("value"));
        assert_eq!(get_env("VB6RUNTIME_TEST_MISSING"), None);
    }

    #[test]
    fn env_at_returns_entries_in_order_and_reports_ends() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        set_env("VB6RUNTIME_TEST_A", "1");
        set_env("VB6RUNTIME_TEST_B", "2");
        set_env("VB6RUNTIME_TEST_C", "3");
        let first = position_of("VB6RUNTIME_TEST_A");
        assert_eq!(env_at(first).as_deref(), Some("VB6RUNTIME_TEST_A=1"));
        assert_eq!(env_at(first + 1).as_deref(), Some("VB6RUNTIME_TEST_B=2"));
        assert_eq!(env_at(first + 2).as_deref(), Some("VB6RUNTIME_TEST_C=3"));
        // Appended entries sit at the very end of the table.
        assert_eq!(env_at(first + 3), None);
    }

    #[test]
    fn reset_restores_the_process_environment() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_env("VB6RUNTIME_TEST_RESET", "gone");
        reset();
        assert_eq!(get_env("VB6RUNTIME_TEST_RESET"), None);
    }

    #[test]
    fn remove_env_deletes_and_compacts_positions() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset();
        set_env("VB6RUNTIME_TEST_A", "1");
        set_env("VB6RUNTIME_TEST_B", "2");
        let a = position_of("VB6RUNTIME_TEST_A");
        remove_env("vb6runtime_test_b");
        assert_eq!(get_env("VB6RUNTIME_TEST_B"), None);
        assert_eq!(get_env("VB6RUNTIME_TEST_A").as_deref(), Some("1"));
        // Removing B shifts the entry after it up by one position.
        assert_eq!(env_at(a + 1), None);
    }
}
