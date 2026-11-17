//! Counts, name indices and symbol metadata must come from bound AST/resolved
//! pools; no guessed helper results and no opaque runtime fallback are used.
use crate::addressing::{AddressError, AddressInput, AddressPlan, select_address};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CleanupStep {
    /// Concrete AST evaluation; caller uses its ordinary AST dispatcher.
    Evaluate {
        node: u32,
        mode: u16,
        original_function: u32,
    },
    Emit {
        opcode: u16,
        payload: Vec<u8>,
        original_function: u32,
    },
    /// Raw native operand words following a previously emitted opcode.
    Words(Vec<u16>),
    Address(AddressPlan),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupPlan {
    pub steps: Vec<CleanupStep>,
    pub native_return: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CleanupError {
    UnknownCategory(u8),
    MissingMetadata(&'static str),
    UnknownKind(u16),
    UnresolvedNativeBranch(&'static str),
    UnknownVariantTag(u16),
    Address(AddressError),
}
#[derive(Debug, Clone)]
pub struct CleanupListInput {
    /// Native AST word1, including category bits8/9 and H bit15.
    pub flags: u16,
    /// Count read from *(AST[5][4])+0x10 when H is clear. H forces count1.
    pub count: Option<u16>,
    /// Ordered slots obtained after list37 traversal and wrapper11 stripping.
    /// Native code does not check this list length against its count.
    pub symbol_slots: Vec<u16>,
}
fn emit(opcode: u16, words: &[u16], function: u32) -> CleanupStep {
    CleanupStep::Emit {
        opcode,
        payload: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
        original_function: function,
    }
}
fn need<T>(value: Option<T>, name: &'static str) -> Result<T, CleanupError> {
    value.ok_or(CleanupError::MissingMetadata(name))
}
/// Kind41; generic categories0..2 are exact, category3 is deliberately unresolved.
pub fn plan_cleanup_list(i: CleanupListInput) -> Result<CleanupPlan, CleanupError> {
    let category = ((i.flags & 0x300) >> 8) as u8;
    if category > 2 {
        return Err(CleanupError::UnknownCategory(category));
    }
    let h = i.flags & 0x8000 != 0;
    let count = if h {
        1
    } else {
        need(i.count, "native list count")?
    };
    let opcode = if h { 0x299 } else { 0x3b2 } + u16::from(category);
    let mut steps = vec![emit(
        opcode,
        if count < 2 {
            vec![]
        } else {
            vec![count.wrapping_mul(2)]
        }
        .as_slice(),
        0x0fabf2d2,
    )];
    if !i.symbol_slots.is_empty() {
        steps.push(CleanupStep::Words(i.symbol_slots));
    }
    Ok(CleanupPlan {
        steps,
        native_return: 0,
    })
}
#[derive(Debug, Clone)]
pub struct VariantAccess {
    /// Native VARTYPE stored at variant address+0.
    pub tag: u16,
    /// Native bytes at variant address+8. Tag4 already holds f32 bytes,
    /// unlike literal optimization FAC31C9 which narrows f64 to f32.
    pub bytes_at_8: [u8; 8],
    pub destination_slot: Option<u16>,
    /// FAB268C followed by FAC2E29; must be resolved externally for tag8.
    pub interned_string: Option<u16>,
}
#[derive(Debug, Clone)]
pub struct CleanupTarget {
    /// Actual AST passed to FAC3668: original RHS[4], not stripped probe AST.
    pub node: u32,
    pub ast_kind: u16,
    pub native_type: u16,
    pub flags_word: u32,
    /// Child at AST[4] for wrapper12.
    pub wrapped_child: Option<u32>,
    /// FAC0F35-resolved addressing descriptor/input for kinds60/69.
    pub resolved_address: Option<AddressInput>,
    pub variant: Option<VariantAccess>,
}
#[derive(Debug, Clone)]
pub struct NamedCleanupInput {
    pub ast_kind: u16,
    pub native_type: u16,
    /// FAC2E29(name AST after optional one wrapper11).
    pub name_index: Option<u16>,
    /// Kind reached by probing RHS through wrappers11/12.
    pub unwrapped_rhs_kind: u16,
    /// Exact bool FAB0B42(symbol): symbol kind4 and field(byte+1)&7 ==4.
    pub symbol_special: Option<bool>,
    /// FAAB81F(symbol) used by specialized kind60/type2 path.
    pub symbol_slot: Option<u16>,
    pub target: Option<CleanupTarget>,
}
fn result_adjustment(ty: u16, source_rep: u16, mode: u16) -> Option<u16> {
    match ty {
        2 if source_rep == 0x17 => Some(0x18b),
        10 if mode == 1 || mode == 3 => Some(0x189),
        11 | 12 if mode == 1 || mode == 3 => Some(0x18a),
        15 if mode == 3 => Some(0x18c),
        _ => None,
    }
}
/// Port of FAC3668's mode5 path, including direct variant writes and resolved
/// addressing. Evaluation steps are explicit work, not assumed helper outputs.
fn append_access(target: CleanupTarget, steps: &mut Vec<CleanupStep>) -> Result<(), CleanupError> {
    let function = 0x0fac3668;
    match target.ast_kind {
        0x12 => steps.push(CleanupStep::Evaluate {
            node: need(target.wrapped_child, "wrapper12 child")?,
            mode: 6,
            original_function: function,
        }),
        0x60 | 0x69 => {
            if target.ast_kind == 0x69 && target.flags_word & 0x8000 != 0 {
                return Err(CleanupError::UnresolvedNativeBranch("FUN_0fb35088"));
            }
            let mut input = need(target.resolved_address, "FUN_0fac0f35 resolved descriptor")?;
            input.mode = 5;
            input.flags = 0;
            input.native_type = target.native_type;
            steps.push(CleanupStep::Address(
                select_address(input).map_err(CleanupError::Address)?,
            ));
        }
        5 => {
            let v = need(target.variant, "native Variant descriptor")?;
            let slot = need(v.destination_slot, "FUN_0faab81f")?;
            let (opcode, len) = match v.tag {
                0 => (Some(0x167), 0),
                1 => (Some(0x164), 0),
                2 => (Some(0x3c0), 2),
                3 => (Some(0x3c1), 4),
                4 => (Some(0x3c2), 4),
                5 => (Some(0x3c4), 8),
                6 => (Some(0x3c3), 8),
                7 => (Some(0x3c5), 8),
                8 => (Some(0x3c6), 2),
                10 => (
                    if target.flags_word & 0x8000 != 0 {
                        Some(0x168)
                    } else {
                        None
                    },
                    0,
                ),
                11 => (
                    Some(
                        if u16::from_le_bytes([v.bytes_at_8[0], v.bytes_at_8[1]]) == 0 {
                            0x166
                        } else {
                            0x165
                        },
                    ),
                    0,
                ),
                0x11 => (Some(0x33d), 2),
                _ => return Err(CleanupError::UnknownVariantTag(v.tag)),
            };
            if let Some(opcode) = opcode {
                let mut payload = slot.to_le_bytes().to_vec();
                if v.tag == 8 {
                    payload.extend_from_slice(
                        &need(v.interned_string, "FUN_0fab268c/FUN_0fac2e29")?.to_le_bytes(),
                    );
                } else if v.tag == 0x11 {
                    payload.extend_from_slice(&u16::from(v.bytes_at_8[0]).to_le_bytes());
                } else {
                    payload.extend_from_slice(&v.bytes_at_8[..len]);
                }
                steps.push(CleanupStep::Emit {
                    opcode,
                    payload,
                    original_function: function,
                });
            }
            if target.flags_word & 0x4000 != 0 {
                steps.push(emit(0x202, &[], function));
            }
            // FAC0B67(type15, source0, mode5) emits nothing.
        }
        _ => steps.push(CleanupStep::Evaluate {
            node: target.node,
            mode: if target.native_type == 15 { 1 } else { 5 },
            original_function: function,
        }),
    }
    Ok(())
}
/// Kinds42/43, FB0B52F. Name pool and symbol classification are explicit.
pub fn plan_named_cleanup(i: NamedCleanupInput) -> Result<CleanupPlan, CleanupError> {
    if i.ast_kind != 0x42 && i.ast_kind != 0x43 {
        return Err(CleanupError::UnknownKind(i.ast_kind));
    }
    let name = need(i.name_index, "FUN_0fac2e29(name)")?;
    let mut steps = Vec::new();
    if i.unwrapped_rhs_kind == 0x60 && i.native_type == 2 && need(i.symbol_special, "FUN_0fab0b42")?
    {
        let slot = need(i.symbol_slot, "FUN_0faab81f(symbol)")?;
        steps.push(emit(
            if i.ast_kind == 0x42 { 0x2ff } else { 0x3f6 },
            &[slot, name],
            0x0fb0b52f,
        ));
    } else {
        append_access(need(i.target, "original RHS[4] target")?, &mut steps)?;
        steps.push(emit(0x42a, &[name], 0x0fb0b52f));
        if let Some(op) = result_adjustment(i.native_type, 0x17, 1) {
            steps.push(emit(op, &[], 0x0fac0b67));
        }
    }
    Ok(CleanupPlan {
        steps,
        native_return: 0,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn category_h_and_count_payload_are_exact() {
        for category in 0..=2 {
            let p = plan_cleanup_list(CleanupListInput {
                flags: category << 8,
                count: Some(3),
                symbol_slots: vec![7, 8, 9],
            })
            .unwrap();
            assert_eq!(p.steps[0], emit(0x3b2 + category, &[6], 0x0fabf2d2));
            let p = plan_cleanup_list(CleanupListInput {
                flags: 0x8000 | category << 8,
                count: None,
                symbol_slots: vec![7],
            })
            .unwrap();
            assert_eq!(p.steps[0], emit(0x299 + category, &[], 0x0fabf2d2));
        }
    }
    #[test]
    fn category_three_is_not_guessed() {
        assert_eq!(
            plan_cleanup_list(CleanupListInput {
                flags: 0x300,
                count: Some(1),
                symbol_slots: vec![]
            }),
            Err(CleanupError::UnknownCategory(3))
        );
    }
    fn named(kind: u16, ty: u16) -> NamedCleanupInput {
        NamedCleanupInput {
            ast_kind: kind,
            native_type: ty,
            name_index: Some(12),
            unwrapped_rhs_kind: 1,
            symbol_special: None,
            symbol_slot: None,
            target: Some(CleanupTarget {
                node: 123,
                ast_kind: 1,
                native_type: ty,
                flags_word: 0,
                wrapped_child: None,
                resolved_address: None,
                variant: None,
            }),
        }
    }
    #[test]
    fn fast_paths_have_exact_slot_and_name_words() {
        for (kind, op) in [(0x42, 0x2ff), (0x43, 0x3f6)] {
            let mut i = named(kind, 2);
            i.unwrapped_rhs_kind = 0x60;
            i.symbol_special = Some(true);
            i.symbol_slot = Some(7);
            assert_eq!(
                plan_named_cleanup(i).unwrap().steps,
                vec![emit(op, &[7, 12], 0x0fb0b52f)]
            );
        }
    }
    #[test]
    fn general_variant_and_type22_use_different_eval_modes() {
        for (ty, mode) in [(15, 1), (22, 5)] {
            let p = plan_named_cleanup(named(0x42, ty)).unwrap();
            assert_eq!(
                p.steps[0],
                CleanupStep::Evaluate {
                    node: 123,
                    mode,
                    original_function: 0x0fac3668
                }
            );
            assert_eq!(p.steps[1], emit(0x42a, &[12], 0x0fb0b52f));
        }
    }
    #[test]
    fn wrapper_requests_mode6_and_float_result_adjustment() {
        let mut i = named(0x43, 11);
        i.target.as_mut().unwrap().ast_kind = 0x12;
        i.target.as_mut().unwrap().wrapped_child = Some(456);
        let p = plan_named_cleanup(i).unwrap();
        assert_eq!(
            p.steps,
            vec![
                CleanupStep::Evaluate {
                    node: 456,
                    mode: 6,
                    original_function: 0x0fac3668
                },
                emit(0x42a, &[12], 0x0fb0b52f),
                emit(0x18a, &[], 0x0fac0b67)
            ]
        );
    }
    #[test]
    fn variant_byte_is_zero_extended_and_flag4000_emits202() {
        let mut i = named(0x42, 15);
        let t = i.target.as_mut().unwrap();
        t.ast_kind = 5;
        t.flags_word = 0x4000;
        t.variant = Some(VariantAccess {
            tag: 0x11,
            bytes_at_8: [0xfe, 0xab, 0, 0, 0, 0, 0, 0],
            destination_slot: Some(7),
            interned_string: None,
        });
        let p = plan_named_cleanup(i).unwrap();
        assert_eq!(
            p.steps[0],
            CleanupStep::Emit {
                opcode: 0x33d,
                payload: vec![7, 0, 0xfe, 0],
                original_function: 0x0fac3668
            }
        );
        assert_eq!(p.steps[1], emit(0x202, &[], 0x0fac3668));
    }
}
