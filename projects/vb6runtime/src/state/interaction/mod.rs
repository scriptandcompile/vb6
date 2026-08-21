//! Process-global user interaction state for VB6.
//!
//! VB6 interaction functions (`Command$`, `DoEvents`, `Beep`) delegate to a
//! pluggable [`InteractionBackend`] so the same API works across platforms:
//!
//! - **Native** (default): reads real command-line args, yields the thread,
//!   and beeps via the terminal bell character.
//! - **WASM/tests**: [`MemoryBackend`](memory::MemoryBackend) provides
//!   injectable, deterministic behavior with no OS side effects.
//!
//! The backend can be switched at runtime with [`set_backend`].

pub mod backend;
pub mod memory;
pub mod native;

use std::sync::{Mutex, OnceLock};

pub use backend::InteractionBackend;

/// The active interaction backend.
static BACKEND: OnceLock<Mutex<Box<dyn InteractionBackend>>> = OnceLock::new();

/// Get the active backend, initializing with the default if needed.
fn backend() -> &'static Mutex<Box<dyn InteractionBackend>> {
    BACKEND.get_or_init(|| Mutex::new(default_backend()))
}

/// Create the default backend for the current platform.
fn default_backend() -> Box<dyn InteractionBackend> {
    if cfg!(target_arch = "wasm32") {
        Box::new(memory::MemoryBackend::new())
    } else {
        Box::new(native::NativeBackend::new())
    }
}

/// Set the active interaction backend.
///
/// This is the primary way to switch how interaction operations behave
/// at runtime. Use [`MemoryBackend`](memory::MemoryBackend) for tests or
/// WASM hosts that need injectable command-line arguments.
pub fn set_backend(new_backend: Box<dyn InteractionBackend>) {
    *backend().lock().unwrap_or_else(|e| e.into_inner()) = new_backend;
}

/// Reset to the default backend for the current platform.
pub fn reset_backend() {
    set_backend(default_backend());
}

/// Get the command-line arguments passed to the program.
///
/// Returns individual arguments (not including the program name).
/// Corresponds to VB6's `Command` and `Command$` functions.
pub fn command_args() -> Vec<String> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .command_args()
}

/// Get the command-line arguments as a single space-joined string.
///
/// This is the `Command$` / `Command` return value: arguments joined
/// with spaces, or an empty string when there are none.
pub fn command_string() -> String {
    command_args().join(" ")
}

/// Yield execution to the operating system.
///
/// Returns the number of open forms (always 0 for our interpreter).
/// Corresponds to VB6's `DoEvents` function.
pub fn do_events() -> i16 {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .do_events()
}

/// Play the system beep sound.
///
/// On native platforms this writes the terminal bell character (`\x07`)
/// to stderr. On WASM this is a no-op.
/// Corresponds to VB6's `Beep` statement.
pub fn beep() {
    backend().lock().unwrap_or_else(|e| e.into_inner()).beep()
}

/// Signal a `Stop` statement break request.
///
/// Backends with an attached debugger (the WASM playground) record the
/// request so the host can enter break mode; platforms without one ignore
/// it and the interpreter applies the compiled-`.exe` behavior instead.
/// Corresponds to VB6's `Stop` statement.
pub fn stop() {
    backend().lock().unwrap_or_else(|e| e.into_inner()).stop()
}

/// Reset all interaction state (for testing).
pub fn reset() {
    reset_backend();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::test_support::lock_test;

    #[test]
    fn default_backend_returns_args() {
        let _guard = lock_test();
        // Just verify it doesn't panic; actual args depend on test runner.
        let _args = command_args();
    }

    #[test]
    fn command_string_joins_with_spaces() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::with_args(vec![
            "foo".into(),
            "bar baz".into(),
        ])));
        assert_eq!(command_string(), "foo bar baz");
        reset_backend();
    }

    #[test]
    fn command_string_empty_when_no_args() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::new()));
        assert_eq!(command_string(), "");
        reset_backend();
    }

    #[test]
    fn do_events_returns_zero() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::new()));
        assert_eq!(do_events(), 0i16);
        reset_backend();
    }

    #[test]
    fn beep_does_not_panic() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::new()));
        beep();
        reset_backend();
    }

    #[test]
    fn stop_does_not_panic() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::new()));
        stop();
        reset_backend();
    }
}
