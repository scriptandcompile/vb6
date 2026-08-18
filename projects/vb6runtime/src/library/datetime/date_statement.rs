//! # `Date` Statement
//!
//! Sets the current system date.
//!
//! ## Syntax
//!
//! ```vb
//! Date = dateexpression
//! ```
//!
//! ## Parts
//!
//! - **dateexpression**: Required. Any expression that can represent a date,
//!   including date literals (`#1/1/2025#`), date serial values, or string
//!   expressions that parse as dates.
//!
//! ## Return Value
//!
//! None. This is a statement, not a function.
//!
//! ## Remarks
//!
//! The `Date` statement sets the system date to the value of `dateexpression`.
//! In VB6 on Windows this modifies the real system clock and requires
//! administrator privileges.  In this runtime the behavior depends on the
//! [`allow_system_clock`](crate::state::clock) flag:
//!
//! - **Default (system clock allowed)**: The date is written to the real
//!   system clock via platform APIs.  Subsequent `Date` and `Date$` calls
//!   return the real system date.
//! - **Internal-only mode**: The date is stored in an offset-based mock
//!   clock that advances in real time from the set point.  The real system
//!   clock is never modified.
//!
//! - **Date Storage**: Internally, dates are stored as `Double` values
//!   representing the number of days since December 30, 1899.
//! - **Date Range**: January 1, 100 through December 31, 9999.
//! - **Permissions**: On real VB6, changing the system date requires
//!   administrator privileges.
//! - **Date Function**: Use the `Date` function (without assignment) to
//!   retrieve the current system date.
//! - **Date$ Function**: Use `Date$` to retrieve the current date as a string.
//! - **Error Handling**: Invalid date values generate a runtime error (error 13,
//!   type mismatch).
//!
//! ## Examples
//!
//! ### Set Date to Specific Value
//!
//! ```vb
//! Date = #1/1/2025#
//! ```
//!
//! ### Set Date Using Variable
//!
//! ```vb
//! Dim newDate As Date
//! newDate = #6/15/2025#
//! Date = newDate
//! ```
//!
//! ### Set Date Using DateSerial
//!
//! ```vb
//! Date = DateSerial(2025, 12, 25)
//! ```
//!
//! ### Set Date Using String
//!
//! ```vb
//! Date = "January 1, 2025"
//! ```
//!
//! ### Set Date Using DateAdd
//!
//! ```vb
//! Date = DateAdd("d", 30, Date)
//! ```
//!
//! ### Set Date from User Input
//!
//! ```vb
//! Dim userDate As String
//! userDate = InputBox("Enter new date (MM/DD/YYYY):")
//! If IsDate(userDate) Then
//!     Date = userDate
//! Else
//!     MsgBox "Invalid date format"
//! End If
//! ```
//!
//! ### Set Date with Error Handling
//!
//! ```vb
//! On Error Resume Next
//! Date = #1/1/2025#
//! If Err.Number <> 0 Then
//!     MsgBox "Failed to set date: " & Err.Description
//! End If
//! On Error GoTo 0
//! ```
//!
//! ### Reset Date to Current System Date
//!
//! ```vb
//! ' Clear any override by setting to today's real date
//! Date = Date
//! ```
//!
//! ## Common Errors
//!
//! - **Error 13**: Type mismatch - occurs when the expression cannot be
//!   converted to a date.
//! - **Error 5**: Invalid procedure call or argument - occurs with invalid
//!   date values (e.g., February 30).
//!
//! ## Best Practices
//!
//! - Validate dates before assignment using `IsDate()`.
//! - Use error handling when setting dates from user input.
//! - Prefer `DateSerial` for programmatic date construction.
//! - Be aware that changing the system date affects all applications.
//!
//! ## See Also
//!
//! - `Date` function (retrieve current system date)
//! - `Date$` function (retrieve current date as string)
//! - `Time` statement (set system time)
//! - `DateSerial` function (create date from components)
//! - `DateValue` function (convert string to date)
//! - `Now` function (get current date and time)
//!
//! ## References
//!
//! - [Date Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/date-statement)

use crate::error::VBResult;
use crate::value::VBVariant;

/// Implementation of the `Date` statement.
///
/// VB6 behavior:
/// - sets the system date to the value of the given expression
/// - in this runtime, when the system clock is allowed, writes to the real
///   clock; otherwise stores in the offset-based mock clock
/// - raises error 13 (type mismatch) if the value cannot be converted to a date
pub fn date_statement(value: &VBVariant) -> VBResult<()> {
    let serial = value.as_date_serial()?;
    use jiff::civil::Date;

    let base = Date::new(1899, 12, 30).expect("valid epoch");
    let days = serial.floor();
    let date = base
        .checked_add(jiff::SignedDuration::from_secs((days * 86400.0) as i64))
        .map_err(|_| crate::error::VBError::type_mismatch())?;

    crate::state::clock::set_date(date);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::date_statement;
    use crate::state::test_support::TEST_LOCK;
    use crate::VBVariant;

    #[test]
    fn sets_date_from_literal() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        let input = VBVariant::from_date_serial(45292.0); // 2024-01-01
        date_statement(&input).unwrap();
        let result = crate::library::datetime::date::date().unwrap();
        let serial = result.as_date_serial().unwrap();
        assert_eq!(serial.floor() as i32, 45292);
        crate::state::clock::reset();
    }

    #[test]
    fn sets_date_from_variant() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        let input = VBVariant::Date(45658.0); // 2025-01-01
        date_statement(&input).unwrap();
        let result = crate::library::datetime::date::date().unwrap();
        let serial = result.as_date_serial().unwrap();
        assert_eq!(serial.floor() as i32, 45658);
        crate::state::clock::reset();
    }

    #[test]
    fn sets_date_from_string() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        let input = VBVariant::from_string("1/15/2025");
        date_statement(&input).unwrap();
        let result = crate::library::datetime::date::date().unwrap();
        assert_eq!(result.var_type(), 7); // vbDate
        crate::state::clock::reset();
    }

    #[test]
    fn reset_clears_override() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        let input = VBVariant::from_date_serial(45292.0);
        date_statement(&input).unwrap();
        crate::state::clock::reset();
        let result = crate::library::datetime::date::date().unwrap();
        let serial = result.as_date_serial().unwrap();
        assert_ne!(serial.floor() as i32, 45292);
    }

    #[test]
    fn rejects_null() {
        let input = VBVariant::Null;
        let result = date_statement(&input);
        assert!(result.is_err());
    }
}
