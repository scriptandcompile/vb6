//! VB6 graphics function and statement registry.
//!
//! One [`Builtin`](super::Builtin) entry per graphics function, each wrapping
//! the `vb6runtime::library::graphics` implementation. The `SavePicture`
//! statement is not registered here: it has a dedicated statement kind that
//! the interpreter executes directly.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::graphics as graphicsfn;
use vb6runtime::VBVariant;

/// Register the graphics functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("loadpicture", 0, 1, |args| {
        let filename = match args.first() {
            Some(value) => Some(value.as_string()?),
            None => None,
        };
        graphicsfn::loadpicture::loadpicture(filename.as_deref())
    }));
}
