//! Process-global user interaction state for VB6.
//!
//! VB6 interaction functions (`Command$`, `DoEvents`, `Beep`, `MsgBox`,
//! `InputBox`, `AppActivate`, `Shell`) delegate to a pluggable
//! [`InteractionBackend`] so the same API works across platforms:
//!
//! - **Native** (default everywhere): reads real command-line args, yields
//!   the thread, beeps via the terminal bell character, shows real dialogs
//!   (`MessageBoxW` on Windows, `osascript` on macOS, `zenity` on Linux,
//!   browser `alert`/`confirm` on wasm32), and starts real processes
//!   (`CreateProcessW` on Windows, detached spawns with quoted command-line
//!   splitting on Linux/macOS).
//! - **Memory** (tests): injectable, deterministic behavior with no OS side
//!   effects — including scripted response lists for `MsgBox`, `InputBox`,
//!   `AppActivate`, and `Shell`.
//!
//! The backend can be switched at runtime with [`set_backend`]; test
//! harnesses wanting deterministic, scripted answers install
//! [`MemoryBackend`](memory::MemoryBackend).

pub mod appactivate;
pub mod backend;
pub mod inputbox;
pub mod memory;
pub mod msgbox;
pub mod native;
pub mod shell;

use std::sync::{Mutex, OnceLock};

pub use appactivate::{AppActivateRecord, AppActivateRequest};
pub use backend::InteractionBackend;
pub use inputbox::{InputBoxRecord, InputBoxRequest};
pub use msgbox::{
    MsgBoxButton, MsgBoxButtonSet, MsgBoxIcon, MsgBoxModality, MsgBoxRecord, MsgBoxRequest,
};
pub use shell::{ShellRecord, ShellRequest, WindowStyle};

/// The active interaction backend.
static BACKEND: OnceLock<Mutex<Box<dyn InteractionBackend>>> = OnceLock::new();

/// Get the active backend, initializing with the default if needed.
fn backend() -> &'static Mutex<Box<dyn InteractionBackend>> {
    BACKEND.get_or_init(|| Mutex::new(default_backend()))
}

/// Create the default backend for the current platform.
fn default_backend() -> Box<dyn InteractionBackend> {
    Box::new(native::NativeBackend::new())
}

/// Set the active interaction backend.
///
/// This is the primary way to switch how interaction operations behave
/// at runtime. Use [`MemoryBackend`](memory::MemoryBackend) for tests
/// needing deterministic, scripted responses.
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

/// Show a modal message box and report which button was clicked.
///
/// The `request` must already be validated (see
/// [`MsgBoxRequest::parse`]); this only routes it to the active backend.
/// Corresponds to VB6's `MsgBox` function.
pub fn msg_box(request: &MsgBoxRequest) -> crate::error::VBResult<MsgBoxButton> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .msg_box(request)
}

/// Show a modal input box and return the entered text (or `""` for Cancel).
///
/// The `request` must already be assembled (see [`InputBoxRequest::new`]);
/// this only routes it to the active backend. Corresponds to VB6's
/// `InputBox` function.
pub fn input_box(request: &InputBoxRequest) -> crate::error::VBResult<String> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .input_box(request)
}

/// Bring an application window to the foreground.
///
/// The `request` must already be assembled (see
/// [`AppActivateRequest::new`]); this only routes it to the active backend.
/// Corresponds to VB6's `AppActivate` statement: raises VB6 error 5 when a
/// platform with real windows has no matching window.
pub fn app_activate(request: &AppActivateRequest) -> crate::error::VBResult<()> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .app_activate(request)
}

/// Start a program asynchronously and return its task ID.
///
/// The `request` must already be validated (see [`ShellRequest::parse`]);
/// this only routes it to the active backend. Corresponds to VB6's `Shell`
/// function: the returned task ID is the child's process id on native
/// platforms, and a program that cannot be started raises VB6 error 53
/// ("File not found") or 70 ("Permission denied").
pub fn shell(request: &ShellRequest) -> crate::error::VBResult<f64> {
    backend()
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .shell(request)
}

/// Run `f` with the active backend downcast to [`MemoryBackend`].
///
/// Hosts and tests use this to reach the memory backend's scripting API
/// (queued `MsgBox` responses, the request log) after installing it with
/// [`set_backend`]. Returns `None` when the active backend is not the
/// memory backend.
pub fn with_memory_backend<R>(f: impl FnOnce(&memory::MemoryBackend) -> R) -> Option<R> {
    let guard = backend().lock().unwrap_or_else(|e| e.into_inner());
    guard
        .as_any()
        .downcast_ref::<memory::MemoryBackend>()
        .map(f)
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

    #[test]
    fn msg_box_routes_to_the_active_backend() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::with_msgbox_responses([
            msgbox::MsgBoxButton::Retry,
        ])));
        let request = MsgBoxRequest::parse("continue?", 5).unwrap();
        assert_eq!(msg_box(&request).unwrap(), msgbox::MsgBoxButton::Retry);
        reset_backend();
    }

    #[test]
    fn msg_box_defaults_without_scripted_responses() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::new()));
        // vbOKCancel: default button is OK.
        let request = MsgBoxRequest::parse("proceed?", 1).unwrap();
        assert_eq!(msg_box(&request).unwrap(), msgbox::MsgBoxButton::Ok);
        reset_backend();
    }

    #[test]
    fn input_box_routes_to_the_active_backend() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::with_input_responses([
            "scripted",
        ])));
        let request = InputBoxRequest::new("value?").with_default("default");
        assert_eq!(input_box(&request).unwrap(), "scripted");
        reset_backend();
    }

    #[test]
    fn input_box_defaults_without_scripted_responses() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::new()));
        let request = InputBoxRequest::new("value?").with_default("fallback");
        assert_eq!(input_box(&request).unwrap(), "fallback");
        reset_backend();
    }

    #[test]
    fn app_activate_routes_to_the_active_backend() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::with_activate_responses([
            false,
        ])));
        let request = AppActivateRequest::new("Calculator");
        assert!(app_activate(&request).is_err());
        reset_backend();
    }

    #[test]
    fn app_activate_defaults_without_scripted_responses() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::new()));
        assert!(app_activate(&AppActivateRequest::new("Calculator")).is_ok());
        reset_backend();
    }

    #[test]
    fn shell_routes_to_the_active_backend() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::with_shell_responses([
            31337.0,
        ])));
        assert_eq!(shell(&ShellRequest::new("calc.exe")).unwrap(), 31337.0);
        reset_backend();
    }

    #[test]
    fn shell_synthesizes_task_ids_without_scripted_responses() {
        let _guard = lock_test();
        set_backend(Box::new(memory::MemoryBackend::new()));
        let first = shell(&ShellRequest::new("notepad.exe")).unwrap();
        let second = shell(&ShellRequest::new("calc.exe")).unwrap();
        assert!(first > 0.0);
        assert_ne!(first, second);
        reset_backend();
    }
}
