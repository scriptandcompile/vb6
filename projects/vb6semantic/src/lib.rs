#![allow(rustdoc::private_intra_doc_links)]
#![warn(missing_docs)]
//! Semantic analysis library for VB6 code.
//!
//! This library provides tools for analyzing the semantics of VB6 code, including:
//! - Scope management and symbol resolution
//! - Type checking and inference
//! - Name resolution and disambiguation
//! - Error reporting and diagnostics
//!
//! The main entry point is the `SemanticAnalyzer` struct, which can analyze VB6
//! project files and produce a detailed analysis result. The library is designed
//! to be used in conjunction with the `vb6parse` library for parsing VB6 code from
//! source files into a CST (Concrete Syntax Tree).
//!
//! # Examples
//!
//! ```rust, no_run
//! use vb6semantic::SemanticAnalyzer;
//! use vb6parse::io::SourceFile;
//! use vb6parse::files::ProjectFile;
//!
//! let mut analyzer = SemanticAnalyzer::new();
//! let project_source = SourceFile::from_file("MyProject.vbp").expect("Failed to read project file");
//! let (project_opt, failures) = ProjectFile::parse(&project_source).unpack();
//! if !failures.is_empty() {
//!     eprintln!("Failed to parse project file: {:?}", failures);
//!     return;
//! }
//!
//! let project = project_opt.expect("Project file should have parsed successfully");
//!
//! let analysis_result = analyzer.analyze_project(&project).expect("Failed to analyze project");
//! println!("Analysis completed with {} errors and {} warnings", analysis_result.errors.len(), analysis_result.warnings.len());
//! ```

pub mod analyzer;
pub mod error;
pub mod location;
pub mod query;
pub mod references;
pub mod resolution;
pub mod scope;
pub mod symbols;
pub mod types;

// wasm module for playground use.
#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export core types
/// Semantic analysis engine for VB6 projects.
pub use analyzer::SemanticAnalyzer;
/// Semantic analysis errors and results.
pub use error::{Result, SemanticError, SourceLocation};
/// Resolved references registry and resolvers.
pub use references::{
    ManifestReferenceResolver, ReferenceContext, ReferenceInfo, ReferenceRegistry,
    ReferenceResolver, StaticReferenceResolver,
};
/// Name resolution across scopes.
pub use resolution::NameResolver;
/// Scope management for symbol lookup.
pub use scope::{Scope, ScopeKind, ScopeManager};
/// A symbol in the symbol table.
pub use symbols::{Symbol, SymbolKind, SymbolTable, Visibility};
/// Type checking and inference.
pub use types::{TypeChecker, TypeInfo};
/// VB6 type definition (re-exported from vb6core).
pub use vb6core::types::VBType;

/// Version of the semantic analysis library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
