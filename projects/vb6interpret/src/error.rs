//! Interpreter runtime errors.
//!
//! An interpretation error carries the underlying VB6 error (mirroring the
//! `Err` object) together with optional source location context.

use std::fmt;

use vb6core::error::VBError;

/// An error raised during interpretation of VB6 code.
#[derive(Debug, Clone)]
pub struct RunError {
    /// The underlying VB6 error (`Err.Number` / `Err.Description`).
    pub error: VBError,
    /// The 1-based source line where the error occurred, when known.
    pub line: Option<usize>,
    /// The name of the procedure that was executing, when known.
    pub procedure: Option<String>,
}

impl RunError {
    /// Create an error without source context.
    pub fn new(error: VBError) -> Self {
        Self {
            error,
            line: None,
            procedure: None,
        }
    }

    /// Attach the 1-based source line.
    pub fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Attach the executing procedure name.
    pub fn in_procedure(mut self, name: &str) -> Self {
        self.procedure = Some(name.to_string());
        self
    }

    /// Build from a VB6 error number.
    pub fn err_number(number: i32) -> Self {
        Self::new(VBError::new(number))
    }

    /// Error 5: Invalid procedure call or argument.
    pub fn invalid_procedure_call() -> Self {
        Self::new(VBError::invalid_procedure_call())
    }

    /// Error 13: Type mismatch.
    pub fn type_mismatch() -> Self {
        Self::new(VBError::type_mismatch())
    }

    /// Error 35: Sub or Function not defined.
    pub fn sub_or_function_not_defined() -> Self {
        Self::new(VBError::new(35))
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(line) = self.line {
            write!(f, "line {line}: {}", self.error)
        } else {
            write!(f, "{}", self.error)
        }
    }
}

impl std::error::Error for RunError {}

impl From<VBError> for RunError {
    fn from(error: VBError) -> Self {
        Self::new(error)
    }
}

impl From<RunError> for VBError {
    fn from(run_error: RunError) -> Self {
        run_error.error
    }
}

/// Convenience alias for interpreter results.
pub type RunResult<T> = Result<T, RunError>;
