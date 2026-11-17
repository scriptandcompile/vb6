//! C source: research/sources/decompiled/0fac13f1.c, 0fac142f.c.
//! Missing C tail setup recovered from original static machine bytes embedded
//! in tip_compile_module.rs; evidence work/addressing_native.asm and _tail.asm.
//! Numeric descriptor/mode/type tags are intentionally not renamed semantically.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressDescriptor {
    pub kind: u32,
    /// Native descriptor byte +4; bit2 requests post-address adjustment.
    pub flags_at_4: u8,
    pub word_at_6: u16,
    pub word_at_8: u16,
    pub word_at_10: u16,
    pub word_at_12: u16,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct AddressMetadata {
    pub type_18_is_special: Option<bool>,
    pub underlying_type: Option<u16>,
    /// Same two helpers applied again to the first result, if underlying=23.
    pub nested_underlying_type: Option<u16>,
    /// Exact address and byte recovered for the FAC29D8 indexed lookup.
    /// Explicit because its inferred boundaries/type legality are not proven.
    pub mode4_lookup: Option<(u32, u8)>,
}
#[derive(Debug, Clone, Copy)]
pub struct AddressInput {
    pub descriptor: AddressDescriptor,
    pub mode: u16,
    pub flags: u32,
    pub native_type: u16,
    pub metadata: AddressMetadata,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressInstruction {
    pub opcode: u16,
    pub words: Vec<u16>,
    pub original_address: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressPlan {
    pub instructions: Vec<AddressInstruction>,
    pub effective_mode: u16,
    pub effective_type: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressError {
    UnresolvedHelper(&'static str),
    UnknownDescriptor(u32),
    UnknownMode(u16),
    UnknownType(u16),
    UnsafeTypeIndex(u8),
    LookupAddressMismatch { expected: u32, provided: u32 },
}
const TYPE_INDEX: [u8; 30] = [
    19, 19, 19, 9, 19, 0, 1, 19, 2, 19, 3, 4, 10, 5, 19, 6, 7, 14, 19, 19, 12, 19, 8, 2, 19, 19, 2,
    0, 19, 0,
];
fn raw_index(t: u16) -> Result<u8, AddressError> {
    let v = *TYPE_INDEX
        .get(t as usize)
        .ok_or(AddressError::UnknownType(t))?;
    if v == 19 {
        Err(AddressError::UnsafeTypeIndex(v))
    } else {
        Ok(v)
    }
}
fn norm(v: u8) -> u16 {
    match v {
        9 => 1,
        10 => 4,
        _ => v as u16,
    }
}
fn need<T>(x: Option<T>, name: &'static str) -> Result<T, AddressError> {
    x.ok_or(AddressError::UnresolvedHelper(name))
}
fn instruction(opcode: u16, words: Vec<u16>, original_address: u32) -> AddressInstruction {
    AddressInstruction {
        opcode,
        words,
        original_address,
    }
}
/// Reference descriptor construction and type classification remain caller-owned.
pub fn select_address(i: AddressInput) -> Result<AddressPlan, AddressError> {
    let d = i.descriptor;
    let mut mode = i.mode;
    let mut ty = i.native_type;
    if !(1..=6).contains(&mode) {
        return Err(AddressError::UnknownMode(mode));
    }
    let mut instructions = Vec::new();
    if [8, 9, 11].contains(&d.kind) {
        // Descriptor8 skips FAC17EB; 9 and11 call it before selecting opcode.
        if d.kind != 8 && [17, 18].contains(&ty) && mode <= 3 {
            mode = 5;
        }
        let (op, words, at) = match d.kind {
            8 => (
                0x3ca + u16::from(mode != 5),
                vec![d.word_at_6, d.word_at_8, d.word_at_10],
                0x0fb3597d,
            ),
            9 => (0x18d + u16::from(mode == 5), vec![d.word_at_6], 0x0fb358db),
            _ => (
                0x406 + u16::from(mode == 5),
                vec![d.word_at_8, d.word_at_6],
                0x0fb35944,
            ),
        };
        instructions.push(instruction(op, words, at));
        if mode != 5 && (mode != 6 || d.flags_at_4 & 4 != 0) {
            let mut follow = i;
            follow.descriptor = AddressDescriptor {
                kind: 3,
                flags_at_4: 0,
                word_at_6: 0,
                word_at_8: 0,
                word_at_10: 0,
                word_at_12: 0,
            };
            follow.mode = mode;
            instructions.extend(select_address(follow)?.instructions);
        }
        return Ok(AddressPlan {
            instructions,
            effective_mode: mode,
            effective_type: ty,
        });
    }
    // Base and emission shape are recovered from the native descriptor jump table.
    let (base, mut words) = match d.kind {
        1 => (0x1e0, vec![d.word_at_10]),
        2 => (
            0x210,
            if d.word_at_6 & 1 != 0 {
                vec![d.word_at_10, d.word_at_8]
            } else {
                vec![d.word_at_10]
            },
        ),
        3 => (0x270, vec![d.word_at_6]),
        4 => (0x2d0, vec![d.word_at_10, d.word_at_12]),
        5 => (0x2a0, vec![d.word_at_6]),
        6 => (0x3d0, vec![d.word_at_10]),
        7 => (
            0x240,
            if d.word_at_6 & 1 != 0 {
                vec![d.word_at_10, d.word_at_8]
            } else {
                vec![d.word_at_10]
            },
        ),
        10 => (
            0x190,
            if d.word_at_6 & 1 != 0 {
                vec![d.word_at_8]
            } else {
                vec![]
            },
        ),
        // FAC3B64 creates kind12. Native default FB359AE -> FAC14C5 keeps
        // native_type in both ESI (base) and BP (operand); no invented base.
        0 | 12 => (ty, vec![ty]),
        _ => return Err(AddressError::UnknownDescriptor(d.kind)),
    };
    if ty == 18 && need(i.metadata.type_18_is_special, "FUN_0fab0d6c")? {
        ty = 23;
    } else if [17, 18].contains(&ty) && mode <= 3 {
        mode = 5;
    }
    let f = i.flags;
    let mut op = match mode {
        1 | 2 if f & 0x4000 != 0 => {
            if ty == 16 {
                0x23f
            } else if ty == 23
                && need(i.metadata.underlying_type, "FUN_0faabba1/FUN_0faa9441")? == 23
                && need(
                    i.metadata.nested_underlying_type,
                    "FUN_0faabba1/FUN_0faa9441(nested)",
                )? == 18
            {
                if words.is_empty() {
                    words.push(i.native_type);
                }
                words.truncate(1);
                words.push(d.word_at_8);
                0x438
            } else {
                0x262
            }
        }
        1 | 2 | 5 if f & 0x1000 != 0 => 0x1b2,
        1 => base | norm(raw_index(ty)?),
        2 => {
            let n = norm(raw_index(ty)?);
            (base | n) + if n == 3 || n == 4 { 6 } else { 0 }
        }
        3 if f & 0x1000 != 0 => 0x1b3 + u16::from(f & 0x2000 != 0),
        3 if ty == 15 => base + 13,
        3 => base | norm(raw_index(ty)?),
        4 => {
            let raw = raw_index(ty)?;
            let mut v = (base + 0x10) | norm(raw);
            if f & 0x8000 != 0 {
                if f & 0x20 != 0 {
                    v = 0x1b7 + if f & 0x800 != 0 { 2 } else { 0 };
                } else {
                    let idx = (((((!f >> 12) & 1) as i32 - 12) + i32::from(raw) * 2) * 2)
                        + ((!f >> 11) & 1) as i32;
                    let address = (0x0fac29d8i64 + i64::from(idx)) as u32;
                    let (provided, b) = need(
                        i.metadata.mode4_lookup,
                        "DAT_0fac29d8 byte at computed address",
                    )?;
                    if provided != address {
                        return Err(AddressError::LookupAddressMismatch {
                            expected: address,
                            provided,
                        });
                    }
                    v = base + 0x10 + u16::from(b);
                }
            } else if f & 0x20 != 0 {
                v = 0x1b5 + u16::from(f & 0x1000 != 0);
            } else if f & 0x80 != 0 {
                v += 6;
            } else if f & 0x200 != 0 {
                v = 0x3f8 + if v != 0x1f6 { 0x11 } else { 0 };
            } else if f & 0x400 != 0 {
                v = 0x3f7;
            } else if f & 0x1000 != 0 {
                if ty == 15 || ty == 16 {
                    v += 10;
                }
            } else if f & 0x800 != 0 {
                v = 0x439;
            }
            v
        }
        5 => base + 12,
        6 => base + 11,
        _ => unreachable!(),
    };
    if mode == 4 && f & 0x40 != 0 && d.kind == 1 {
        op = match op {
            0x1f2 => {
                instructions.push(instruction(op, vec![d.word_at_10], 0x0fb35a5a));
                0x1e2
            }
            0x1f6 => 0x263,
            0x1f7 => 0x264,
            0x1f8 => 0x29c,
            0x1ff => 0x29d,
            0x200 => {
                instructions.push(instruction(op, vec![d.word_at_10], 0x0fb35a6d));
                0x1ec
            }
            0x201 => {
                instructions.push(instruction(op, vec![d.word_at_10], 0x0fac17a0));
                0x1e7
            }
            _ => op,
        };
    }
    instructions.push(instruction(op, words, 0x0fac1690));
    if [5, 7, 10].contains(&d.kind) && mode == 6 && d.flags_at_4 & 4 != 0 {
        let mut follow = i;
        follow.descriptor = AddressDescriptor {
            kind: 3,
            flags_at_4: 0,
            word_at_6: 0,
            word_at_8: 0,
            word_at_10: 0,
            word_at_12: 0,
        };
        follow.mode = 6;
        follow.flags = 0;
        follow.native_type = ty;
        instructions.extend(select_address(follow)?.instructions);
    }
    Ok(AddressPlan {
        instructions,
        effective_mode: mode,
        effective_type: ty,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn i(k: u32, m: u16, t: u16) -> AddressInput {
        AddressInput {
            descriptor: AddressDescriptor {
                kind: k,
                flags_at_4: 0,
                word_at_6: 0,
                word_at_8: 8,
                word_at_10: 10,
                word_at_12: 12,
            },
            mode: m,
            flags: 0,
            native_type: t,
            metadata: AddressMetadata::default(),
        }
    }
    #[test]
    fn recovered_two_word_descriptors() {
        let p = select_address(i(4, 1, 5)).unwrap();
        assert_eq!(p.instructions[0].opcode, 0x2d0);
        assert_eq!(p.instructions[0].words, vec![10, 12]);
        let mut x = i(2, 2, 10);
        x.descriptor.word_at_6 = 1;
        let p = select_address(x).unwrap();
        assert_eq!(p.instructions[0].opcode, 0x219);
        assert_eq!(p.instructions[0].words, vec![10, 8]);
    }
    #[test]
    fn opcode_only_and_flagged_descriptor10() {
        assert!(
            select_address(i(10, 6, 5)).unwrap().instructions[0]
                .words
                .is_empty()
        );
        let mut x = i(10, 6, 5);
        x.descriptor.word_at_6 = 1;
        assert_eq!(select_address(x).unwrap().instructions[0].words, vec![8]);
    }
    #[test]
    fn mode_rewrite_and_descriptor8_exception() {
        assert_eq!(select_address(i(9, 1, 17)).unwrap().instructions.len(), 1);
        let p = select_address(i(8, 1, 17)).unwrap();
        assert_eq!(p.instructions[0].opcode, 0x3cb);
        assert_eq!(p.instructions[1].opcode, 0x27c);
    }
    #[test]
    fn nested_special_emits438_and_extra_operand() {
        let mut x = i(1, 1, 23);
        x.flags = 0x4000;
        x.metadata.underlying_type = Some(23);
        x.metadata.nested_underlying_type = Some(18);
        assert_eq!(
            select_address(x).unwrap().instructions[0],
            instruction(0x438, vec![10, 8], 0x0fac1690)
        );
    }
    #[test]
    fn post_adjustment_is_ported() {
        let mut x = i(7, 6, 5);
        x.descriptor.flags_at_4 = 4;
        let p = select_address(x).unwrap();
        assert_eq!(
            p.instructions.iter().map(|v| v.opcode).collect::<Vec<_>>(),
            vec![0x24b, 0x27b]
        );
    }
    #[test]
    fn priority_and_double_emission() {
        let mut x = i(1, 4, 8);
        x.flags = 0x40;
        let p = select_address(x).unwrap();
        assert_eq!(
            p.instructions.iter().map(|v| v.opcode).collect::<Vec<_>>(),
            vec![0x1f2, 0x1e2]
        );
        x.flags = 0x20 | 0x80 | 0x1000;
        assert_eq!(select_address(x).unwrap().instructions[0].opcode, 0x1b6);
    }
    #[test]
    fn lookup_needs_exact_address() {
        let mut x = i(1, 4, 15);
        x.flags = 0x8000;
        assert!(matches!(
            select_address(x),
            Err(AddressError::UnresolvedHelper(_))
        ));
        x.metadata.mode4_lookup = Some((0x0fac29db, 1));
        assert_eq!(select_address(x).unwrap().instructions[0].opcode, 0x1f1);
    }
}
