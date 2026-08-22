//! VB6 interaction function registry.
//!
//! One [`Builtin`](super::Builtin) entry per interaction function, each
//! wrapping the typed `vb6runtime::library::interaction`
//! implementation.

use super::{arg_string, Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::interaction::beep::beep;
use vb6runtime::library::interaction::command::command;
use vb6runtime::library::interaction::command_dollar::command_dollar;
use vb6runtime::library::interaction::doevents::do_events;
use vb6runtime::library::interaction::msgbox::msg_box;
use vb6runtime::library::interaction::shell::shell;
use vb6runtime::value::{VBLong, VBString};
use vb6runtime::VBVariant;

/// Register the interaction functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    // `Beep` is a Sub, not a Function: it takes no arguments and yields no
    // value, so the registry entry returns `Empty`.
    registry.insert(builtin!("beep", 0, 0, |_args| {
        beep();
        Ok(VBVariant::Empty)
    }));
    registry.insert(builtin!("command", 0, 0, |_args| {
        Ok(VBVariant::from(command()?))
    }));
    registry.insert(builtin!("command$", 0, 0, |_args| {
        Ok(VBVariant::from(command_dollar()?))
    }));
    registry.insert(builtin!("doevents", 0, 0, |_args| { do_events() }));
    registry.insert(builtin!("msgbox", 1, 5, |args| {
        let prompt = arg_string(args, 0)?;
        let buttons = args.get(1).map(VBLong::try_from).transpose()?;
        let title = args.get(2).map(VBString::try_from).transpose()?;
        let helpfile = args.get(3).map(VBString::try_from).transpose()?;
        let context = args.get(4).map(VBLong::try_from).transpose()?;
        msg_box(
            &prompt,
            buttons.as_ref(),
            title.as_ref(),
            helpfile.as_ref(),
            context.as_ref(),
        )
    }));
    registry.insert(builtin!("shell", 1, 2, |args| {
        let pathname = arg_string(args, 0)?;
        shell(&pathname, args.get(1))
    }));
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
}
