//! Native conversion selection ports, with explicit resolved helper outputs.
//! Source: research/sources/decompiled/0fac0c65.c and 0fac31c9.c.
//! Type IDs are native numeric tags; this module does not prove type legality.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionInstruction {
    pub opcode: u16,
    pub payload: Vec<u8>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionPlan {
    pub instructions: Vec<ConversionInstruction>,
    pub source_function: u32,
    pub source_line: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionError {
    UnresolvedHelper(&'static str),
    UnknownType(u16),
    UnsafeTableIndex(u8),
    UnknownComputedOpcode(u16),
    NativeHelperFailure(&'static str),
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ResolvedConversionMetadata {
    pub destination_slot: Option<u16>,
    pub source_descriptor: Option<u16>,
    pub destination_reference: Option<u16>,
    pub source_type_descriptor: Option<u16>,
    pub destination_type_descriptor: Option<u16>,
    pub source_reference: Option<u16>,
    pub source_22_resolution: Option<Result<bool, ()>>,
}
#[derive(Debug, Clone, Copy)]
pub struct ConversionInput {
    pub destination_type: u16,
    pub source_type: u16,
    pub destination_flags_word: u32,
    pub metadata: ResolvedConversionMetadata,
}
const TYPE_INDEX: [u8; 30] = [
    19, 19, 19, 9, 19, 0, 1, 19, 2, 19, 3, 4, 10, 5, 19, 6, 7, 14, 19, 19, 12, 19, 8, 2, 19, 19, 2,
    0, 19, 0,
];
// Only the first twelve dwords at FAC2940 are conversion bases. Later bytes
// belong to the switch jump table and must never become generated opcodes.
const BASE: [u16; 12] = [
    0x10c, 0x114, 0x11c, 0x124, 0x12c, 0x104, 0x267, 0xfc, 0x446, 0x150, 0x12c, 0x293,
];
fn index(t: u16) -> Result<u8, ConversionError> {
    let i = *TYPE_INDEX
        .get(t as usize)
        .ok_or(ConversionError::UnknownType(t))?;
    if i >= 12 {
        Err(ConversionError::UnsafeTableIndex(i))
    } else {
        Ok(i)
    }
}
fn normalized(i: u8) -> u16 {
    match i {
        9 => 1,
        10 => 4,
        _ => i as u16,
    }
}
fn required<T>(v: Option<T>, name: &'static str) -> Result<T, ConversionError> {
    v.ok_or(ConversionError::UnresolvedHelper(name))
}
fn plan(function: u32, line: u16, opcode: Option<u16>, words: &[u16]) -> ConversionPlan {
    ConversionPlan {
        source_function: function,
        source_line: line,
        instructions: opcode
            .into_iter()
            .map(|opcode| ConversionInstruction {
                opcode,
                payload: words.iter().flat_map(|w| w.to_le_bytes()).collect(),
            })
            .collect(),
    }
}
/// A successful table lookup is observed selection, not proof of legal VB input.
pub fn select_conversion(i: ConversionInput) -> Result<ConversionPlan, ConversionError> {
    let plan = select_conversion_impl(i)?;
    for instruction in &plan.instructions {
        if instruction.opcode >= 1095 {
            return Err(ConversionError::UnknownComputedOpcode(instruction.opcode));
        }
    }
    Ok(plan)
}
fn select_conversion_impl(i: ConversionInput) -> Result<ConversionPlan, ConversionError> {
    let (d, s, m) = (i.destination_type, i.source_type, i.metadata);
    let out = |line, op, words: &[u16]| plan(0x0fac0c65, line, op, words);
    if d != 15 {
        if d == 12 {
            match s {
                11 => return Ok(out(28, Some(0x147), &[])),
                15 => return Ok(out(32, Some(0x14f), &[])),
                16 => return Ok(out(36, Some(0x3c9), &[])),
                _ => {}
            }
        }
        if (10..=12).contains(&d) && (10..=12).contains(&s) {
            if i.destination_flags_word & 0x8000 == 0 {
                return Ok(out(44, None, &[]));
            }
        } else {
            if s == 12 && d == 16 {
                return Ok(out(61, Some(0x3c8), &[]));
            }
            if s == 3 {
                match d {
                    5 => return Ok(out(67, Some(0x138), &[])),
                    6 => return Ok(out(71, None, &[])),
                    16 => return Ok(out(74, Some(0x3c7), &[])),
                    _ => {}
                }
            }
            if s == 17 {
                return Ok(out(
                    80,
                    Some(0x361),
                    &[required(m.source_descriptor, "FUN_0fb0261c")?],
                ));
            }
            if s == 15 && d == 20 {
                return Ok(out(
                    86,
                    Some(0x435),
                    &[required(
                        m.destination_reference,
                        "FUN_0fb8e6b0(destination)",
                    )?],
                ));
            }
            if s == 15 && d == 18 {
                return Ok(out(
                    90,
                    Some(0x436),
                    &[required(
                        m.destination_type_descriptor,
                        "FUN_0fab8628(destination[2])",
                    )?],
                ));
            }
        }
        return Ok(out(
            104,
            Some(BASE[index(d)? as usize] + normalized(index(s)?)),
            &[],
        ));
    }
    let slot = required(m.destination_slot, "FUN_0faab81f")?;
    match s {
        3 => Ok(out(113, Some(0x2f8), &[slot])),
        12 => Ok(out(127, Some(0x2fb), &[slot])),
        20 => Ok(out(
            134,
            Some(0x434),
            &[slot, required(m.source_reference, "FUN_0fb8e6b0(source)")?],
        )),
        22 => {
            let non_null = required(m.source_22_resolution, "FUN_0faba86b")?
                .map_err(|_| ConversionError::NativeHelperFailure("FUN_0fb211d4"))?;
            Ok(out(
                142,
                Some(if non_null { 0x26f } else { 0x172 }),
                &[slot],
            ))
        }
        18 | 23 => {
            let desc = required(m.source_type_descriptor, "FUN_0fab8628")?;
            let masked = desc & 0x9fff;
            if masked == 0x24 || masked == 9 {
                let helper = if masked == 9 {
                    "FUN_0fb8e64d"
                } else {
                    "FUN_0fb8e6b0(source)"
                };
                Ok(out(
                    151,
                    Some(0x43a),
                    &[slot, desc & 0xbfff, required(m.source_reference, helper)?],
                ))
            } else if s == 23 {
                Err(ConversionError::UnresolvedHelper(
                    "switchD_0fac0d62::caseD_4",
                ))
            } else {
                Ok(out(166, Some(0x294), &[slot, desc]))
            }
        }
        _ => Ok(out(123, Some(BASE[6] + normalized(index(s)?)), &[slot])),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantConversionInput {
    pub source_type: u16,
    pub source_ast_id: u16,
    pub source_flags_word: u32,
    /// Native bytes at source + 0x10. Numeric f64 constants use little endian.
    pub literal_bytes: [u8; 8],
    pub destination_slot: Option<u16>,
    pub interned_string: Option<u16>,
}
/// it is not an error and the caller must use ordinary conversion emission.
pub fn select_constant_conversion(
    i: ConstantConversionInput,
) -> Result<Option<ConversionPlan>, ConversionError> {
    let (op, line, len) = match i.source_type {
        3 => (
            if u32::from_le_bytes(i.literal_bytes[..4].try_into().unwrap()) != 0 {
                0x165
            } else {
                0x166
            },
            18,
            0,
        ),
        5 => (0x33d, 24, 2),
        6 => (0x3c0, 28, 2),
        8 => (0x3c1, 32, 4),
        10 => (0x3c2, 37, 4),
        11 => (0x3c4, 41, 8),
        12 => (0x3c5, 45, 8),
        13 => (0x3c3, 49, 8),
        16 if i.source_ast_id == 4 && i.source_flags_word & 0x8000 == 0 => (0x3c6, 54, 2),
        _ => return Ok(None),
    };
    let slot = required(i.destination_slot, "FUN_0faab81f")?;
    let mut p = plan(0x0fac31c9, line, Some(op), &[slot]);
    let payload = &mut p.instructions[0].payload;
    if i.source_type == 10 {
        payload.extend_from_slice(&(f64::from_le_bytes(i.literal_bytes) as f32).to_le_bytes());
    } else if i.source_type == 16 {
        payload.extend_from_slice(&required(i.interned_string, "FUN_0fac2e29")?.to_le_bytes());
    } else {
        payload.extend_from_slice(&i.literal_bytes[..len]);
    }
    Ok(Some(p))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input(d: u16, s: u16) -> ConversionInput {
        ConversionInput {
            destination_type: d,
            source_type: s,
            destination_flags_word: 0,
            metadata: ResolvedConversionMetadata::default(),
        }
    }
    #[test]
    fn floating_override_precedes_noop() {
        assert_eq!(
            select_conversion(input(12, 11)).unwrap().instructions[0].opcode,
            0x147
        );
        assert!(
            select_conversion(input(10, 12))
                .unwrap()
                .instructions
                .is_empty()
        );
        let mut i = input(10, 12);
        i.destination_flags_word = 0x8000;
        assert_eq!(select_conversion(i).unwrap().instructions[0].opcode, 0x128);
    }
    #[test]
    fn computed_ids_outside_recovered_opcode_extent_are_rejected() {
        assert_eq!(
            select_conversion(input(22, 6)),
            Err(ConversionError::UnknownComputedOpcode(0x447))
        );
        assert_eq!(
            select_conversion(input(22, 5)).unwrap().instructions[0].opcode,
            0x446
        );
    }
    #[test]
    fn unsafe_jump_table_entries_are_rejected() {
        assert_eq!(
            select_conversion(input(20, 5)),
            Err(ConversionError::UnsafeTableIndex(12))
        );
        assert_eq!(
            select_conversion(input(5, 17)),
            Err(ConversionError::UnresolvedHelper("FUN_0fb0261c"))
        );
    }
    #[test]
    fn descriptor_emits_three_words_and_clears_bit14() {
        let mut i = input(15, 18);
        i.metadata.destination_slot = Some(7);
        i.metadata.source_type_descriptor = Some(0x4024);
        i.metadata.source_reference = Some(9);
        assert_eq!(
            select_conversion(i).unwrap().instructions[0],
            ConversionInstruction {
                opcode: 0x43a,
                payload: vec![7, 0, 0x24, 0, 9, 0]
            }
        );
    }
    #[test]
    fn constant_f64_narrows_to_f32_and_carries_slot() {
        let i = ConstantConversionInput {
            source_type: 10,
            source_ast_id: 1,
            source_flags_word: 0,
            literal_bytes: 1.5f64.to_le_bytes(),
            destination_slot: Some(0x1234),
            interned_string: None,
        };
        assert_eq!(
            select_constant_conversion(i).unwrap().unwrap().instructions[0],
            ConversionInstruction {
                opcode: 0x3c2,
                payload: vec![0x34, 0x12, 0, 0, 0xc0, 0x3f]
            }
        );
    }
    #[test]
    fn string_optimization_guard_and_missing_pool_are_explicit() {
        let mut i = ConstantConversionInput {
            source_type: 16,
            source_ast_id: 4,
            source_flags_word: 0x8000,
            literal_bytes: [0; 8],
            destination_slot: Some(1),
            interned_string: None,
        };
        assert_eq!(select_constant_conversion(i).unwrap(), None);
        i.source_flags_word = 0;
        assert_eq!(
            select_constant_conversion(i),
            Err(ConversionError::UnresolvedHelper("FUN_0fac2e29"))
        );
    }
}
