//! Recovered FAC2A60 typed-operator opcode selection tables.
//!
//! This selector returns logical opcode IDs, not dispatch-table identities.
//! Its inputs are native AST/type IDs; callers must map their HIR types and
//! operators to those IDs and resolve Variant storage separately.

/// Typed expression facts consumed by the recovered opcode selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedOperator {
    /// Native expression kind, bounded by the recovered FAC2BA8 table.
    pub kind: u16,
    /// Native result type ID.
    pub result_type: u16,
    /// Native left/only operand type ID.
    pub left_type: u16,
    /// Native right operand type ID for binary operators.
    pub right_type: Option<u16>,
    /// The high bit used by FAC2A60 for selected operator/type cases.
    pub aux_highbit: bool,
    /// Resolved Variant storage slot, when the result is a Variant.
    pub variant_slot: Option<u16>,
}

/// The logical opcode and optional Variant slot selected for an operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedInstruction {
    /// Logical ID consumed by the separate FAC2090 opcode encoder.
    pub logical_opcode: u16,
    /// Variant storage slot operand required by selected Variant results.
    pub operand: Option<u16>,
}

/// A selected operator resolved to a Microsoft dispatch-table identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedInstruction {
    /// Exact dispatch table and slot.
    pub opcode: crate::program::MicrosoftOpcodeId,
    /// Variant storage operand, when needed.
    pub operand: Option<u16>,
}

/// FAC2B97 bases indexed by native AST kind; `0x446` is the unsupported sentinel.
pub const OPCODE_BASE: [u16; 115] = [
    1094, 1094, 1094, 1094, 1094, 1094, 190, 198, 211, 227, 219, 242, 235, 237, 1094, 1094, 1094,
    1094, 1094, 1094, 1094, 1094, 142, 150, 174, 182, 206, 1094, 1094, 158, 166, 1, 9, 25, 17, 33,
    239, 119, 41, 54, 67, 80, 93, 106, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094,
    210, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094,
    1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094,
    1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094,
    1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 1094, 132,
];

/// Per-kind selection flags read by FAC2A60.
pub const AST_FLAGS: [u8; 115] = [
    0, 67, 67, 67, 67, 67, 68, 68, 68, 68, 68, 68, 68, 68, 68, 68, 4, 4, 4, 4, 4, 4, 72, 72, 72,
    72, 72, 72, 72, 72, 72, 104, 104, 104, 104, 104, 72, 88, 88, 88, 88, 88, 88, 88, 72, 72, 8, 8,
    8, 8, 8, 8, 8, 8, 72, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 4, 8, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 24,
];

/// Native type ID to typed-opcode delta table used by FAC2A60.
pub const TYPE_INDEX: [u16; 30] = [
    19, 19, 19, 9, 19, 0, 1, 19, 2, 19, 3, 4, 10, 5, 19, 6, 7, 14, 19, 19, 12, 19, 8, 2, 19, 19, 2,
    0, 19, 0,
];

fn type_index(ty: u16) -> Result<u16, String> {
    let index = *TYPE_INDEX
        .get(usize::from(ty))
        .ok_or_else(|| format!("native type ID 0x{ty:X} is outside the recovered table"))?;
    Ok(match index {
        10 => 4,
        9 => 1,
        _ => index,
    })
}

/// Select an opcode using the recovered kind base, flags, type table, and
/// comparison-specific cases. This performs no HIR type validation.
pub fn select(node: TypedOperator) -> Result<SelectedInstruction, String> {
    let base = *OPCODE_BASE.get(usize::from(node.kind)).ok_or_else(|| {
        format!(
            "native AST kind 0x{:X} is outside the recovered table",
            node.kind
        )
    })?;
    if base == 0x446 {
        return Err(format!(
            "native AST kind 0x{:X} has the unsupported sentinel",
            node.kind
        ));
    }
    let flags = AST_FLAGS[usize::from(node.kind)];
    let delta = if flags & 0x10 == 0 {
        type_index(node.result_type)?
    } else if node.kind != 0x72 && node.result_type == 3 && node.left_type == 15 {
        if node.aux_highbit { 11 } else { 10 }
    } else if node.left_type == 13 && matches!(node.right_type, Some(10 | 11 | 12)) {
        12
    } else {
        type_index(node.left_type)? + if node.aux_highbit { 2 } else { 0 }
    };
    let operand = if node.result_type == 15 {
        Some(
            node.variant_slot
                .ok_or("Variant result requires a resolved storage slot")?,
        )
    } else {
        None
    };
    Ok(SelectedInstruction {
        logical_opcode: base + delta,
        operand,
    })
}

/// Select a typed opcode and translate its logical ID through FAC2090.
pub fn select_microsoft(node: TypedOperator) -> Result<ResolvedInstruction, String> {
    let selected = select(node)?;
    Ok(ResolvedInstruction {
        opcode: crate::logical_opcode_map::microsoft_identity(selected.logical_opcode)?,
        operand: selected.operand,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(kind: u16, result_type: u16, left_type: u16, right_type: Option<u16>) -> TypedOperator {
        TypedOperator {
            kind,
            result_type,
            left_type,
            right_type,
            aux_highbit: false,
            variant_slot: None,
        }
    }

    #[test]
    fn selects_type_specialized_arithmetic_and_comparison() {
        assert_eq!(
            select(node(0x16, 8, 6, Some(6))).unwrap().logical_opcode,
            0x90
        );
        assert_eq!(
            select_microsoft(node(0x16, 8, 6, Some(6))).unwrap(),
            ResolvedInstruction {
                opcode: crate::program::MicrosoftOpcodeId::new(crate::DispatchTable::Primary, 0xaa),
                operand: None,
            }
        );
        assert_eq!(
            select(node(0x26, 3, 8, Some(8))).unwrap().logical_opcode,
            0x2b
        );
        let mut signed_comparison = node(0x26, 3, 8, Some(8));
        signed_comparison.aux_highbit = true;
        assert_eq!(select(signed_comparison).unwrap().logical_opcode, 0x2d);
    }

    #[test]
    fn normalizes_boolean_date_and_currency_comparison_types() {
        assert_eq!(select(node(6, 3, 3, None)).unwrap().logical_opcode, 0xbf);
        assert_eq!(select(node(7, 12, 12, None)).unwrap().logical_opcode, 0xca);
        for right_type in [10, 11, 12] {
            let mut comparison = node(0x26, 3, 13, Some(right_type));
            comparison.aux_highbit = true;
            assert_eq!(select(comparison).unwrap().logical_opcode, 0x35);
        }
    }

    #[test]
    fn requires_variant_result_slot_and_rejects_out_of_range_inputs() {
        let variant_result = node(0x16, 15, 15, Some(15));
        assert!(select(variant_result).is_err());
        assert_eq!(
            select(TypedOperator {
                variant_slot: Some(0x8234),
                ..variant_result
            })
            .unwrap()
            .operand,
            Some(0x8234)
        );
        assert!(select(node(0x73, 8, 8, None)).is_err());
        assert!(select(node(0x16, 30, 8, None)).is_err());
        assert!(select(node(0x00, 8, 8, None)).is_err());
    }
}
