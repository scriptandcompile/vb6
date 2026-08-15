//! WebAssembly bindings for the VB6 semantic analyzer.
//!
//! This module provides functions for performing semantic analysis on VB6 code in a
//! WebAssembly environment. It exposes these functions to JavaScript via WebAssembly,
//! allowing VB6 semantic analysis (symbol tables, scopes, type checks, and diagnostics)
//! to run in the browser.
//!
//! Predominantly, this is designed for the needs of playground-style web tools that
//! analyze VB6 code without a server.
//!
//! Note: `init_panic_hook` is intentionally not re-exported here. The `vb6parse`
//! dependency is linked into the same wasm module and already exports it, so
//! consumers of this crate's bindings can call it to initialize the panic hook.
//!

use crate::analyzer::SemanticAnalyzer;
use crate::error::{SemanticError, SourceLocation};
use crate::query::QueryIndex;
use crate::scope::{Scope, ScopeManager};
use crate::symbols::Symbol;
use crate::types::TypeInfo;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use vb6parse::io::SourceFile;
use wasm_bindgen::prelude::*;

/// A source location in the analyzed code.
#[derive(Serialize, Deserialize)]
pub struct LocationInfo {
    /// Name of the source file.
    pub file: String,
    /// Line number in the source file (1-based).
    pub line: usize,
    /// Column number in the source file (1-based).
    pub column: usize,
}

/// Information about a semantic error.
#[derive(Serialize, Deserialize)]
pub struct SemanticErrorInfo {
    /// The semantic error category (e.g., `UndefinedSymbol`, `TypeMismatch`).
    #[serde(rename = "type")]
    pub type_name: String,
    /// A human-readable description of the error.
    pub message: String,
    /// The location of the error in the source code, if one is available.
    pub location: Option<LocationInfo>,
}

/// Information about a symbol in the symbol table.
#[derive(Serialize, Deserialize)]
pub struct SymbolInfo {
    /// Name of the symbol.
    pub name: String,
    /// Kind of symbol (e.g., `Variable`, `Function`, `Class`).
    pub kind: String,
    /// Structured type information for the symbol.
    pub type_info: TypeInfo,
    /// Human-readable type of the symbol.
    pub type_display: String,
    /// Visibility of the symbol (e.g., `Public`, `Private`, `Friend`).
    pub visibility: String,
    /// Location where the symbol is defined.
    pub location: LocationInfo,
    /// Scope ID where the symbol is defined.
    pub scope_id: usize,
    /// Additional symbol attributes.
    pub attributes: Vec<(String, String)>,
}

/// Information about a scope in the scope manager.
#[derive(Serialize, Deserialize)]
pub struct ScopeInfo {
    /// Unique identifier for this scope.
    pub id: usize,
    /// Kind of scope (e.g., `Global`, `Class`, `Procedure`).
    pub kind: String,
    /// Parent scope ID (None for the global scope).
    pub parent: Option<usize>,
    /// Child scope IDs.
    pub children: Vec<usize>,
    /// Name of the scope.
    pub name: String,
    /// Symbols defined directly in this scope.
    pub symbols: Vec<SymbolInfo>,
}

/// Information about a project reference.
#[derive(Serialize, Deserialize)]
pub struct WasmReferenceInfo {
    /// The GUID of the referenced type library, if this is a compiled reference.
    pub guid: Option<String>,
    /// The path from the `Reference=` line.
    pub path: String,
    /// The human-readable description.
    pub description: String,
    /// Whether this is a sub-project reference rather than a compiled library.
    pub is_subproject: bool,
    /// A short, human-readable name used in diagnostics.
    pub display_name: String,
}

/// A single occurrence of a symbol, as recorded by the query index.
#[derive(Serialize, Deserialize)]
pub struct WasmSymbolReference {
    /// The scope the symbol lives in.
    pub scope_id: usize,
    /// The lowercased symbol name (VB6 names are case-insensitive).
    pub name: String,
    /// The role this occurrence plays: `Definition`, `Usage`, or `TypeReference`.
    pub kind: String,
    /// 1-based position of the identifier.
    pub location: LocationInfo,
    /// Inclusive start byte offset of the identifier.
    pub start_offset: u32,
    /// Exclusive end byte offset of the identifier.
    pub end_offset: u32,
    /// 1-based exclusive end column of the identifier.
    pub end_column: usize,
}

/// Information about the output of the VB6 semantic analyzer.
#[derive(Serialize, Deserialize)]
pub struct AnalysisOutput {
    /// All scopes in the final scope manager, including their symbols.
    pub scopes: Vec<ScopeInfo>,
    /// Semantic errors found during analysis.
    pub errors: Vec<SemanticErrorInfo>,
    /// Warnings generated during analysis.
    pub warnings: Vec<String>,
    /// References that a registered resolver supplied symbols for.
    pub resolved_references: Vec<WasmReferenceInfo>,
    /// References no registered resolver could handle.
    pub unresolved_references: Vec<WasmReferenceInfo>,
    /// Every resolved identifier occurrence in the source, grouped per symbol
    /// by scope and name. Drives go-to-definition and find-references in the
    /// editor extension.
    pub references: Vec<WasmSymbolReference>,
    /// Whether analysis completed without any errors.
    pub successful: bool,
    /// The total number of errors.
    pub error_count: usize,
    /// The total number of warnings.
    pub warning_count: usize,
    /// The total number of symbols across all scopes.
    pub symbol_count: usize,
    /// The total number of scopes.
    pub scope_count: usize,
    /// The time taken to analyze the source code, in milliseconds.
    pub analyze_time_ms: f64,
}

/// Convert a `SourceLocation` to the wasm-facing `LocationInfo`.
fn convert_location(location: &SourceLocation) -> LocationInfo {
    LocationInfo {
        file: location.file.clone(),
        line: location.line,
        column: location.column,
    }
}

/// Convert a `Symbol` to the wasm-facing `SymbolInfo`.
fn convert_symbol(symbol: &Symbol) -> SymbolInfo {
    let mut attributes: Vec<(String, String)> = symbol
        .attributes
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    attributes.sort();

    SymbolInfo {
        name: symbol.name.clone(),
        kind: format!("{:?}", symbol.kind),
        type_info: symbol.type_info.clone(),
        type_display: symbol.type_info.to_string(),
        visibility: format!("{:?}", symbol.visibility),
        location: convert_location(&symbol.location),
        scope_id: symbol.scope_id,
        attributes,
    }
}

/// Convert a `Scope` to the wasm-facing `ScopeInfo`.
fn convert_scope(scope: &Scope) -> ScopeInfo {
    let mut symbols: Vec<SymbolInfo> = scope.symbols.values().map(convert_symbol).collect();
    symbols.sort_by(|a, b| a.name.cmp(&b.name));

    ScopeInfo {
        id: scope.id,
        kind: format!("{:?}", scope.kind),
        parent: scope.parent,
        children: scope.children.clone(),
        name: scope.name.clone(),
        symbols,
    }
}

/// Convert all scopes in a `ScopeManager` to wasm-facing `ScopeInfo` values.
fn convert_scope_manager(scope_manager: &ScopeManager) -> Vec<ScopeInfo> {
    let mut scopes: Vec<ScopeInfo> = scope_manager
        .all_scopes()
        .into_iter()
        .map(convert_scope)
        .collect();
    scopes.sort_by_key(|scope| scope.id);
    scopes
}

/// The semantic error category name for a `SemanticError`.
fn error_type_name(error: &SemanticError) -> &'static str {
    match error {
        SemanticError::UndefinedSymbol { .. } => "UndefinedSymbol",
        SemanticError::DuplicateSymbol { .. } => "DuplicateSymbol",
        SemanticError::TypeMismatch { .. } => "TypeMismatch",
        SemanticError::InvalidScope { .. } => "InvalidScope",
        SemanticError::InvalidType { .. } => "InvalidType",
        SemanticError::CircularDependency { .. } => "CircularDependency",
        SemanticError::InvalidOperation { .. } => "InvalidOperation",
        SemanticError::InaccessibleSymbol { .. } => "InaccessibleSymbol",
        SemanticError::InvalidAssignment { .. } => "InvalidAssignment",
        SemanticError::ParameterMismatch { .. } => "ParameterMismatch",
        SemanticError::FileReadError { .. } => "FileReadError",
        SemanticError::FileParseError { .. } => "FileParseError",
        SemanticError::AnalysisError(_) => "AnalysisError",
    }
}

/// The location associated with a `SemanticError`, if any.
fn error_location(error: &SemanticError) -> Option<LocationInfo> {
    match error {
        SemanticError::UndefinedSymbol { location, .. }
        | SemanticError::DuplicateSymbol { location, .. }
        | SemanticError::TypeMismatch { location, .. }
        | SemanticError::InvalidType { location, .. }
        | SemanticError::InvalidOperation { location, .. }
        | SemanticError::InaccessibleSymbol { location, .. }
        | SemanticError::InvalidAssignment { location, .. }
        | SemanticError::ParameterMismatch { location, .. } => Some(convert_location(location)),
        _ => None,
    }
}

/// Convert a `SemanticError` to the wasm-facing `SemanticErrorInfo`.
fn convert_error(error: &SemanticError) -> SemanticErrorInfo {
    SemanticErrorInfo {
        type_name: error_type_name(error).to_string(),
        message: error.to_string(),
        location: error_location(error),
    }
}

/// Build the wasm-facing `AnalysisOutput` from an analyzer's final state.
fn build_analysis_output(analyzer: &SemanticAnalyzer) -> AnalysisOutput {
    let scopes = convert_scope_manager(analyzer.scope_manager());
    let errors: Vec<SemanticErrorInfo> = analyzer.errors().iter().map(convert_error).collect();
    let warnings = analyzer.warnings().to_vec();
    let references = convert_query_index(analyzer.query_index());

    AnalysisOutput {
        successful: errors.is_empty(),
        error_count: errors.len(),
        warning_count: warnings.len(),
        symbol_count: scopes.iter().map(|scope| scope.symbols.len()).sum(),
        scope_count: scopes.len(),
        scopes,
        errors,
        warnings,
        resolved_references: Vec::new(),
        unresolved_references: Vec::new(),
        references,
        analyze_time_ms: 0.0,
    }
}

/// Convert every query-index occurrence to its wasm-facing form.
fn convert_query_index(index: &QueryIndex) -> Vec<WasmSymbolReference> {
    index
        .iter()
        .flat_map(|(key, references)| {
            references.iter().map(move |reference| WasmSymbolReference {
                scope_id: key.scope_id,
                name: key.name.clone(),
                kind: format!("{:?}", reference.kind),
                location: convert_location(&reference.location),
                start_offset: reference.start_offset,
                end_offset: reference.end_offset,
                end_column: reference.end_column,
            })
        })
        .collect()
}

/// Analyze a single VB6 source string and populate the analyzer.
///
/// The `file_type` argument selects which parser to use and must be one of
/// `module`/`bas`, `class`/`cls`, or `form`/`frm`. Project analysis is not
/// supported because it requires reading referenced files from disk.
fn analyze_source(
    analyzer: &mut SemanticAnalyzer,
    source: &SourceFile,
    file_type: &str,
) -> Result<(), JsError> {
    match file_type {
        "module" | "bas" => {
            let (module_opt, _failures) = vb6parse::files::ModuleFile::parse(source).unpack();
            let Some(module) = module_opt else {
                return Err(JsError::new(
                    "Failed to parse the input code as a VB6 module (.bas).",
                ));
            };
            analyzer
                .analyze_module(&module)
                .map_err(|e| JsError::new(&e.to_string()))?;
        }
        "class" | "cls" => {
            let (class_opt, _failures) = vb6parse::files::ClassFile::parse(source).unpack();
            let Some(class) = class_opt else {
                return Err(JsError::new(
                    "Failed to parse the input code as a VB6 class (.cls).",
                ));
            };
            analyzer
                .analyze_class(&class)
                .map_err(|e| JsError::new(&e.to_string()))?;
        }
        "form" | "frm" => {
            let (form_opt, _failures) = vb6parse::files::FormFile::parse(source).unpack();
            let Some(form) = form_opt else {
                return Err(JsError::new(
                    "Failed to parse the input code as a VB6 form (.frm).",
                ));
            };
            analyzer
                .analyze_form(&form)
                .map_err(|e| JsError::new(&e.to_string()))?;
        }
        other => {
            return Err(JsError::new(&format!(
                "Unknown file type '{other}'. Supported types are 'module', 'class', 'form'."
            )));
        }
    }
    Ok(())
}

/// Performs semantic analysis on VB6 code and returns an `AnalysisOutput` object
/// containing scopes, symbols, errors, and warnings.
///
/// The `file_type` argument selects which parser to use and must be one of
/// `module`/`bas`, `class`/`cls`, or `form`/`frm`.
///
/// # Errors
///
/// Returns an error if the input code cannot be parsed into the requested file
/// type or if the analyzer fails during analysis.
///
#[wasm_bindgen]
pub fn analyze_vb6_code(code: &str, file_type: &str) -> Result<JsValue, JsError> {
    let source = SourceFile::from_string("test.bas", code);

    let mut analyzer = SemanticAnalyzer::new();
    analyze_source(&mut analyzer, &source, file_type)?;

    let output = build_analysis_output(&analyzer);
    Ok(to_value(&output).unwrap())
}
