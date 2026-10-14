//! Versioned program container carrying Microsoft P-Code identities and
//! normalized operands for the Rust VM.

use std::fmt;

use vb6core::types::TypeInfo;

use crate::{table, DispatchTable, OpcodeInfo};

/// Version of the modern, platform-independent P-Code container.
pub const PROGRAM_FORMAT_VERSION: u16 = 1;

/// Reserved MX extension namespace version. No MX opcode is emitted yet.
pub const MX_OPCODE_VERSION: u16 = 1;

/// Base of the separate MX extension identifier space.
pub const MX_OPCODE_BASE: u16 = 0x8000;

/// An opcode identity in the single Microsoft ISA with a separate MX extension
/// namespace reserved for future instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpcodeIdentity {
    /// Original Microsoft dispatch-table identity.
    Microsoft(MicrosoftOpcodeId),
    /// Versioned extension identity. The compiler currently emits none.
    MxExtension { version: u16, opcode: u16 },
}

/// Microsoft opcode identity, including its dispatch table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MicrosoftOpcodeId {
    /// Dispatch table selected by the leading byte (Primary has no prefix).
    pub table: DispatchTable,
    /// Opcode byte within the selected table.
    pub opcode: u8,
}

impl MicrosoftOpcodeId {
    /// Construct an identity without changing the catalog's table or slot.
    pub const fn new(table: DispatchTable, opcode: u8) -> Self {
        Self { table, opcode }
    }

    /// Return the catalog entry associated with this dispatch-table slot.
    pub fn catalog_entry(self) -> &'static OpcodeInfo {
        &table(self.table)[usize::from(self.opcode)]
    }
}

/// A normalized frame location, independent of x86 offsets and ABI layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameSlot {
    /// A declared procedure parameter, zero-based in source order.
    Parameter(u16),
    /// The implicit result variable of a Function.
    FunctionResult,
}

/// Normalized operands owned by the modern container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    /// No operand.
    None,
    /// Signed 32-bit immediate used by literal opcodes such as `LitI4`.
    ImmediateI32(i32),
    /// A typed index in the module-shared constant pool.
    ConstantPool(u32),
    /// A symbolic frame slot, not a copied PE/x86 frame displacement.
    FrameSlot(FrameSlot),
}

/// Source symbol identity retained in procedure and debugger metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolRef {
    /// Semantic scope identity.
    pub scope_id: usize,
    /// Case-folded VB6 symbol name.
    pub name: String,
}

/// Parameter passing metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterPassing {
    /// Passed by value.
    ByVal,
    /// Passed by reference.
    ByRef,
}

/// Typed parameter in a procedure frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterSlot {
    /// Zero-based source-order parameter slot.
    pub slot: u16,
    /// Source spelling.
    pub name: String,
    /// Resolved VB6 type.
    pub ty: TypeInfo,
    /// VB6 parameter passing mode.
    pub passing: ParameterPassing,
    /// Bound semantic symbol.
    pub symbol: SymbolRef,
}

/// Source literal retained in the module-shared pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant {
    /// Literal's static VB6 type.
    pub ty: TypeInfo,
    /// Exact representation where currently supported.
    pub value: ConstantValue,
}

/// Constant payload variants for future supported literal lowering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstantValue {
    /// Integer payload.
    Integer(i64),
    /// UTF-8 string payload.
    String(String),
}

/// Value type plus reference mode on the evaluation stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackValue {
    /// Static VB6 type.
    pub ty: TypeInfo,
    /// True when the item is a managed reference/address.
    pub by_ref: bool,
}

/// Typed evaluation-stack effect for one instruction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StackEffect {
    /// Values consumed, in stack-pop order.
    pub pops: Vec<StackValue>,
    /// Values produced, in push order.
    pub pushes: Vec<StackValue>,
}

/// Effects relevant to validation and debugger presentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SideEffect {
    /// Reads or writes a procedure frame slot.
    FrameAccess,
    /// Performs checked arithmetic.
    MayRaise,
    /// Transfers control to the procedure return.
    ControlFlow,
}

/// P-Code instruction retaining the original opcode identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    /// Microsoft identity or versioned MX extension identity.
    pub opcode: OpcodeIdentity,
    /// Normalized modern operand.
    pub operand: Operand,
    /// Stack contract checked against the supported Microsoft handler subset.
    pub stack: StackEffect,
    /// Source map entry index.
    pub source_map_index: u32,
    /// Side effects for stepping and diagnostics.
    pub effects: Vec<SideEffect>,
}

/// P-Code-to-source map entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceMapEntry {
    /// Instruction index in its procedure.
    pub instruction: u32,
    /// Byte offsets in source text.
    pub span: (u32, u32),
    /// Source file.
    pub file: String,
    /// One-based line.
    pub line: usize,
    /// One-based column.
    pub column: usize,
    /// Relevant semantic symbol when available.
    pub symbol: Option<SymbolRef>,
}

/// One procedure's typed signature and Microsoft-identity instruction stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureCode {
    /// Stable module-local identity.
    pub id: u32,
    /// Source procedure name.
    pub name: String,
    /// Procedure symbol.
    pub symbol: SymbolRef,
    /// Typed parameter slots.
    pub parameters: Vec<ParameterSlot>,
    /// Function return type.
    pub return_type: TypeInfo,
    /// Microsoft-identity P-Code instructions.
    pub instructions: Vec<Instruction>,
    /// Instruction-to-source mappings.
    pub source_map: Vec<SourceMapEntry>,
    /// Maximum verified evaluation-stack depth.
    pub max_stack: u32,
}

/// Modern P-Code module container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    /// Container version; not an opcode namespace or ISA version.
    pub format_version: u16,
    /// Module name.
    pub module_name: String,
    /// Module-shared typed constant pool.
    pub constants: Vec<Constant>,
    /// Procedure table and code.
    pub procedures: Vec<ProcedureCode>,
}

/// Errors produced by static P-Code validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The container version is unsupported.
    UnsupportedContainerVersion(u16),
    /// An emitted opcode is not in the currently semantically supported subset.
    UnsupportedOpcode { procedure: String, instruction: u32 },
    /// The catalog entry's identity or operand format is inconsistent.
    CatalogMismatch { procedure: String, instruction: u32 },
    /// The normalized operand does not match the opcode semantics.
    InvalidOperand { procedure: String, instruction: u32 },
    /// Stack declaration differs from the documented handler contract.
    InvalidStackContract { procedure: String, instruction: u32 },
    /// Evaluation stack underflow.
    StackUnderflow { procedure: String, instruction: u32 },
    /// Actual and expected stack values differ.
    StackTypeMismatch { procedure: String, instruction: u32 },
    /// Source map is incomplete or inconsistent.
    InvalidSourceMap { procedure: String, instruction: u32 },
    /// A procedure does not terminate with a supported return instruction.
    MissingReturn { procedure: String },
    /// Procedure table metadata is inconsistent.
    InvalidProcedureTable,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ValidationError {}

/// Validate identities against the Microsoft catalog and verify the supported
/// typed stack contracts. This does not execute the program.
pub fn validate(program: &mut Program) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    if program.format_version != PROGRAM_FORMAT_VERSION {
        return Err(vec![ValidationError::UnsupportedContainerVersion(
            program.format_version,
        )]);
    }

    for (index, procedure) in program.procedures.iter_mut().enumerate() {
        if procedure.id as usize != index
            || procedure
                .parameters
                .iter()
                .enumerate()
                .any(|(slot, parameter)| parameter.slot as usize != slot)
        {
            errors.push(ValidationError::InvalidProcedureTable);
        }
        if procedure.source_map.len() != procedure.instructions.len() {
            errors.push(ValidationError::InvalidSourceMap {
                procedure: procedure.name.clone(),
                instruction: 0,
            });
        }

        let mut stack = Vec::new();
        let mut max_stack = 0usize;
        let mut stored_result = false;
        for (index, instruction) in procedure.instructions.iter().enumerate() {
            let pc = index as u32;
            if instruction.source_map_index as usize != index
                || procedure
                    .source_map
                    .get(index)
                    .is_none_or(|entry| entry.instruction != pc)
            {
                errors.push(ValidationError::InvalidSourceMap {
                    procedure: procedure.name.clone(),
                    instruction: pc,
                });
            }
            let Some((expected, expected_effects)) = expected_contract(procedure, instruction)
            else {
                errors.push(match instruction.opcode {
                    OpcodeIdentity::Microsoft(id) => {
                        if catalog_identity_is_valid(id) {
                            ValidationError::UnsupportedOpcode {
                                procedure: procedure.name.clone(),
                                instruction: pc,
                            }
                        } else {
                            ValidationError::CatalogMismatch {
                                procedure: procedure.name.clone(),
                                instruction: pc,
                            }
                        }
                    }
                    OpcodeIdentity::MxExtension { .. } => ValidationError::UnsupportedOpcode {
                        procedure: procedure.name.clone(),
                        instruction: pc,
                    },
                });
                continue;
            };
            if instruction.stack != expected || instruction.effects != expected_effects {
                errors.push(ValidationError::InvalidStackContract {
                    procedure: procedure.name.clone(),
                    instruction: pc,
                });
            }
            if matches!(
                instruction.opcode,
                OpcodeIdentity::Microsoft(id) if id == opcode_id(0x71) || id == opcode_id(0x70)
            ) {
                stored_result = true;
            }
            if stack.len() < expected.pops.len() {
                errors.push(ValidationError::StackUnderflow {
                    procedure: procedure.name.clone(),
                    instruction: pc,
                });
                continue;
            }
            let mut mismatch = false;
            for expected in &expected.pops {
                if stack.pop().as_ref() != Some(expected) {
                    mismatch = true;
                }
            }
            if mismatch {
                errors.push(ValidationError::StackTypeMismatch {
                    procedure: procedure.name.clone(),
                    instruction: pc,
                });
                continue;
            }
            stack.extend(expected.pushes.iter().cloned());
            max_stack = max_stack.max(stack.len());
            if is_exit(procedure, instruction)
                && (index + 1 != procedure.instructions.len() || !stack.is_empty())
            {
                errors.push(ValidationError::InvalidOperand {
                    procedure: procedure.name.clone(),
                    instruction: pc,
                });
            }
        }
        if procedure
            .instructions
            .last()
            .is_none_or(|instruction| !is_exit(procedure, instruction))
            || !stored_result
        {
            errors.push(ValidationError::MissingReturn {
                procedure: procedure.name.clone(),
            });
        }
        procedure.max_stack = max_stack as u32;
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

const fn opcode_id(opcode: u8) -> MicrosoftOpcodeId {
    MicrosoftOpcodeId::new(DispatchTable::Primary, opcode)
}

fn catalog_identity_is_valid(id: MicrosoftOpcodeId) -> bool {
    let info = id.catalog_entry();
    !info.mnemonic.is_empty() && info.opcode == usize::from(id.opcode)
}

fn is_exit(procedure: &ProcedureCode, instruction: &Instruction) -> bool {
    let expected = if procedure.return_type == TypeInfo::new(vb6core::types::VBType::Boolean) {
        opcode_id(0x15)
    } else {
        opcode_id(0x14)
    };
    instruction.opcode == OpcodeIdentity::Microsoft(expected)
}

fn expected_contract(
    procedure: &ProcedureCode,
    instruction: &Instruction,
) -> Option<(StackEffect, Vec<SideEffect>)> {
    let OpcodeIdentity::Microsoft(id) = instruction.opcode else {
        return None;
    };
    let info = id.catalog_entry();
    let val = |ty: &TypeInfo| StackValue {
        ty: ty.clone(),
        by_ref: false,
    };
    let long = TypeInfo::long();
    let boolean = TypeInfo::new(vb6core::types::VBType::Boolean);
    let contract = if id == opcode_id(0x6c) {
        let Operand::FrameSlot(FrameSlot::Parameter(index)) = instruction.operand else {
            return None;
        };
        if procedure.parameters.get(usize::from(index))?.ty != long
            || info.mnemonic != "ILdRf"
            || info.size != 3
            || info.operand_format != "%a"
            || info.pops != 0
            || info.pushes != 1
        {
            return None;
        }
        (
            StackEffect {
                pops: vec![],
                pushes: vec![val(&long)],
            },
            vec![SideEffect::FrameAccess],
        )
    } else if id == opcode_id(0xf5) {
        if let Operand::ImmediateI32(_) = instruction.operand {
            if info.mnemonic != "LitI4"
                || info.size != 5
                || info.operand_format != "%4"
                || info.pops != 0
                || info.pushes != 1
            {
                return None;
            }
            (
                StackEffect {
                    pops: vec![],
                    pushes: vec![val(&long)],
                },
                vec![],
            )
        } else {
            return None;
        }
    } else if id == opcode_id(0xaa) {
        if instruction.operand != Operand::None
            || info.mnemonic != "AddI4"
            || info.size != 1
            || !info.operand_format.is_empty()
            || info.pops != 2
            || info.pushes != 1
        {
            return None;
        }
        (
            StackEffect {
                pops: vec![val(&long), val(&long)],
                pushes: vec![val(&long)],
            },
            vec![SideEffect::MayRaise],
        )
    } else if let Some(contract) = long_operator_contract(id, instruction, info, &long) {
        contract
    } else if let Some(contract) = long_comparison_contract(id, instruction, info, &long, &boolean)
    {
        contract
    } else if id == opcode_id(0x70) {
        if instruction.operand != Operand::FrameSlot(FrameSlot::FunctionResult)
            || procedure.return_type != boolean
            || info.mnemonic != "FStI2"
            || info.size != 3
            || info.operand_format != "%a"
            || info.pops != 1
            || info.pushes != 0
        {
            return None;
        }
        (
            StackEffect {
                pops: vec![val(&boolean)],
                pushes: vec![],
            },
            vec![SideEffect::FrameAccess],
        )
    } else if id == opcode_id(0x71) {
        if instruction.operand != Operand::FrameSlot(FrameSlot::FunctionResult)
            || procedure.return_type != long
            || info.mnemonic != "FStR4"
            || info.size != 3
            || info.operand_format != "%a"
            || info.pops != 1
            || info.pushes != 0
        {
            return None;
        }
        (
            StackEffect {
                pops: vec![val(&long)],
                pushes: vec![],
            },
            vec![SideEffect::FrameAccess],
        )
    } else if id == opcode_id(0x14) {
        if instruction.operand != Operand::None
            || procedure.return_type != long
            || info.mnemonic != "ExitProc"
            || info.size != 1
            || !info.operand_format.is_empty()
            || info.pops != 0
            || info.pushes != 0
        {
            return None;
        }
        (StackEffect::default(), vec![SideEffect::ControlFlow])
    } else if id == opcode_id(0x15) {
        if instruction.operand != Operand::None
            || procedure.return_type != boolean
            || info.mnemonic != "ExitProcI2"
            || info.size != 1
            || !info.operand_format.is_empty()
            || info.pops != 0
            || info.pushes != 0
        {
            return None;
        }
        (StackEffect::default(), vec![SideEffect::ControlFlow])
    } else {
        return None;
    };
    Some(contract)
}

fn long_comparison_contract(
    id: MicrosoftOpcodeId,
    instruction: &Instruction,
    info: &OpcodeInfo,
    long: &TypeInfo,
    boolean: &TypeInfo,
) -> Option<(StackEffect, Vec<SideEffect>)> {
    let mnemonic = match (id.table, id.opcode) {
        (DispatchTable::Primary, 0xc7) => "EqI4",
        (DispatchTable::Primary, 0xcc) => "NeI4",
        (DispatchTable::Primary, 0xd1) => "LtI4",
        (DispatchTable::Primary, 0xd6) => "LeI4",
        (DispatchTable::Primary, 0xdb) => "GtI4",
        (DispatchTable::Primary, 0xe0) => "GeI4",
        _ => return None,
    };
    if instruction.operand != Operand::None
        || info.mnemonic != mnemonic
        || info.size != 1
        || !info.operand_format.is_empty()
        || info.pops != 2
        || info.pushes != 1
    {
        return None;
    }
    Some((
        StackEffect {
            pops: vec![
                StackValue {
                    ty: long.clone(),
                    by_ref: false,
                },
                StackValue {
                    ty: long.clone(),
                    by_ref: false,
                },
            ],
            pushes: vec![StackValue {
                ty: boolean.clone(),
                by_ref: false,
            }],
        },
        vec![],
    ))
}

fn long_operator_contract(
    id: MicrosoftOpcodeId,
    instruction: &Instruction,
    info: &OpcodeInfo,
    long: &TypeInfo,
) -> Option<(StackEffect, Vec<SideEffect>)> {
    let (mnemonic, arity, may_raise) = match (id.table, id.opcode) {
        (crate::DispatchTable::Primary, 0xae) => ("SubI4", 2, true),
        (crate::DispatchTable::Primary, 0xb2) => ("MulI4", 2, true),
        (crate::DispatchTable::Primary, 0xb8) => ("UMiI4", 1, true),
        (crate::DispatchTable::Primary, 0xc0) => ("IDvI4", 2, true),
        (crate::DispatchTable::Primary, 0xc2) => ("ModI4", 2, true),
        (crate::DispatchTable::Primary, 0xc3) => ("NotI4", 1, false),
        (crate::DispatchTable::Primary, 0xc4) => ("AndI4", 2, false),
        (crate::DispatchTable::Primary, 0xc5) => ("OrI4", 2, false),
        (crate::DispatchTable::Lead0, 0x11 | 0x12 | 0x13) => ("XorI4", 2, false),
        _ => return None,
    };
    if instruction.operand != Operand::None
        || info.mnemonic != mnemonic
        || info.size != 1
        || !info.operand_format.is_empty()
        || info.pops != arity
        || info.pushes != 1
    {
        return None;
    }
    let value = || StackValue {
        ty: long.clone(),
        by_ref: false,
    };
    Some((
        StackEffect {
            pops: (0..arity).map(|_| value()).collect(),
            pushes: vec![value()],
        },
        if may_raise {
            vec![SideEffect::MayRaise]
        } else {
            vec![]
        },
    ))
}

/// Build the catalog identity for a Primary-table opcode.
pub const fn primary_opcode(opcode: u8) -> OpcodeIdentity {
    OpcodeIdentity::Microsoft(opcode_id(opcode))
}
