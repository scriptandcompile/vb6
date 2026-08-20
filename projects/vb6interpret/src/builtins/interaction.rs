//! VB6 interaction function registry.
//!
//! One [`Builtin`](super::Builtin) entry per interaction function, each
//! wrapping the typed `vb6runtime::library::interaction`
//! implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::interaction::command::command;
use vb6runtime::library::interaction::command_dollar::command_dollar;
use vb6runtime::library::interaction::doevents::do_events;
use vb6runtime::VBVariant;

/// Register the interaction functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("command", 0, 0, |_args| {
        Ok(VBVariant::from(command()?))
    }));
    registry.insert(builtin!("command$", 0, 0, |_args| {
        Ok(VBVariant::from(command_dollar()?))
    }));
    registry.insert(builtin!("doevents", 0, 0, |_args| { do_events() }));
}

#[cfg(test)]
mod tests {
    use super::super::call_builtin;

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
}
