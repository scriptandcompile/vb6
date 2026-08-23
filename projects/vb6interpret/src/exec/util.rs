//! Small shared helpers used across the execution submodules.

use vb6parse::parsers::cst::CstNode;
use vb6runtime::VBVariant;

/// Number of `\n` characters in a node's text span.
pub(crate) fn count_newlines(node: &CstNode) -> usize {
    node.text().matches('\n').count()
}

/// Coerce a value to a static type following VB6 conversion semantics.
pub(crate) fn coerce(value: VBVariant, ty: &vb6core::types::VBType) -> VBVariant {
    match ty {
        vb6core::types::VBType::Byte => value.as_byte().map(VBVariant::Byte).unwrap_or(value),
        vb6core::types::VBType::Integer => value.as_i16().map(VBVariant::Integer).unwrap_or(value),
        vb6core::types::VBType::Long => value.as_i32().map(VBVariant::Long).unwrap_or(value),
        vb6core::types::VBType::Single => value.as_f32().map(VBVariant::Single).unwrap_or(value),
        vb6core::types::VBType::Double => value.as_f64().map(VBVariant::Double).unwrap_or(value),
        vb6core::types::VBType::Currency => value
            .as_currency_scaled()
            .map(VBVariant::Currency)
            .unwrap_or(value),
        vb6core::types::VBType::String => value
            .as_string()
            .map(VBVariant::from_string)
            .unwrap_or(value),
        vb6core::types::VBType::Boolean => value.as_bool().map(VBVariant::Boolean).unwrap_or(value),
        vb6core::types::VBType::Date => {
            value.as_date_serial().map(VBVariant::Date).unwrap_or(value)
        }
        _ => value,
    }
}
