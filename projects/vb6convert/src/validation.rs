/// Validation utilities for converted code
///
/// This module provides validation functionality to ensure converted code
/// maintains semantic equivalence with the original VB6 code where possible.
use crate::error::Result;
use crate::types::*;

/// Validator for converted code
pub struct ConversionValidator {
    #[allow(dead_code)]
    strict_mode: bool,
}

impl ConversionValidator {
    /// Create a new [`ConversionValidator`].
    ///
    /// * `strict_mode` - when `true`, validation errors are treated as hard failures.
    pub fn new(strict_mode: bool) -> Self {
        Self { strict_mode }
    }

    /// Validate a conversion result
    pub fn validate(&self, _result: &ConversionResult) -> Result<ValidationReport> {
        // TODO: Implement validation
        todo!("Validation not yet implemented")
    }
}

impl Default for ConversionValidator {
    fn default() -> Self {
        Self::new(false)
    }
}

/// Report from validation.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether all checks passed.
    pub passed: bool,
    /// Errors found during validation.
    pub errors: Vec<ValidationError>,
    /// Non-fatal warnings.
    pub warnings: Vec<String>,
}

/// A single validation error.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Human-readable error description.
    pub message: String,
    /// Location in the source where the error occurred.
    pub location: Option<SourceLocation>,
}
