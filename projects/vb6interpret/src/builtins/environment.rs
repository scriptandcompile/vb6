//! VB6 environment function registry.
//!
//! One [`Builtin`](super::Builtin) entry per environment function, each
//! wrapping the typed `vb6runtime::library::environment`
//! implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::environment::environ::environ;
use vb6runtime::library::environment::environ_dollar::environ_dollar;
use vb6runtime::library::environment::error::error;
use vb6runtime::library::environment::error_dollar::error_dollar;
use vb6runtime::library::environment::getallsettings::get_all_settings;
use vb6runtime::library::environment::getsetting::get_setting;
use vb6runtime::library::environment::savesetting::save_setting;
use vb6runtime::VBVariant;

/// Register the environment functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("environ", 1, 1, |args| { environ(&args[0]) }));
    registry.insert(builtin!("environ$", 1, 1, |args| {
        environ_dollar(&args[0])
    }));
    registry.insert(builtin!("error", 0, 1, |args| {
        error(args.first().unwrap_or(&VBVariant::Empty))
    }));
    registry.insert(builtin!("error$", 0, 1, |args| {
        error_dollar(args.first().unwrap_or(&VBVariant::Empty))
    }));
    registry.insert(builtin!("getsetting", 3, 4, |args| {
        get_setting(
            &args[0],
            &args[1],
            &args[2],
            args.get(3).unwrap_or(&VBVariant::Empty),
        )
    }));
    registry.insert(builtin!("savesetting", 4, 4, |args| {
        save_setting(&args[0], &args[1], &args[2], &args[3])
    }));
    registry.insert(builtin!("imestatus", 0, 0, |_args| {
        vb6runtime::library::environment::imestatus::imestatus()
    }));
    registry.insert(builtin!("getallsettings", 2, 2, |args| {
        get_all_settings(&args[0], &args[1])
    }));
}
