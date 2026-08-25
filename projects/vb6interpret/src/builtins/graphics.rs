//! VB6 graphics function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per graphics function.
//! `LoadPicture`'s single argument is declared As String in VB6 with an
//! omitted form meaning "an empty picture", so it uses the `opt_string` kind
//! (absent → `None`, present `Null` → 94 at the boundary).
//!
//! Not registered here: `SavePicture` is a statement form executed directly
//! by the interpreter, and `RGB`/`QBColor`/`Spc`/`Tab` have no registry
//! entries yet (`RGB` keeps a genuinely-Variant parameter — its per-type
//! error selection observes the raw variant).

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::graphics::loadpicture::loadpicture;

/// Register the graphics functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("loadpicture", 0, 1, (filename: opt_string),
        loadpicture(filename.as_ref())));
}
