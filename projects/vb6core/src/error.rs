//! VB6 runtime error handling.
//!
//! This module mirrors the VB6 `Err` object: every error carries a `number`, a
//! human readable `description`, and optional source/help metadata. Runtime
//! operations that can fail return a [`VBResult`].

use std::fmt;

/// Standard VB6 runtime error numbers.
pub mod err_number {
    /// Invalid procedure call or argument.
    pub const INVALID_PROCEDURE_CALL: i32 = 5;
    /// Overflow.
    pub const OVERFLOW: i32 = 6;
    /// Out of memory.
    pub const OUT_OF_MEMORY: i32 = 7;
    /// Subscript out of range.
    pub const SUBSCRIPT_OUT_OF_RANGE: i32 = 9;
    /// Division by zero.
    pub const DIVISION_BY_ZERO: i32 = 11;
    /// Type mismatch.
    pub const TYPE_MISMATCH: i32 = 13;
    /// Out of string space.
    pub const OUT_OF_STRING_SPACE: i32 = 14;
    /// Object variable or With block variable not set.
    pub const OBJECT_VARIABLE_NOT_SET: i32 = 91;
    /// Invalid use of Null.
    pub const INVALID_USE_OF_NULL: i32 = 94;
    /// Object required.
    pub const OBJECT_REQUIRED: i32 = 424;
    /// Object doesn't support this property or method.
    pub const OBJECT_DOESNT_SUPPORT_PROPERTY_OR_METHOD: i32 = 438;
    /// Wrong number of arguments or invalid property assignment.
    pub const WRONG_NUMBER_OF_ARGUMENTS: i32 = 450;
}

/// The built-in description for a given VB6 error number.
pub fn default_description(number: i32) -> String {
    match number {
        5 => "Invalid procedure call or argument",
        6 => "Overflow",
        7 => "Out of memory",
        9 => "Subscript out of range",
        10 => "This array is fixed or temporarily locked",
        11 => "Division by zero",
        13 => "Type mismatch",
        14 => "Out of string space",
        18 => "User interrupt occurred",
        20 => "Resume without error",
        28 => "Out of stack space",
        35 => "Sub or Function not defined",
        48 => "Error in loading DLL",
        52 => "Bad file name or number",
        53 => "File not found",
        54 => "Bad file mode",
        55 => "File already open",
        57 => "Device I/O error",
        61 => "Disk full",
        62 => "Input past end of file",
        63 => "Bad record number",
        67 => "Too many files",
        68 => "Device unavailable",
        70 => "Permission denied",
        71 => "Disk not ready",
        75 => "Path/File access error",
        76 => "Path not found",
        91 => "Object variable or With block variable not set",
        94 => "Invalid use of Null",
        424 => "Object required",
        438 => "Object doesn't support this property or method",
        449 => "Argument not optional",
        450 => "Wrong number of arguments",
        451 => {
            "Property Let procedure not defined and Property Get procedure did not return an object"
        }
        462 => "The remote server machine does not exist or is unavailable",
        _ => "Application-defined or object-defined error",
    }
    .to_string()
}

/// A VB6 runtime error, mirroring the `Err` object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VBError {
    /// The error number (`Err.Number`).
    pub number: i32,
    /// Human readable description (`Err.Description`).
    pub description: String,
    /// The object or application that generated the error (`Err.Source`).
    pub source: String,
    /// The Help file (`Err.HelpFile`).
    pub help_file: String,
    /// The context ID within the Help file (`Err.HelpContext`).
    pub help_context: i32,
}

/// Convenience result type for runtime operations.
pub type VBResult<T> = Result<T, VBError>;

impl VBError {
    /// Create an error with the built-in description for `number`.
    pub fn new(number: i32) -> Self {
        let description = default_description(number);
        Self {
            number,
            description,
            source: String::new(),
            help_file: String::new(),
            help_context: 0,
        }
    }

    /// Create an error with an explicit description.
    pub fn with_description(number: i32, description: impl Into<String>) -> Self {
        Self {
            number,
            description: description.into(),
            source: String::new(),
            help_file: String::new(),
            help_context: 0,
        }
    }

    /// Create an error using the full `Err.Raise` signature.
    #[allow(clippy::too_many_arguments)]
    pub fn raise(
        number: i32,
        source: impl Into<String>,
        description: impl Into<String>,
        help_file: impl Into<String>,
        help_context: i32,
    ) -> Self {
        Self {
            number,
            description: description.into(),
            source: source.into(),
            help_file: help_file.into(),
            help_context,
        }
    }

    /// Error 5: Invalid procedure call or argument.
    pub fn invalid_procedure_call() -> Self {
        Self::new(err_number::INVALID_PROCEDURE_CALL)
    }

    /// Error 6: Overflow.
    pub fn overflow() -> Self {
        Self::new(err_number::OVERFLOW)
    }

    /// Error 7: Out of memory.
    pub fn out_of_memory() -> Self {
        Self::new(err_number::OUT_OF_MEMORY)
    }

    /// Error 9: Subscript out of range.
    pub fn subscript_out_of_range() -> Self {
        Self::new(err_number::SUBSCRIPT_OUT_OF_RANGE)
    }

    /// Error 11: Division by zero.
    pub fn division_by_zero() -> Self {
        Self::new(err_number::DIVISION_BY_ZERO)
    }

    /// Error 13: Type mismatch.
    pub fn type_mismatch() -> Self {
        Self::new(err_number::TYPE_MISMATCH)
    }

    /// Error 91: Object variable or With block variable not set.
    pub fn object_not_set() -> Self {
        Self::new(err_number::OBJECT_VARIABLE_NOT_SET)
    }

    /// Error 94: Invalid use of Null.
    pub fn invalid_use_of_null() -> Self {
        Self::new(err_number::INVALID_USE_OF_NULL)
    }

    /// Error 424: Object required.
    pub fn object_required() -> Self {
        Self::new(err_number::OBJECT_REQUIRED)
    }
}

impl fmt::Display for VBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.description.is_empty() {
            write!(f, "Error {}", self.number)
        } else {
            write!(f, "{} (Error {})", self.description, self.number)
        }
    }
}

impl std::error::Error for VBError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_descriptions_are_known() {
        assert_eq!(default_description(5), "Invalid procedure call or argument");
        assert_eq!(default_description(13), "Type mismatch");
        assert_eq!(default_description(94), "Invalid use of Null");
        assert_eq!(
            default_description(999),
            "Application-defined or object-defined error"
        );
    }

    #[test]
    fn constructors_use_builtin_descriptions() {
        let err = VBError::type_mismatch();
        assert_eq!(err.number, 13);
        assert_eq!(err.description, "Type mismatch");
    }

    #[test]
    fn raise_builds_full_error() {
        let err = VBError::raise(13, "proj", "custom", "help.hlp", 42);
        assert_eq!(err.number, 13);
        assert_eq!(err.source, "proj");
        assert_eq!(err.help_context, 42);
    }

    #[test]
    fn display_includes_description_and_number() {
        assert_eq!(VBError::overflow().to_string(), "Overflow (Error 6)");
    }
}
