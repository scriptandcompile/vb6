//! Numeric dispatch coverage for all116 observed AST IDs. These are routes, not
//! a false one-to-one AST→opcode enum. Actual selection consumes node/type/context.
use crate::tables::{AST_FLAGS, OPCODE_BASE};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmissionPath {
    TypedOperator,
    Dispatcher,
    RuntimeBuiltinLowering {
        builtin: u16,
    },
    /// Only the original expression dispatcher is return-only for this tag.
    NoDirectEmission,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstEmissionRule {
    pub ast_kind: u16,
    pub kind_flags: Option<u8>,
    /// Raw original table entry, including0x446 sentinel. Not a final opcode.
    pub table_base: Option<u16>,
    pub path: EmissionPath,
    pub native_dispatcher: u32,
    /// Principal helper when the source dispatcher delegates selection.
    pub selection_entry: Option<u32>,
}
const fn build_rules() -> [AstEmissionRule; 116] {
    let mut rules = [AstEmissionRule {
        ast_kind: 0,
        kind_flags: None,
        table_base: None,
        path: EmissionPath::Dispatcher,
        native_dispatcher: 0x0fabf2d2,
        selection_entry: None,
    }; 116];
    let mut i = 0;
    while i < 116 {
        rules[i].ast_kind = i as u16;
        if i < 115 {
            rules[i].kind_flags = Some(AST_FLAGS[i]);
            rules[i].table_base = Some(OPCODE_BASE[i]);
        }
        rules[i].path = match i {
            0x3c => EmissionPath::RuntimeBuiltinLowering { builtin: 9 },
            0x3d => EmissionPath::RuntimeBuiltinLowering { builtin: 12 },
            0x40 => EmissionPath::RuntimeBuiltinLowering { builtin: 0x9b },
            0x5b => EmissionPath::RuntimeBuiltinLowering { builtin: 0x140 },
            0 | 0x1b | 0x1c | 0x2e | 0x35 | 0x62 | 0x64 | 0x6f | 0x70 | 0x71 => {
                EmissionPath::NoDirectEmission
            }
            6..=10 | 0x16 | 0x17 | 0x1d..=0x23 | 0x25..=0x2b | 0x72 => EmissionPath::TypedOperator,
            _ => EmissionPath::Dispatcher,
        };
        rules[i].selection_entry = match i {
            6..=10 | 0x16 | 0x17 | 0x1d..=0x23 | 0x25..=0x2b | 0x72 => Some(0x0fac2a60),
            0x0b => Some(0x0fac2f28),
            0x0c | 0x0d => Some(0x0fac2ff3),
            0x11 => Some(0x0fac3668),
            0x2c => Some(0x0fac0827),
            0x32 | 0x34 | 0x54 => Some(0x0fac409d),
            0x3c | 0x3d | 0x40 | 0x5b => Some(0x0faec8d3),
            0x70 => Some(0x0fac53d7),
            0x71 => Some(0x0fab557a),
            0x42 | 0x43 => Some(0x0fb0b52f),
            0x51 | 0x52 => Some(0x0fac4125),
            0x5a => Some(0x0faeb01b),
            0x60 => Some(0x0fac35b5),
            0x61 => Some(0x0fac3506),
            0x69 => Some(0x0fac13f1),
            0x6b..=0x6e => Some(0x0fac3e10),
            _ => None,
        };
        i += 1;
    }
    rules
}
pub const AST_EMISSION_RULES: [AstEmissionRule; 116] = build_rules();
pub fn ast_emission_rule(kind: u16) -> Option<&'static AstEmissionRule> {
    AST_EMISSION_RULES.get(kind as usize)
}
