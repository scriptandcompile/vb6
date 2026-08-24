//! VB6 array function registry.
//!
//! One [`typed_builtin!`](crate::typed_builtin) entry per array function.
//! `sourcearray` parameters stay Variant kinds because they are structural
//! (the runtime must observe Array-ness); scalar options use typed `opt_*`
//! kinds. `Array()` binds its element list as a raw rest slice — elements
//! are stored unconverted.

use super::{Builtin, Registry};
use crate::typed_builtin;
use vb6runtime::library::arrays as arrayfn;

/// Register the array functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(typed_builtin!("array", 0, usize::MAX, (),
        rest elements: variants,
        Ok(arrayfn::array::array(elements))));
    registry.insert(typed_builtin!("filter", 2, 4,
            (sourcearray: variant, matchstring: string,
             include: opt_boolean, compare: opt_long),
        arrayfn::filter::filter(sourcearray, &matchstring,
            include.as_ref(), compare.as_ref())));
    registry.insert(typed_builtin!("join", 1, 2,
        (sourcearray: variant, delimiter: opt_string),
        arrayfn::join::join(sourcearray,
            delimiter.as_ref()).map(vb6runtime::VBVariant::from_string)));
    registry.insert(typed_builtin!("lbound", 1, 2,
        (array: variant, dimension: opt_long),
        arrayfn::lbound::lbound(array, dimension.as_ref())));
    registry.insert(typed_builtin!("split", 1, 4,
            (expression: string, delimiter: opt_string,
             limit: opt_long, compare: opt_long),
        arrayfn::split::split(&expression, delimiter.as_ref(),
            limit.as_ref(), compare.as_ref())));
    registry.insert(typed_builtin!("ubound", 1, 2,
        (array: variant, dimension: opt_long),
        arrayfn::ubound::ubound(array, dimension.as_ref())));
}
