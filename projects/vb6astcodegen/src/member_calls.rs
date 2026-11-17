//! Caller resolves symbol/COM metadata; plans explicitly retain every helper dependency.
//! Tail word proven by native blocks FAC03D5(word7), FAC039D..03BD(result descriptor),
//! FB34845..34869(indirect descriptor low16); FAC03E9 consumes that pushed word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberStep {
    Arguments {
        helper: u32,
        mode: u16,
    },
    Target {
        mode: u16,
    },
    Emit {
        opcode: u16,
        words: Vec<u16>,
    },
    Convert {
        source_type: u16,
        destination_type: u16,
        context_mode: u16,
    },
    EvaluateSymbol {
        context_mode: u16,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberError {
    MissingMetadata(&'static str),
    UnresolvedBranch(&'static str),
    UnknownOpcodeBase(u16),
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ResolvedMemberMetadata {
    pub symbol_kind: Option<u32>,
    pub class: Option<u32>,
    /// Selected DAT_0faab770/7E0 descriptor byte at+0x1D.
    pub descriptor_flags: Option<u8>,
    pub foreign_context: Option<bool>,
    /// Numeric TYPE_INDEX[result_type], resolved from recovered table.
    pub result_type_index: Option<u32>,
    pub symbol_slot: Option<u16>,
    pub indirect_descriptor: Option<u32>,
    /// Target type indexes DAT_0fac39c0; may use TARGET_BYTES constant below.
    pub target_bytes: Option<u16>,
    pub argument_bytes: Option<u16>,
    pub call_descriptor: Option<u16>,
    pub result_descriptor: Option<u16>,
}
#[derive(Debug, Clone, Copy)]
pub struct MemberInput {
    pub result_type: u16,
    pub flags: u16,
    pub context_mode: u16,
    pub has_arguments: bool,
    pub word7: u16,
    pub metadata: ResolvedMemberMetadata,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberPlan {
    pub steps: Vec<MemberStep>,
    pub selected_opcode: u16,
    pub selector_marker: u8,
}
/// Snapshot initial DAT_0FC10330. Caller must use the active table if runtime modifies it.
pub const BASE_OPCODE_SNAPSHOT: [u16; 6] = [0x340, 0x310, 0x350, 0x300, 0x330, 0x320];
/// Snapshot DAT_0FAC39C0, indexed by native target type.
pub const TARGET_BYTES: [u16; 30] = [
    0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 0x10, 0x10, 4, 0, 4, 0, 4, 0, 4, 4, 0, 0, 4, 4, 4, 4,
];
pub fn select_member_base(table: &[u16; 6], kind: u32, class: u32, flag2000: bool) -> u16 {
    let idx = if class == 1 {
        usize::from(kind != 1) + 2 * usize::from(flag2000)
    } else {
        usize::from(kind != 1) + 4
    };
    table[idx]
}
fn require<T: Copy>(v: Option<T>, name: &'static str) -> Result<T, MemberError> {
    v.ok_or(MemberError::MissingMetadata(name))
}
fn typed_offset(n: u32) -> u32 {
    match n {
        10 => 4,
        9 => 1,
        x => x,
    }
}
pub fn transform_member_opcode(
    base: u16,
    flags: u16,
    result_type: u16,
    descriptor_flags: u8,
    result_type_index: Option<u32>,
) -> Result<(u16, u8), MemberError> {
    if flags & 0x8000 != 0 {
        if flags & 0x1000 != 0 && base == 0x320 {
            return Ok((0x41e, 1));
        }
        return Ok((
            match base {
                0x300 => 0x16a,
                0x310 => 0x169,
                0x320 => 0x34f,
                0x340 => 0x16b,
                0x350 => 0x16c,
                _ => 0x446,
            },
            1,
        ));
    }
    if flags & 0x0800 != 0 {
        return Ok((
            match base {
                0x320 => 0x32d,
                0x310 => 0x31d,
                0x300 => 0x30d,
                _ => 0x446,
            },
            2,
        ));
    }
    if result_type == 2 {
        return Ok((base.wrapping_add(12), 0));
    } // uint* +3 native pointer stride4.
    let index = if result_type == 0x12 {
        2
    } else {
        require(result_type_index, "DAT_0FAAA670[result_type]")?
    };
    let special = matches!(result_type, 10..=12) && descriptor_flags & 8 != 0;
    let offset = if special {
        typed_offset(typed_offset(index))
    } else {
        typed_offset(index)
    };
    let mut selected = u32::from(base) | offset;
    if special && (offset == 3 || offset == 4) {
        selected += 6;
    }
    Ok((selected as u16, 0))
}
pub fn select_member_call(
    input: MemberInput,
    base_table: &[u16; 6],
) -> Result<MemberPlan, MemberError> {
    let m = input.metadata;
    let kind = require(m.symbol_kind, "FUN_0fab7f18")?;
    let class = require(m.class, "FUN_0fab7f4d")?;
    let df = require(m.descriptor_flags, "selected descriptor+0x1D")?;
    let foreign = if class == 1 && input.flags & 0x2000 != 0 {
        require(m.foreign_context, "FUN_0fab5d51 interface comparison")?
    } else {
        false
    };
    let base = select_member_base(
        base_table,
        kind,
        class,
        if foreign {
            false
        } else {
            input.flags & 0x2000 != 0
        },
    );
    let (mut opcode, marker) = transform_member_opcode(
        base,
        input.flags,
        input.result_type,
        df,
        m.result_type_index,
    )?;
    // Tail0FB347D8 special result-type2/class1/flag0x200.
    if input.flags & 0x8800 == 0 && input.result_type == 2 && input.flags & 0x200 != 0 && class == 1
    {
        opcode = 0x42b + u16::from(base != 0x310);
    }
    if marker == 2 {
        require(m.indirect_descriptor, "FUN_0faeba30/FUN_0fab4e58")?;
    }
    let slot = if df & 0x10 != 0 || marker == 2 {
        Some(require(m.symbol_slot, "FUN_0faab81f(node[8])")?)
    } else {
        None
    };
    let mut steps = Vec::new();
    if df & 0x10 != 0 {
        steps.push(MemberStep::Emit {
            opcode: 0x265,
            words: vec![slot.unwrap()],
        });
    }
    if input.has_arguments {
        steps.push(MemberStep::Arguments {
            helper: if input.flags & 0x4000 == 0 {
                0x0fafbd6a
            } else {
                0x0fac34c0
            },
            mode: 3,
        });
    }
    let mut words = Vec::new();
    if class == 1 {
        if input.flags & 0x2000 == 0 {
            steps.push(MemberStep::Target { mode: 6 });
        } else if foreign {
            steps.push(MemberStep::Emit {
                opcode: 0x29e,
                words: vec![],
            });
        }
        if df & 0x10 != 0 {
            words.push(slot.unwrap());
        }
        match marker {
            0 => words.push(input.word7),
            1 => {
                words.push(input.word7);
                words.push(require(m.result_descriptor, "FUN_0fac2e29(node[9])")?);
            }
            2 => {
                words.push(slot.unwrap());
                words.push(input.word7);
                words.push(require(m.indirect_descriptor, "FUN_0faeba30/FUN_0fab4e58")? as u16);
            }
            _ => {
                return Err(MemberError::UnresolvedBranch(
                    "unexpected class1 selector marker",
                ));
            }
        }
    } else {
        let target_bytes = if class == 0 {
            steps.push(MemberStep::Target { mode: 1 });
            require(m.target_bytes, "DAT_0FAC39C0[target.type]")?
        } else {
            0
        };
        let total =
            target_bytes.wrapping_add(require(m.argument_bytes, "FUN_0fac38d1(node[5],0)")?);
        let descriptor = require(m.call_descriptor, "successful FUN_0fac3945")?;
        if df & 0x10 != 0 {
            words.push(slot.unwrap());
        }
        if marker == 2 {
            words.push(slot.unwrap());
        }
        words.push(descriptor);
        words.push(total);
        if marker == 2 {
            words.push(require(m.indirect_descriptor, "FUN_0faeba30/FUN_0fab4e58")? as u16);
        }
    }
    steps.push(MemberStep::Emit { opcode, words });
    if input.result_type == 0x14 {
        steps.push(MemberStep::EvaluateSymbol {
            context_mode: input.context_mode,
        });
    } else if df & 8 != 0 || input.result_type == 0x0f {
        steps.push(MemberStep::Convert {
            source_type: input.result_type,
            destination_type: 0,
            context_mode: input.context_mode,
        });
    }
    Ok(MemberPlan {
        steps,
        selected_opcode: opcode,
        selector_marker: marker,
    })
}
