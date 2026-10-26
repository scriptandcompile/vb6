//! Typed VB6 intermediate representation with a native-compatible lowering view.
//!
//! HIR spans are byte ranges in the module CST, which remains the authoritative
//! source representation. The typed statement tree remains available for
//! semantic checks; each procedure also carries a flattened native control-flow
//! view with recovered Microsoft AST tags and evidence.
//! Most MX-only forms remain reserved; opt-in `Return <expr>` is lowered and
//! checked against its Function result type.

use crate::{ScopeKind, ScopeManager, SemanticError};
use crate::{SourceLocation, TypeInfo};
use vb6parse::files::ModuleFile;
use vb6parse::parsers::SyntaxKind;
use vb6parse::parsers::cst::CstNode;

/// Byte range of an HIR node in the original CST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceSpan {
    /// Inclusive starting byte offset.
    pub start: u32,
    /// Exclusive ending byte offset.
    pub end: u32,
}

/// Stable identity of a symbol within one analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolId {
    /// Scope that owns the symbol.
    pub scope_id: usize,
    /// Case-folded VB6 name.
    pub name: String,
}

/// A typed node's common source and semantic metadata.
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// Source span in the module CST.
    pub span: SourceSpan,
    /// Resolved VB6 type, if this node has a value.
    pub ty: Option<TypeInfo>,
    /// Bound declaration, when applicable.
    pub symbol: Option<SymbolId>,
    /// File and line location for diagnostics and tooling.
    pub location: SourceLocation,
}

/// A typed VB6 module or class unit.
#[derive(Debug, Clone)]
pub struct TypedProgram {
    /// Module or class name.
    pub name: String,
    /// Source unit category.
    pub unit_kind: UnitKind,
    /// Module source extent.
    pub info: NodeInfo,
    /// Module-scope declarations.
    pub declarations: Vec<Declaration>,
    /// Procedures and properties in source order.
    pub procedures: Vec<Procedure>,
    /// True only when semantic validation completed without blocking errors.
    pub compilable: bool,
}

/// Top-level VB6 source unit represented by the HIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitKind {
    /// Standard `.bas` module.
    Module,
    /// `.cls` class module.
    Class,
}

/// Declaration kinds retained in HIR.
#[derive(Debug, Clone)]
pub enum DeclarationKind {
    /// Variable or constant.
    Variable {
        /// True for a `Const` declaration.
        constant: bool,
    },
    /// A declaration not yet represented by a dedicated HIR form.
    Other {
        /// CST syntax kind name.
        syntax: String,
    },
}

/// A declaration and its resolved binding.
#[derive(Debug, Clone)]
pub struct Declaration {
    /// Name as written in source.
    pub name: String,
    /// Declaration category.
    pub kind: DeclarationKind,
    /// Type and source metadata.
    pub info: NodeInfo,
}

/// Parameter passing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassingMode {
    /// Argument is passed by value.
    ByVal,
    /// Argument is passed by reference (VB6 default).
    ByRef,
    /// Variable number of arguments, reserved for explicit ParamArray declarations.
    ParamArray,
}

/// A formal procedure parameter.
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name.
    pub name: String,
    /// Passing mode.
    pub passing: PassingMode,
    /// Whether the argument may be omitted.
    pub optional: bool,
    /// Resolved declared type.
    pub ty: TypeInfo,
    /// Parameter source and symbol metadata.
    pub info: NodeInfo,
}

/// VB6 procedure signature.
#[derive(Debug, Clone)]
pub struct ProcedureSignature {
    /// Procedure result type (Sub is represented by `None`).
    pub return_type: Option<TypeInfo>,
    /// Formal parameters in declaration order.
    pub parameters: Vec<Parameter>,
}

/// A procedure body with its resolved signature.
#[derive(Debug, Clone)]
pub struct Procedure {
    /// Procedure name.
    pub name: String,
    /// Signature.
    pub signature: ProcedureSignature,
    /// Typed body statements.
    pub body: Vec<Statement>,
    /// Native-compatible linear control-flow view of the body.
    pub native_flow: Vec<NativeFlowNode>,
    /// Procedure source and symbol metadata.
    pub info: NodeInfo,
}

/// Namespace for a recovered Microsoft AST kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeAstNamespace {
    /// Expression-kind tag read from an expression node.
    Expression,
    /// Statement-kind tag read from a statement node.
    Statement,
}

/// Confidence in a recovered source-HIR to Microsoft AST association.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeAstEvidence {
    /// Directly observed in the recovered native dispatcher or construction path.
    Observed,
    /// Inferred from the recovered behavior and opcode matrix.
    Inferred,
    /// No defensible native kind is currently known.
    Unknown,
    /// The source construct is represented by another node and has no native node of its own.
    Passthrough,
}

/// Microsoft AST identity retained for a source-HIR node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeAstMapping {
    /// Expression or statement namespace.
    pub namespace: NativeAstNamespace,
    /// Numeric native tag, absent while the mapping remains unresolved.
    pub kind: Option<u16>,
    /// Provenance of the association.
    pub evidence: NativeAstEvidence,
}

/// Operation in the flattened native-compatible control-flow view.
#[derive(Debug, Clone)]
pub enum NativeFlowKind {
    /// A statement whose internal expression tree remains in the typed HIR.
    SourceStatement(Box<Statement>),
    /// A native label definition.
    Label(u32),
    /// Conditional branch to a native label.
    Branch {
        /// Condition evaluated by the branch.
        condition: Expression,
        /// Destination label.
        target: u32,
        /// Branch polarity.
        when_true: bool,
    },
    /// Unconditional branch to a native label.
    Jump {
        /// Destination label.
        target: u32,
    },
    /// Native label fixup retained for a source-level symbolic jump.
    FixupJump {
        /// Label spelling as written in source.
        target: String,
    },
    /// Native loop form retained alongside its flattened branch flow.
    LoopForm(Box<Statement>),
    /// Control construct not yet safely lowered to native branch operations.
    UnloweredControl(Box<Statement>),
}

/// One operation in a procedure's native-compatible linear control flow.
#[derive(Debug, Clone)]
pub struct NativeFlowNode {
    /// Flattened operation.
    pub kind: NativeFlowKind,
    /// Native statement identity and evidence.
    pub native_ast: NativeAstMapping,
    /// Original source and semantic metadata.
    pub info: NodeInfo,
}

/// Binary or unary operators retained from source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    /// Exact source spelling, preserving VB6 operator distinctions.
    Source(String),
}

/// Typed expression forms.
#[derive(Debug, Clone)]
pub enum ExpressionKind {
    /// Literal token text.
    Literal(String),
    /// Bound name.
    Name(String),
    /// Unary operator.
    Unary {
        /// Operator applied to the operand.
        operator: Operator,
        /// Operand.
        operand: Box<Expression>,
    },
    /// Binary operator.
    Binary {
        /// Operator between operands.
        operator: Operator,
        /// Left operand.
        left: Box<Expression>,
        /// Right operand.
        right: Box<Expression>,
    },
    /// MX Boolean short-circuit expression; a future HIR executor must evaluate
    /// the right operand only when required by the operator.
    ShortCircuit {
        /// `AndAlso` or `OrElse`.
        operator: Operator,
        /// Left operand.
        left: Box<Expression>,
        /// Lazily evaluated right operand.
        right: Box<Expression>,
    },
    /// MX `??`; only a `Nothing` left operand evaluates the right side.
    Coalesce {
        /// Left operand.
        left: Box<Expression>,
        /// Lazily evaluated fallback.
        right: Box<Expression>,
    },
    /// Invocation.
    Call {
        /// Invoked procedure or expression.
        callee: Box<Expression>,
        /// Arguments in source order.
        arguments: Vec<Expression>,
    },
    /// Member access.
    Member {
        /// Receiver expression.
        receiver: Box<Expression>,
        /// Member identifier.
        name: String,
    },
    /// Parenthesized value.
    Parenthesized(Box<Expression>),
    /// A conversion validated by the VB6 type checker.
    Conversion {
        /// Value being converted.
        expression: Box<Expression>,
        /// Destination type.
        target_type: TypeInfo,
        /// Conversion category.
        conversion: ConversionKind,
    },
    /// Reserved for future closure lowering; not generated for VB6.
    Lambda {
        /// Captured-lambda parameter list.
        parameters: Vec<Parameter>,
        /// Lambda body.
        body: Vec<Statement>,
    },
    /// Reserved for future spread lowering; not generated for VB6.
    Spread(Box<Expression>),
}

/// Conversion kinds recognized by the typed HIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionKind {
    /// Implicit conversion accepted by the existing VB6 type rules.
    Implicit,
}

/// Loop kind targeted by MX `Continue`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinueTarget {
    /// Counted `For ... Next` loop.
    For,
    /// `Do ... Loop`.
    Do,
}

/// Structured loop construct retained in typed HIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopKind {
    /// `Do ... Loop`.
    Do,
    /// `While ... Wend`.
    While,
}

/// Typed expression.
#[derive(Debug, Clone)]
pub struct Expression {
    /// Expression form.
    pub kind: ExpressionKind,
    /// Recovered Microsoft AST kind associated with this expression.
    pub native_ast: NativeAstMapping,
    /// Type, binding and source metadata.
    pub info: NodeInfo,
}

/// Control-flow and executable statement forms.
#[derive(Debug, Clone)]
pub enum StatementKind {
    /// Assignment.
    Assignment {
        /// Assignment destination.
        target: Expression,
        /// Assigned value.
        value: Expression,
    },
    /// Expression used as a statement.
    Expression(Expression),
    /// MX expression return from a Function.
    Return(Expression),
    /// Native VB6 control statement retained with its source operands.
    NativeControl(NativeControl),
    /// MX continue to the nearest enclosing loop of the requested kind.
    Continue(ContinueTarget),
    /// Structured conditional.
    If {
        /// Branch condition.
        condition: Expression,
        /// Then branch.
        then_body: Vec<Statement>,
        /// Else branch, if present.
        else_body: Vec<Statement>,
    },
    /// Structured loop.
    Loop {
        /// Source loop construct.
        loop_kind: LoopKind,
        /// Loop condition, absent for unconditional loops.
        condition: Option<Expression>,
        /// True when the source tests at the end of each iteration.
        post_test: bool,
        /// True when the condition is negated by `Until`.
        until: bool,
        /// Loop body.
        body: Vec<Statement>,
    },
    /// Counted `For ... Next` loop.
    For {
        /// Counter variable.
        variable: Expression,
        /// Initial value.
        start: Expression,
        /// Inclusive end value.
        end: Expression,
        /// Optional explicit `Step` value.
        step: Option<Expression>,
        /// Loop body.
        body: Vec<Statement>,
    },
    /// Select Case.
    Select {
        /// Value tested by the Select.
        value: Expression,
        /// Case arms.
        cases: Vec<CaseClause>,
    },
    /// Local declaration.
    Declaration(Declaration),
    /// Source construct not yet semantically lowered.
    Unsupported {
        /// CST syntax kind.
        syntax: String,
    },
    /// Reserved for future Try/Catch/Finally lowering.
    Try {
        /// Protected body.
        body: Vec<Statement>,
        /// Catch bodies.
        catches: Vec<Vec<Statement>>,
        /// Finally body.
        finally_body: Vec<Statement>,
    },
    /// Reserved for future short-circuit semantics.
    ShortCircuit {
        /// `AndAlso` or `OrElse`.
        operator: Operator,
        /// Left operand.
        left: Expression,
        /// Lazily evaluated right operand.
        right: Expression,
    },
    /// Reserved for future VBTask.Then/Catch call chains.
    TaskContinuation {
        /// Task-producing expression.
        source: Expression,
        /// Then callback body.
        then_body: Option<Vec<Statement>>,
        /// Catch callback body.
        catch_body: Option<Vec<Statement>>,
    },
}

/// Native VB6 statement forms represented in source HIR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeControl {
    /// Label declaration.
    Label(String),
    /// Unconditional GoTo.
    Goto(String),
    /// GoSub call.
    Gosub(String),
    /// Return from the most recent GoSub.
    GosubReturn,
    /// Exit Sub.
    ExitSub,
    /// Exit Function or Property.
    ExitFunction,
    /// Stop execution.
    Stop,
    /// End the program.
    End,
    /// On ... GoSub with a single target.
    OnGosubA(String),
    /// On ... GoSub with multiple targets.
    OnGosubB(String),
    /// On Error handler selection.
    OnError(String),
    /// Resume from an error handler.
    Resume(String),
}

/// A statement with source and semantic metadata.
#[derive(Debug, Clone)]
pub struct Statement {
    /// Statement form.
    pub kind: StatementKind,
    /// Source and type metadata.
    pub info: NodeInfo,
}

/// A Select Case arm.
#[derive(Debug, Clone)]
pub struct CaseClause {
    /// Match expressions; empty denotes Case Else.
    pub matches: Vec<Expression>,
    /// Case body.
    pub body: Vec<Statement>,
    /// Clause source span and location.
    pub info: NodeInfo,
}

/// Result of semantic analysis and HIR construction.
#[derive(Debug, Clone)]
pub struct HirAnalysis {
    /// HIR is absent whenever a blocking diagnostic prevents safe compilation.
    pub program: Option<TypedProgram>,
    /// Errors produced while binding or constructing HIR.
    pub errors: Vec<SemanticError>,
    /// Analyzer warnings.
    pub warnings: Vec<String>,
}

pub(crate) fn check_option_strict(analysis: &mut HirAnalysis) {
    let Some(program) = analysis.program.as_mut() else {
        return;
    };
    for procedure in &program.procedures {
        check_strict_statements(&procedure.body, &mut analysis.errors);
    }
    if !analysis.errors.is_empty() {
        analysis.program = None;
    }
}

fn check_strict_statements(statements: &[Statement], errors: &mut Vec<SemanticError>) {
    for statement in statements {
        match &statement.kind {
            StatementKind::Assignment { target, value } => {
                let Some(target_type) = target.info.ty.as_ref() else {
                    errors.push(SemanticError::UnsupportedFeature {
                        feature: "MX S08 Option Strict".to_string(),
                        reason: "assignment target has no statically known type".to_string(),
                        location: target.info.location.clone(),
                    });
                    continue;
                };
                let source = unconverted_expression(value);
                if matches!(
                    source.kind,
                    ExpressionKind::Call { .. } | ExpressionKind::Member { .. }
                ) {
                    errors.push(SemanticError::UnsupportedFeature {
                        feature: "MX S08 Option Strict".to_string(),
                        reason: "procedure calls and late-bound member expressions are not checked by this slice".to_string(),
                        location: source.info.location.clone(),
                    });
                    continue;
                }
                let Some(source_type) = source.info.ty.as_ref() else {
                    errors.push(SemanticError::UnsupportedFeature {
                        feature: "MX S08 Option Strict".to_string(),
                        reason: "assignment source has no statically known type".to_string(),
                        location: source.info.location.clone(),
                    });
                    continue;
                };
                if let Err(error) = crate::TypeChecker::new().check_strict_assignment(
                    target_type,
                    source_type,
                    &target.info.location,
                ) {
                    errors.push(error);
                }
            }
            StatementKind::If {
                then_body,
                else_body,
                ..
            } => {
                check_strict_statements(then_body, errors);
                check_strict_statements(else_body, errors);
            }
            StatementKind::Loop { body, .. } | StatementKind::For { body, .. } => {
                check_strict_statements(body, errors)
            }
            StatementKind::Select { cases, .. } => {
                for case in cases {
                    check_strict_statements(&case.body, errors);
                }
            }
            StatementKind::Try {
                body,
                catches,
                finally_body,
            } => {
                check_strict_statements(body, errors);
                for catch in catches {
                    check_strict_statements(catch, errors);
                }
                check_strict_statements(finally_body, errors);
            }
            _ => {}
        }
    }
}

fn unconverted_expression(expression: &Expression) -> &Expression {
    match &expression.kind {
        ExpressionKind::Conversion { expression, .. } => unconverted_expression(expression),
        _ => expression,
    }
}

/// Construct the structural HIR for a module after its declarations are bound.
pub(crate) fn lower_module(
    module: &ModuleFile,
    scopes: &ScopeManager,
    procedure_scopes: &std::collections::HashMap<u32, usize>,
    errors: &[SemanticError],
    warnings: &[String],
) -> HirAnalysis {
    lower_unit(
        module,
        scopes,
        procedure_scopes,
        errors,
        warnings,
        UnitKind::Module,
    )
}

pub(crate) fn lower_class(
    class: &vb6parse::files::ClassFile,
    scopes: &ScopeManager,
    procedure_scopes: &std::collections::HashMap<u32, usize>,
    errors: &[SemanticError],
    warnings: &[String],
) -> HirAnalysis {
    let module = ModuleFile {
        name: class.header.attributes.name.clone(),
        cst: class.cst.clone(),
        line_offset: class.line_offset,
    };
    lower_unit(
        &module,
        scopes,
        procedure_scopes,
        errors,
        warnings,
        UnitKind::Class,
    )
}

fn lower_unit(
    module: &ModuleFile,
    scopes: &ScopeManager,
    procedure_scopes: &std::collections::HashMap<u32, usize>,
    errors: &[SemanticError],
    warnings: &[String],
    unit_kind: UnitKind,
) -> HirAnalysis {
    let root = module.cst.to_root_node();
    let index = crate::location::LineIndex::from_cst_root(&root);
    let scope_kind = match unit_kind {
        UnitKind::Module => ScopeKind::Global,
        UnitKind::Class => ScopeKind::Class,
    };
    let module_scope = scopes
        .get_scopes_by_kind(scope_kind)
        .into_iter()
        .find(|scope| scope.name.eq_ignore_ascii_case(&module.name));
    let make_info = |node: &CstNode, scope_id: Option<usize>, ty: Option<TypeInfo>| {
        let (line, column) = index.position(node.start_offset());
        let bound = scope_id.and_then(|id| scopes.lookup_in_scope(id, &first_name(node)));
        NodeInfo {
            span: SourceSpan {
                start: node.start_offset(),
                end: node.end_offset(),
            },
            ty: ty.or_else(|| bound.map(|symbol| symbol.type_info.clone())),
            symbol: bound.map(|symbol| SymbolId {
                scope_id: symbol.scope_id,
                name: symbol.name.to_ascii_lowercase(),
            }),
            location: SourceLocation {
                file: module.name.clone(),
                line: line + module.line_offset,
                column,
            },
        }
    };
    let mut declarations = Vec::new();
    let mut procedures = Vec::new();
    let mut diagnostics = errors.to_vec();
    for node in root.children().into_iter().filter(|node| !node.is_token()) {
        match node.kind() {
            SyntaxKind::SubStatement
            | SyntaxKind::FunctionStatement
            | SyntaxKind::PropertyStatement => {
                let name = first_name(&node);
                let scope_id = procedure_scopes.get(&node.start_offset()).copied();
                let mut parameter_symbols: Vec<_> = scope_id
                    .and_then(|id| scopes.get_scope(id))
                    .map(|scope| {
                        scope
                            .symbols
                            .values()
                            .filter(|symbol| symbol.kind == crate::SymbolKind::Parameter)
                            .collect()
                    })
                    .unwrap_or_default();
                parameter_symbols
                    .sort_by_key(|symbol| (symbol.location.line, symbol.location.column));
                let parameters = node
                    .first_child_by_kind(SyntaxKind::ParameterList)
                    .map(|list| {
                        parameter_symbols
                            .into_iter()
                            .map(|symbol| {
                                let param_span = list
                                    .descendants()
                                    .find(|token| {
                                        token.is_token()
                                            && token.kind() == SyntaxKind::Identifier
                                            && token.text().eq_ignore_ascii_case(&symbol.name)
                                    })
                                    .map(|token| (token.start_offset(), token.end_offset()))
                                    .unwrap_or((list.start_offset(), list.start_offset()));
                                let info = NodeInfo {
                                    span: SourceSpan {
                                        start: param_span.0,
                                        end: param_span.1,
                                    },
                                    ty: Some(symbol.type_info.clone()),
                                    symbol: Some(SymbolId {
                                        scope_id: symbol.scope_id,
                                        name: symbol.name.to_ascii_lowercase(),
                                    }),
                                    location: symbol.location.clone(),
                                };
                                let passing = if symbol.attributes.contains_key("paramarray") {
                                    PassingMode::ParamArray
                                } else if symbol.type_info.is_reference {
                                    PassingMode::ByRef
                                } else {
                                    PassingMode::ByVal
                                };
                                Parameter {
                                    name: symbol.name.clone(),
                                    passing,
                                    optional: symbol.attributes.contains_key("optional"),
                                    ty: symbol.type_info.clone(),
                                    info,
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let is_function = node.kind() == SyntaxKind::FunctionStatement
                    || node
                        .descendants()
                        .any(|n| n.kind() == SyntaxKind::GetKeyword);
                let mut body: Vec<Statement> = node
                    .first_child_by_kind(SyntaxKind::StatementList)
                    .map(|list| {
                        list.children()
                            .into_iter()
                            .filter(|child| !child.is_token())
                            .filter_map(|child| {
                                match lower_statement(&child, scope_id, scopes, &make_info) {
                                    Ok(statement) => Some(statement),
                                    Err(error) => {
                                        diagnostics.push(error);
                                        None
                                    }
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                let return_type = if is_function {
                    let symbol = module_scope.and_then(|s| scopes.lookup_in_scope(s.id, &name));
                    symbol.and_then(|s| match &s.type_info.kind {
                        vb6core::types::VBType::Function { return_type } => {
                            Some((**return_type).clone())
                        }
                        _ => Some(TypeInfo::variant()),
                    })
                } else {
                    None
                };
                check_return_statements(&mut body, return_type.as_ref(), &mut diagnostics);
                check_continue_statements(&body, &mut Vec::new(), &mut diagnostics);
                let procedure_info = make_info(&node, module_scope.map(|s| s.id), None);
                let native_flow = lower_native_flow(&body);
                procedures.push(Procedure {
                    name,
                    signature: ProcedureSignature {
                        return_type,
                        parameters,
                    },
                    body,
                    native_flow,
                    info: procedure_info,
                });
            }
            SyntaxKind::DimStatement => {
                let name = first_name(&node);
                let info = make_info(&node, module_scope.map(|s| s.id), None);
                declarations.push(Declaration {
                    name,
                    kind: DeclarationKind::Variable {
                        constant: node
                            .descendants()
                            .any(|n| n.kind() == SyntaxKind::ConstKeyword),
                    },
                    info,
                });
            }
            _ => {}
        }
    }
    let compilable = diagnostics.is_empty();
    HirAnalysis {
        program: compilable.then_some(TypedProgram {
            name: module.name.clone(),
            unit_kind,
            info: NodeInfo {
                span: SourceSpan {
                    start: root.start_offset(),
                    end: root.end_offset(),
                },
                ty: None,
                symbol: None,
                location: SourceLocation {
                    file: module.name.clone(),
                    line: 1 + module.line_offset,
                    column: 1,
                },
            },
            declarations,
            procedures,
            compilable,
        }),
        errors: diagnostics,
        warnings: warnings.to_vec(),
    }
}

fn first_name(node: &CstNode) -> String {
    node.descendants()
        .find(|child| child.is_token() && child.kind() == SyntaxKind::Identifier)
        .map(|child| child.text().to_string())
        .unwrap_or_default()
}

fn statement_text_after_keyword(node: &CstNode, keyword: &str) -> String {
    let text = node.text().trim();
    let text_lower = text.to_ascii_lowercase();
    let keyword_lower = keyword.to_ascii_lowercase();
    text_lower
        .find(&keyword_lower)
        .and_then(|start| text.get(start + keyword.len()..))
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn lower_statement(
    node: &CstNode,
    scope_id: Option<usize>,
    scopes: &ScopeManager,
    make_info: &impl Fn(&CstNode, Option<usize>, Option<TypeInfo>) -> NodeInfo,
) -> Result<Statement, SemanticError> {
    let kids: Vec<_> = node
        .children()
        .into_iter()
        .filter(|child| !child.is_token())
        .collect();
    match node.kind() {
        SyntaxKind::IfStatement => {
            let condition_node = node
                .children()
                .iter()
                .find(|child| !child.is_token() && is_expression_kind(child.kind()))
                .ok_or_else(|| unsupported(node, make_info))?;
            let condition = lower_expression(condition_node, scope_id, scopes, make_info)?;
            if condition.info.ty.as_ref().is_some_and(|ty| {
                !matches!(
                    ty.kind,
                    vb6core::types::VBType::Boolean | vb6core::types::VBType::Variant
                )
            }) {
                return Err(SemanticError::InvalidOperation {
                    message: "If condition must have Boolean or Variant type; implicit condition conversion is not implemented".to_string(),
                    location: condition.info.location.clone(),
                });
            }
            let mut then_body = Vec::new();
            let mut else_body = Vec::new();
            for child in node.children().iter().filter(|child| !child.is_token()) {
                if is_expression_kind(child.kind()) {
                    continue;
                }
                match child.kind() {
                    SyntaxKind::ElseClause => {
                        else_body = lower_statement_children(child, scope_id, scopes, make_info)?;
                    }
                    SyntaxKind::ElseIfClause => {
                        let nested_condition = child
                            .children()
                            .iter()
                            .find(|n| !n.is_token() && is_expression_kind(n.kind()))
                            .ok_or_else(|| unsupported(child, make_info))?;
                        let nested_condition =
                            lower_expression(nested_condition, scope_id, scopes, make_info)?;
                        let nested_body =
                            lower_statement_children(child, scope_id, scopes, make_info)?;
                        let nested_info = make_info(child, scope_id, None);
                        else_body.push(Statement {
                            kind: StatementKind::If {
                                condition: nested_condition,
                                then_body: nested_body,
                                else_body: Vec::new(),
                            },
                            info: nested_info,
                        });
                    }
                    SyntaxKind::StatementList => {
                        then_body.extend(lower_statement_children(
                            child, scope_id, scopes, make_info,
                        )?);
                    }
                    _ => then_body.push(lower_statement(child, scope_id, scopes, make_info)?),
                }
            }
            let info = make_info(node, scope_id, None);
            Ok(Statement {
                kind: StatementKind::If {
                    condition,
                    then_body,
                    else_body,
                },
                info,
            })
        }
        SyntaxKind::ForStatement => {
            let expressions: Vec<_> = node
                .children()
                .iter()
                .filter(|child| !child.is_token() && is_expression_kind(child.kind()))
                .collect();
            if expressions.len() < 3 {
                return Err(unsupported(node, make_info));
            }
            let variable = lower_expression(expressions[0], scope_id, scopes, make_info)?;
            let Some(target_type) = variable.info.ty.clone() else {
                return Err(unsupported(node, make_info));
            };
            if !is_numeric_type(&target_type) {
                return Err(SemanticError::InvalidOperation {
                    message: "For counter must have a numeric type supported by typed HIR"
                        .to_string(),
                    location: variable.info.location.clone(),
                });
            }
            let start = checked_loop_bound(
                lower_expression(expressions[1], scope_id, scopes, make_info)?,
                &target_type,
            )?;
            let end = checked_loop_bound(
                lower_expression(expressions[2], scope_id, scopes, make_info)?,
                &target_type,
            )?;
            let step = expressions
                .get(3)
                .map(|expression| {
                    lower_expression(expression, scope_id, scopes, make_info)
                        .and_then(|step| checked_loop_bound(step, &target_type))
                })
                .transpose()?;
            let body = node
                .children()
                .iter()
                .find(|child| child.kind() == SyntaxKind::StatementList)
                .map(|body| lower_statement_children(body, scope_id, scopes, make_info))
                .transpose()?
                .unwrap_or_default();
            let info = make_info(node, scope_id, None);
            Ok(Statement {
                kind: StatementKind::For {
                    variable,
                    start,
                    end,
                    step,
                    body,
                },
                info,
            })
        }
        SyntaxKind::WhileStatement | SyntaxKind::DoStatement => {
            let body_node = node
                .children()
                .iter()
                .find(|child| child.kind() == SyntaxKind::StatementList);
            let condition_node = node
                .children()
                .iter()
                .find(|child| !child.is_token() && is_expression_kind(child.kind()));
            let condition = condition_node
                .map(|condition| lower_expression(condition, scope_id, scopes, make_info))
                .transpose()?;
            if condition
                .as_ref()
                .and_then(|expr| expr.info.ty.as_ref())
                .is_some_and(|ty| {
                    !matches!(
                        ty.kind,
                        vb6core::types::VBType::Boolean | vb6core::types::VBType::Variant
                    )
                })
            {
                return Err(SemanticError::InvalidOperation {
                    message: "loop condition must have Boolean or Variant type".to_string(),
                    location: condition
                        .as_ref()
                        .expect("checked condition exists")
                        .info
                        .location
                        .clone(),
                });
            }
            let body = body_node
                .map(|body| lower_statement_children(body, scope_id, scopes, make_info))
                .transpose()?
                .unwrap_or_default();
            let post_test = node.kind() == SyntaxKind::DoStatement
                && condition_node.is_some_and(|condition| {
                    body_node.is_some_and(|body| condition.start_offset() > body.start_offset())
                });
            let until = condition_node.is_some_and(|condition| {
                node.children().iter().any(|token| {
                    token.is_token()
                        && token.kind() == SyntaxKind::UntilKeyword
                        && token.start_offset() < condition.start_offset()
                })
            });
            let info = make_info(node, scope_id, None);
            Ok(Statement {
                kind: StatementKind::Loop {
                    loop_kind: if node.kind() == SyntaxKind::DoStatement {
                        LoopKind::Do
                    } else {
                        LoopKind::While
                    },
                    condition,
                    post_test,
                    until,
                    body,
                },
                info,
            })
        }
        SyntaxKind::DimStatement => {
            let name = first_name(node);
            let info = make_info(node, scope_id, None);
            Ok(Statement {
                kind: StatementKind::Declaration(Declaration {
                    name,
                    kind: DeclarationKind::Variable {
                        constant: node
                            .descendants()
                            .any(|n| n.kind() == SyntaxKind::ConstKeyword),
                    },
                    info: info.clone(),
                }),
                info,
            })
        }
        SyntaxKind::AssignmentStatement | SyntaxKind::LetStatement | SyntaxKind::SetStatement => {
            if kids.len() < 2 {
                return Err(unsupported(node, make_info));
            }
            let mut target = lower_expression(&kids[0], scope_id, scopes, make_info)?;
            let mut value = lower_expression(&kids[1], scope_id, scopes, make_info)?;
            if let ExpressionKind::Name(name) = &target.kind {
                if let Some(symbol) = scope_id.and_then(|id| lookup_in_chain(scopes, id, name)) {
                    if symbol.kind == crate::SymbolKind::Constant {
                        return Err(SemanticError::InvalidAssignment {
                            message: format!("cannot assign to constant `{name}`"),
                            location: target.info.location.clone(),
                        });
                    }
                    if symbol.kind == crate::SymbolKind::Function
                        && let vb6core::types::VBType::Function { return_type } =
                            &symbol.type_info.kind
                    {
                        target.info.ty = Some((**return_type).clone());
                    }
                }
            }
            if let (Some(expected), Some(found)) = (&target.info.ty, &value.info.ty) {
                crate::TypeChecker::new().check_assignment(
                    expected,
                    found,
                    &target.info.location,
                )?;
                if expected != found {
                    value = implicit_conversion(value, expected.clone());
                }
            }
            let info = make_info(node, scope_id, None);
            Ok(Statement {
                kind: StatementKind::Assignment { target, value },
                info,
            })
        }
        SyntaxKind::CallStatement => {
            let expression = node
                .descendants()
                .find(|child| !child.is_token() && child.kind() == SyntaxKind::CallExpression)
                .ok_or_else(|| unsupported(node, make_info))?;
            let expression = lower_expression(&expression, scope_id, scopes, make_info)?;
            let info = make_info(node, scope_id, expression.info.ty.clone());
            Ok(Statement {
                kind: StatementKind::Expression(expression),
                info,
            })
        }
        SyntaxKind::ReturnStatement => {
            let expression = node
                .children()
                .iter()
                .find(|child| !child.is_token() && is_expression_kind(child.kind()));
            let info = make_info(node, scope_id, None);
            let kind = if let Some(expression) = expression {
                let value = lower_expression(expression, scope_id, scopes, make_info)?;
                StatementKind::Return(value)
            } else {
                StatementKind::NativeControl(NativeControl::GosubReturn)
            };
            Ok(Statement { kind, info })
        }
        SyntaxKind::GotoStatement => Ok(Statement {
            kind: StatementKind::NativeControl(NativeControl::Goto(statement_text_after_keyword(
                node, "GoTo",
            ))),
            info: make_info(node, scope_id, None),
        }),
        SyntaxKind::GoSubStatement => Ok(Statement {
            kind: StatementKind::NativeControl(NativeControl::Gosub(statement_text_after_keyword(
                node, "GoSub",
            ))),
            info: make_info(node, scope_id, None),
        }),
        SyntaxKind::LabelStatement => Ok(Statement {
            kind: StatementKind::NativeControl(NativeControl::Label(
                node.text().trim().trim_end_matches(':').trim().to_string(),
            )),
            info: make_info(node, scope_id, None),
        }),
        SyntaxKind::ExitStatement => {
            let is_function = node.descendants().any(|child| {
                child.is_token()
                    && matches!(
                        child.text().to_ascii_lowercase().as_str(),
                        "function" | "property"
                    )
            });
            Ok(Statement {
                kind: StatementKind::NativeControl(if is_function {
                    NativeControl::ExitFunction
                } else {
                    NativeControl::ExitSub
                }),
                info: make_info(node, scope_id, None),
            })
        }
        SyntaxKind::StopStatement => Ok(Statement {
            kind: StatementKind::NativeControl(NativeControl::Stop),
            info: make_info(node, scope_id, None),
        }),
        SyntaxKind::EndStatement => Ok(Statement {
            kind: StatementKind::NativeControl(NativeControl::End),
            info: make_info(node, scope_id, None),
        }),
        SyntaxKind::OnGoSubStatement => {
            let raw = node.text().trim().to_string();
            let labels = statement_text_after_keyword(node, "GoSub");
            let has_multiple_targets = labels.contains(',');
            Ok(Statement {
                kind: StatementKind::NativeControl(if has_multiple_targets {
                    NativeControl::OnGosubB(raw)
                } else {
                    NativeControl::OnGosubA(raw)
                }),
                info: make_info(node, scope_id, None),
            })
        }
        SyntaxKind::OnErrorStatement => Ok(Statement {
            kind: StatementKind::NativeControl(NativeControl::OnError(
                node.text().trim().to_string(),
            )),
            info: make_info(node, scope_id, None),
        }),
        SyntaxKind::ResumeStatement => Ok(Statement {
            kind: StatementKind::NativeControl(NativeControl::Resume(
                statement_text_after_keyword(node, "Resume"),
            )),
            info: make_info(node, scope_id, None),
        }),
        SyntaxKind::ContinueStatement => {
            let target = node
                .children()
                .iter()
                .filter(|child| child.is_token())
                .find_map(|token| {
                    if token.text().eq_ignore_ascii_case("For") {
                        Some(ContinueTarget::For)
                    } else if token.text().eq_ignore_ascii_case("Do") {
                        Some(ContinueTarget::Do)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| unsupported(node, make_info))?;
            let info = make_info(node, scope_id, None);
            Ok(Statement {
                kind: StatementKind::Continue(target),
                info,
            })
        }
        _ => Err(unsupported(node, make_info)),
    }
}

fn check_continue_statements(
    statements: &[Statement],
    active_loops: &mut Vec<ContinueTarget>,
    errors: &mut Vec<SemanticError>,
) {
    for statement in statements {
        match &statement.kind {
            StatementKind::Continue(target) if !active_loops.contains(target) => {
                let target_name = match target {
                    ContinueTarget::For => "For",
                    ContinueTarget::Do => "Do",
                };
                errors.push(SemanticError::InvalidOperation {
                    message: format!("Continue {target_name} has no enclosing matching loop"),
                    location: statement.info.location.clone(),
                });
            }
            StatementKind::If {
                then_body,
                else_body,
                ..
            } => {
                check_continue_statements(then_body, active_loops, errors);
                check_continue_statements(else_body, active_loops, errors);
            }
            StatementKind::For { body, .. } => {
                active_loops.push(ContinueTarget::For);
                check_continue_statements(body, active_loops, errors);
                active_loops.pop();
            }
            StatementKind::Loop {
                loop_kind, body, ..
            } => {
                if *loop_kind == LoopKind::Do {
                    active_loops.push(ContinueTarget::Do);
                    check_continue_statements(body, active_loops, errors);
                    active_loops.pop();
                } else {
                    check_continue_statements(body, active_loops, errors);
                }
            }
            StatementKind::Select { cases, .. } => {
                for case in cases {
                    check_continue_statements(&case.body, active_loops, errors);
                }
            }
            StatementKind::Try {
                body,
                catches,
                finally_body,
            } => {
                check_continue_statements(body, active_loops, errors);
                for catch in catches {
                    check_continue_statements(catch, active_loops, errors);
                }
                check_continue_statements(finally_body, active_loops, errors);
            }
            _ => {}
        }
    }
}

fn check_return_statements(
    statements: &mut [Statement],
    return_type: Option<&TypeInfo>,
    errors: &mut Vec<SemanticError>,
) {
    for statement in statements {
        match &mut statement.kind {
            StatementKind::Return(value) => {
                let Some(expected) = return_type else {
                    errors.push(SemanticError::InvalidOperation {
                        message: "Return <expr> is valid only in a Function".to_string(),
                        location: statement.info.location.clone(),
                    });
                    continue;
                };
                let Some(found) = value.info.ty.as_ref() else {
                    errors.push(SemanticError::UnsupportedFeature {
                        feature: "MX S01 Return".to_string(),
                        reason: "return expression has no statically known type".to_string(),
                        location: value.info.location.clone(),
                    });
                    continue;
                };
                if found.kind == vb6core::types::VBType::Variant
                    && expected.kind != vb6core::types::VBType::Variant
                {
                    *value = implicit_conversion(value.clone(), expected.clone());
                    continue;
                }
                match crate::TypeChecker::new().check_assignment(
                    expected,
                    found,
                    &value.info.location,
                ) {
                    Ok(()) if expected != found => {
                        *value = implicit_conversion(value.clone(), expected.clone());
                    }
                    Ok(()) => {}
                    Err(error) => errors.push(error),
                }
            }
            StatementKind::If {
                then_body,
                else_body,
                ..
            } => {
                check_return_statements(then_body, return_type, errors);
                check_return_statements(else_body, return_type, errors);
            }
            StatementKind::Loop { body, .. } | StatementKind::For { body, .. } => {
                check_return_statements(body, return_type, errors)
            }
            StatementKind::Select { cases, .. } => {
                for case in cases {
                    check_return_statements(&mut case.body, return_type, errors);
                }
            }
            StatementKind::Try {
                body,
                catches,
                finally_body,
            } => {
                check_return_statements(body, return_type, errors);
                for catch in catches {
                    check_return_statements(catch, return_type, errors);
                }
                check_return_statements(finally_body, return_type, errors);
            }
            _ => {}
        }
    }
}

fn lower_statement_children(
    parent: &CstNode,
    scope_id: Option<usize>,
    scopes: &ScopeManager,
    make_info: &impl Fn(&CstNode, Option<usize>, Option<TypeInfo>) -> NodeInfo,
) -> Result<Vec<Statement>, SemanticError> {
    let mut output = Vec::new();
    for child in parent.children().iter().filter(|child| !child.is_token()) {
        if is_expression_kind(child.kind()) {
            continue;
        }
        if child.kind() == SyntaxKind::StatementList {
            output.extend(lower_statement_children(
                child, scope_id, scopes, make_info,
            )?);
        } else {
            output.push(lower_statement(child, scope_id, scopes, make_info)?);
        }
    }
    Ok(output)
}

fn lower_expression(
    node: &CstNode,
    scope_id: Option<usize>,
    scopes: &ScopeManager,
    make_info: &impl Fn(&CstNode, Option<usize>, Option<TypeInfo>) -> NodeInfo,
) -> Result<Expression, SemanticError> {
    let children: Vec<_> = node
        .children()
        .into_iter()
        .filter(|child| !child.is_token())
        .collect();
    let (kind, ty) = match node.kind() {
        SyntaxKind::IdentifierExpression => {
            let name = first_name(node);
            let symbol = scope_id.and_then(|id| lookup_in_chain(scopes, id, &name));
            let Some(symbol) = symbol else {
                return Err(SemanticError::UndefinedSymbol {
                    name,
                    location: make_info(node, scope_id, None).location,
                });
            };
            (ExpressionKind::Name(name), Some(symbol.type_info.clone()))
        }
        SyntaxKind::MemberAccessExpression => {
            let receiver = if let Some(receiver_node) = children
                .iter()
                .find(|child| is_expression_kind(child.kind()))
            {
                lower_expression(receiver_node, scope_id, scopes, make_info)?
            } else {
                let receiver_name = node.text().split('.').next().unwrap_or_default().trim();
                let Some(symbol) =
                    scope_id.and_then(|id| lookup_in_chain(scopes, id, receiver_name))
                else {
                    return Err(unsupported(node, make_info));
                };
                let info = make_info(node, Some(symbol.scope_id), Some(symbol.type_info.clone()));
                let kind = ExpressionKind::Name(symbol.name.clone());
                Expression {
                    native_ast: native_expression_mapping(&kind, info.ty.as_ref()),
                    kind,
                    info: NodeInfo {
                        symbol: Some(SymbolId {
                            scope_id: symbol.scope_id,
                            name: symbol.name.to_ascii_lowercase(),
                        }),
                        ..info
                    },
                }
            };
            let name = node
                .text()
                .rsplit(['.', '!'])
                .next()
                .unwrap_or_default()
                .trim()
                .to_string();
            if name.is_empty() {
                return Err(unsupported(node, make_info));
            }
            let Some(vb6core::types::VBType::UserType(type_name)) =
                receiver.info.ty.as_ref().map(|ty| &ty.kind)
            else {
                return Err(unsupported(node, make_info));
            };
            let type_scope = scopes
                .get_scopes_by_kind(crate::ScopeKind::Type)
                .into_iter()
                .find(|candidate| candidate.name.eq_ignore_ascii_case(type_name));
            let Some(member) = type_scope.and_then(|scope| {
                scope
                    .symbols
                    .values()
                    .find(|symbol| symbol.name.eq_ignore_ascii_case(&name))
            }) else {
                return Err(SemanticError::UndefinedSymbol {
                    name,
                    location: make_info(node, scope_id, None).location,
                });
            };
            (
                ExpressionKind::Member {
                    receiver: Box::new(receiver),
                    name,
                },
                Some(member.type_info.clone()),
            )
        }
        SyntaxKind::NumericLiteralExpression => {
            let text = node.text().trim();
            let ty = if text.contains('.') {
                TypeInfo::new(vb6core::types::VBType::Double)
            } else {
                TypeInfo::integer()
            };
            (ExpressionKind::Literal(text.to_string()), Some(ty))
        }
        SyntaxKind::StringLiteralExpression => (
            ExpressionKind::Literal(node.text().to_string()),
            Some(TypeInfo::string()),
        ),
        SyntaxKind::BooleanLiteralExpression => (
            ExpressionKind::Literal(node.text().to_string()),
            Some(TypeInfo::new(vb6core::types::VBType::Boolean)),
        ),
        SyntaxKind::LiteralExpression => {
            let text = node.text().trim();
            let ty = if text.eq_ignore_ascii_case("Nothing") {
                TypeInfo::new(vb6core::types::VBType::Nothing)
            } else if text.eq_ignore_ascii_case("Null") {
                TypeInfo::new(vb6core::types::VBType::Null)
            } else if text.eq_ignore_ascii_case("Empty") {
                TypeInfo::new(vb6core::types::VBType::Empty)
            } else {
                TypeInfo::variant()
            };
            (ExpressionKind::Literal(text.to_string()), Some(ty))
        }
        SyntaxKind::ParenthesizedExpression if !children.is_empty() => {
            let value = lower_expression(&children[0], scope_id, scopes, make_info)?;
            let ty = value.info.ty.clone();
            (ExpressionKind::Parenthesized(Box::new(value)), ty)
        }
        SyntaxKind::UnaryExpression if !children.is_empty() => {
            let operand = lower_expression(&children[0], scope_id, scopes, make_info)?;
            let operator = node
                .children()
                .into_iter()
                .find(|child| {
                    child.is_token()
                        && !matches!(child.kind(), SyntaxKind::Whitespace | SyntaxKind::Newline)
                })
                .map(|child| child.text().to_string())
                .unwrap_or_default();
            let operand_type =
                operand
                    .info
                    .ty
                    .as_ref()
                    .ok_or_else(|| SemanticError::UnsupportedFeature {
                        feature: format!("unary operator `{operator}`"),
                        reason: "operand has no statically known type".to_string(),
                        location: operand.info.location.clone(),
                    })?;
            let result = crate::TypeChecker::new().check_unary_operation(
                operand_type,
                &operator,
                &make_info(node, scope_id, None).location,
            )?;
            (
                ExpressionKind::Unary {
                    operator: Operator::Source(operator),
                    operand: Box::new(operand),
                },
                Some(result),
            )
        }
        SyntaxKind::BinaryExpression if children.len() >= 2 => {
            let mut left = lower_expression(&children[0], scope_id, scopes, make_info)?;
            let mut right = lower_expression(&children[1], scope_id, scopes, make_info)?;
            let operator = node
                .children()
                .into_iter()
                .find(|child| {
                    child.is_token()
                        && !matches!(child.kind(), SyntaxKind::Whitespace | SyntaxKind::Newline)
                })
                .map(|child| child.text().to_string())
                .unwrap_or_default();
            let is_coalesce = operator == "??";
            let is_short_circuit =
                matches!(operator.to_ascii_lowercase().as_str(), "andalso" | "orelse");
            let (kind, result) = if is_coalesce {
                let left_type =
                    left.info
                        .ty
                        .clone()
                        .ok_or_else(|| SemanticError::UnsupportedFeature {
                            feature: "MX S10 ??".to_string(),
                            reason: "left operand has no statically known type".to_string(),
                            location: left.info.location.clone(),
                        })?;
                let right_type =
                    right
                        .info
                        .ty
                        .clone()
                        .ok_or_else(|| SemanticError::UnsupportedFeature {
                            feature: "MX S10 ??".to_string(),
                            reason: "fallback has no statically known type".to_string(),
                            location: right.info.location.clone(),
                        })?;
                let (result, convert_left, convert_right) =
                    coalesce_result_type(&left_type, &right_type, &left.info.location)?;
                if convert_left && left_type != result {
                    left = implicit_conversion(left, result.clone());
                }
                if convert_right && right_type != result {
                    right = implicit_conversion(right, result.clone());
                }
                (
                    ExpressionKind::Coalesce {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    result,
                )
            } else if is_short_circuit {
                left = coerce_short_circuit_operand(left)?;
                right = coerce_short_circuit_operand(right)?;
                (
                    ExpressionKind::ShortCircuit {
                        operator: Operator::Source(operator),
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    TypeInfo::boolean(),
                )
            } else {
                let result = match (&left.info.ty, &right.info.ty) {
                    (Some(l), Some(r)) => crate::TypeChecker::new().check_operation(
                        l,
                        r,
                        &operator,
                        &make_info(node, scope_id, None).location,
                    )?,
                    _ => TypeInfo::variant(),
                };
                if matches!(
                    operator.to_ascii_lowercase().as_str(),
                    "+" | "-" | "*" | "/" | "\\" | "^" | "mod"
                ) {
                    if left.info.ty.as_ref().is_some_and(|ty| ty != &result) {
                        left = implicit_conversion(left, result.clone());
                    }
                    if right.info.ty.as_ref().is_some_and(|ty| ty != &result) {
                        right = implicit_conversion(right, result.clone());
                    }
                } else if operator == "&" {
                    let target = TypeInfo::string();
                    if left.info.ty.as_ref() != Some(&target) {
                        left = implicit_conversion(left, target.clone());
                    }
                    if right.info.ty.as_ref() != Some(&target) {
                        right = implicit_conversion(right, target);
                    }
                }
                (
                    ExpressionKind::Binary {
                        operator: Operator::Source(operator),
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    result,
                )
            };
            (kind, Some(result))
        }
        SyntaxKind::CallExpression if !children.is_empty() => {
            let called_name = node
                .text()
                .split('(')
                .next()
                .unwrap_or_default()
                .trim()
                .rsplit('.')
                .next()
                .unwrap_or_default()
                .to_string();
            if called_name.is_empty() {
                return Err(unsupported(node, make_info));
            }
            let Some(symbol) = scope_id.and_then(|id| lookup_in_chain(scopes, id, &called_name))
            else {
                return Err(SemanticError::UndefinedSymbol {
                    name: called_name,
                    location: make_info(node, scope_id, None).location,
                });
            };
            let callee_info = NodeInfo {
                span: SourceSpan {
                    start: node.start_offset(),
                    end: node.start_offset() + called_name.len() as u32,
                },
                ty: Some(symbol.type_info.clone()),
                symbol: Some(SymbolId {
                    scope_id: symbol.scope_id,
                    name: symbol.name.to_ascii_lowercase(),
                }),
                location: symbol.location.clone(),
            };
            let callee_kind = ExpressionKind::Name(symbol.name.clone());
            let callee = Expression {
                native_ast: native_expression_mapping(&callee_kind, callee_info.ty.as_ref()),
                kind: callee_kind,
                info: callee_info,
            };
            let mut arguments = Vec::new();
            if let Some(argument_list) = children
                .iter()
                .find(|child| child.kind() == SyntaxKind::ArgumentList)
            {
                for item in argument_list
                    .children()
                    .iter()
                    .filter(|item| !item.is_token())
                {
                    let expression = item
                        .descendants()
                        .find(|child| is_expression_kind(child.kind()))
                        .unwrap_or(item);
                    arguments.push(lower_expression(expression, scope_id, scopes, make_info)?);
                }
            }
            let is_array = symbol.type_info.is_array
                || matches!(symbol.type_info.kind, vb6core::types::VBType::Array(_));
            let return_type = match &symbol.type_info.kind {
                vb6core::types::VBType::Array(element) if !arguments.is_empty() => {
                    Some(TypeInfo::new(element.as_ref().clone()))
                }
                _ if is_array && !arguments.is_empty() => {
                    let mut element_type = symbol.type_info.clone();
                    element_type.is_array = false;
                    Some(element_type)
                }
                vb6core::types::VBType::Function { return_type } => Some((**return_type).clone()),
                vb6core::types::VBType::Sub => None,
                _ => {
                    return Err(SemanticError::InvalidOperation {
                        message: format!("`{}` is not callable", symbol.name),
                        location: callee.info.location.clone(),
                    });
                }
            };
            let formal_scope = scopes.get_scope(symbol.scope_id).and_then(|parent| {
                parent.children.iter().find_map(|child_id| {
                    scopes
                        .get_scope(*child_id)
                        .filter(|child| child.name.eq_ignore_ascii_case(&symbol.name))
                })
            });
            if !is_array && let Some(formals) = formal_scope {
                let mut params: Vec<_> = formals
                    .symbols
                    .values()
                    .filter(|formal| formal.kind == crate::SymbolKind::Parameter)
                    .collect();
                params.sort_by_key(|formal| (formal.location.line, formal.location.column));
                let required = params
                    .iter()
                    .take_while(|formal| {
                        !formal.attributes.contains_key("optional")
                            && !formal.attributes.contains_key("paramarray")
                    })
                    .count();
                let has_param_array = params
                    .iter()
                    .any(|formal| formal.attributes.contains_key("paramarray"));
                if arguments.len() < required
                    || (!has_param_array && arguments.len() > params.len())
                {
                    return Err(SemanticError::ParameterMismatch {
                        message: format!(
                            "`{}` expects {}..{} arguments, received {}",
                            symbol.name,
                            required,
                            if has_param_array {
                                "many".to_string()
                            } else {
                                params.len().to_string()
                            },
                            arguments.len()
                        ),
                        location: callee.info.location.clone(),
                    });
                }
                for (argument, formal) in arguments.iter_mut().zip(params.iter()) {
                    if let Some(found) = &argument.info.ty {
                        crate::TypeChecker::new().check_assignment(
                            &formal.type_info,
                            found,
                            &argument.info.location,
                        )?;
                        if found != &formal.type_info {
                            *argument =
                                implicit_conversion(argument.clone(), formal.type_info.clone());
                        }
                    }
                }
            }
            (
                ExpressionKind::Call {
                    callee: Box::new(callee),
                    arguments,
                },
                return_type,
            )
        }
        _ => return Err(unsupported(node, make_info)),
    };
    let info = make_info(node, scope_id, ty);
    let native_ast = native_expression_mapping(&kind, info.ty.as_ref());
    Ok(Expression {
        kind,
        native_ast,
        info,
    })
}

fn unknown_native_ast(namespace: NativeAstNamespace) -> NativeAstMapping {
    NativeAstMapping {
        namespace,
        kind: None,
        evidence: NativeAstEvidence::Unknown,
    }
}

fn native_ast_mapping(
    namespace: NativeAstNamespace,
    kind: u16,
    evidence: NativeAstEvidence,
) -> NativeAstMapping {
    NativeAstMapping {
        namespace,
        kind: Some(kind),
        evidence,
    }
}

fn native_expression_mapping(
    expression: &ExpressionKind,
    ty: Option<&TypeInfo>,
) -> NativeAstMapping {
    use NativeAstEvidence::{Inferred, Observed, Passthrough};
    use NativeAstNamespace::Expression;

    let (kind, evidence) = match expression {
        ExpressionKind::Literal(_) => match ty.map(|ty| &ty.kind) {
            Some(vb6core::types::VBType::Boolean)
            | Some(vb6core::types::VBType::Byte)
            | Some(vb6core::types::VBType::Integer)
            | Some(vb6core::types::VBType::Long)
            | Some(vb6core::types::VBType::Nothing) => (Some(0x01), Observed),
            Some(vb6core::types::VBType::Currency) => (Some(0x02), Inferred),
            Some(vb6core::types::VBType::Single)
            | Some(vb6core::types::VBType::Double)
            | Some(vb6core::types::VBType::Date) => (Some(0x03), Observed),
            Some(vb6core::types::VBType::String) => (Some(0x04), Inferred),
            Some(vb6core::types::VBType::Variant)
            | Some(vb6core::types::VBType::Empty)
            | Some(vb6core::types::VBType::Null)
            | Some(vb6core::types::VBType::Error) => (Some(0x05), Observed),
            _ => (None, NativeAstEvidence::Unknown),
        },
        ExpressionKind::Name(_) => (Some(0x0f), Inferred),
        ExpressionKind::Unary { operator, .. } => match operator {
            Operator::Source(operator) if operator.trim() == "+" => (None, Passthrough),
            Operator::Source(operator) if operator.eq_ignore_ascii_case("not") => {
                (Some(0x06), Inferred)
            }
            Operator::Source(operator) if operator.trim() == "-" => (Some(0x07), Inferred),
            _ => (None, NativeAstEvidence::Unknown),
        },
        ExpressionKind::Binary { operator, .. } => match operator {
            Operator::Source(operator) => {
                let kind = match operator.trim().to_ascii_lowercase().as_str() {
                    "+" => Some(0x16),
                    "-" => Some(0x17),
                    "*" => Some(0x18),
                    "/" => Some(0x19),
                    "^" => Some(0x1a),
                    "mod" => Some(0x1d),
                    "\\" => Some(0x1e),
                    "imp" => Some(0x1f),
                    "eqv" => Some(0x20),
                    "or" => Some(0x21),
                    "xor" => Some(0x22),
                    "and" => Some(0x23),
                    "&" => Some(0x24),
                    "like" => Some(0x25),
                    "=" => Some(0x26),
                    "<>" => Some(0x27),
                    "<=" => Some(0x28),
                    ">=" => Some(0x29),
                    "<" => Some(0x2a),
                    ">" => Some(0x2b),
                    "is" => Some(0x36),
                    _ => None,
                };
                (
                    kind,
                    if kind.is_some() {
                        Inferred
                    } else {
                        NativeAstEvidence::Unknown
                    },
                )
            }
        },
        ExpressionKind::Call { callee, .. } => match callee.info.ty.as_ref() {
            Some(ty) if ty.is_array || matches!(ty.kind, vb6core::types::VBType::Array(_)) => {
                (Some(0x6a), Inferred)
            }
            _ => (Some(0x61), Inferred),
        },
        ExpressionKind::Member { receiver, .. } => {
            match receiver.info.ty.as_ref().map(|ty| &ty.kind) {
                Some(vb6core::types::VBType::Class(_))
                | Some(vb6core::types::VBType::UserType(_)) => (Some(0x68), Inferred),
                Some(vb6core::types::VBType::Object) | Some(vb6core::types::VBType::Variant) => {
                    (Some(0x6b), Inferred)
                }
                _ => (None, NativeAstEvidence::Unknown),
            }
        }
        ExpressionKind::Parenthesized(_) => (Some(0x65), Observed),
        ExpressionKind::Conversion { target_type, .. } => {
            let kind = match &target_type.kind {
                vb6core::types::VBType::Array(element)
                    if matches!(element.as_ref(), vb6core::types::VBType::Variant) =>
                {
                    0x10
                }
                _ => 0x0e,
            };
            (Some(kind), Inferred)
        }
        ExpressionKind::ShortCircuit { .. }
        | ExpressionKind::Coalesce { .. }
        | ExpressionKind::Lambda { .. }
        | ExpressionKind::Spread(_) => (None, NativeAstEvidence::Unknown),
    };
    match kind {
        Some(kind) => native_ast_mapping(Expression, kind, evidence),
        None => NativeAstMapping {
            namespace: Expression,
            kind: None,
            evidence,
        },
    }
}

fn lower_native_flow(statements: &[Statement]) -> Vec<NativeFlowNode> {
    let mut output = Vec::new();
    let mut next_label = 0;
    for statement in statements {
        lower_native_statement(statement, &mut next_label, &mut output);
    }
    output
}

fn lower_native_statement(
    statement: &Statement,
    next_label: &mut u32,
    output: &mut Vec<NativeFlowNode>,
) {
    use NativeAstEvidence::{Inferred, Observed};
    use NativeAstNamespace::Statement;

    match &statement.kind {
        StatementKind::If {
            condition,
            then_body,
            else_body,
        } => {
            let else_label = *next_label;
            *next_label += 1;
            let end_label = if else_body.is_empty() {
                else_label
            } else {
                let label = *next_label;
                *next_label += 1;
                label
            };
            push_native_flow(
                output,
                statement,
                NativeFlowKind::Branch {
                    condition: condition.clone(),
                    target: else_label,
                    when_true: false,
                },
                0x06,
                Inferred,
            );
            for child in then_body {
                lower_native_statement(child, next_label, output);
            }
            if !else_body.is_empty() {
                push_native_flow(
                    output,
                    statement,
                    NativeFlowKind::Jump { target: end_label },
                    0x04,
                    Inferred,
                );
                push_native_flow(
                    output,
                    statement,
                    NativeFlowKind::Label(else_label),
                    0x03,
                    Observed,
                );
                for child in else_body {
                    lower_native_statement(child, next_label, output);
                }
            }
            push_native_flow(
                output,
                statement,
                NativeFlowKind::Label(end_label),
                0x03,
                Observed,
            );
        }
        StatementKind::Loop {
            loop_kind,
            condition,
            post_test,
            until,
            body,
        } => {
            push_native_flow(
                output,
                statement,
                NativeFlowKind::LoopForm(Box::new(statement.clone())),
                if *loop_kind == LoopKind::Do {
                    0x0b
                } else {
                    0x09
                },
                Inferred,
            );
            let start = *next_label;
            let end = start + 1;
            *next_label += 2;
            push_native_flow(
                output,
                statement,
                NativeFlowKind::Label(start),
                0x03,
                Observed,
            );
            if !post_test {
                if let Some(condition) = condition {
                    push_native_flow(
                        output,
                        statement,
                        NativeFlowKind::Branch {
                            condition: condition.clone(),
                            target: end,
                            when_true: *until,
                        },
                        0x0c,
                        Inferred,
                    );
                }
            }
            for child in body {
                lower_native_statement(child, next_label, output);
            }
            if *post_test {
                if let Some(condition) = condition {
                    if *until {
                        push_native_flow(
                            output,
                            statement,
                            NativeFlowKind::Branch {
                                condition: condition.clone(),
                                target: end,
                                when_true: true,
                            },
                            0x0c,
                            Inferred,
                        );
                        push_native_flow(
                            output,
                            statement,
                            NativeFlowKind::Jump { target: start },
                            0x04,
                            Inferred,
                        );
                    } else {
                        push_native_flow(
                            output,
                            statement,
                            NativeFlowKind::Branch {
                                condition: condition.clone(),
                                target: start,
                                when_true: true,
                            },
                            0x0c,
                            Inferred,
                        );
                    }
                } else {
                    push_native_flow(
                        output,
                        statement,
                        NativeFlowKind::Jump { target: start },
                        0x04,
                        Inferred,
                    );
                }
            } else {
                push_native_flow(
                    output,
                    statement,
                    NativeFlowKind::Jump { target: start },
                    0x04,
                    Inferred,
                );
            }
            push_native_flow(
                output,
                statement,
                NativeFlowKind::Label(end),
                0x03,
                Observed,
            );
        }
        StatementKind::For { .. } => output.push(NativeFlowNode {
            kind: NativeFlowKind::LoopForm(Box::new(statement.clone())),
            native_ast: native_ast_mapping(Statement, 0x09, Inferred),
            info: statement.info.clone(),
        }),
        StatementKind::Select { .. } | StatementKind::Try { .. } | StatementKind::Continue(_) => {
            output.push(NativeFlowNode {
                kind: NativeFlowKind::UnloweredControl(Box::new(statement.clone())),
                native_ast: unknown_native_ast(Statement),
                info: statement.info.clone(),
            });
        }
        StatementKind::NativeControl(control) => {
            let (tag, evidence) = native_control_mapping(control);
            push_native_flow(
                output,
                statement,
                NativeFlowKind::SourceStatement(Box::new(statement.clone())),
                tag,
                evidence,
            );
            if let NativeControl::Goto(target) = control {
                push_native_flow(
                    output,
                    statement,
                    NativeFlowKind::FixupJump {
                        target: target.clone(),
                    },
                    0x13,
                    Inferred,
                );
            }
        }
        _ => {
            let mapping = match &statement.kind {
                StatementKind::Assignment { .. } => native_ast_mapping(Statement, 0x02, Observed),
                StatementKind::Expression(expression) => {
                    let tracked_call = matches!(
                        expression.kind,
                        ExpressionKind::Call { .. } | ExpressionKind::Member { .. }
                    );
                    native_ast_mapping(Statement, if tracked_call { 0x0f } else { 0x02 }, Observed)
                }
                _ => unknown_native_ast(Statement),
            };
            output.push(NativeFlowNode {
                kind: NativeFlowKind::SourceStatement(Box::new(statement.clone())),
                native_ast: mapping,
                info: statement.info.clone(),
            });
        }
    }
}

fn native_control_mapping(control: &NativeControl) -> (u16, NativeAstEvidence) {
    let tag = match control {
        NativeControl::Label(_) => 0x03,
        NativeControl::Goto(_) => 0x04,
        NativeControl::Gosub(_) => 0x05,
        NativeControl::GosubReturn => 0x14,
        NativeControl::ExitSub => 0x11,
        NativeControl::ExitFunction => 0x12,
        NativeControl::Stop => 0x15,
        NativeControl::End => 0x16,
        NativeControl::OnGosubA(_) => 0x17,
        NativeControl::OnGosubB(_) => 0x18,
        NativeControl::OnError(_) => 0x19,
        NativeControl::Resume(_) => 0x1a,
    };
    let evidence = match control {
        NativeControl::Label(_) | NativeControl::Stop | NativeControl::End => {
            NativeAstEvidence::Observed
        }
        _ => NativeAstEvidence::Inferred,
    };
    (tag, evidence)
}

fn push_native_flow(
    output: &mut Vec<NativeFlowNode>,
    source: &Statement,
    kind: NativeFlowKind,
    native_kind: u16,
    evidence: NativeAstEvidence,
) {
    output.push(NativeFlowNode {
        kind,
        native_ast: native_ast_mapping(NativeAstNamespace::Statement, native_kind, evidence),
        info: source.info.clone(),
    });
}

fn lookup_in_chain<'a>(
    scopes: &'a ScopeManager,
    mut id: usize,
    name: &str,
) -> Option<&'a crate::Symbol> {
    loop {
        let scope = scopes.get_scope(id)?;
        if let Some(symbol) = scopes.lookup_in_scope(id, name) {
            return Some(symbol);
        }
        id = scope.parent?;
    }
}

fn is_expression_kind(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::BinaryExpression
            | SyntaxKind::UnaryExpression
            | SyntaxKind::LiteralExpression
            | SyntaxKind::IdentifierExpression
            | SyntaxKind::MemberAccessExpression
            | SyntaxKind::CallExpression
            | SyntaxKind::ParenthesizedExpression
            | SyntaxKind::NumericLiteralExpression
            | SyntaxKind::StringLiteralExpression
            | SyntaxKind::BooleanLiteralExpression
    )
}

fn coalesce_result_type(
    left: &TypeInfo,
    right: &TypeInfo,
    location: &SourceLocation,
) -> Result<(TypeInfo, bool, bool), SemanticError> {
    use vb6core::types::VBType;

    match left.kind {
        VBType::Unknown => Err(SemanticError::UnsupportedFeature {
            feature: "MX S10 ??".to_string(),
            reason: "left operand type is unknown".to_string(),
            location: location.clone(),
        }),
        VBType::Nothing => Ok((right.clone(), false, false)),
        VBType::Null | VBType::Empty => Ok((left.clone(), false, false)),
        VBType::Variant => Ok((TypeInfo::variant(), false, false)),
        VBType::Object | VBType::Class(_) if right.kind == VBType::Nothing => {
            Ok((left.clone(), false, false))
        }
        VBType::Object | VBType::Class(_) => {
            if left.can_assign_to(right) {
                Ok((right.clone(), true, false))
            } else if right.can_assign_to(left) {
                Ok((left.clone(), false, true))
            } else {
                let variant = TypeInfo::variant();
                Ok((variant, true, true))
            }
        }
        _ => Ok((left.clone(), false, false)),
    }
}

fn coerce_short_circuit_operand(expression: Expression) -> Result<Expression, SemanticError> {
    let Some(source_type) = expression.info.ty.as_ref() else {
        return Err(SemanticError::UnsupportedFeature {
            feature: "MX S03 AndAlso/OrElse".to_string(),
            reason: "operand has no statically known type".to_string(),
            location: expression.info.location.clone(),
        });
    };
    let kind = &source_type.kind;
    if *kind == vb6core::types::VBType::Unknown {
        return Err(SemanticError::UnsupportedFeature {
            feature: "MX S03 AndAlso/OrElse".to_string(),
            reason: "operand type is unknown".to_string(),
            location: expression.info.location.clone(),
        });
    }
    if !matches!(
        kind,
        vb6core::types::VBType::Boolean
            | vb6core::types::VBType::Byte
            | vb6core::types::VBType::Integer
            | vb6core::types::VBType::Long
            | vb6core::types::VBType::Single
            | vb6core::types::VBType::Double
            | vb6core::types::VBType::Currency
            | vb6core::types::VBType::Decimal
            | vb6core::types::VBType::String
            | vb6core::types::VBType::Date
            | vb6core::types::VBType::Variant
            | vb6core::types::VBType::Null
            | vb6core::types::VBType::Empty
            | vb6core::types::VBType::Error
            | vb6core::types::VBType::Enum(_)
    ) {
        return Err(SemanticError::InvalidOperation {
            message: format!(
                "MX short-circuit Boolean conversion is not supported for {source_type}"
            ),
            location: expression.info.location.clone(),
        });
    }
    let target_type = TypeInfo::boolean();
    if source_type == &target_type {
        Ok(expression)
    } else {
        Ok(implicit_conversion(expression, target_type))
    }
}

fn implicit_conversion(expression: Expression, target_type: TypeInfo) -> Expression {
    let mut info = expression.info.clone();
    info.ty = Some(target_type.clone());
    let kind = ExpressionKind::Conversion {
        expression: Box::new(expression),
        target_type,
        conversion: ConversionKind::Implicit,
    };
    let native_ast = native_expression_mapping(&kind, info.ty.as_ref());
    Expression {
        kind,
        native_ast,
        info,
    }
}

fn checked_loop_bound(
    expression: Expression,
    target_type: &TypeInfo,
) -> Result<Expression, SemanticError> {
    if let Some(found) = &expression.info.ty {
        crate::TypeChecker::new().check_assignment(
            target_type,
            found,
            &expression.info.location,
        )?;
        if found != target_type {
            return Ok(implicit_conversion(expression, target_type.clone()));
        }
    }
    Ok(expression)
}

fn is_numeric_type(ty: &TypeInfo) -> bool {
    matches!(
        ty.kind,
        vb6core::types::VBType::Byte
            | vb6core::types::VBType::Integer
            | vb6core::types::VBType::Long
            | vb6core::types::VBType::Single
            | vb6core::types::VBType::Double
            | vb6core::types::VBType::Currency
            | vb6core::types::VBType::Decimal
    )
}

fn unsupported(
    node: &CstNode,
    make_info: &impl Fn(&CstNode, Option<usize>, Option<TypeInfo>) -> NodeInfo,
) -> SemanticError {
    SemanticError::UnsupportedFeature {
        feature: format!("VB6 CST construct {:?}", node.kind()),
        reason: "the construct is not yet supported by typed HIR lowering".to_string(),
        location: make_info(node, None, None).location,
    }
}
