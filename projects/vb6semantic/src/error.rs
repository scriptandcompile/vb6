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
use vb6parse::errors::ErrorDetails;

/// Represents an error that can occur during semantic analysis of VB6 code
///
/// Each variant includes relevant information about the error, such as the symbol name,
/// expected and found types, and source location. This allows for detailed error messages
/// to help users understand and fix issues in their VB6 code.
#[derive(Error, Debug, Clone)]
pub enum SemanticError {
    /// Represents an undefined symbol error, where a symbol is referenced but not defined
    UndefinedSymbol {
        /// Name of the undefined symbol
        name: String,
        /// Location where the undefined symbol is referenced
        location: SourceLocation,
    },

    /// Represents a duplicate symbol error, where a symbol is defined multiple times
    DuplicateSymbol {
        /// Name of the duplicate symbol
        name: String,
        /// Location where the duplicate symbol is defined
        location: SourceLocation,
        /// Location where the symbol was previously defined
        previous_location: SourceLocation,
    },

    /// Represents a type mismatch error, where the expected and found types do not match
    TypeMismatch {
        /// Expected type
        expected: String,
        /// Found type
        found: String,
        /// Location where the type mismatch occurs
        location: SourceLocation,
    },

    /// Represents an invalid scope error, where a symbol is defined in an invalid scope
    InvalidScope {
        /// Message describing the invalid scope error
        message: String,
    },

    /// Represents an invalid type error, where a type is not valid in the given context
    InvalidType {
        /// Message describing the invalid type error
        message: String,
        /// Location where the invalid type error occurs
        location: SourceLocation,
    },

    /// Represents a circular dependency error, where symbols depend on each other in a cycle
    CircularDependency {
        /// Message describing the circular dependency error
        message: String,
    },

    /// Represents an invalid operation error, where an operation is not valid for the given types
    InvalidOperation {
        /// Message describing the invalid operation error
        message: String,
        /// Location where the invalid operation occurs
        location: SourceLocation,
    },

    /// Represents an inaccessible symbol error, where a symbol is not accessible due to its visibility
    InaccessibleSymbol {
        /// Name of the inaccessible symbol
        name: String,
        /// Visibility of the inaccessible symbol (Public, Private, Friend)
        visibility: String,
        /// Location where the inaccessible symbol is referenced
        location: SourceLocation,
    },

    /// Represents an invalid assignment error, where an assignment is not valid due to type mismatch or other issues
    InvalidAssignment {
        /// Message describing the invalid assignment error
        message: String,
        /// Location where the invalid assignment occurs
        location: SourceLocation,
    },

    /// Represents a parameter mismatch error, where the provided parameters do not match the expected ones
    ParameterMismatch {
        /// Message describing the parameter mismatch error
        message: String,
        /// Location where the parameter mismatch occurs
        location: SourceLocation,
    },

    /// Represents a file read error, where a source file could not be read from disk
    FileReadError {
        /// Path of the file that could not be read
        file: String,
        /// Read error message
        message: String,
    },

    /// Represents a parse error for a source file that could not be parsed successfully
    FileParseError {
        /// Path of the file that could not be parsed
        file: String,
        /// Underlying parser diagnostics
        diagnostics: Vec<ErrorDetails<'static>>,
    },

    /// Represents a general analysis error that does not fit into other categories
    AnalysisError(String),
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::UndefinedSymbol { name, location } => {
                write!(f, "Undefined symbol: {name} at {location}")
            }
            SemanticError::DuplicateSymbol {
                name,
                location,
                previous_location,
            } => write!(
                f,
                "Symbol already defined: {name} at {location}, previously defined at {previous_location}"
            ),
            SemanticError::TypeMismatch {
                expected,
                found,
                location,
            } => write!(
                f,
                "Type mismatch: expected {expected}, found {found} at {location}"
            ),
            SemanticError::InvalidScope { message } => write!(f, "Invalid scope: {message}"),
            SemanticError::InvalidType { message, location } => {
                write!(f, "Invalid type: {message} at {location}")
            }
            SemanticError::CircularDependency { message } => {
                write!(f, "Circular dependency detected: {message}")
            }
            SemanticError::InvalidOperation { message, location } => {
                write!(f, "Invalid operation: {message} at {location}")
            }
            SemanticError::InaccessibleSymbol {
                name,
                visibility,
                location,
            } => write!(
                f,
                "Inaccessible symbol: {name} is {visibility} at {location}"
            ),
            SemanticError::InvalidAssignment { message, location } => {
                write!(f, "Invalid assignment: {message} at {location}")
            }
            SemanticError::ParameterMismatch { message, location } => {
                write!(f, "Parameter mismatch: {message} at {location}")
            }
            SemanticError::FileReadError { file, message } => {
                write!(f, "Failed to read file {file}: {message}")
            }
            SemanticError::FileParseError { file, diagnostics } => {
                write!(f, "Failed to parse file {file}")?;
                for diagnostic in diagnostics {
                    match diagnostic.print_to_string() {
                        Ok(text) => write!(f, "\n{text}")?,
                        Err(_) => write!(f, "\n{diagnostic:?}")?,
                    }
                }
                Ok(())
            }
            SemanticError::AnalysisError(message) => write!(f, "Analysis error: {message}"),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_read_error_formats_file_and_source() {
        let error = SemanticError::FileReadError {
            file: "module.bas".to_string(),
            message: "permission denied".to_string(),
        };

        let message = error.to_string();
        assert!(message.contains("module.bas"));
        assert!(message.contains("permission denied"));
    }

    #[test]
    fn file_parse_error_pretty_prints_source_diagnostics() {
        let error = SemanticError::FileParseError {
            file: "module.bas".to_string(),
            diagnostics: vec![ErrorDetails {
                source_name: "module.bas".to_string().into_boxed_str(),
                source_content: "Dim x As ?",
                error_offset: 8,
                line_start: 1,
                line_end: 1,
                kind: Box::new(vb6parse::errors::ErrorKind::Lexer(
                    vb6parse::errors::LexerError::UnknownToken {
                        token: "?".to_string(),
                    },
                )),
                severity: vb6parse::errors::Severity::Error,
                labels: vec![],
                notes: vec![],
            }],
        };

        let message = error.to_string();
        assert!(message.contains("module.bas"));
        assert!(message.contains("error here"));
        assert!(
            !message.contains("ErrorDetails {"),
            "diagnostics should be rendered with the source display, not Debug"
        );
    }
}
