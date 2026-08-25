//! VB6 object function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per object function.
//! `TypeName` is a predicate: it observes the un-coerced variant (returning
//! `"Empty"`, `"Null"`, `"Nothing"`, class names, or array type names), so
//! its parameter stays raw by design.
//!
//! Not registered: `CreateObject`, `GetObject`, `CallByName` are doc-only
//! placeholders in the runtime; `Load`/`Unload` are statement forms.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::objects as objfn;

/// Register the object functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("typename", 1, 1, (value: variant),
        objfn::typename::type_name(value)));
}
