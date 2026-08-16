//! # `Time$` Function
//!
//! The `Time$` function in Visual Basic 6 returns a string representing the current system time.
//! The dollar sign (`$`) suffix indicates that this function always returns a `String` type,
//! never a `Variant`.
//!
//! ## Syntax
//!
//! ```vb6
//! Time$
//! ```
//!
//! ## Parameters
//!
//! None. `Time$` takes no parameters.
//!
//! ## Return Value
//!
//! Returns a `String` representing the current system time in the format "HH:MM:SS" (24-hour format).
//!
//! ## Behavior and Characteristics
//!
//! ### Time Format
//!
//! - Always returns 24-hour format (e.g., "14:30:45" for 2:30:45 PM)
//! - Format: "HH:MM:SS" where HH is 00-23, MM is 00-59, SS is 00-59
//! - Always includes leading zeros (e.g., "09:05:03")
//! - Does not include AM/PM indicator
//! - Does not include milliseconds or fractional seconds
//!
//! ### Type Differences: `Time$` vs `Time`
//!
//! - `Time$`: Always returns `String` type (never `Variant`)
//! - `Time`: Returns `Variant` containing a Date/Time value
//! - Use `Time$` when you need a string representation
//! - Use `Time` when you need to perform date/time arithmetic
//!
//! ### System Time
//!
//! - Reflects the current system clock time
//! - Updates each time the function is called
//! - Accuracy depends on system clock resolution
//! - No time zone information included
//!
//! ## Common Usage Patterns
//!
//! ### 1. Display Current Time
//!
//! ```vb6
//! Sub ShowTime()
//!     Debug.Print "Current time: " & Time$
//! End Sub
//! ```
//!
//! ### 2. Timestamp for Logging
//!
//! ```vb6
//! Sub LogMessage(message As String)
//!     Dim logEntry As String
//!     logEntry = Time$ & " - " & message
//!     Debug.Print logEntry
//! End Sub
//!
//! LogMessage "Application started"
//! ```
//!
//! ### 3. Time-Based File Naming
//!
//! ```vb6
//! Function GenerateTimeStampedFileName(baseName As String) As String
//!     Dim timeStamp As String
//!     timeStamp = Replace$(Time$, ":", "")
//!     GenerateTimeStampedFileName = baseName & "_" & timeStamp & ".log"
//! End Function
//!
//! ' Generates: "logfile_143045.log" at 2:30:45 PM
//! ```
//!
//! ### 4. Update Time Display
//!
//! ```vb6
//! Sub Timer1_Timer()
//!     ' Update label every second
//!     lblTime.Caption = Time$
//! End Sub
//! ```
//!
//! ### 5. Record Processing Time
//!
//! ```vb6
//! Sub ProcessData()
//!     Dim startTime As String
//!     startTime = Time$
//!     
//!     ' ... processing code ...
//!     
//!     Debug.Print "Started at: " & startTime
//!     Debug.Print "Completed at: " & Time$
//! End Sub
//! ```
//!
//! ### 6. Time-Based Greetings
//!
//! ```vb6
//! Function GetGreeting() As String
//!     Dim currentHour As Integer
//!     currentHour = Hour(Now)
//!     
//!     If currentHour < 12 Then
//!         GetGreeting = "Good morning! Time: " & Time$
//!     ElseIf currentHour < 18 Then
//!         GetGreeting = "Good afternoon! Time: " & Time$
//!     Else
//!         GetGreeting = "Good evening! Time: " & Time$
//!     End If
//! End Function
//! ```
//!
//! ### 7. Audit Trail Entries
//!
//! ```vb6
//! Sub RecordAudit(action As String, userName As String)
//!     Dim auditEntry As String
//!     auditEntry = Date$ & " " & Time$ & " - " & userName & " - " & action
//!     Print #1, auditEntry
//! End Sub
//! ```
//!
//! ### 8. Periodic Task Checking
//!
//! ```vb6
//! Sub CheckScheduledTask()
//!     Dim currentTimeStr As String
//!     currentTimeStr = Time$
//!     
//!     If currentTimeStr = "09:00:00" Then
//!         ' Execute morning task
//!         RunMorningReport
//!     End If
//! End Sub
//! ```
//!
//! ### 9. Status Bar Updates
//!
//! ```vb6
//! Sub UpdateStatusBar()
//!     StatusBar1.Panels(1).Text = "Current Time: " & Time$
//! End Sub
//! ```
//!
//! ### 10. Debug Output with Timestamps
//!
//! ```vb6
//! Sub DebugLog(category As String, message As String)
//!     Debug.Print "[" & Time$ & "] " & category & ": " & message
//! End Sub
//!
//! DebugLog "INFO", "Processing started"
//! ```
//!
//! ## Related Functions
//!
//! - `Time` - Returns a `Variant` containing the current system time (can be used in calculations)
//! - `Date$` - Returns a string representing the current system date
//! - `Now` - Returns the current system date and time as a `Date` value
//! - `Timer` - Returns the number of seconds elapsed since midnight
//! - `Hour()` - Extracts the hour component from a time value
//! - `Minute()` - Extracts the minute component from a time value
//! - `Second()` - Extracts the second component from a time value
//! - `Format$()` - Formats time values with custom formatting
//! - `TimeSerial()` - Creates a time value from hour, minute, and second
//! - `TimeValue()` - Converts a string to a time value
//!
//! ## Best Practices
//!
//! ### When to Use `Time$` vs `Time` vs `Now`
//!
//! ```vb6
//! ' Use Time$ for string display/logging
//! Debug.Print "Current time: " & Time$  ' "14:30:45"
//!
//! ' Use Time for time arithmetic
//! Dim currentTime As Date
//! currentTime = Time
//! Dim laterTime As Date
//! laterTime = currentTime + TimeSerial(1, 0, 0)  ' Add 1 hour
//!
//! ' Use Now for complete timestamp
//! Dim timestamp As Date
//! timestamp = Now  ' Includes both date and time
//! ```
//!
//! ### Combine with `Date$` for Full Timestamp
//!
//! ```vb6
//! Function GetFullTimestamp() As String
//!     GetFullTimestamp = Date$ & " " & Time$
//! End Function
//!
//! Debug.Print GetFullTimestamp()  ' "12/02/2025 14:30:45"
//! ```
//!
//! ### Use `Format$` for Custom Time Formatting
//!
//! ```vb6
//! ' Time$ always returns 24-hour format
//! Debug.Print Time$  ' "14:30:45"
//!
//! ' Use Format$ for 12-hour format or other formats
//! Debug.Print Format$(Now, "hh:mm:ss AM/PM")  ' "02:30:45 PM"
//! Debug.Print Format$(Now, "Long Time")       ' "2:30:45 PM"
//! ```
//!
//! ### Replace Colons for File Names
//!
//! ```vb6
//! Function SafeTimeStamp() As String
//!     ' Colons are invalid in filenames on Windows
//!     SafeTimeStamp = Replace$(Time$, ":", "")
//! End Function
//!
//! Dim fileName As String
//! fileName = "backup_" & SafeTimeStamp() & ".dat"  ' "backup_143045.dat"
//! ```
//!
//! ## Performance Considerations
//!
//! - `Time$` is a system call and relatively fast
//! - Calling repeatedly in tight loops may impact performance
//! - Cache the value if you need it multiple times in the same operation
//!
//! ```vb6
//! ' Less efficient: multiple calls
//! For i = 1 To 1000
//!     Debug.Print Time$ & " - Item " & i
//! Next i
//!
//! ' More efficient: cache the time
//! Dim currentTime As String
//! currentTime = Time$
//! For i = 1 To 1000
//!     Debug.Print currentTime & " - Item " & i
//! Next i
//! ```
//!
//! ## Common Pitfalls
//!
//! ### 1. 24-Hour Format Only
//!
//! ```vb6
//! ' Time$ always uses 24-hour format
//! Debug.Print Time$  ' "14:30:45" (not "2:30:45 PM")
//!
//! ' For 12-hour format, use Format$
//! Debug.Print Format$(Now, "hh:mm:ss AM/PM")  ' "02:30:45 PM"
//! ```
//!
//! ### 2. String Comparison Issues
//!
//! ```vb6
//! ' Comparing time strings can be tricky
//! If Time$ = "9:00:00" Then  ' Will NEVER match!
//!     ' Time$ returns "09:00:00" with leading zero
//! End If
//!
//! ' Correct: include leading zero
//! If Time$ = "09:00:00" Then
//!     ' This works
//! End If
//!
//! ' Better: use Time value for comparisons
//! If Time > TimeSerial(9, 0, 0) Then
//!     ' More reliable
//! End If
//! ```
//!
//! ### 3. Colons in File Names
//!
//! ```vb6
//! ' Invalid filename on Windows (colons not allowed)
//! fileName = "log_" & Time$ & ".txt"  ' "log_14:30:45.txt" - ERROR!
//!
//! ' Remove or replace colons
//! fileName = "log_" & Replace$(Time$, ":", "") & ".txt"  ' "log_143045.txt"
//! fileName = "log_" & Replace$(Time$, ":", "-") & ".txt"  ' "log_14-30-45.txt"
//! ```
//!
//! ### 4. Time Changes During Execution
//!
//! ```vb6
//! ' Problem: time can change between calls
//! Debug.Print "Start: " & Time$
//! ' ... long operation ...
//! Debug.Print "End: " & Time$  ' Different value!
//!
//! ' Solution: capture at start if consistency needed
//! Dim operationTime As String
//! operationTime = Time$
//! Debug.Print "Start: " & operationTime
//! ' ... long operation ...
//! Debug.Print "Started at: " & operationTime
//! Debug.Print "Completed at: " & Time$
//! ```
//!
//! ### 5. No Milliseconds
//!
//! ```vb6
//! ' Time$ only has second precision
//! Debug.Print Time$  ' "14:30:45" (no milliseconds)
//!
//! ' For higher precision, use Timer function
//! Dim startTimer As Single
//! startTimer = Timer
//! ' ... operation ...
//! Debug.Print "Elapsed: " & (Timer - startTimer) & " seconds"
//! ```
//!
//! ### 6. Locale Independence
//!
//! ```vb6
//! ' Time$ format is NOT affected by locale settings
//! ' Always returns "HH:MM:SS" regardless of system locale
//! Debug.Print Time$  ' Always "14:30:45" format
//!
//! ' For locale-specific formatting, use Format$
//! Debug.Print Format$(Now, "Long Time")  ' Respects locale
//! ```
//!
//! ## Practical Examples
//!
//! ### Creating Log Files with Timestamps
//!
//! ```vb6
//! Sub WriteLog(message As String)
//!     Dim logFile As String
//!     Dim timeStamp As String
//!     
//!     logFile = App.Path & "\application.log"
//!     timeStamp = Date$ & " " & Time$
//!     
//!     Open logFile For Append As #1
//!     Print #1, timeStamp & " - " & message
//!     Close #1
//! End Sub
//! ```
//!
//! ### Digital Clock Display
//!
//! ```vb6
//! Private Sub tmrClock_Timer()
//!     lblClock.Caption = Time$
//!     
//!     ' Update every second
//!     tmrClock.Interval = 1000
//! End Sub
//! ```
//!
//! ### Performance Monitoring
//!
//! ```vb6
//! Sub MonitorPerformance()
//!     Dim startTime As Double
//!     Dim endTime As Double
//!     
//!     Debug.Print "Operation started at: " & Time$
//!     startTime = Timer
//!     
//!     ' ... operation to monitor ...
//!     
//!     endTime = Timer
//!     Debug.Print "Operation ended at: " & Time$
//!     Debug.Print "Elapsed time: " & (endTime - startTime) & " seconds"
//! End Sub
//! ```
//!
//! ### Scheduled Task Execution
//!
//! ```vb6
//! Private Sub tmrScheduler_Timer()
//!     Dim currentTimeStr As String
//!     currentTimeStr = Time$
//!     
//!     Select Case currentTimeStr
//!         Case "09:00:00"
//!             RunMorningReport
//!         Case "12:00:00"
//!             RunNoonBackup
//!         Case "17:00:00"
//!             RunEveningCleanup
//!     End Select
//! End Sub
//! ```
//!
//! ## Limitations
//!
//! - Always returns 24-hour format (no AM/PM)
//! - No millisecond or sub-second precision
//! - No time zone information
//! - Format cannot be customized (use `Format$` for that)
//! - Returns string, not suitable for time arithmetic (use `Time` function instead)
//! - Colons in output make it unsuitable for filenames without modification
//! - Cannot be set (read-only; use `Time =` statement to set system time)

use crate::{error::VBResult, value::VBString};

/// Implementation of the `Time$` function.
///
/// VB6 behavior:
/// - returns the current system time as a `String` in 24-hour `HH:MM:SS`
///   format with leading zeros
/// - never raises an error
pub fn time_dollar() -> VBResult<VBString> {
    let now = jiff::Zoned::now().datetime();
    Ok(VBString::from(format!(
        "{:02}:{:02}:{:02}",
        now.hour(),
        now.minute(),
        now.second()
    )))
}

#[cfg(test)]
mod tests {
    use super::time_dollar;

    fn parse_time(s: &str) -> (u8, u8, u8) {
        let mut it = s.split(':');
        let h = it.next().unwrap().parse().unwrap();
        let m = it.next().unwrap().parse().unwrap();
        let sec = it.next().unwrap().parse().unwrap();
        assert!(it.next().is_none(), "unexpected extra parts in {s:?}");
        (h, m, sec)
    }

    #[test]
    fn format_is_hh_mm_ss() {
        let value = time_dollar().unwrap();
        let s = value.as_str();
        assert_eq!(s.len(), 8);
        let (h, m, sec) = parse_time(s);
        assert!(h <= 23 && m <= 59 && sec <= 59);
    }

    #[test]
    fn matches_system_time() {
        let before = jiff::Zoned::now().time();
        let value = time_dollar().unwrap();
        let after = jiff::Zoned::now().time();
        let (h, m, s) = parse_time(value.as_str());
        let result = h as i64 * 3600 + m as i64 * 60 + s as i64;
        let b = before.hour() as i64 * 3600 + before.minute() as i64 * 60 + before.second() as i64;
        let a = after.hour() as i64 * 3600 + after.minute() as i64 * 60 + after.second() as i64;
        assert!(result >= b && result <= a);
    }
}
