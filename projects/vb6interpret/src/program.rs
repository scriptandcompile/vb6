//! The procedure model: procedures extracted from a parsed module's CST.
//!
//! A [`Program`] holds the module-level statements (executed on startup),
//! the procedures declared in the module, and the resolved entry point.

use std::collections::HashMap;

use vb6core::types::VBType;
use vb6parse::parsers::cst::CstNode;
use vb6parse::parsers::SyntaxKind;

/// A reference to the VB6 data type named by a type keyword in the CST.
pub(crate) fn type_from_keyword(node: &CstNode) -> Option<VBType> {
    match node.kind() {
        SyntaxKind::ByteKeyword => Some(VBType::Byte),
        SyntaxKind::IntegerKeyword => Some(VBType::Integer),
        SyntaxKind::LongKeyword => Some(VBType::Long),
        SyntaxKind::SingleKeyword => Some(VBType::Single),
        SyntaxKind::DoubleKeyword => Some(VBType::Double),
        SyntaxKind::CurrencyKeyword => Some(VBType::Currency),
        SyntaxKind::StringKeyword => Some(VBType::String),
        SyntaxKind::BooleanKeyword => Some(VBType::Boolean),
        SyntaxKind::DateKeyword => Some(VBType::Date),
        SyntaxKind::VariantKeyword => Some(VBType::Variant),
        SyntaxKind::ObjectKeyword => Some(VBType::Object),
        SyntaxKind::DecimalKeyword => Some(VBType::Double),
        _ => None,
    }
}

/// Whether a node is a VB6 type keyword.
pub(crate) fn is_identifier_like(node: &CstNode) -> bool {
    let text = node.text().trim();
    if text.is_empty() {
        return false;
    }
    let mut chars = text.chars();
    let first_ok = matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_');
    first_ok
        && chars.all(|c| {
            c.is_ascii_alphanumeric() || c == '_' || matches!(c, '$' | '%' | '&' | '!' | '#' | '@')
        })
}

/// The identifier name of a node: its first identifier-like significant child
/// (tolerates keyword tokens used as identifiers, e.g. `Base`).
pub(crate) fn identifier_name(node: &CstNode) -> String {
    node.significant_children()
        .find(|c| is_identifier_like(c))
        .map(|c| c.text().trim().to_string())
        .unwrap_or_default()
}

/// Whether a node is a statement (used to separate body statements from the
/// structural children of compound statements such as single-line `If`).
pub(crate) fn is_statement_kind(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::AssignmentStatement
            | SyntaxKind::DimStatement
            | SyntaxKind::ConstStatement
            | SyntaxKind::ReDimStatement
            | SyntaxKind::IfStatement
            | SyntaxKind::ForStatement
            | SyntaxKind::ForEachStatement
            | SyntaxKind::DoStatement
            | SyntaxKind::WhileStatement
            | SyntaxKind::SelectCaseStatement
            | SyntaxKind::CallStatement
            | SyntaxKind::SetStatement
            | SyntaxKind::LetStatement
            | SyntaxKind::ExitStatement
            | SyntaxKind::EndStatement
            | SyntaxKind::StopStatement
            | SyntaxKind::BeepStatement
            | SyntaxKind::PrintStatement
            | SyntaxKind::OpenStatement
            | SyntaxKind::CloseStatement
            | SyntaxKind::GoSubStatement
            | SyntaxKind::GotoStatement
            | SyntaxKind::OnErrorStatement
            | SyntaxKind::ReturnStatement
            | SyntaxKind::ResumeStatement
            | SyntaxKind::EraseStatement
            | SyntaxKind::TypeStatement
            | SyntaxKind::EnumStatement
            | SyntaxKind::MidStatement
            | SyntaxKind::DateStatement
            | SyntaxKind::TimeStatement
            | SyntaxKind::ErrorStatement
            | SyntaxKind::RandomizeStatement
            | SyntaxKind::AppActivateStatement
            | SyntaxKind::SendKeysStatement
            | SyntaxKind::OptionStatement
            | SyntaxKind::DeclareStatement
            | SyntaxKind::AttributeStatement
    )
}

/// A procedure parameter.
#[derive(Debug, Clone)]
pub struct Param {
    /// Parameter name (original casing).
    pub name: String,
    /// Whether the parameter is passed by reference (default in VB6).
    pub by_ref: bool,
    /// The declared static type (`Variant` when untyped).
    pub ty: VBType,
    /// Whether the parameter is `Optional`.
    pub optional: bool,
}

/// A declared procedure (Sub or Function).
#[derive(Debug, Clone)]
pub struct Procedure {
    /// Procedure name (original casing).
    pub name: String,
    /// Whether this is a Function (has a return value) or a Sub.
    pub is_function: bool,
    /// Parameters, in declaration order.
    pub params: Vec<Param>,
    /// The declared return type (`Variant` for untyped Functions).
    pub return_type: VBType,
    /// The procedure body (`StatementList` child), if any.
    pub body: Option<CstNode>,
    /// The CST line the procedure declaration starts on (1-based, header
    /// `Attribute` lines stripped).
    pub line: usize,
    /// The CST line the `End Sub` / `End Function` terminator is on.
    pub end_line: usize,
}

impl Procedure {
    /// The normalized (case-insensitive) name used for lookup.
    pub fn key(&self) -> String {
        self.name.to_lowercase()
    }
}

/// A loaded module ready for interpretation.
#[derive(Debug, Clone)]
pub struct Program {
    /// The module's CST root. Its significant children are the module-level
    /// statements and procedure declarations; module-level statements are
    /// executed on startup (before the entry procedure).
    pub root: CstNode,
    /// Procedures by normalized name.
    pub procedures: HashMap<String, Procedure>,
    /// The entry procedure name (normalized).
    pub entry: String,
}

/// Determine the return type of a procedure from its CST node.
///
/// The return type, when present, appears as `AsKeyword` followed by a type
/// keyword directly after the parameter list.
fn procedure_return_type(node: &CstNode) -> VBType {
    let mut significant = node.significant_children().peekable();
    // Skip leading whitespace-less tokens: Sub/Function keyword is first.
    // Walk significant children looking for `AsKeyword` followed by a type
    // keyword. The parameter list's own `As` tokens are nested inside the
    // `ParameterList` node, so any `AsKeyword` at this level is the return type.
    while let Some(child) = significant.next() {
        if child.kind() == SyntaxKind::AsKeyword {
            if let Some(next) = significant.next() {
                if let Some(ty) = type_from_keyword(next) {
                    return ty;
                }
            }
        }
    }
    VBType::Variant
}

/// Extract the `ParameterList` node from a procedure node.
fn parameter_list(node: &CstNode) -> Option<&CstNode> {
    node.first_child_by_kind(SyntaxKind::ParameterList)
}

/// Parse the parameter declarations from a `ParameterList`.
fn parse_params(parameter_list: &CstNode) -> Vec<Param> {
    let mut params = Vec::new();
    let mut current: Option<Param> = None;

    for child in parameter_list.significant_children() {
        match child.kind() {
            SyntaxKind::Identifier => {
                // A new parameter starts. Finalize the previous one.
                if let Some(prev) = current.take() {
                    params.push(prev);
                }
                current = Some(Param {
                    name: child.text().trim().to_string(),
                    by_ref: true,
                    ty: VBType::Variant,
                    optional: false,
                });
            }
            SyntaxKind::ByValKeyword | SyntaxKind::ByRefKeyword => {
                if let Some(param) = current.as_mut() {
                    param.by_ref = child.kind() == SyntaxKind::ByRefKeyword;
                }
            }
            SyntaxKind::OptionalKeyword => {
                if let Some(param) = current.as_mut() {
                    param.optional = true;
                }
            }
            _ => {
                if let Some(ty) = type_from_keyword(child) {
                    if let Some(param) = current.as_mut() {
                        param.ty = ty;
                    }
                }
            }
        }
    }
    if let Some(last) = current {
        params.push(last);
    }
    params
}

/// Build a [`Program`] from a parsed module's CST root.
///
/// `ModuleFile::parse` strips `Attribute` statements, so the root's children
/// are the module-level statements and procedure declarations.
pub(crate) fn build_program(root: &CstNode, module_name: &str) -> Program {
    let mut procedures: HashMap<String, Procedure> = HashMap::new();
    let mut entry: Option<String> = None;
    let mut line = 1;

    for child in root.children() {
        match child.kind() {
            SyntaxKind::Newline => line += 1,
            SyntaxKind::SubStatement | SyntaxKind::FunctionStatement => {
                let name = procedure_name(child);
                let key = name.to_lowercase();
                let is_function = child.kind() == SyntaxKind::FunctionStatement;
                let params = parameter_list(child).map(parse_params).unwrap_or_default();
                let return_type = if is_function {
                    procedure_return_type(child)
                } else {
                    VBType::Variant
                };
                let body = child
                    .first_child_by_kind(SyntaxKind::StatementList)
                    .cloned();

                let end_line = line
                    + child
                        .children()
                        .iter()
                        .take_while(|c| c.kind() != SyntaxKind::EndKeyword)
                        .map(crate::exec::count_newlines)
                        .sum::<usize>();

                if entry.is_none() {
                    entry = Some(key.clone());
                }

                procedures.insert(
                    key,
                    Procedure {
                        name,
                        is_function,
                        params,
                        return_type,
                        body,
                        line,
                        end_line,
                    },
                );
                line += crate::exec::count_newlines(child);
            }
            _ => line += crate::exec::count_newlines(child),
        }
    }

    // Prefer `Sub Main` when present, as real VB6 programs do.
    let entry = if procedures.contains_key("main") {
        "main".to_string()
    } else {
        entry.unwrap_or_else(|| "main".to_string())
    };

    let _ = module_name;
    Program {
        root: root.clone(),
        procedures,
        entry,
    }
}

/// The identifier naming a procedure.
fn procedure_name(node: &CstNode) -> String {
    node.significant_children()
        .find(|c| c.kind() == SyntaxKind::Identifier)
        .map(|c| c.text().trim().to_string())
        .unwrap_or_default()
}
