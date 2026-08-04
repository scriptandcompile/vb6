//! Semantic analyzer for VB6 code. This module performs semantic analysis on the parsed CST
//! to build symbol tables, resolve names, check types, and report errors.
//!
//! The main entry point is the `SemanticAnalyzer` struct, which provides methods to analyze
//! project files, module files, class files, and form files. The analyzer maintains a scope
//! manager to handle symbol resolution and a type checker for validating type usage. Errors and
//! warnings are collected during analysis and can be retrieved after the process is complete.
//!
//! The `NameResolver` struct is used for resolving symbol references within the current
//! scope context. It provides methods to resolve simple names and qualified names, as well
//! as checking symbol accessibility.
//!
//! The `ScopeManager` struct manages the hierarchy of scopes and symbols, allowing for lookups
//! and scope transitions during analysis. The `TypeChecker` struct provides methods for checking
//! type compatibility and assignment validity.
//!
//! Overall, this module is responsible for the core semantic analysis logic that ensures the VB6 code
//! is semantically correct and provides meaningful error messages for any issues found.
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

use crate::error::{Result, SourceLocation};
use crate::scope::{ScopeKind, ScopeManager};
use crate::symbols::Symbol;
use crate::types::TypeChecker;

/// Main semantic analyzer that processes VB6 code
pub struct SemanticAnalyzer {
    /// Scope manager for symbol resolution
    scope_manager: ScopeManager,

    /// Type checker for type validation
    #[allow(dead_code)]
    type_checker: TypeChecker,

    /// Current file being analyzed
    current_file: Option<String>,

    /// Collected errors
    errors: Vec<crate::error::SemanticError>,

    /// Collected warnings
    warnings: Vec<String>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer instance
    pub fn new() -> Self {
        Self {
            scope_manager: ScopeManager::new(),
            type_checker: TypeChecker::new(),
            current_file: None,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Analyze a VB6 project file
    pub fn analyze_project(
        &mut self,
        project: &vb6parse::files::ProjectFile,
    ) -> Result<AnalysisResult> {
        // Create project-level scope
        let _project_scope = self
            .scope_manager
            .push_scope(ScopeKind::Global, project.properties.name.to_string());

        // TODO: Analyze project references and external dependencies

        for module_reference in project.modules() {
            self.analyze_module_reference(module_reference)?;
        }

        for class_reference in project.classes() {
            self.analyze_class_reference(class_reference)?;
        }

        for form_file_name in project.forms() {
            self.analyze_form_path(form_file_name)?;
        }

        // TODO: Analyze any additional project-level constructs if necessary

        // For now, return empty result
        Ok(AnalysisResult {
            scope_manager: self.scope_manager.clone(),
            errors: self.errors.clone(),
            warnings: self.warnings.clone(),
        })
    }

    /// Analyze a module reference from the project file reference
    pub fn analyze_module_reference(
        &mut self,
        module_reference: &vb6parse::files::project::ProjectModuleReference,
    ) -> Result<()> {
        let source_file =
            vb6parse::io::SourceFile::from_file(module_reference.path).map_err(|e| {
                crate::error::SemanticError::FileReadError {
                    file: module_reference.path.to_string(),
                    message: e.to_string(),
                }
            })?;

        let (module_opt, failures) = vb6parse::files::ModuleFile::parse(&source_file).unpack();
        if let Some(module) = module_opt {
            self.analyze_module(&module)?;
        } else if !failures.is_empty() {
            let diagnostics = failures
                .into_iter()
                .map(|failure| vb6parse::errors::ErrorDetails {
                    source_name: failure.source_name.clone(),
                    source_content: Box::leak(failure.source_content.to_string().into_boxed_str()),
                    error_offset: failure.error_offset,
                    line_start: failure.line_start,
                    line_end: failure.line_end,
                    kind: failure.kind,
                    severity: failure.severity,
                    labels: failure.labels,
                    notes: failure.notes,
                })
                .collect();
            return Err(crate::error::SemanticError::FileParseError {
                file: module_reference.path.to_string(),
                diagnostics,
            });
        }
        Ok(())
    }

    /// Analyze a module file
    pub fn analyze_module(&mut self, module: &vb6parse::files::ModuleFile) -> Result<()> {
        self.current_file = Some(module.name.clone());

        // Create module scope
        let _module_scope = self
            .scope_manager
            .push_scope(ScopeKind::Global, module.name.clone());

        // TODO: Process module-level declarations
        // - Process Option Explicit, Option Base, etc.
        // - Process module-level variables
        // - Process constants
        // - Process procedures and functions
        // - Process type definitions
        // - Process enums

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Analyze a class reference from the project file class reference
    pub fn analyze_class_reference(
        &mut self,
        class_reference: &vb6parse::files::project::ProjectClassReference,
    ) -> Result<()> {
        let source_file =
            vb6parse::io::SourceFile::from_file(class_reference.path).map_err(|e| {
                crate::error::SemanticError::FileReadError {
                    file: class_reference.path.to_string(),
                    message: e.to_string(),
                }
            })?;

        let (class_opt, failures) = vb6parse::files::ClassFile::parse(&source_file).unpack();
        if let Some(class) = class_opt {
            self.analyze_class(&class)?;
        } else if !failures.is_empty() {
            let diagnostics = failures
                .into_iter()
                .map(|failure| vb6parse::errors::ErrorDetails {
                    source_name: failure.source_name.clone(),
                    source_content: Box::leak(failure.source_content.to_string().into_boxed_str()),
                    error_offset: failure.error_offset,
                    line_start: failure.line_start,
                    line_end: failure.line_end,
                    kind: failure.kind,
                    severity: failure.severity,
                    labels: failure.labels,
                    notes: failure.notes,
                })
                .collect();
            return Err(crate::error::SemanticError::FileParseError {
                file: class_reference.path.to_string(),
                diagnostics,
            });
        }
        Ok(())
    }

    /// Analyze a class file
    pub fn analyze_class(&mut self, _class: &vb6parse::files::ClassFile) -> Result<()> {
        self.current_file = Some("class".to_string());

        // Create class scope
        let _class_scope = self
            .scope_manager
            .push_scope(ScopeKind::Class, "class".to_string());

        // TODO: Process class members
        // - Process properties
        // - Process methods
        // - Process events
        // - Process implements

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Analyze a form file by its path
    pub fn analyze_form_path(&mut self, form_reference_path: &str) -> Result<()> {
        let source_file =
            vb6parse::io::SourceFile::from_file(form_reference_path).map_err(|e| {
                crate::error::SemanticError::FileReadError {
                    file: form_reference_path.to_string(),
                    message: e.to_string(),
                }
            })?;

        let (form_opt, failures) = vb6parse::files::FormFile::parse(&source_file).unpack();
        if let Some(form) = form_opt {
            self.analyze_form(&form)?;
        } else if !failures.is_empty() {
            let diagnostics = failures
                .into_iter()
                .map(|failure| vb6parse::errors::ErrorDetails {
                    source_name: failure.source_name.clone(),
                    source_content: Box::leak(failure.source_content.to_string().into_boxed_str()),
                    error_offset: failure.error_offset,
                    line_start: failure.line_start,
                    line_end: failure.line_end,
                    kind: failure.kind,
                    severity: failure.severity,
                    labels: failure.labels,
                    notes: failure.notes,
                })
                .collect();
            return Err(crate::error::SemanticError::FileParseError {
                file: form_reference_path.to_string(),
                diagnostics,
            });
        }
        Ok(())
    }

    /// Analyze a form file
    pub fn analyze_form(&mut self, _form: &vb6parse::files::FormFile) -> Result<()> {
        self.current_file = Some("form".to_string());

        // Create form scope (forms are like classes)
        let _form_scope = self
            .scope_manager
            .push_scope(ScopeKind::Class, "form".to_string());

        // TODO: Process form structure
        // - Process controls (add as symbols)
        // - Process event handlers
        // - Process form-level variables

        self.scope_manager.pop_scope()?;
        Ok(())
    }

    /// Add a symbol to the current scope
    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<()> {
        match self.scope_manager.add_symbol(symbol) {
            Ok(()) => Ok(()),
            Err(e) => {
                self.errors.push(e.clone());
                Err(e)
            }
        }
    }

    /// Lookup a symbol
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.scope_manager.lookup(name)
    }

    /// Get the scope manager (for inspection)
    pub fn scope_manager(&self) -> &ScopeManager {
        &self.scope_manager
    }

    /// Get collected errors
    pub fn errors(&self) -> &[crate::error::SemanticError] {
        &self.errors
    }

    /// Get collected warnings
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Add a warning
    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }

    /// Create a source location for current file
    #[allow(dead_code)]
    fn make_location(&self, line: usize, column: usize) -> SourceLocation {
        SourceLocation {
            file: self
                .current_file
                .clone()
                .unwrap_or_else(|| "<unknown>".to_string()),
            line,
            column,
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of semantic analysis
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Final scope manager with all symbols
    pub scope_manager: ScopeManager,

    /// Errors found during analysis
    pub errors: Vec<crate::error::SemanticError>,

    /// Warnings generated
    pub warnings: Vec<String>,
}

impl AnalysisResult {
    /// Check if analysis was successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbols::{SymbolKind, Visibility};
    use crate::types::TypeInfo;
    use std::{collections::HashMap, fs};
    use tempfile::tempdir;

    #[test]
    fn analyze_empty_project() {
        let mut analyzer = SemanticAnalyzer::new();
        let project = vb6parse::files::ProjectFile::default();

        let result = analyzer
            .analyze_project(&project)
            .expect("Analysis should succeed");

        assert!(result.is_successful());
        assert_eq!(result.error_count(), 0);
        assert_eq!(result.warning_count(), 0);
    }

    #[test]
    fn add_symbol_and_lookup_in_current_scope() {
        let mut analyzer = SemanticAnalyzer::new();
        let child_scope_id = analyzer
            .scope_manager
            .push_scope(ScopeKind::Procedure, "proc".to_string());

        let symbol = Symbol {
            name: "counter".to_string(),
            kind: SymbolKind::Variable,
            type_info: TypeInfo::integer(),
            visibility: Visibility::Private,
            location: SourceLocation {
                file: "Module1.bas".to_string(),
                line: 1,
                column: 1,
            },
            scope_id: child_scope_id,
            attributes: HashMap::new(),
        };

        analyzer.add_symbol(symbol).expect("Symbol should be added");

        let resolved = analyzer
            .lookup_symbol("counter")
            .expect("Symbol should be resolvable");
        assert_eq!(resolved.name, "counter");
        assert_eq!(resolved.kind, SymbolKind::Variable);
    }

    #[test]
    fn duplicate_symbol_records_an_error() {
        let mut analyzer = SemanticAnalyzer::new();

        let first = Symbol {
            name: "value".to_string(),
            kind: SymbolKind::Variable,
            type_info: TypeInfo::integer(),
            visibility: Visibility::Private,
            location: SourceLocation {
                file: "Module1.bas".to_string(),
                line: 1,
                column: 1,
            },
            scope_id: analyzer.scope_manager.global_scope_id(),
            attributes: HashMap::new(),
        };
        let second = Symbol {
            name: "value".to_string(),
            kind: SymbolKind::Variable,
            type_info: TypeInfo::integer(),
            visibility: Visibility::Private,
            location: SourceLocation {
                file: "Module1.bas".to_string(),
                line: 2,
                column: 1,
            },
            scope_id: analyzer.scope_manager.global_scope_id(),
            attributes: HashMap::new(),
        };

        analyzer
            .add_symbol(first)
            .expect("First symbol should be added");
        let duplicate = analyzer.add_symbol(second);

        assert!(duplicate.is_err());
        assert!(matches!(
            analyzer.errors().first(),
            Some(crate::error::SemanticError::DuplicateSymbol { .. })
        ));
    }

    #[test]
    fn analyze_module_reference_reports_missing_files() {
        let mut analyzer = SemanticAnalyzer::new();
        let module_reference = vb6parse::files::project::ProjectModuleReference {
            name: "Missing",
            path: "/tmp/does-not-exist.bas",
        };

        let error = analyzer
            .analyze_module_reference(&module_reference)
            .expect_err("Missing files should fail analysis");

        assert!(matches!(
            error,
            crate::error::SemanticError::FileReadError { .. }
        ));
    }

    #[test]
    fn analyze_module_reference_sets_current_file_for_valid_module() {
        let temp_dir = tempdir().expect("Temporary directory should be created");
        let module_path = temp_dir.path().join("Module1.bas");
        fs::write(
            &module_path,
            "Attribute VB_Name = \"Module1\"\nOption Explicit\n",
        )
        .unwrap();

        let mut analyzer = SemanticAnalyzer::new();
        let module_reference = vb6parse::files::project::ProjectModuleReference {
            name: "Module1",
            path: module_path
                .to_str()
                .expect("Module path should be valid UTF-8"),
        };

        analyzer
            .analyze_module_reference(&module_reference)
            .expect("Valid module should be analyzed");

        assert_eq!(analyzer.current_file.as_deref(), Some("Module1"));
        assert_eq!(
            analyzer
                .scope_manager
                .get_scopes_by_kind(ScopeKind::Global)
                .len(),
            2
        );
    }
}
