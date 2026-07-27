//! Name resolution logic for resolving symbol references in the VB6 semantic analysis
//! This module contains the `NameResolver` struct, which is responsible for resolving symbol references
//! within the current scope context. It provides methods to resolve simple names and qualified names, as well
//! as checking symbol accessibility. The `NameResolver` interacts closely with the `ScopeManager` to perform
//! lookups and determine accessibility based on symbol visibility and scope hierarchy.
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
//! };
//!
//! let project = project_opt.expect("Project file should have parsed successfully");
//! let analysis_result = analyzer.analyze_project(&project).expect("Failed to analyze project");
//! println!("Analysis completed with {} errors and {} warnings", analysis_result.errors.len(), analysis_result.warnings.len());
//! ```

use crate::error::{Result, SourceLocation};
use crate::scope::ScopeManager;
use crate::symbols::Symbol;

/// Name resolver for resolving symbol references
pub struct NameResolver {
    /// Scope manager for lookups
    scope_manager: ScopeManager,
}

impl NameResolver {
    /// Create a new name resolver with the given scope manager
    pub fn new(scope_manager: ScopeManager) -> Self {
        Self { scope_manager }
    }

    /// Resolve a simple name to a symbol
    pub fn resolve_name(&self, name: &str, location: &SourceLocation) -> Result<&Symbol> {
        self.scope_manager.lookup(name).ok_or_else(|| {
            crate::error::SemanticError::UndefinedSymbol {
                name: name.to_string(),
                location: location.clone(),
            }
        })
    }

    /// Resolve a qualified name (e.g., Module.Function)
    pub fn resolve_qualified_name(
        &self,
        parts: &[String],
        location: &SourceLocation,
    ) -> Result<&Symbol> {
        if parts.is_empty() {
            return Err(crate::error::SemanticError::AnalysisError(
                "Empty qualified name".to_string(),
            ));
        }

        if parts.len() == 1 {
            return self.resolve_name(&parts[0], location);
        }

        // First resolve the first part (module/class name)
        let _container = self.resolve_name(&parts[0], location)?;

        // Then look for the member in that container's scope
        // TODO: Implement member lookup in container scope

        Err(crate::error::SemanticError::AnalysisError(format!(
            "Qualified name resolution not yet fully implemented: {}",
            parts.join(".")
        )))
    }

    /// Check if a symbol is accessible from the current scope
    pub fn is_accessible(&self, symbol: &Symbol) -> bool {
        self.scope_manager.can_access(symbol)
    }

    /// Get the scope manager
    pub fn scope_manager(&self) -> &ScopeManager {
        &self.scope_manager
    }
}
