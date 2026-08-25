//! VB6 environment function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per environment
//! function. The settings-family path components are declared `As String`
//! in VB6 and use `string`/`opt_string` kinds (Null → 94 at the boundary).
//! Two categories of parameter stay raw by design:
//!
//! - `Environ`/`Environ$` take a single union argument — a String name or a
//!   numeric table position — so no single wrapper can coerce it; the runtime
//!   inspects the variant and owns the Null policy (`Environ` propagates,
//!   `Environ$` raises error 5).
//! - `Error`/`Error$` keep an optional raw Variant because VB6 documents an
//!   explicit `Empty` argument as "the current error number", which must stay
//!   distinct from a coercible number; `Null` still rejects with 94 in-body.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::environment::delete_setting::delete_setting;
use vb6runtime::library::environment::environ::environ;
use vb6runtime::library::environment::environ_dollar::environ_dollar;
use vb6runtime::library::environment::error::error;
use vb6runtime::library::environment::error_dollar::error_dollar;
use vb6runtime::library::environment::getallsettings::get_all_settings;
use vb6runtime::library::environment::getautoserversettings::get_auto_server_settings;
use vb6runtime::library::environment::getsetting::get_setting;
use vb6runtime::library::environment::savesetting::save_setting;

/// Register the environment functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("environ", 1, 1, (arg: variant),
        environ(arg)));
    registry.insert(typed_builtin!("environ$", 1, 1, (arg: variant),
        environ_dollar(arg)));
    registry.insert(typed_builtin!("error", 0, 1, (errornumber: opt_variant),
        error(errornumber)));
    registry.insert(typed_builtin!("error$", 0, 1, (errornumber: opt_variant),
        error_dollar(errornumber)));
    registry.insert(typed_builtin!("getsetting", 3, 4,
        (appname: string, section: string, key: string, default: opt_string),
        get_setting(&appname, &section, &key, default.as_ref())));
    registry.insert(typed_builtin!("savesetting", 4, 4,
        (appname: string, section: string, key: string, value: string),
        save_setting(&appname, &section, &key, &value)));
    registry.insert(typed_builtin!("deletesetting", 2, 3,
        (appname: string, section: string, key: opt_string),
        delete_setting(&appname, &section, key.as_ref())));
    registry.insert(typed_builtin!(
        "imestatus",
        0,
        0,
        (),
        vb6runtime::library::environment::imestatus::imestatus()
    ));
    registry.insert(typed_builtin!("getallsettings", 2, 2,
        (appname: string, section: string),
        get_all_settings(&appname, &section)));
    registry.insert(typed_builtin!("getautoserversettings", 3, 3,
        (progid: string, clsid: string, machine: string),
        get_auto_server_settings(&progid, &clsid, &machine)));
}
