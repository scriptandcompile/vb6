//! Error types for semantic analysis
//!
//! This module defines the `SemanticError` enum, which represents
//! various kinds of errors that can occur during semantic analysis
//! of VB6 code. Each variant includes relevant information about
//! the error, such as the symbol name, expected and found types,
//! and source location. The `SourceLocation` struct provides a
//! standardized way to represent the location of errors in the
//! source code.
//!
//! The `Result` type alias is defined for convenience, allowing functions
//! to return `Result<T, SemanticError>` without needing to specify the
//! error type each time. This module is essential for providing meaningful
//! error messages to users of the semantic analysis library, helping them
//! understand and fix issues in their VB6 code.
//!
//! # Examples
//!
//! ```rust
//! use vb6semantic::SemanticError;
//! use vb6semantic::SourceLocation;
//!
//! let error = SemanticError::UndefinedSymbol {
//!     name: "myVariable".to_string(),
//!     location: SourceLocation {
//!         file: "Module1.bas".to_string(),
//!         line: 10,
//!         column: 5,
//!     },
//! };
//! println!("{}", error);
//! ```

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents an error that can occur during semantic analysis of VB6 code
///
/// Each variant includes relevant information about the error, such as the symbol name,
/// expected and found types, and source location. This allows for detailed error messages
/// to help users understand and fix issues in their VB6 code.
#[derive(Error, Debug, Clone)]
pub enum SemanticError {
    /// Represents an undefined symbol error, where a symbol is referenced but not defined
    #[error("Undefined symbol: {name} at {location}")]
    UndefinedSymbol {
        /// Name of the undefined symbol
        name: String,
        /// Location where the undefined symbol is referenced
        location: SourceLocation,
    },

    /// Represents a duplicate symbol error, where a symbol is defined multiple times
    #[error(
        "Symbol already defined: {name} at {location}, previously defined at {previous_location}"
    )]
    DuplicateSymbol {
        /// Name of the duplicate symbol
        name: String,
        /// Location where the duplicate symbol is defined
        location: SourceLocation,
        /// Location where the symbol was previously defined
        previous_location: SourceLocation,
    },

    /// Represents a type mismatch error, where the expected and found types do not match
    #[error("Type mismatch: expected {expected}, found {found} at {location}")]
    TypeMismatch {
        /// Expected type
        expected: String,
        /// Found type
        found: String,
        /// Location where the type mismatch occurs
        location: SourceLocation,
    },

    /// Represents an invalid scope error, where a symbol is defined in an invalid scope
    #[error("Invalid scope: {message}")]
    InvalidScope {
        /// Message describing the invalid scope error
        message: String,
    },

    /// Represents an invalid type error, where a type is not valid in the given context
    #[error("Invalid type: {message} at {location}")]
    InvalidType {
        /// Message describing the invalid type error
        message: String,
        /// Location where the invalid type error occurs
        location: SourceLocation,
    },

    /// Represents a circular dependency error, where symbols depend on each other in a cycle
    #[error("Circular dependency detected: {message}")]
    CircularDependency {
        /// Message describing the circular dependency error
        message: String,
    },

    /// Represents an invalid operation error, where an operation is not valid for the given types
    #[error("Invalid operation: {message} at {location}")]
    InvalidOperation {
        /// Message describing the invalid operation error
        message: String,
        /// Location where the invalid operation occurs
        location: SourceLocation,
    },

    /// Represents an inaccessible symbol error, where a symbol is not accessible due to its visibility
    #[error("Inaccessible symbol: {name} is {visibility} at {location}")]
    InaccessibleSymbol {
        /// Name of the inaccessible symbol
        name: String,
        /// Visibility of the inaccessible symbol (Public, Private, Friend)
        visibility: String,
        /// Location where the inaccessible symbol is referenced
        location: SourceLocation,
    },

    /// Represents an invalid assignment error, where an assignment is not valid due to type mismatch or other issues
    #[error("Invalid assignment: {message} at {location}")]
    InvalidAssignment {
        /// Message describing the invalid assignment error
        message: String,
        /// Location where the invalid assignment occurs
        location: SourceLocation,
    },

    /// Represents a parameter mismatch error, where the provided parameters do not match the expected ones
    #[error("Parameter mismatch: {message} at {location}")]
    ParameterMismatch {
        /// Message describing the parameter mismatch error
        message: String,
        /// Location where the parameter mismatch occurs
        location: SourceLocation,
    },

    /// Represents a general analysis error that does not fit into other categories
    #[error("Analysis error: {0}")]
    AnalysisError(String),
}

/// Represents a location in the source code, including file name, line number, and column number
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Name of the source file
    pub file: String,
    /// Line number in the source file (1-based)
    pub line: usize,
    /// Column number in the source file (1-based)
    pub column: usize,
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// Type alias for results returned by semantic analysis functions, using `SemanticError` as the error type
pub type Result<T> = std::result::Result<T, SemanticError>;
