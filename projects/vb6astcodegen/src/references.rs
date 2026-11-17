//! FAC0FA8, FAC1354, FAC41E0 and FB8E726 with resolved symbol/type/helper data.
//! Includes environment prefixes, classes1..15, and observed default behavior.
//! The compact selector retains its ordinary-class1..3 input contract.
use crate::{addressing::AddressDescriptor, tables::TYPE_INDEX};
#[derive(Debug, Clone, Copy)]
pub struct CommonStorageReference {
    pub storage_class: u16,
    /// Record byte0 bit80 && (byte1 &7)==4 selects FAC0FA8's separate global
    /// object/environment path, even when the helper originally returned1..3.
    pub special_object_record: bool,
    /// Native member descriptor low six bits and bit0x800.
    pub member_kind: u8,
    pub member_flag_800: bool,
    pub type23_is_udt: Option<bool>,
    /// Record byte+1 bit0x20 and context byte this+0xC.
    pub record_context_bit20: bool,
    pub context_flags: u8,
    /// FAA BBA1/FAA9441 outcomes for native flags4000/type23.
    pub underlying_type: Option<u16>,
    pub nested_underlying_type: Option<u16>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceError {
    UnknownType(u16),
    UnresolvedStorageClass(u16),
    UnresolvedObjectEnvironment,
    MissingMetadata(&'static str),
    NativeHelperFailure { function: u32, status: i32 },
}

/// Native symbol record fields read by FAC0FA8/3A76/FB8E726.
#[derive(Debug, Clone, Copy)]
pub struct SymbolRecord {
    pub flags_at_0: u8,
    pub flags_at_1: u8,
    pub word_at_8: u16,
    pub word_at_10: u16,
    pub flags_at_16: u8,
    pub dword_at_20: u32,
}
#[derive(Debug, Clone, Copy)]
pub struct ResolvedReference {
    /// Helper outcomes and type information; raw record flags below take precedence.
    pub common: CommonStorageReference,
    pub record: SymbolRecord,
    pub symbol_slot: Option<u16>,
    pub auxiliary_slot: Option<u16>,
    /// Selected descriptor word at DAT_FAAB770/7E0 + TYPE_INDEX[type]*2.
    pub class4_descriptor_words: Option<[u16; 20]>,
    pub environment_word174: Option<u16>,
    pub environment_word74: Option<u16>,
    pub environment_word176: Option<u16>,
    pub remote_slot_3b02: Option<Result<u16, i32>>,
    pub remote_slot_2e51: Option<Result<u16, i32>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceStep {
    /// Native prefix calls always use mode6, flags0, null type descriptor/type0.
    Address(AddressDescriptor),
    Emit(u16),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferencePlan {
    pub prefix: Vec<ReferenceStep>,
    pub descriptor: AddressDescriptor,
    pub effective_storage_class: u16,
    pub original_function: u32,
}
pub fn zero_descriptor() -> AddressDescriptor {
    AddressDescriptor {
        kind: 0,
        flags_at_4: 0,
        word_at_6: 0,
        word_at_8: 0,
        word_at_10: 0,
        word_at_12: 0,
    }
}
fn helper_result(
    v: Option<Result<u16, i32>>,
    function: u32,
    name: &'static str,
) -> Result<u16, ReferenceError> {
    need(v, name)?.map_err(|status| ReferenceError::NativeHelperFailure { function, status })
}
fn name_index(r: SymbolRecord) -> u16 {
    if r.flags_at_1 & 0x50 == 0 && r.flags_at_0 & 8 == 0 && r.flags_at_16 & 4 == 0 {
        r.word_at_10.wrapping_add(1) & 0xfff
    } else {
        0
    }
}
/// FAC41E0 composition. The original mutates the incoming descriptor as well;
/// this plan returns its copied result and the ordered prefix effects.
fn compose(
    mut incoming: AddressDescriptor,
    output: AddressDescriptor,
    context: u8,
    prefix: &mut Vec<ReferenceStep>,
) -> AddressDescriptor {
    let clear = incoming.flags_at_4 & 4 == 0;
    match incoming.kind {
        1 if clear => {
            incoming.word_at_10 = incoming.word_at_10.wrapping_add(output.word_at_6);
            incoming.flags_at_4 = (incoming.flags_at_4 & !5) | (output.flags_at_4 & 5);
            return incoming;
        }
        1 | 2 | 4 if incoming.kind == 1 || clear => {
            if output.kind == 3 || output.flags_at_4 & 8 != 0 {
                incoming.kind = 4;
                incoming.word_at_12 = incoming.word_at_12.wrapping_add(output.word_at_6);
                incoming.flags_at_4 = (incoming.flags_at_4 & !5) | (output.flags_at_4 & 5);
                return incoming;
            }
        }
        3 if clear => {
            incoming.word_at_6 = incoming.word_at_6.wrapping_add(output.word_at_6);
            incoming.flags_at_4 = (incoming.flags_at_4 & !5) | (output.flags_at_4 & 5);
            return incoming;
        }
        6 if clear => {
            incoming.word_at_10 = incoming.word_at_10.wrapping_add(output.word_at_6);
            incoming.flags_at_4 = (incoming.flags_at_4 & !5) | (output.flags_at_4 & 5);
            return incoming;
        }
        2 | 4 | 5 | 6 | 7 => {
            prefix.push(ReferenceStep::Address(incoming));
            if !clear && context & 2 != 0 && incoming.flags_at_4 & 1 != 0 {
                prefix.push(ReferenceStep::Emit(0x408));
            }
            return output;
        }
        0 | 12.. => return output,
        _ => {}
    }
    prefix.push(ReferenceStep::Address(incoming));
    output
}
/// Complete FAC0FA8 selection with resolved helper inputs, including environment
/// prefixes, storage classes13..15, and the native default/zero-descriptor path.
pub fn select_reference(
    native_type: u16,
    node_flags: u32,
    incoming: AddressDescriptor,
    m: ResolvedReference,
) -> Result<ReferencePlan, ReferenceError> {
    let index = *TYPE_INDEX
        .get(native_type as usize)
        .ok_or(ReferenceError::UnknownType(native_type))?;
    let flags = node_flags & !0x1000;
    let r = m.record;
    let mut class = m.common.storage_class;
    let mut prefix = Vec::new();
    let mut d = zero_descriptor();
    let special = r.flags_at_0 & 0x80 != 0 && r.flags_at_1 & 7 == 4;
    let named = name_index(r);
    if special && named == 0 {
        if flags & 0x100 == 0 {
            let mut first = zero_descriptor();
            first.kind = 4;
            first.word_at_10 = 8;
            first.word_at_12 = need(m.environment_word174, "this[1]+0x174")?;
            prefix.push(ReferenceStep::Address(first));
        } else {
            let mut first = zero_descriptor();
            first.kind = 6;
            first.word_at_10 = 8;
            prefix.push(ReferenceStep::Address(first));
            let mut second = zero_descriptor();
            second.kind = 3;
            second.word_at_6 = need(m.environment_word74, "this[5]+0x74")?;
            prefix.push(ReferenceStep::Address(second));
        }
        d.kind = 3;
        d.word_at_6 = r.word_at_8;
        if class == 3 {
            d.flags_at_4 |= 5;
        }
        if r.flags_at_1 & 0x20 != 0 && m.common.context_flags & 2 == 0 {
            d.flags_at_4 |= 2;
        }
    } else {
        if special {
            class = match class {
                1 => 13,
                2 => 14,
                3 => 15,
                x => x,
            };
        }
        match class {
            1..=3 => {
                let mut common = m.common;
                common.storage_class = class;
                common.special_object_record = false;
                common.record_context_bit20 = r.flags_at_1 & 0x20 != 0;
                d = select_common_storage_reference(
                    native_type,
                    flags,
                    need(m.symbol_slot, "FUN_0faab81f(node[5])")?,
                    m.auxiliary_slot,
                    common,
                )?;
            }
            4 | 7 | 8 | 9 | 12 => {
                let (by_value, pointer) = match class {
                    4 => (
                        need(m.class4_descriptor_words, "class4 DAT_FAAB770/7E0 words")?
                            [index as usize]
                            & 0x4000
                            == 0,
                        true,
                    ),
                    7 | 8 => (false, m.common.member_kind != 0x1b),
                    9 => (true, false),
                    _ => (false, false),
                };
                d = slot_descriptor(
                    native_type,
                    flags,
                    need(m.symbol_slot, "FUN_0faab81f(node[5])")?,
                    m.auxiliary_slot,
                    by_value,
                    pointer,
                    m.common,
                )?;
            }
            13..=15 => {
                if named == 0 {
                    if r.flags_at_0 & 0x80 == 0 && incoming.kind != 0 {
                        let mut field = zero_descriptor();
                        field.kind = 3;
                        field.word_at_6 = r.word_at_8;
                        d = compose(incoming, field, m.common.context_flags, &mut prefix);
                    } else if flags & 0x200 == 0
                        && (r.flags_at_0 & 7 != 2
                            || r.flags_at_16 & 1 != 0
                            || r.dword_at_20 != 0xffff_fffe)
                    {
                        d.kind = 4;
                        d.word_at_10 = 8;
                        d.word_at_12 = r.word_at_8;
                    } else {
                        d.kind = 5;
                        d.word_at_6 = if m.common.context_flags & 1 != 0 {
                            0xffff
                        } else {
                            helper_result(m.remote_slot_3b02, 0x0fac3b02, "FUN_0fac3b02")?
                        };
                    }
                } else {
                    let mut first = zero_descriptor();
                    if flags & 0x300 != 0x300 {
                        first.kind = 1;
                        first.word_at_10 = 8;
                    } else {
                        first.kind = 5;
                        first.word_at_6 = if m.common.context_flags & 1 != 0 {
                            0xffff
                        } else {
                            helper_result(m.remote_slot_2e51, 0x0fac2e51, "FUN_0fac2e51")?
                        };
                    }
                    prefix.push(ReferenceStep::Address(first));
                    d.kind = 8;
                    d.word_at_8 = r.word_at_8;
                    d.word_at_6 = need(m.environment_word176, "symbol record context+0x176")?;
                    d.word_at_10 = named;
                }
                if class == 15 {
                    d.flags_at_4 |= 5;
                }
                if r.flags_at_1 & 0x20 != 0 && m.common.context_flags & 2 == 0 {
                    d.flags_at_4 |= 2;
                }
            }
            _ => {}
        }
    }
    if index == 14 {
        d.flags_at_4 |= 8;
    }
    Ok(ReferencePlan {
        prefix,
        descriptor: d,
        effective_storage_class: class,
        original_function: 0x0fac0fa8,
    })
}
fn need<T>(v: Option<T>, name: &'static str) -> Result<T, ReferenceError> {
    v.ok_or(ReferenceError::MissingMetadata(name))
}
pub fn select_common_storage_reference(
    native_type: u16,
    node_flags: u32,
    symbol_slot: u16,
    auxiliary_slot: Option<u16>,
    m: CommonStorageReference,
) -> Result<AddressDescriptor, ReferenceError> {
    if !(1..=3).contains(&m.storage_class) {
        return Err(ReferenceError::UnresolvedStorageClass(m.storage_class));
    }
    if m.special_object_record {
        return Err(ReferenceError::UnresolvedObjectEnvironment);
    }
    let type_index = *TYPE_INDEX
        .get(native_type as usize)
        .ok_or(ReferenceError::UnknownType(native_type))?;
    let flags = node_flags & !0x1000;
    let pointer = !(m.storage_class == 3
        || (m.member_kind == 0x1b && m.member_flag_800)
        || (native_type == 23 && need(m.type23_is_udt, "FUN_0fb0f0f7(node[2])")?));
    let mut d = slot_descriptor(
        native_type,
        flags,
        symbol_slot,
        auxiliary_slot,
        true,
        pointer,
        m,
    )?;
    if m.record_context_bit20 && m.context_flags & 2 == 0 {
        d.flags_at_4 |= 2;
    }
    if type_index == 14 {
        d.flags_at_4 |= 8;
    }
    Ok(d)
}

fn slot_descriptor(
    native_type: u16,
    flags: u32,
    symbol_slot: u16,
    auxiliary_slot: Option<u16>,
    by_value: bool,
    pointer: bool,
    m: CommonStorageReference,
) -> Result<AddressDescriptor, ReferenceError> {
    let mut d = AddressDescriptor {
        kind: if flags & 0x100 == 0 || symbol_slot == 8 {
            2 - u32::from(by_value)
        } else {
            7 - u32::from(by_value)
        },
        flags_at_4: 0,
        word_at_6: 0,
        word_at_8: 0,
        word_at_10: symbol_slot,
        word_at_12: 0,
    };
    let has_auxiliary = (flags & 0x2000 != 0 && flags & 1 == 0)
        || (flags & 0x4000 != 0
            && native_type == 23
            && need(m.underlying_type, "FUN_0faabba1/FUN_0faa9441")? == 23
            && need(m.nested_underlying_type, "nested FUN_0faabba1/FUN_0faa9441")? == 18);
    if has_auxiliary {
        d.word_at_6 |= 1;
        d.word_at_8 = need(auxiliary_slot, "FUN_0faab81f(node[6])")?;
    }
    if !pointer {
        d.flags_at_4 |= 4;
        if native_type == 22 {
            d.flags_at_4 |= 1;
        }
    }
    Ok(d)
}
