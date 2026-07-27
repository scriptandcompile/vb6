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
pub mod resolution;
pub mod scope;
pub mod symbols;
pub mod types;

// Re-export core types
pub use analyzer::SemanticAnalyzer;
pub use error::{Result, SemanticError, SourceLocation};
pub use resolution::NameResolver;
pub use scope::{Scope, ScopeKind, ScopeManager};
pub use symbols::{Symbol, SymbolKind, SymbolTable, Visibility};
pub use types::{TypeChecker, TypeInfo, TypeKind};

/// Version of the semantic analysis library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
