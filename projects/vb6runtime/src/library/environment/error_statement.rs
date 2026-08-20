//! VB6 Error statement syntax:
//! - Error errornumber
//!
//! Generates a run-time error; can be used instead of the Err.Raise method.
//!
//! The Error statement syntax has this part:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | errornumber   | Required. Any valid error number. |
//!
//! Remarks:
//! - The Error statement is supported for backward compatibility.
//! - In new code, use the Err object's Raise method to generate run-time errors.
//! - If errornumber is defined, the Error statement calls the error handler after the properties
//!   of the Err object are assigned the following default values:
//!   * Err.Number: The value specified as the argument to the Error statement
//!   * Err.Source: The name of the current Visual Basic project
//!   * Err.Description: String expression corresponding to the return value of the Error function
//!     for the specified Number, if this string exists
//!
//! Examples:
//! ```vb
//! Error 11  ' Generate "Division by zero" error
//! Error 53  ' Generate "File not found" error
//! Error vbObjectError + 1000  ' Generate custom error
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/error-statement)

use crate::error::{VBError, VBResult};
use crate::state::err as err_state;
use crate::value::VBVariant;

/// Raises a run-time error with the given error number.
///
/// The `Error` statement is the statement-form counterpart of `Err.Raise`:
/// it populates the `Err` object properties and then propagates the error
/// to the active error handler.
///
/// - **Err.Number** is set to `errornumber`.
/// - **Err.Description** is set to the built-in description for that number
///   (or the generic `"Application-defined or object-defined error"` for
///   unknown numbers).
/// - The function then returns `Err(VBError)` so the interpreter can route
///   the error to the active `On Error` handler.
///
/// Returns error 13 (type mismatch) for a value that does not convert to a
/// number.
pub fn error_statement(arg: &VBVariant) -> VBResult<()> {
    let number = arg.as_i32()?;
    err_state::set_number(number);
    Err(VBError::new(number))
}
