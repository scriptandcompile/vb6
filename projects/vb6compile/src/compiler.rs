use vb6core::types::TypeInfo;
use vb6parse::files::ModuleFile;
use vb6parse::io::SourceFile;
use vb6pcode::program::{
    Constant, FrameSlot, Instruction, MicrosoftOpcodeId, OpcodeIdentity, Operand,
    PROGRAM_FORMAT_VERSION, ParameterPassing, ParameterSlot, ProcedureCode, Program, SideEffect,
    SourceMapEntry, StackEffect, StackValue, SymbolRef, primary_opcode,
};
use vb6semantic::{
    ExpressionKind, HirAnalysis, NodeInfo, Operator, PassingMode, Procedure, StatementKind,
    TypedProgram,
};

/// A source-located compiler diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileDiagnostic {
    /// Human-readable reason compilation stopped.
    pub message: String,
    /// Source location when available.
    pub location: Option<vb6semantic::SourceLocation>,
}

/// Compile a typed HIR module to the currently supported Microsoft P-Code subset.
pub fn compile_hir(analysis: &HirAnalysis) -> Result<Program, Vec<CompileDiagnostic>> {
    let Some(typed) = analysis.program.as_ref() else {
        return Err(analysis
            .errors
            .iter()
            .map(|error| CompileDiagnostic {
                message: error.to_string(),
                location: None,
            })
            .collect());
    };
    if !typed.compilable || !analysis.errors.is_empty() {
        return Err(vec![CompileDiagnostic {
            message: "semantic analysis did not produce compilable HIR".to_string(),
            location: Some(typed.info.location.clone()),
        }]);
    }
    PCodeBuilder::new().build(typed)
}

/// Parse, analyze, and compile one VB6 source module.
pub fn compile_source(name: &str, source: &str) -> Result<Program, Vec<CompileDiagnostic>> {
    let file = SourceFile::from_string(name, source);
    let (module, parse_errors) = ModuleFile::parse(&file).unpack();
    if !parse_errors.is_empty() {
        return Err(parse_errors
            .into_iter()
            .map(|error| CompileDiagnostic {
                message: format!("{error:?}"),
                location: None,
            })
            .collect());
    }
    let Some(module) = module else {
        return Err(vec![CompileDiagnostic {
            message: "parser produced no module".to_string(),
            location: None,
        }]);
    };
    let analysis = vb6semantic::SemanticAnalyzer::new()
        .analyze_module_hir(&module)
        .map_err(|error| {
            vec![CompileDiagnostic {
                message: error.to_string(),
                location: None,
            }]
        })?;
    compile_hir(&analysis)
}

/// Builder for the modern module container and the implemented Microsoft Long subset.
#[derive(Debug, Default)]
pub struct PCodeBuilder;

impl PCodeBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self
    }

    /// Lower one Long `Function` assigning an implemented Long expression over
    /// its parameters. Other HIR forms return a diagnostic.
    pub fn build(&self, typed: &TypedProgram) -> Result<Program, Vec<CompileDiagnostic>> {
        if !typed.declarations.is_empty() || typed.procedures.len() != 1 {
            return Err(vec![diagnostic(
                "only one Function with the implemented Long-expression body is supported",
                &typed.info,
            )]);
        }
        let procedure = &typed.procedures[0];
        let code = compile_supported_function(procedure)?;
        let program = Program {
            format_version: PROGRAM_FORMAT_VERSION,
            module_name: typed.name.clone(),
            constants: Vec::<Constant>::new(),
            procedures: vec![code],
        };
        Ok(program)
    }
}

fn compile_supported_function(
    procedure: &Procedure,
) -> Result<ProcedureCode, Vec<CompileDiagnostic>> {
    let return_type = procedure
        .signature
        .return_type
        .as_ref()
        .map(value_type)
        .ok_or_else(|| {
            vec![diagnostic(
                "Function has no supported return type",
                &procedure.info,
            )]
        })?;
    let boolean_type = TypeInfo::new(vb6core::types::VBType::Boolean);
    if return_type != TypeInfo::long() && return_type != boolean_type {
        return Err(vec![diagnostic(
            "the implemented expression subset requires a Long or Boolean Function result",
            &procedure.info,
        )]);
    }
    if procedure.signature.parameters.len() != 2
        || procedure
            .signature
            .parameters
            .iter()
            .any(|parameter| value_type(&parameter.ty) != TypeInfo::long())
    {
        return Err(vec![diagnostic(
            "the implemented Microsoft Long-expression subset requires exactly two Long parameters",
            &procedure.info,
        )]);
    }
    if procedure.body.len() != 1 {
        return Err(vec![diagnostic(
            "the implemented Microsoft Long-expression subset requires one result assignment",
            &procedure.info,
        )]);
    }
    let StatementKind::Assignment { target, value } = &procedure.body[0].kind else {
        return Err(vec![diagnostic(
            "the implemented Microsoft Long-expression body must assign the Function result",
            &procedure.body[0].info,
        )]);
    };
    if !matches!(&target.kind, ExpressionKind::Name(name) if name.eq_ignore_ascii_case(&procedure.name))
    {
        return Err(vec![diagnostic(
            "the assignment target must be the Long Function result",
            &target.info,
        )]);
    }
    if value.info.ty.as_ref().map(value_type) != Some(return_type.clone()) {
        return Err(vec![diagnostic(
            "the result expression type must match the Function return type",
            &value.info,
        )]);
    }

    let symbol = procedure
        .info
        .symbol
        .as_ref()
        .map(symbol_ref)
        .ok_or_else(|| {
            vec![diagnostic(
                "Function has no bound semantic symbol",
                &procedure.info,
            )]
        })?;
    let parameters = procedure
        .signature
        .parameters
        .iter()
        .enumerate()
        .map(|(slot, parameter)| {
            let slot = u16::try_from(slot).map_err(|_| {
                vec![diagnostic(
                    "procedure frame exceeds u16 slots",
                    &parameter.info,
                )]
            })?;
            let symbol = parameter
                .info
                .symbol
                .as_ref()
                .map(symbol_ref)
                .ok_or_else(|| {
                    vec![diagnostic(
                        "parameter has no bound semantic symbol",
                        &parameter.info,
                    )]
                })?;
            Ok(ParameterSlot {
                slot,
                name: parameter.name.clone(),
                ty: value_type(&parameter.ty),
                passing: match parameter.passing {
                    PassingMode::ByVal => ParameterPassing::ByVal,
                    PassingMode::ByRef => ParameterPassing::ByRef,
                    PassingMode::ParamArray => {
                        return Err(vec![diagnostic(
                            "ParamArray is outside the supported Add signature",
                            &parameter.info,
                        )]);
                    }
                },
                symbol,
            })
        })
        .collect::<Result<Vec<_>, Vec<CompileDiagnostic>>>()?;

    let mut instructions = Vec::new();
    let mut source_map = Vec::new();
    if return_type == TypeInfo::long() {
        emit_long_expression(procedure, value, &mut instructions, &mut source_map)?;
    } else {
        emit_boolean_expression(procedure, value, &mut instructions, &mut source_map)?;
    }
    let (store_opcode, exit_opcode) = if return_type == TypeInfo::long() {
        (0x71, 0x14)
    } else {
        (0x70, 0x15)
    };
    emit(
        &mut instructions,
        &mut source_map,
        store_opcode,
        Operand::FrameSlot(FrameSlot::FunctionResult),
        StackEffect {
            pops: vec![stack_value(&return_type)],
            pushes: vec![],
        },
        vec![SideEffect::FrameAccess],
        &procedure.body[0].info,
    );
    emit(
        &mut instructions,
        &mut source_map,
        exit_opcode,
        Operand::None,
        StackEffect::default(),
        vec![SideEffect::ControlFlow],
        &procedure.info,
    );
    let max_stack = max_stack_depth(&instructions, &procedure.info)?;

    Ok(ProcedureCode {
        id: 0,
        name: procedure.name.clone(),
        symbol,
        parameters,
        return_type,
        instructions,
        source_map,
        max_stack,
    })
}

fn emit_boolean_expression(
    procedure: &Procedure,
    expression: &vb6semantic::Expression,
    instructions: &mut Vec<Instruction>,
    source_map: &mut Vec<SourceMapEntry>,
) -> Result<(), Vec<CompileDiagnostic>> {
    let boolean = TypeInfo::new(vb6core::types::VBType::Boolean);
    if expression.info.ty.as_ref().map(value_type) != Some(boolean.clone()) {
        return Err(vec![diagnostic(
            "only Boolean comparison expressions are supported here",
            &expression.info,
        )]);
    }
    match &expression.kind {
        ExpressionKind::Parenthesized(inner) => {
            emit_boolean_expression(procedure, inner, instructions, source_map)?;
        }
        ExpressionKind::Binary {
            operator: Operator::Source(_),
            left,
            right,
        } => {
            let result_type = expression.info.ty.as_ref().map(value_type).ok_or_else(|| {
                vec![diagnostic(
                    "Boolean expression has no resolved type",
                    &expression.info,
                )]
            })?;
            let left_type = left.info.ty.as_ref().map(value_type).ok_or_else(|| {
                vec![diagnostic(
                    "left comparison operand has no resolved type",
                    &left.info,
                )]
            })?;
            let right_type = right.info.ty.as_ref().map(value_type).ok_or_else(|| {
                vec![diagnostic(
                    "right comparison operand has no resolved type",
                    &right.info,
                )]
            })?;
            let (resolved, _) =
                select_native_operator(expression, &result_type, &left_type, Some(&right_type))
                    .map_err(|error| vec![diagnostic(&error, &expression.info)])?;
            let resolved = resolved.ok_or_else(|| {
                vec![diagnostic(
                    "Boolean native AST mapping did not select an opcode",
                    &expression.info,
                )]
            })?;
            emit_long_expression(procedure, left, instructions, source_map)?;
            emit_long_expression(procedure, right, instructions, source_map)?;
            emit_with_opcode(
                instructions,
                source_map,
                OpcodeIdentity::Microsoft(resolved),
                Operand::None,
                StackEffect {
                    pops: vec![
                        stack_value(&TypeInfo::long()),
                        stack_value(&TypeInfo::long()),
                    ],
                    pushes: vec![stack_value(&boolean)],
                },
                vec![],
                &expression.info,
            );
        }
        _ => {
            return Err(vec![diagnostic(
                "only comparisons between Long expressions are supported for Boolean results",
                &expression.info,
            )]);
        }
    }
    Ok(())
}

fn emit_long_expression(
    procedure: &Procedure,
    expression: &vb6semantic::Expression,
    instructions: &mut Vec<Instruction>,
    source_map: &mut Vec<SourceMapEntry>,
) -> Result<(), Vec<CompileDiagnostic>> {
    if expression.info.ty.as_ref().map(value_type) != Some(TypeInfo::long()) {
        return Err(vec![diagnostic(
            "only Long-valued expression nodes are supported",
            &expression.info,
        )]);
    }

    match &expression.kind {
        ExpressionKind::Name(name) => {
            let Some((slot, parameter)) = procedure
                .signature
                .parameters
                .iter()
                .enumerate()
                .find(|(_, parameter)| parameter.name.eq_ignore_ascii_case(name))
            else {
                return Err(vec![diagnostic(
                    "expression names must resolve to one of the Long parameters",
                    &expression.info,
                )]);
            };
            if expression.info.symbol.as_ref().map(symbol_ref)
                != parameter.info.symbol.as_ref().map(symbol_ref)
            {
                return Err(vec![diagnostic(
                    "expression name is not bound to the matching parameter",
                    &expression.info,
                )]);
            }
            let slot = u16::try_from(slot).map_err(|_| {
                vec![diagnostic(
                    "procedure frame exceeds u16 slots",
                    &parameter.info,
                )]
            })?;
            emit(
                instructions,
                source_map,
                0x6c,
                Operand::FrameSlot(FrameSlot::Parameter(slot)),
                StackEffect {
                    pops: vec![],
                    pushes: vec![stack_value(&TypeInfo::long())],
                },
                vec![SideEffect::FrameAccess],
                &expression.info,
            );
        }
        ExpressionKind::Literal(text) => {
            let value = parse_long_literal(text).ok_or_else(|| {
                vec![diagnostic(
                    "only decimal Long literals are currently supported",
                    &expression.info,
                )]
            })?;
            emit_long_literal(value, &expression.info, instructions, source_map);
        }
        ExpressionKind::Parenthesized(inner) => {
            emit_long_expression(procedure, inner, instructions, source_map)?;
        }
        ExpressionKind::Unary { operator, operand } => {
            let Operator::Source(_) = operator;
            let result_type = expression.info.ty.as_ref().map(value_type).ok_or_else(|| {
                vec![diagnostic(
                    "unary expression has no resolved type",
                    &expression.info,
                )]
            })?;
            let operand_type = operand.info.ty.as_ref().map(value_type).ok_or_else(|| {
                vec![diagnostic(
                    "unary operand has no resolved type",
                    &operand.info,
                )]
            })?;
            let (resolved, may_raise) =
                select_native_operator(expression, &result_type, &operand_type, None)
                    .map_err(|error| vec![diagnostic(&error, &expression.info)])?;
            emit_long_expression(procedure, operand, instructions, source_map)?;
            if let Some(resolved) = resolved {
                emit_with_opcode(
                    instructions,
                    source_map,
                    OpcodeIdentity::Microsoft(resolved),
                    Operand::None,
                    StackEffect {
                        pops: vec![stack_value(&TypeInfo::long())],
                        pushes: vec![stack_value(&TypeInfo::long())],
                    },
                    if may_raise {
                        vec![SideEffect::MayRaise]
                    } else {
                        vec![]
                    },
                    &expression.info,
                );
            }
        }
        ExpressionKind::Conversion {
            expression: inner,
            target_type,
            ..
        } if value_type(target_type) == TypeInfo::long()
            && inner.info.ty.as_ref().map(value_type) == Some(TypeInfo::integer()) =>
        {
            let ExpressionKind::Literal(text) = &inner.kind else {
                return Err(vec![diagnostic(
                    "only integer literals may be widened to Long in this subset",
                    &expression.info,
                )]);
            };
            let value = parse_long_literal(text).filter(|value| i16::try_from(*value).is_ok());
            let value = value.ok_or_else(|| {
                vec![diagnostic(
                    "widened integer literal is outside the signed 16-bit range",
                    &inner.info,
                )]
            })?;
            emit_long_literal(value, &expression.info, instructions, source_map);
        }
        ExpressionKind::Binary {
            operator: Operator::Source(_),
            left,
            right,
        } => {
            let result_type = expression.info.ty.as_ref().map(value_type).ok_or_else(|| {
                vec![diagnostic(
                    "binary expression has no resolved type",
                    &expression.info,
                )]
            })?;
            let left_type = left.info.ty.as_ref().map(value_type).ok_or_else(|| {
                vec![diagnostic(
                    "left binary operand has no resolved type",
                    &left.info,
                )]
            })?;
            let right_type = right.info.ty.as_ref().map(value_type).ok_or_else(|| {
                vec![diagnostic(
                    "right binary operand has no resolved type",
                    &right.info,
                )]
            })?;
            let (resolved, may_raise) =
                select_native_operator(expression, &result_type, &left_type, Some(&right_type))
                    .map_err(|error| vec![diagnostic(&error, &expression.info)])?;
            let resolved = resolved.ok_or_else(|| {
                vec![diagnostic(
                    "native AST mapping did not select an opcode",
                    &expression.info,
                )]
            })?;
            emit_long_expression(procedure, left, instructions, source_map)?;
            emit_long_expression(procedure, right, instructions, source_map)?;
            emit_with_opcode(
                instructions,
                source_map,
                OpcodeIdentity::Microsoft(resolved),
                Operand::None,
                StackEffect {
                    pops: vec![
                        stack_value(&TypeInfo::long()),
                        stack_value(&TypeInfo::long()),
                    ],
                    pushes: vec![stack_value(&TypeInfo::long())],
                },
                if may_raise {
                    vec![SideEffect::MayRaise]
                } else {
                    vec![]
                },
                &expression.info,
            );
        }
        _ => {
            return Err(vec![diagnostic(
                "unsupported expression in the implemented Long-expression subset",
                &expression.info,
            )]);
        }
    }
    Ok(())
}

fn parse_long_literal(text: &str) -> Option<i32> {
    text.trim()
        .strip_suffix('&')
        .unwrap_or(text.trim())
        .parse()
        .ok()
}

fn select_native_operator(
    expression: &vb6semantic::Expression,
    result_type: &TypeInfo,
    left_type: &TypeInfo,
    right_type: Option<&TypeInfo>,
) -> Result<(Option<MicrosoftOpcodeId>, bool), String> {
    let Some(kind) = expression.native_ast.kind else {
        if expression.native_ast.evidence == vb6semantic::NativeAstEvidence::Passthrough {
            return Ok((None, false));
        }
        return Err("expression has no resolved native AST kind".to_string());
    };
    let native_type = |ty: &TypeInfo| match ty.kind {
        vb6core::types::VBType::Boolean => Ok(3),
        vb6core::types::VBType::Byte => Ok(5),
        vb6core::types::VBType::Integer => Ok(6),
        vb6core::types::VBType::Long => Ok(8),
        vb6core::types::VBType::Single => Ok(10),
        vb6core::types::VBType::Double => Ok(11),
        vb6core::types::VBType::Date => Ok(12),
        vb6core::types::VBType::Currency => Ok(13),
        vb6core::types::VBType::Variant => Ok(15),
        vb6core::types::VBType::String => Ok(16),
        _ => Err(format!("type {ty} has no recovered native AST type ID")),
    };
    let (logical, _) =
        vba6_ast_codegen::tables::select_typed(vba6_ast_codegen::tables::TypedInput {
            kind,
            result_type: native_type(result_type)?,
            left_type: native_type(left_type)?,
            right_type: right_type.map(native_type).transpose()?,
            flags: 0,
            slot: None,
        })
        .map_err(|error| format!("native AST opcode selection failed: {error:?}"))?;
    let opcode = vb6pcode::logical_opcode_map::microsoft_identity(logical)?;
    let may_raise = matches!(kind, 0x07 | 0x16..=0x1e);
    Ok((Some(opcode), may_raise))
}

fn emit_long_literal(
    value: i32,
    info: &NodeInfo,
    instructions: &mut Vec<Instruction>,
    source_map: &mut Vec<SourceMapEntry>,
) {
    emit(
        instructions,
        source_map,
        0xf5,
        Operand::ImmediateI32(value),
        StackEffect {
            pops: vec![],
            pushes: vec![stack_value(&TypeInfo::long())],
        },
        vec![],
        info,
    );
}

fn emit(
    instructions: &mut Vec<Instruction>,
    source_map: &mut Vec<SourceMapEntry>,
    opcode_byte: u8,
    operand: Operand,
    stack: StackEffect,
    effects: Vec<SideEffect>,
    info: &NodeInfo,
) {
    emit_with_opcode(
        instructions,
        source_map,
        primary_opcode(opcode_byte),
        operand,
        stack,
        effects,
        info,
    );
}

fn emit_with_opcode(
    instructions: &mut Vec<Instruction>,
    source_map: &mut Vec<SourceMapEntry>,
    opcode: OpcodeIdentity,
    operand: Operand,
    stack: StackEffect,
    effects: Vec<SideEffect>,
    info: &NodeInfo,
) {
    let instruction = instructions.len() as u32;
    source_map.push(SourceMapEntry {
        instruction,
        span: (info.span.start, info.span.end),
        file: info.location.file.clone(),
        line: info.location.line,
        column: info.location.column,
        symbol: info.symbol.as_ref().map(symbol_ref),
    });
    instructions.push(Instruction {
        opcode,
        operand,
        stack,
        source_map_index: instruction,
        effects,
    });
}

fn max_stack_depth(
    instructions: &[Instruction],
    info: &NodeInfo,
) -> Result<u32, Vec<CompileDiagnostic>> {
    let mut depth = 0u32;
    let mut maximum = 0u32;
    for instruction in instructions {
        let pops = u32::try_from(instruction.stack.pops.len())
            .map_err(|_| vec![diagnostic("stack effect exceeds u32", info)])?;
        let pushes = u32::try_from(instruction.stack.pushes.len())
            .map_err(|_| vec![diagnostic("stack effect exceeds u32", info)])?;
        depth = depth.checked_sub(pops).ok_or_else(|| {
            vec![diagnostic(
                "generated P-Code has an evaluation-stack underflow",
                info,
            )]
        })?;
        depth = depth
            .checked_add(pushes)
            .ok_or_else(|| vec![diagnostic("generated P-Code stack depth exceeds u32", info)])?;
        maximum = maximum.max(depth);
    }
    if depth != 0 {
        return Err(vec![diagnostic(
            "generated P-Code leaves values on the evaluation stack",
            info,
        )]);
    }
    Ok(maximum)
}

fn stack_value(ty: &TypeInfo) -> StackValue {
    StackValue {
        ty: ty.clone(),
        by_ref: false,
    }
}

fn value_type(ty: &TypeInfo) -> TypeInfo {
    let mut value = ty.clone();
    value.is_reference = false;
    value
}

fn symbol_ref(symbol: &vb6semantic::SymbolId) -> SymbolRef {
    SymbolRef {
        scope_id: symbol.scope_id,
        name: symbol.name.clone(),
    }
}

fn diagnostic(message: &str, info: &NodeInfo) -> CompileDiagnostic {
    CompileDiagnostic {
        message: message.to_string(),
        location: Some(info.location.clone()),
    }
}
