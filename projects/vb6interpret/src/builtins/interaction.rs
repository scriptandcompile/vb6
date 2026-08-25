//! VB6 interaction function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per interaction
//! function, each wrapping the typed `vb6runtime::library::interaction`
//! implementation.
//!
//! - `Beep` is a Sub: no arguments, no value, so the entry returns `Empty`.
//! - `Command`/`Command$` return runtime strings that are lifted into
//!   Variants here.
//! - `MsgBox`'s optional tail and `Shell`'s window style are declared
//!   `As Long` / `As String` in VB6: present `Null` rejects with 94 at the
//!   boundary; value validation of the style itself stays in-body.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::interaction::beep::beep;
use vb6runtime::library::interaction::command::command;
use vb6runtime::library::interaction::command_dollar::command_dollar;
use vb6runtime::library::interaction::doevents::do_events;
use vb6runtime::library::interaction::msgbox::msg_box;
use vb6runtime::library::interaction::shell::shell;
use vb6runtime::VBVariant;

/// Register the interaction functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("beep", 0, 0, (), {
        beep();
        Ok(VBVariant::Empty)
    }));
    registry.insert(typed_builtin!(
        "command",
        0,
        0,
        (),
        command().map(VBVariant::from)
    ));
    registry.insert(typed_builtin!(
        "command$",
        0,
        0,
        (),
        command_dollar().map(VBVariant::from)
    ));
    registry.insert(typed_builtin!("doevents", 0, 0, (), do_events()));
    registry.insert(typed_builtin!("msgbox", 1, 5,
        (prompt: string, buttons: opt_long, title: opt_string,
         helpfile: opt_string, context: opt_long),
        msg_box(&prompt, buttons.as_ref(), title.as_ref(),
                helpfile.as_ref(), context.as_ref())));
    registry.insert(typed_builtin!("shell", 1, 2,
        (pathname: string, window_style: opt_long),
        shell(&pathname, window_style.as_ref())));
}

#[cfg(test)]
mod tests {
    use super::super::call_builtin;
    use vb6runtime::state::interaction::{self, MsgBoxButton};
    use vb6runtime::VBVariant;

    #[test]
    fn beep_dispatches() {
        assert_eq!(call_builtin("Beep", &[]).unwrap(), VBVariant::Empty);
    }

    #[test]
    fn beep_rejects_arguments() {
        let err = call_builtin("Beep", &[VBVariant::from_long(1)]).unwrap_err();
        assert_eq!(
            err.number,
            vb6core::error::err_number::WRONG_NUMBER_OF_ARGUMENTS
        );
    }

    #[test]
    fn command_dispatches() {
        let result = call_builtin("Command", &[]).unwrap();
        assert!(result.as_string().is_ok());
    }

    #[test]
    fn command_dollar_dispatches() {
        let result = call_builtin("Command$", &[]).unwrap();
        assert!(result.as_string().is_ok());
    }

    #[test]
    fn doevents_dispatches() {
        let result = call_builtin("DoEvents", &[]).unwrap();
        assert_eq!(result.as_vbinteger().unwrap(), 0.into());
    }

    /// The registry tests share the process-global interaction backend, so
    /// they serialize on this lock and always restore the default backend
    /// afterwards.
    static BACKEND_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn with_memory_backend(responses: &[MsgBoxButton], f: impl FnOnce()) {
        let _guard = BACKEND_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        interaction::set_backend(Box::new(
            vb6runtime::state::interaction::memory::MemoryBackend::with_msgbox_responses(
                responses.to_vec(),
            ),
        ));
        f();
        interaction::reset_backend();
    }

    #[test]
    fn msgbox_requires_a_prompt() {
        let err = call_builtin("MsgBox", &[]).unwrap_err();
        assert_eq!(
            err.number,
            vb6core::error::err_number::WRONG_NUMBER_OF_ARGUMENTS
        );
    }

    #[test]
    fn msgbox_returns_the_scripted_button_id() {
        with_memory_backend(&[MsgBoxButton::Yes], || {
            let result = call_builtin(
                "MsgBox",
                &[
                    VBVariant::from_string("save?"),
                    VBVariant::from_long(4), // vbYesNo
                ],
            )
            .unwrap();
            assert_eq!(result.as_vbinteger().unwrap(), 6.into()); // vbYes
        });
    }

    #[test]
    fn msgbox_defaults_to_vbok_without_scripting() {
        with_memory_backend(&[], || {
            let result = call_builtin("MsgBox", &[VBVariant::from_string("hi")]).unwrap();
            assert_eq!(result.as_vbinteger().unwrap(), 1.into()); // vbOK
        });
    }

    #[test]
    fn msgbox_reports_incompatible_scripted_response() {
        with_memory_backend(&[MsgBoxButton::Cancel], || {
            // Dialog offers Yes/No; the queued Cancel does not match.
            let err = call_builtin(
                "MsgBox",
                &[
                    VBVariant::from_string("go?"),
                    VBVariant::from_long(4), // vbYesNo
                ],
            )
            .unwrap_err();
            assert_eq!(err.number, 5);
            assert!(err.description.contains("Cancel"));
        });
    }

    #[test]
    fn msgbox_accepts_all_five_arguments_when_paired() {
        with_memory_backend(&[], || {
            let result = call_builtin(
                "MsgBox",
                &[
                    VBVariant::from_string("x"),
                    VBVariant::from_long(0),
                    VBVariant::from_string("Title"),
                    VBVariant::from_string("help.hlp"),
                    VBVariant::from_long(10),
                ],
            )
            .unwrap();
            assert_eq!(result.as_vbinteger().unwrap(), 1.into());
        });
    }

    #[test]
    fn shell_dispatches_and_returns_a_double_task_id() {
        let _guard = BACKEND_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        interaction::set_backend(Box::new(
            vb6runtime::state::interaction::memory::MemoryBackend::with_shell_responses([4242.0]),
        ));
        let result = call_builtin(
            "Shell",
            &[
                VBVariant::from_string("notepad.exe"),
                VBVariant::from_long(1), // vbNormalFocus
            ],
        )
        .unwrap();
        assert_eq!(result.as_f64().unwrap(), 4242.0);
        interaction::reset_backend();
    }

    #[test]
    fn shell_requires_a_pathname() {
        let err = call_builtin("Shell", &[]).unwrap_err();
        assert_eq!(
            err.number,
            vb6core::error::err_number::WRONG_NUMBER_OF_ARGUMENTS
        );
    }

    #[test]
    fn shell_rejects_undefined_window_styles() {
        let _guard = BACKEND_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        interaction::set_backend(Box::new(
            vb6runtime::state::interaction::memory::MemoryBackend::new(),
        ));
        // 5 is not a VbAppWinStyle value.
        let err = call_builtin(
            "Shell",
            &[VBVariant::from_string("x"), VBVariant::from_long(5)],
        )
        .unwrap_err();
        assert_eq!(err.number, 5);
        interaction::reset_backend();
    }

    #[test]
    fn optional_arguments_reject_null_at_the_boundary() {
        // Both rejections happen during argument conversion, before any
        // backend access, so no fixture is needed.
        let err = call_builtin(
            "MsgBox",
            &[
                VBVariant::from_string("x"),
                VBVariant::from_long(0),
                VBVariant::Null,
            ],
        )
        .unwrap_err();
        assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);

        let err =
            call_builtin("Shell", &[VBVariant::from_string("x"), VBVariant::Null]).unwrap_err();
        assert_eq!(err.number, vb6core::error::err_number::INVALID_USE_OF_NULL);
    }
}
