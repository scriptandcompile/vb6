//! Process-global runtime state shared across the VB6 standard library.
//!
//! VB6 keeps mutable process-wide state that spans individual functions and
//! statements rather than being owned by a single call: the environment
//! snapshot read by `Environ` and written by the `Environ` assignment
//! statement, the random-number generator seed shared by `Rnd` and the
//! `Randomize` statement, the current run-time error number read by the
//! omitted-argument form of `Error`/`Error$`, the application settings
//! store read by `GetSetting`/`GetAllSettings` and written by
//! `SaveSetting`/`DeleteSetting`, and the mock clock used by the `Date` and
//! `Time` statements. A host installs a controlled baseline before a program
//! runs; statements mutate it as the program runs.
//!
//! Each piece of state lives in its own submodule ([`clock`], [`environment`],
//! [`random`], [`err`], [`settings`]), exposing a small typed API over an
//! internal mutex or atomic so callers never touch the raw storage.

pub mod clock;
pub mod environment;
pub mod err;
pub mod file;
pub mod interaction;
pub mod random;
pub mod resources;
pub mod settings;

#[cfg(test)]
pub(crate) mod test_support {
    use std::path::Path;
    use std::sync::Mutex;

    /// Serializes tests that read or write shared runtime state so parallel
    /// test execution cannot interfere with a test's fixed state. Shared by
    /// every test module that touches the environment snapshot, the settings
    /// store, or the RNG seed.
    pub(crate) static TEST_LOCK: Mutex<()> = Mutex::new(());

    /// Acquire the test lock, recovering from poison if a prior test panicked.
    pub(crate) fn lock_test() -> std::sync::MutexGuard<'static, ()> {
        TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Find the 1-based position of `name` in the environment snapshot.
    pub(crate) fn position_of(name: &str) -> usize {
        let mut i = 1;
        while let Some(entry) = super::environment::env_at(i) {
            if entry.starts_with(&format!("{name}=")) {
                return i;
            }
            i += 1;
        }
        panic!("{name} not found in environment snapshot");
    }

    /// Run `f` with the settings store pointed at a fresh temporary directory.
    ///
    /// Serializes against [`TEST_LOCK`] and restores the default store root
    /// afterwards so the user's real settings are never touched and later
    /// tests start from a clean baseline.
    pub(crate) fn with_temp_settings_store<T>(f: impl FnOnce(&Path) -> T) -> T {
        let _guard = lock_test();
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        super::settings::set_store_root(dir.path());
        let result = f(dir.path());
        super::settings::reset_store_root();
        result
    }
}
