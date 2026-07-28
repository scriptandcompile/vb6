//! Parser-level error types.
//!
//! This module defines errors specific to the CST parsing phase,
//! representing unexpected or missing tokens discovered during parsing.

use crate::language::Token;

/// Errors that occur during CST-level parsing.
///
/// These represent structural issues in the token stream that the parser
/// cannot recover from locally, such as unexpected tokens or missing
/// required tokens.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    /// One or more unexpected tokens were encountered where a specific
    /// construct was expected.
    #[error("Unexpected token(s): expected {expected:?}, found {found:?}")]
    UnexpectedTokens {
        /// Descriptions of what was expected (e.g. `"end of statement"`, `"expression"`, `"Then"`).
        expected: Vec<String>,
        /// The tokens consumed during error recovery instead of the expected construct.
        found: Vec<Token>,
    },

    /// A required token is missing from the input.
    #[error("Missing token: expected `{expected}`, found `{found:?}`")]
    MissingToken {
        /// Description of what was expected.
        expected: String,
        /// The token present instead, if any.
        found: Option<Token>,
    },
}
