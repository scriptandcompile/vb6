//! Process-global runtime state shared across the VB6 standard library.
//!
//! VB6 keeps mutable process-wide state that spans individual functions and
//! statements rather than being owned by a single call: the environment
//! snapshot read by `Environ` and written by the `Environ` assignment
//! statement, the random-number generator seed shared by `Rnd` and the
//! `Randomize` statement, and the current run-time error number read by the
//! omitted-argument form of `Error`/`Error$`. A host installs a controlled
//! baseline before a program runs; statements mutate it as the program runs.
//!
//! Each piece of state lives in its own submodule ([`environment`],
//! [`random`], [`err`]), exposing a small typed API over an internal mutex or
//! atomic so callers never touch the raw storage.

pub mod environment;
pub mod err;
pub mod random;

#[cfg(test)]
pub(crate) mod test_support {
    use std::sync::Mutex;

    /// Serializes tests that read or write shared runtime state so parallel
    /// test execution cannot interfere with a test's fixed state. Shared by
    /// every test module that touches the environment snapshot or RNG seed.
    pub(crate) static TEST_LOCK: Mutex<()> = Mutex::new(());

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
}
