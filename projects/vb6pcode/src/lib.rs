//! Standalone metadata catalog and byte-stream decoder for Microsoft VB6 P-Code.

pub mod logical_opcode_map;
pub mod operator_selector;
pub mod program;

use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DispatchTable {
    Primary = 0,
    Lead0 = 1,
    Lead1 = 2,
    Lead2 = 3,
    Lead3 = 4,
    Lead4 = 5,
}

impl DispatchTable {
    pub const ALL: [Self; 6] = [
        Self::Primary,
        Self::Lead0,
        Self::Lead1,
        Self::Lead2,
        Self::Lead3,
        Self::Lead4,
    ];
    pub const fn prefix(self) -> Option<u8> {
        match self {
            Self::Primary => None,
            Self::Lead0 => Some(0xfb),
            Self::Lead1 => Some(0xfc),
            Self::Lead2 => Some(0xfd),
            Self::Lead3 => Some(0xfe),
            Self::Lead4 => Some(0xff),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OpcodeInfo {
    pub table: usize,
    pub opcode: usize,
    pub mnemonic: &'static str,
    pub size: i16,
    pub operand_format: &'static str,
    pub pops: i16,
    pub pushes: i16,
    pub fpu_pops: i16,
    pub fpu_push: &'static str,
    pub stack_effect: &'static str,
    pub popped: &'static str,
    pub memory_effect: &'static str,
    pub movement: &'static str,
    pub category: &'static str,
    pub handler_8176: &'static str,
    pub handler_9848: &'static str,
    pub notes: &'static str,
    pub slot_class: &'static str,
    pub m1_classification: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/catalog_generated.rs"));

pub const OPCODE_CSV: &str = include_str!("../data/opcodes.csv");
pub const M1_MATRIX_CSV: &str = include_str!("../data/OPCODE-MATRIX.csv");
pub const OPCODE_FORMS_JSON: &str = include_str!("../data/opcode-forms.json");
pub const DISPATCH_MAP_JSON: &str = include_str!("../data/dispatch-map-v2.json");

pub fn table(table: DispatchTable) -> &'static [OpcodeInfo; 256] {
    &TABLES[table as usize]
}

pub fn lookup(first: u8, second: Option<u8>) -> Result<(&'static OpcodeInfo, usize), DecodeError> {
    let (table_index, opcode, width) = match first {
        0xfb..=0xff => {
            let second = second.ok_or(DecodeError::Truncated {
                offset: 1,
                needed: 1,
            })?;
            (usize::from(first - 0xfa), second, 2)
        }
        _ => (0, first, 1),
    };
    Ok((&TABLES[table_index][usize::from(opcode)], width))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodedOperand {
    Byte(u8),
    Int16(i16),
    Int32(i32),
    StackVar(i16),
    PoolIndex(u16),
    JumpTarget(u16),
    VTableRef { offset: u16, interface: u16 },
    ExternalCall { import: u16, arg_bytes: u16 },
    Raw { format: char, bytes: Vec<u8> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedInstruction {
    pub table: DispatchTable,
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub offset: usize,
    pub length: usize,
    pub operands: Vec<DecodedOperand>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodeError {
    Truncated {
        offset: usize,
        needed: usize,
    },
    InvalidSlot {
        table: DispatchTable,
        opcode: u8,
    },
    OutsideTable {
        table: DispatchTable,
        opcode: u8,
    },
    UnknownLength {
        mnemonic: &'static str,
    },
    InvalidOperandFormat {
        mnemonic: &'static str,
        format: char,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl Error for DecodeError {}

pub fn decode_one(input: &[u8], offset: usize) -> Result<DecodedInstruction, DecodeError> {
    let first = *input
        .get(offset)
        .ok_or(DecodeError::Truncated { offset, needed: 1 })?;
    let second = if (0xfb..=0xff).contains(&first) {
        Some(*input.get(offset + 1).ok_or(DecodeError::Truncated {
            offset: offset + 1,
            needed: 1,
        })?)
    } else {
        None
    };
    let (info, prefix_len) = lookup(first, second)?;
    let table_id = DispatchTable::ALL[info.table];
    if info.slot_class == "INVALID_SLOT" {
        return Err(DecodeError::InvalidSlot {
            table: table_id,
            opcode: info.opcode as u8,
        });
    }
    if info.slot_class == "UNKNOWN_SLOT" {
        return Err(DecodeError::OutsideTable {
            table: table_id,
            opcode: info.opcode as u8,
        });
    }
    if info.slot_class == "PREFIX_SELECTOR" {
        return Err(DecodeError::InvalidOperandFormat {
            mnemonic: info.mnemonic,
            format: '?',
        });
    }
    let length = if info.size > 0 {
        info.size as usize + usize::from(info.table != 0)
    } else if info.size == -1 {
        let size_at = offset + prefix_len;
        let size_bytes = input
            .get(size_at..size_at + 2)
            .ok_or(DecodeError::Truncated {
                offset: size_at,
                needed: 2,
            })?;
        prefix_len + 2 + u16::from_le_bytes([size_bytes[0], size_bytes[1]]) as usize
    } else {
        return Err(DecodeError::UnknownLength {
            mnemonic: info.mnemonic,
        });
    };
    let end = offset.checked_add(length).ok_or(DecodeError::Truncated {
        offset,
        needed: length,
    })?;
    let bytes = input.get(offset..end).ok_or(DecodeError::Truncated {
        offset,
        needed: length,
    })?;
    let mut cursor = prefix_len + usize::from(info.size == -1) * 2;
    let mut operands = Vec::new();
    let format = info.operand_format.as_bytes();
    let mut index = 0;
    while index < format.len() {
        if format[index] != b'%' {
            index += 1;
            continue;
        }
        index += 1;
        let spec = *format.get(index).ok_or(DecodeError::InvalidOperandFormat {
            mnemonic: info.mnemonic,
            format: '%',
        })? as char;
        index += 1;
        let width = match spec {
            '1' => 1,
            '2' | 'a' | 's' | 'l' | 'c' => 2,
            '4' | 'v' | 'x' => 4,
            'N' | 'D' | 'L' | 'F' => end - offset - cursor,
            _ => {
                return Err(DecodeError::InvalidOperandFormat {
                    mnemonic: info.mnemonic,
                    format: spec,
                });
            }
        };
        let raw = bytes
            .get(cursor..cursor + width)
            .ok_or(DecodeError::Truncated {
                offset: offset + cursor,
                needed: width,
            })?;
        let u16_at = |i: usize| u16::from_le_bytes([raw[i], raw[i + 1]]);
        operands.push(match spec {
            '1' => DecodedOperand::Byte(raw[0]),
            '2' => DecodedOperand::Int16(i16::from_le_bytes([raw[0], raw[1]])),
            '4' => DecodedOperand::Int32(i32::from_le_bytes(
                raw.try_into().expect("four-byte operand"),
            )),
            'a' => DecodedOperand::StackVar(i16::from_le_bytes([raw[0], raw[1]])),
            's' | 'c' => DecodedOperand::PoolIndex(u16_at(0)),
            'l' => DecodedOperand::JumpTarget(u16_at(0)),
            'v' => DecodedOperand::VTableRef {
                offset: u16_at(0),
                interface: u16_at(2),
            },
            'x' => DecodedOperand::ExternalCall {
                import: u16_at(0),
                arg_bytes: u16_at(2),
            },
            other => DecodedOperand::Raw {
                format: other,
                bytes: raw.to_vec(),
            },
        });
        cursor += width;
    }
    Ok(DecodedInstruction {
        table: table_id,
        opcode: info.opcode as u8,
        mnemonic: info.mnemonic,
        offset,
        length,
        operands,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_six_complete_tables_and_expected_m1_totals() {
        assert_eq!(TABLES.len(), 6);
        assert!(TABLES.iter().all(|t| t.len() == 256));
        let entries = TABLES.iter().flatten().collect::<Vec<_>>();
        assert_eq!(
            entries
                .iter()
                .filter(|e| e.slot_class == "DOCUMENTED_FORM")
                .count(),
            1158
        );
        assert_eq!(
            entries
                .iter()
                .filter(|e| e.slot_class == "PREFIX_SELECTOR")
                .count(),
            5
        );
        assert_eq!(
            entries
                .iter()
                .filter(|e| e.m1_classification == "INVALID_CONFIRMED")
                .count(),
            188
        );
        assert_eq!(
            entries
                .iter()
                .filter(|e| e.m1_classification == "OUT_OF_PHYSICAL_TABLE")
                .count(),
            185
        );
    }

    #[test]
    fn decodes_operands_prefixes_and_rejects_truncated_streams() {
        let decoded = decode_one(&[0xf5, 1, 2, 3, 4], 0).unwrap();
        assert_eq!(decoded.mnemonic, "LitI4");
        assert_eq!(decoded.length, 5);
        assert_eq!(decoded.operands, [DecodedOperand::Int32(0x0403_0201)]);

        let extended_info = TABLES[1]
            .iter()
            .find(|entry| entry.slot_class == "DOCUMENTED_FORM")
            .unwrap();
        let mut extended_bytes = vec![0xfb, extended_info.opcode as u8];
        extended_bytes.resize(extended_info.size as usize + 1, 0);
        let extended = decode_one(&extended_bytes, 0).unwrap();
        assert_eq!(extended.table, DispatchTable::Lead0);
        assert_eq!(extended.length, extended_info.size as usize + 1);
        assert!(matches!(
            decode_one(&[0xfb], 0),
            Err(DecodeError::Truncated { .. })
        ));
        assert!(matches!(
            decode_one(&[0xf5, 1, 2], 0),
            Err(DecodeError::Truncated { .. })
        ));
    }

    #[test]
    fn every_documented_form_decodes_from_a_minimal_zero_filled_encoding() {
        for info in TABLES
            .iter()
            .flatten()
            .filter(|info| info.slot_class == "DOCUMENTED_FORM")
        {
            let table = DispatchTable::ALL[info.table];
            let mut bytes = match table.prefix() {
                Some(prefix) => vec![prefix, info.opcode as u8],
                None => vec![info.opcode as u8],
            };
            if info.size == -1 {
                let payload_size = info
                    .operand_format
                    .as_bytes()
                    .windows(2)
                    .filter(|pair| pair[0] == b'%')
                    .map(|pair| match pair[1] {
                        b'1' => 1,
                        b'2' | b'a' | b's' | b'l' | b'c' => 2,
                        b'4' | b'v' | b'x' => 4,
                        _ => 0,
                    })
                    .sum::<usize>();
                bytes.extend_from_slice(&(payload_size as u16).to_le_bytes());
                bytes.resize(bytes.len() + payload_size, 0);
            } else {
                let total = info.size as usize + usize::from(info.table != 0);
                bytes.resize(total, 0);
            }
            assert!(
                decode_one(&bytes, 0).is_ok(),
                "{}: {:?}",
                info.mnemonic,
                decode_one(&bytes, 0)
            );
        }
    }

    #[test]
    fn distinguishes_invalid_and_outside_slots() {
        assert!(matches!(
            decode_one(&[0x01], 0),
            Err(DecodeError::InvalidSlot { .. })
        ));
        let outside = TABLES
            .iter()
            .flatten()
            .find(|entry| entry.slot_class == "UNKNOWN_SLOT")
            .unwrap();
        let code = if outside.table == 0 {
            vec![outside.opcode as u8]
        } else {
            vec![0xfa + outside.table as u8, outside.opcode as u8]
        };
        assert!(matches!(
            decode_one(&code, 0),
            Err(DecodeError::OutsideTable { .. })
        ));
    }

    #[test]
    fn exposes_original_catalog_assets() {
        assert!(OPCODE_CSV.contains("operand_format"));
        assert!(M1_MATRIX_CSV.contains("m1_classification"));
        assert!(OPCODE_FORMS_JSON.contains("length_rule"));
        assert!(DISPATCH_MAP_JSON.contains("dispatch-map.v2"));
    }
}
