//! VB6 array function registry.
//!
//! One [`Builtin`](super::Builtin) entry per array function, each wrapping the
//! typed `vb6runtime::library::functions::arrays` implementation.

use super::{Builtin, Registry};
use crate::builtin;
use vb6core::error::VBResult;
use vb6runtime::library::functions::arrays as arrayfn;
use vb6runtime::value::VBString;
use vb6runtime::VBVariant;

/// Register the array functions in `registry`.
pub(super) fn register(registry: &mut Registry) {
    registry.insert(builtin!("array", 0, usize::MAX, |args| {
        Ok(arrayfn::array::array(args))
    }));
    registry.insert(builtin!("filter", 2, 4, |args| {
        let match_string = VBString::try_from(&args[1])?;
        let include = args.get(2).map(VBVariant::as_bool).transpose()?;
        let compare = args.get(3).map(VBVariant::as_i32).transpose()?;
        arrayfn::filter::filter(&args[0], match_string.as_str(), include, compare)
    }));
    registry.insert(builtin!("join", 1, 2, |args| {
        let delimiter = args.get(1).map(VBString::try_from).transpose()?;
        let delimiter = delimiter.as_ref().map(VBString::as_str);
        arrayfn::join::join(&args[0], delimiter).map(VBVariant::from_string)
    }));
    registry.insert(builtin!("lbound", 1, 2, |args| {
        let dimension = args.get(1).map(VBVariant::as_i32).transpose()?;
        arrayfn::lbound::lbound(&args[0], dimension)
    }));
    registry.insert(builtin!("split", 1, 4, |args| {
        let expression = VBString::try_from(&args[0])?;
        let delimiter = args.get(1).map(VBString::try_from).transpose()?;
        let delimiter = delimiter.as_ref().map(VBString::as_str);
        let limit = args.get(2).map(VBVariant::as_i32).transpose()?;
        let compare = args.get(3).map(VBVariant::as_i32).transpose()?;
        arrayfn::split::split(expression.as_str(), delimiter, limit, compare)
    }));
    registry.insert(builtin!("ubound", 1, 2, |args| {
        let dimension = args.get(1).map(VBVariant::as_i32).transpose()?;
        arrayfn::ubound::ubound(&args[0], dimension)
    }));
}
