//! # Time Statement
//!
//! Sets the system time.
//!
//! ## Syntax
//!
//! ```vb
//! Time = time
//! ```
//!
//! ## Parts
//!
//! - **time**: Required. Any numeric expression, string expression, or any
//!   combination that can represent a time.
//!
//! ## Return Value
//!
//! None. This is a statement, not a function.
//!
//! ## Remarks
//!
//! The `Time` statement sets the system time to the value of `time`.  In VB6
//! on Windows this modifies the real system clock and requires administrator
//! privileges.  In this runtime the behavior depends on the
//! [`allow_system_clock`](crate::state::clock) flag:
//!
//! - **Default (system clock allowed)**: The time is written to the real
//!   system clock via platform APIs.  Subsequent `Time` and `Time$` calls
//!   return the real system time.
//! - **Internal-only mode**: The time is stored in an offset-based mock
//!   clock that advances in real time from the set point.  The real system
//!   clock is never modified.
//!
//! - **Time Format**: Accepts times in various formats including "HH:MM:SS",
//!   "HH:MM", or numeric values representing time.
//! - **24-Hour Format**: You can use 24-hour format (e.g., "13:30" for 1:30 PM)
//!   or 12-hour format with AM/PM.
//! - **Current Date Preserved**: Setting the time does not affect the system
//!   date.
//! - **Time Function**: Use the Time function (without assignment) to retrieve
//!   the current system time.
//! - **Error Handling**: Invalid time values will generate a run-time error.
//!
//! ## Examples
//!
//! ### Set Time to Specific Hour and Minute
//!
//! ```vb
//! Time = "14:30:00"  ' Set to 2:30 PM
//! ```
//!
//! ### Set Time Using String
//!
//! ```vb
//! Time = "9:15 AM"
//! ```
//!
//! ### Set Time to Midnight
//!
//! ```vb
//! Time = "00:00:00"
//! ```
//!
//! ### Set Time Using Variable
//!
//! ```vb
//! Dim newTime As String
//! newTime = "15:45:30"
//! Time = newTime
//! ```
//!
//! ### Set Time Using `TimeValue` Function
//!
//! ```vb
//! Time = TimeValue("3:30 PM")
//! ```
//!
//! ### Set Time with Error Handling
//!
//! ```vb
//! On Error Resume Next
//! Time = "10:30:00"
//! If Err.Number <> 0 Then
//!     MsgBox "Failed to set time: " & Err.Description
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Common Errors
//!
//! - **Error 5**: Invalid procedure call or argument - occurs with invalid
//!   time format.
//! - **Error 13**: Type mismatch - occurs with incompatible data types.
//!
//! ## Best Practices
//!
//! - Always use error handling when setting system time.
//! - Validate time strings before assignment using `IsDate()`.
//! - Use `TimeSerial` or `TimeValue` for programmatic time construction.
//!
//! ## See Also
//!
//! - `Time` function (retrieve current system time)
//! - `Date` statement (set system date)
//! - `Now` function (get current date and time)
//! - `TimeSerial` function (create time from components)
//! - `TimeValue` function (convert string to time)
//!
//! ## References
//!
//! - [Time Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/time-statement)

use crate::error::VBResult;
use crate::value::VBVariant;

/// Parse a time-only string (`H:MM[:SS]` or `HH:MM[:SS]`) as a fraction
/// of a day (0.0–1.0).  Returns `None` for invalid formats.
fn parse_time_string(s: &str) -> Option<f64> {
    let s = s.trim();
    // Try "HH:MM:SS" or "H:MM:SS" or "HH:MM" or "H:MM"
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return None;
    }
    let hours: i32 = parts[0].parse().ok()?;
    let minutes: i32 = parts[1].parse().ok()?;
    let seconds: i32 = if parts.len() == 3 {
        parts[2].parse().ok()?
    } else {
        0
    };
    if !(0..=23).contains(&hours) || !(0..=59).contains(&minutes) || !(0..=59).contains(&seconds) {
        return None;
    }
    let total = hours * 3600 + minutes * 60 + seconds;
    Some(total as f64 / 86_400.0)
}

/// Implementation of the `Time` statement.
///
/// VB6 behavior:
/// - sets the system time to the value of the given expression
/// - in this runtime, when the system clock is allowed, writes to the real
///   clock; otherwise stores in the offset-based mock clock
/// - raises error 13 (type mismatch) if the value cannot be converted to a time
pub fn time_statement(value: &VBVariant) -> VBResult<()> {
    let serial = match value {
        VBVariant::String(s) => {
            // First try as a full date/time string; fall back to time-only.
            if let Some(v) = crate::value::parse_vb_date(s) {
                v
            } else if let Some(v) = parse_time_string(s) {
                v
            } else {
                return Err(crate::error::VBError::type_mismatch());
            }
        }
        _ => value.as_date_serial()?,
    };

    // Extract the time portion (fractional part) and convert to hours/minutes/seconds.
    let fraction = serial.fract();
    let total_seconds = (fraction * 86_400.0).round() as i64;
    let hours = (total_seconds / 3600) as i8;
    let minutes = ((total_seconds % 3600) / 60) as i8;
    let seconds = (total_seconds % 60) as i8;

    let time = jiff::civil::Time::new(hours, minutes, seconds, 0)
        .map_err(|_| crate::error::VBError::type_mismatch())?;

    crate::state::clock::set_time(time);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::time_statement;
    use crate::state::test_support::TEST_LOCK;
    use crate::VBVariant;

    #[test]
    fn sets_time_from_string() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        let input = VBVariant::from_string("14:30:00");
        time_statement(&input).unwrap();
        let result = crate::library::datetime::time::time().unwrap();
        // The result should be approximately 14:30:00 = 0.604166... of a day
        let VBVariant::Date(serial) = result else {
            panic!("expected a Date variant");
        };
        let seconds = serial.fract() * 86_400.0;
        let expected = 14.0 * 3600.0 + 30.0 * 60.0;
        assert!(
            (seconds - expected).abs() < 1.0,
            "expected ~{expected}s, got {seconds}s"
        );
        crate::state::clock::reset();
    }

    #[test]
    fn sets_time_from_variant() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        // 0.5 = noon = 12:00:00
        let input = VBVariant::Date(0.5);
        time_statement(&input).unwrap();
        let result = crate::library::datetime::time::time().unwrap();
        let VBVariant::Date(serial) = result else {
            panic!("expected a Date variant");
        };
        let seconds = serial.fract() * 86_400.0;
        assert!(
            (seconds - 43200.0).abs() < 1.0,
            "expected ~43200s (noon), got {seconds}s"
        );
        crate::state::clock::reset();
    }

    #[test]
    fn rejects_null() {
        let _guard = TEST_LOCK.lock().unwrap();
        let input = VBVariant::Null;
        let result = time_statement(&input);
        assert!(result.is_err());
    }
}
