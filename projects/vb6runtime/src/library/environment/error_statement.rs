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
