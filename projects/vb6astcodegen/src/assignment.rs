//! A plan is not emitted bytes: Evaluate/Address/Convert must be executed by codegen.
//! Every unresolved dependency returns an error instead of an empty-success plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRole {
    Left,
    Right,
    IndexedBase,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentStep {
    Evaluate {
        node: NodeRole,
        mode: u16,
    },
    Address {
        mode: u16,
        flags: u32,
        type_id: u16,
    },
    Emit {
        opcode: u16,
        words: Vec<u16>,
    },
    Convert {
        source_type: u16,
        destination_type: u16,
        mode: u16,
    },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentPlan {
    pub steps: Vec<AssignmentStep>,
    pub source_function: u32,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentError {
    MissingMetadata(&'static str),
    UnresolvedBranch(&'static str),
}
#[derive(Debug, Clone, Copy, Default)]
pub struct ResolvedAssignmentMetadata {
    pub rhs_special_call: Option<bool>,
    pub lhs_address_kind: Option<u16>,
    pub rhs_call_slot: Option<u16>,
    pub lhs_address_word: Option<u16>,
    /// Return bit 0 from emitting RHS in mode 2.
    pub rhs_emission_low_bit: Option<bool>,
    pub indexed_symbol_slot: Option<u16>,
    /// lhs[5] (unwrapping kind 0x37), field [4], truncated to u16.
    pub indexed_value: Option<u16>,
    pub assignment_descriptor: Option<u16>,
    pub lhs_type_descriptor: Option<u16>,
}
#[derive(Debug, Clone, Copy)]
pub struct AssignmentInput {
    pub assignment_type: u16,
    pub flags: u16,
    pub context_mode: u16,
    pub lhs_kind: u16,
    pub lhs_type: u16,
    pub lhs_flags: u16,
    pub rhs_kind: u16,
    pub rhs_type: u16,
    pub has_assignment_descriptor: bool,
    /// lhs kind 0x11 with child kind 0x6A has a separate indexed-address path.
    pub lhs_is_11_over_6a: bool,
    pub metadata: ResolvedAssignmentMetadata,
}
fn require<T: Copy>(value: Option<T>, name: &'static str) -> Result<T, AssignmentError> {
    value.ok_or(AssignmentError::MissingMetadata(name))
}
fn emit(steps: &mut Vec<AssignmentStep>, opcode: u16, words: &[u16]) {
    steps.push(AssignmentStep::Emit {
        opcode,
        words: words.to_vec(),
    });
}
pub fn select_assignment(input: AssignmentInput) -> Result<AssignmentPlan, AssignmentError> {
    let m = input.metadata;
    let mut steps = Vec::new();
    // 0fac0827:38-49. This branch skips RHS evaluation entirely.
    if input.flags & 0x0200 != 0
        && input.rhs_kind == 0x60
        && require(m.rhs_special_call, "0fab0b42/0fac2f97(rhs[5])")?
        && require(m.lhs_address_kind, "0fac0f35(lhs).kind")? == 1
    {
        emit(
            &mut steps,
            0x392,
            &[
                require(m.rhs_call_slot, "0faab81f(rhs[5])")?,
                require(m.lhs_address_word, "0fac0f35(lhs).word_at_10")?,
            ],
        );
        return Ok(AssignmentPlan {
            steps,
            source_function: 0x0fac0827,
        });
    }
    steps.push(AssignmentStep::Evaluate {
        node: NodeRole::Right,
        mode: 2,
    });
    // 0fac0827:52-65; 399 decimal is 0x18f.
    if input.flags & 0x0400 != 0 && input.lhs_kind == 0x69 {
        emit(
            &mut steps,
            0x18f,
            &[
                require(m.indexed_symbol_slot, "0faab81f(lhs[4]+0x14)")?,
                require(m.indexed_value, "lhs[5] unwrapped field[4]")?,
            ],
        );
        return Ok(AssignmentPlan {
            steps,
            source_function: 0x0fac0827,
        });
    }
    let mut selected_flags = u32::from(input.flags & 0xff00);
    if input.flags & 0x6000 == 0 {
        if input.flags & 0x8000 != 0 && input.lhs_type == 0x0f && input.rhs_type == 0x0f {
            steps.push(AssignmentStep::Address {
                mode: 5,
                flags: 0,
                type_id: input.lhs_type,
            });
            emit(
                &mut steps,
                0x34e - u16::from(input.flags & 0x1000 != 0),
                &[],
            );
        } else if input.lhs_type != 0x12 {
            if require(m.rhs_emission_low_bit, "0fabf2d2(rhs,2) return bit0")? {
                selected_flags |= 0x80;
            }
            let reload = input.assignment_type != 2 && input.context_mode == 6;
            if input.assignment_type != 2 && !reload {
                selected_flags |= 0x40;
            }
            if input.lhs_kind == 0x69 && input.lhs_flags & 0x8000 != 0 {
                selected_flags |= 0x20;
            }
            steps.push(AssignmentStep::Address {
                mode: 4,
                flags: selected_flags,
                type_id: input.lhs_type,
            });
            if reload {
                steps.push(AssignmentStep::Evaluate {
                    node: NodeRole::Left,
                    mode: input.context_mode,
                });
            }
        } else {
            steps.push(AssignmentStep::Address {
                mode: 5,
                flags: 0,
                type_id: input.assignment_type,
            });
            let with_flag = input.flags & 0x1000 != 0;
            if input.has_assignment_descriptor {
                emit(
                    &mut steps,
                    if with_flag { 0x404 } else { 0x403 },
                    &[require(m.assignment_descriptor, "0fac2e29(assignment[6])")?],
                );
            } else {
                emit(&mut steps, if with_flag { 0x402 } else { 0x401 }, &[]);
            }
        }
    } else {
        // FAC0827:156-210: evaluated indexed address, then common store tail.
        if input.lhs_is_11_over_6a {
            steps.push(AssignmentStep::Evaluate {
                node: NodeRole::IndexedBase,
                mode: 5,
            });
            let base = if input.flags & 0x2000 == 0 {
                0x30e000
            } else {
                0x31e000
            };
            emit(
                &mut steps,
                ((!selected_flags & 0x1000 | base) >> 12) as u16,
                &[0xffff],
            );
            return Ok(AssignmentPlan {
                steps,
                source_function: 0x0fac0827,
            });
        }
        let (address_mode, operand) = match input.lhs_type {
            0x0f => (1, 0xffff),
            0x10 => (1, 0),
            0x11 => (5, require(m.lhs_type_descriptor, "0fb0261c(lhs[2])")?),
            _ => {
                return Err(AssignmentError::UnresolvedBranch(
                    "0fac0827:141-147 pointer-valued fallback",
                ));
            }
        };
        steps.push(AssignmentStep::Address {
            mode: address_mode,
            flags: 0,
            type_id: input.lhs_type,
        });
        // (~(flags & 0xff00) & 0x1000 | base) >> 12, base=0x30e000/0x31e000.
        let base = if input.flags & 0x2000 == 0 {
            0x30e000
        } else {
            0x31e000
        };
        let opcode = ((!selected_flags & 0x1000 | base) >> 12) as u16;
        emit(&mut steps, opcode, &[operand]);
    }
    Ok(AssignmentPlan {
        steps,
        source_function: 0x0fac0827,
    })
}
/// Native branch-entry table at DAT_0fac2028 (11 little-endian addresses).
pub const KIND_2D_BRANCH_TARGETS: [u32; 11] = [
    0x0fabfe6c, 0x0fb34670, 0x0fabfe6c, 0x0fabfe73, 0x0fb3467a, 0x0fb34699, 0x0fb3468f, 0x0fabfe7a,
    0x0fabfe73, 0x0fabfe73, 0x0fb346ae,
];
/// Type tags 3..=0x17 -> branch index, DAT_0fac2054.
pub const KIND_2D_TYPE_BRANCH: [u8; 21] = [
    0, 10, 1, 2, 10, 3, 10, 4, 5, 5, 6, 10, 10, 7, 10, 8, 10, 10, 10, 10, 9,
];
/// Port of native selector 0x0FABFE4B..0x0FABFE65 and target blocks.
/// The default block 0x0FB346AE copies caller context mode to opcode unchanged;
/// its legality is not proven by this observation.
pub fn kind_2d_opcode(
    lhs_type: u16,
    rhs_return_low_bit: Option<bool>,
    context_mode: Option<u16>,
) -> Result<u16, AssignmentError> {
    let branch = lhs_type
        .checked_sub(3)
        .and_then(|i| KIND_2D_TYPE_BRANCH.get(i as usize))
        .copied()
        .unwrap_or(10);
    Ok(match branch {
        0 | 2 => 0x2c3,
        1 => 0x2c2,
        3 | 8 | 9 => 0x2c4,
        4 => 0x2c4 + 4 * u16::from(require(rhs_return_low_bit, "0fabf2d2(rhs,2) bit0")?),
        5 => 0x2c5 + 4 * u16::from(require(rhs_return_low_bit, "0fabf2d2(rhs,2) bit0")?),
        6 => 0x2c5,
        7 => 0x2c7,
        _ => require(
            context_mode,
            "native default opcode from caller context [esp+0x50]",
        )?,
    })
}
/// Fully resolved specialized kind0x2D branch (does not prove input type validity).
pub fn select_kind_2d_nonvariant_resolved(
    lhs_type: u16,
    resolved_lhs_symbol_slot: Option<u16>,
    rhs_return_low_bit: Option<bool>,
    context_mode: Option<u16>,
) -> Result<AssignmentPlan, AssignmentError> {
    let opcode = kind_2d_opcode(lhs_type, rhs_return_low_bit, context_mode)?;
    Ok(AssignmentPlan {
        source_function: 0x0fabf2d2,
        steps: vec![
            AssignmentStep::Evaluate {
                node: NodeRole::Right,
                mode: 2,
            },
            AssignmentStep::Emit {
                opcode,
                words: vec![require(resolved_lhs_symbol_slot, "0faab81f(lhs[5])")?],
            },
        ],
    })
}
/// Compatibility entry point for constant-opcode branches. Dynamic cases demand
/// metadata explicitly via select_kind_2d_nonvariant_resolved.
pub fn select_kind_2d_nonvariant(
    lhs_type: u16,
    resolved_lhs_symbol_slot: Option<u16>,
) -> Result<AssignmentPlan, AssignmentError> {
    select_kind_2d_nonvariant_resolved(lhs_type, resolved_lhs_symbol_slot, None, None)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> AssignmentInput {
        AssignmentInput {
            assignment_type: 2,
            flags: 0,
            context_mode: 1,
            lhs_kind: 0x61,
            lhs_type: 0x12,
            lhs_flags: 0,
            rhs_kind: 1,
            rhs_type: 3,
            has_assignment_descriptor: false,
            lhs_is_11_over_6a: false,
            metadata: ResolvedAssignmentMetadata::default(),
        }
    }
    #[test]
    fn structured_store_opcode_depends_on_flag_and_descriptor() {
        for flag in [0, 0x1000] {
            for descriptor in [false, true] {
                let mut i = input();
                i.flags = flag;
                i.has_assignment_descriptor = descriptor;
                i.metadata.assignment_descriptor = Some(0x1234);
                let p = select_assignment(i).unwrap();
                let expected = 0x401 + u16::from(flag != 0) + 2 * u16::from(descriptor);
                assert_eq!(
                    p.steps[2],
                    AssignmentStep::Emit {
                        opcode: expected,
                        words: if descriptor { vec![0x1234] } else { vec![] }
                    }
                );
            }
        }
    }
    #[test]
    fn reference_store_polarity_is_inverted() {
        for flags in [0x4000, 0x5000, 0x6000, 0x7000] {
            let mut i = input();
            i.flags = flags;
            i.lhs_type = 0x0f;
            let p = select_assignment(i).unwrap();
            let expected = match flags {
                0x4000 => 0x30f,
                0x5000 => 0x30e,
                0x6000 => 0x31f,
                _ => 0x31e,
            };
            assert_eq!(
                p.steps[2],
                AssignmentStep::Emit {
                    opcode: expected,
                    words: vec![0xffff]
                }
            );
        }
    }
    #[test]
    fn special_call_skips_rhs_and_missing_metadata_fails() {
        let mut i = input();
        i.flags = 0x200;
        i.rhs_kind = 0x60;
        assert!(matches!(
            select_assignment(i),
            Err(AssignmentError::MissingMetadata(_))
        ));
        i.metadata.rhs_special_call = Some(true);
        i.metadata.lhs_address_kind = Some(1);
        i.metadata.rhs_call_slot = Some(7);
        i.metadata.lhs_address_word = Some(9);
        assert_eq!(
            select_assignment(i).unwrap().steps,
            vec![AssignmentStep::Emit {
                opcode: 0x392,
                words: vec![7, 9]
            }]
        );
    }
    #[test]
    fn dynamic_switch_needs_metadata_and_indexed_branch_is_recovered() {
        assert_eq!(kind_2d_opcode(3, None, None).unwrap(), 0x2c3);
        assert!(select_kind_2d_nonvariant(10, Some(7)).is_err());
        let mut i = input();
        i.flags = 0x6000;
        i.lhs_is_11_over_6a = true;
        let p = select_assignment(i).unwrap();
        assert!(p.steps.contains(&AssignmentStep::Evaluate {
            node: NodeRole::IndexedBase,
            mode: 5
        }));
        assert_eq!(
            p.steps.last(),
            Some(&AssignmentStep::Emit {
                opcode: 0x31f,
                words: vec![0xffff]
            })
        );
    }
}
