//! # `AppActivate` Statement
//!
//! Activates an application window — gives it the focus.
//!
//! ## Syntax
//!
//! ```vb
//! AppActivate title[, wait]
//! ```
//!
//! ## Arguments
//!
//! - **title**: String expression or numeric Shell task ID naming the
//!   window to activate. String matching follows VB6 rules: the first
//!   window whose title begins with `title` wins; if none, one whose title
//!   ends with `title`. Comparisons are case-insensitive.
//! - **wait** (optional): Boolean. When `True`, activation is deferred
//!   until the calling application itself has the focus; `False` (the
//!   default) activates immediately.
//!
//! ## Remarks
//!
//! - **Error 5**: Raises "Invalid procedure call or argument" when no open
//!   window matches `title`.
//! - **Task IDs**: A numeric `title` — usually the return value of `Shell`
//!   — selects the window of that task.
//! - **Headless platforms**: Where no window facility exists (CI machines,
//!   WASM), the request is logged and the call succeeds so programs stay
//!   runnable.
//!
//! ## Examples
//!
//! ### Activating Notepad before sending keystrokes
//!
//! ```vb
//! AppActivate "Notepad"
//! SendKeys "Hello, world"
//! ```
//!
//! ### Using a Shell task ID
//!
//! ```vb
//! Dim taskId As Double
//! taskId = Shell("CALC.EXE", vbNormalFocus)
//! AppActivate taskId
//! ```
//!
//! ## See Also
//!
//! - `Shell` function (run a program, returning its task ID)
//! - `SendKeys` statement (send keystrokes to the active window)
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/appactivate-statement)

use crate::error::VBResult;
use crate::state;
use crate::value::VBString;

/// Implement VB6's `AppActivate` statement.
///
/// Assembles the request and hands it to the active
/// [`interaction backend`](crate::state::interaction), which locates the
/// window (caption prefix match, then suffix match; numeric titles are also
/// tried as Shell task IDs) and brings it to the foreground. Fails with
/// error 5 when no window matches on a platform that has windows.
pub fn app_activate(title: &VBString, wait: bool) -> VBResult<()> {
    state::interaction::app_activate(
        &state::interaction::AppActivateRequest::new(title.as_str()).with_wait(wait),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::interaction::{memory::MemoryBackend, AppActivateRecord};
    use crate::state::test_support::lock_test;

    #[test]
    fn activation_succeeds_by_default() {
        let _guard = lock_test();
        crate::state::interaction::set_backend(Box::new(MemoryBackend::new()));
        app_activate(&VBString::from("Calculator"), false).unwrap();
        assert_eq!(
            crate::state::interaction::with_memory_backend(
                |backend| backend.take_appactivate_requests()
            ),
            Some(vec![AppActivateRecord {
                title: "Calculator".into(),
                wait: false,
            }]),
        );
        crate::state::interaction::reset_backend();
    }

    #[test]
    fn scripted_failure_is_error_5_with_the_wait_flag_forwarded() {
        let _guard = lock_test();
        let backend = MemoryBackend::new();
        backend.push_activate_response(false);
        crate::state::interaction::set_backend(Box::new(backend));

        let err = app_activate(&VBString::from("Ghost"), true).unwrap_err();
        assert_eq!(err.number, 5);
        assert!(err.description.contains("Ghost"), "{}", err.description);
        crate::state::interaction::reset_backend();
    }

    #[test]
    fn recorded_requests_capture_wait() {
        let _guard = lock_test();
        crate::state::interaction::set_backend(Box::new(MemoryBackend::new()));
        app_activate(&VBString::from("Notepad"), true).unwrap();
        let requests = crate::state::interaction::with_memory_backend(|backend| {
            backend.take_appactivate_requests()
        })
        .unwrap();
        assert!(requests[0].wait);
        crate::state::interaction::reset_backend();
    }
}
