//! # `Date$` Function
//!
//! Returns the current system date as a `String`. The dollar sign suffix (`$`) explicitly
//! indicates that this function returns a `String` type (not a `Variant`).
//!
//! ## Syntax
//!
//! ```vb
//! Date$
//! ```
//!
//! ## Parameters
//!
//! None. The `Date$` function takes no parameters.
//!
//! ## Return Value
//!
//! Returns a `String` containing the current system date. The format depends on the system's
//! regional settings (typically "mm/dd/yyyy" in US or "dd/mm/yyyy" in other regions). The
//! return value is always a `String` type (never `Variant`).
//!
//! ## Remarks
//!
//! - The `Date$` function always returns a `String`, while `Date` (without `$`) returns a `Variant` of subtype `Date`.
//! - Returns only the date portion (no time information).
//! - Uses system date from computer's clock.
//! - Date format depends on system locale/regional settings.
//! - Common formats: "mm/dd/yyyy" (US), "dd/mm/yyyy" (Europe), "yyyy/mm/dd" (ISO).
//! - The string representation may include leading zeros (e.g., "01/05/2025").
//! - For better performance when you need a string, use `Date$` instead of `Date`.
//! - Cannot be used to set the system date (unlike `Date` statement).
//!
//! ## Typical Uses
//!
//! 1. **Date stamping** - Add date stamps to log entries, files, or records
//! 2. **Display formatting** - Show current date to users
//! 3. **File naming** - Include date in filenames
//! 4. **Logging** - Record when events occurred
//! 5. **Report generation** - Add date headers to reports
//! 6. **Audit trails** - Track when data was created or modified
//! 7. **String concatenation** - Combine date with other text
//!
//! ## Basic Examples
//!
//! ```vb
//! ' Example 1: Get current date as string
//! Dim dateStr As String
//! dateStr = Date$
//! ```
//!
//! ```vb
//! ' Example 2: Display current date
//! MsgBox "Today is: " & Date$
//! ```
//!
//! ```vb
//! ' Example 3: Create date stamp
//! Dim stamp As String
//! stamp = "Report generated on " & Date$
//! ```
//!
//! ```vb
//! ' Example 4: Simple assignment
//! currentDate = Date$
//! ```
//!
//! ## Common Patterns
//!
//! ### File Naming with Date
//! ```vb
//! Function CreateDateStampedFilename(baseName As String) As String
//!     Dim dateStr As String
//!     Dim cleanDate As String
//!     
//!     ' Get date and remove slashes
//!     dateStr = Date$
//!     cleanDate = Replace$(dateStr, "/", "")
//!     
//!     CreateDateStampedFilename = baseName & "_" & cleanDate & ".txt"
//! End Function
//! ```
//!
//! ### Log Entry with Date
//! ```vb
//! Sub WriteLogEntry(message As String)
//!     Dim logFile As Integer
//!     Dim logEntry As String
//!     
//!     logFile = FreeFile
//!     Open "application.log" For Append As #logFile
//!     
//!     logEntry = Date$ & " - " & message
//!     Print #logFile, logEntry
//!     
//!     Close #logFile
//! End Sub
//! ```
//!
//! ### Date-Based Conditional Logic
//! ```vb
//! Sub CheckDate()
//!     Dim todayStr As String
//!     todayStr = Date$
//!     
//!     ' Simple string comparison (locale-dependent)
//!     If todayStr = "12/25/2025" Then
//!         MsgBox "Merry Christmas!"
//!     End If
//! End Sub
//! ```
//!
//! ### Report Header
//! ```vb
//! Function CreateReportHeader(title As String) As String
//!     Dim header As String
//!     header = String$(60, "=") & vbCrLf
//!     header = header & title & vbCrLf
//!     header = header & "Generated: " & Date$ & vbCrLf
//!     header = header & String$(60, "=") & vbCrLf
//!     CreateReportHeader = header
//! End Function
//! ```
//!
//! ### Date Display in Status Bar
//! ```vb
//! Sub UpdateStatusBar()
//!     Form1.StatusBar.Panels(1).Text = "Date: " & Date$
//! End Sub
//! ```
//!
//! ### Backup File Naming
//! ```vb
//! Function GetBackupFilename(originalFile As String) As String
//!     Dim baseName As String
//!     Dim extension As String
//!     Dim dotPos As Integer
//!     Dim dateStr As String
//!     
//!     dotPos = InStrRev(originalFile, ".")
//!     If dotPos > 0 Then
//!         baseName = Left$(originalFile, dotPos - 1)
//!         extension = Mid$(originalFile, dotPos)
//!     Else
//!         baseName = originalFile
//!         extension = ""
//!     End If
//!     
//!     ' Clean date string for filename
//!     dateStr = Replace$(Date$, "/", "-")
//!     
//!     GetBackupFilename = baseName & "_backup_" & dateStr & extension
//! End Function
//! ```
//!
//! ### Daily Log File
//! ```vb
//! Function GetDailyLogFilename() As String
//!     Dim dateStr As String
//!     dateStr = Replace$(Date$, "/", "")
//!     GetDailyLogFilename = "log_" & dateStr & ".txt"
//! End Function
//! ```
//!
//! ### Date Validation (Simple)
//! ```vb
//! Function IsToday(dateStr As String) As Boolean
//!     IsToday = (dateStr = Date$)
//! End Function
//! ```
//!
//! ### Combining Date and Time
//! ```vb
//! Function GetDateTimeStamp() As String
//!     GetDateTimeStamp = Date$ & " " & Time$
//! End Function
//! ```
//!
//! ### Data Export Header
//! ```vb
//! Sub ExportData()
//!     Dim exportFile As Integer
//!     
//!     exportFile = FreeFile
//!     Open "export.csv" For Output As #exportFile
//!     
//!     ' Write header with date
//!     Print #exportFile, "Data Export - " & Date$
//!     Print #exportFile, "Name,Value,Status"
//!     
//!     ' Export data...
//!     
//!     Close #exportFile
//! End Sub
//! ```
//!
//! ## Related Functions
//!
//! - `Date`: Returns current date as `Variant` instead of `String`
//! - `Now`: Returns current date and time
//! - `Time$`: Returns current time as `String`
//! - `Format$`: Formats dates with custom patterns
//! - `Year`: Extracts year from date
//! - `Month`: Extracts month from date
//! - `Day`: Extracts day from date
//! - `DateSerial`: Creates date from year, month, day
//! - `DateValue`: Converts string to date
//!
//! ## Best Practices
//!
//! 1. Use `Format$` instead of `Date$` when you need specific date formats
//! 2. Be aware that `Date$` format depends on system locale settings
//! 3. For file naming, clean the date string (remove or replace slashes)
//! 4. Use `Date$` instead of `Date` when you need a string result
//! 5. For date comparisons, use `Date` (Variant) instead of `Date$` (String)
//! 6. Don't assume a specific date format - it varies by locale
//! 7. For consistent formatting, use `Format$(Date, "yyyy-mm-dd")`
//! 8. Test with different regional settings if your app is international
//! 9. Store dates in consistent format (ISO 8601 recommended)
//! 10. Use `DateValue` to parse date strings reliably
//!
//! ## Performance Considerations
//!
//! - `Date$` is slightly more efficient than `Date` when you need a string
//! - System date/time calls are fast but not free
//! - Cache the result if you need it multiple times in quick succession
//! - For high-frequency logging, consider caching the date string
//!
//! ## Locale Considerations
//!
//! The format of `Date$` varies by system locale:
//!
//! | Locale | Example Format | Sample Output |
//! |--------|----------------|---------------|
//! | US (English) | mm/dd/yyyy | "12/25/2025" |
//! | UK (English) | dd/mm/yyyy | "25/12/2025" |
//! | Germany | dd.mm.yyyy | "25.12.2025" |
//! | Japan | yyyy/mm/dd | "2025/12/25" |
//! | France | dd/mm/yyyy | "25/12/2025" |
//!
//! ## Common Pitfalls
//!
//! 1. **String Comparison**: Comparing `Date$` strings directly is locale-dependent and unreliable
//!    ```vb
//!    ' BAD - locale-dependent
//!    If Date$ = "12/25/2025" Then
//!    
//!    ' GOOD - use Date variants
//!    If Date = #12/25/2025# Then
//!    ```
//!
//! 2. **Date Parsing**: Don't parse `Date$` manually - use `DateValue` instead
//!    ```vb
//!    ' BAD - fragile parsing
//!    parts = Split(Date$, "/")
//!    
//!    ' GOOD - use built-in functions
//!    currentYear = Year(Date)
//!    currentMonth = Month(Date)
//!    ```
//!
//! 3. **Filename Safety**: Date strings may contain invalid filename characters
//!    ```vb
//!    ' BAD - slashes invalid in filenames
//!    filename = "report_" & Date$ & ".txt"
//!    
//!    ' GOOD - replace invalid characters
//!    filename = "report_" & Replace$(Date$, "/", "-") & ".txt"
//!    ```
//!
//! ## Limitations
//!
//! - Cannot be used to set the system date (use `Date` statement for that)
//! - Format is system-dependent and cannot be directly controlled
//! - No time information included (use `Now` or `Time$` for time)
//! - String comparison of dates is unreliable across locales
//! - Cannot specify date format (use `Format$` for custom formats)

use crate::error::VBResult;
use crate::value::{date_serial_to_string, VBString};

/// Implementation of the `Date$` function.
///
/// VB6 behavior:
/// - returns the current system date as a `String`, formatted like the
///   runtime's `CStr(Date)` (`M/D/YYYY` with no time component)
/// - never raises an error
pub fn date_dollar() -> VBResult<VBString> {
    use jiff::civil::Date;
    use jiff::{SpanRelativeTo, Unit};

    let today = jiff::Zoned::now().date();
    let base = Date::new(1899, 12, 30).expect("valid epoch");
    let serial = today
        .since(base)
        .ok()
        .and_then(|span| {
            span.total((Unit::Day, SpanRelativeTo::days_are_24_hours()))
                .ok()
        })
        .unwrap_or(0.0);
    Ok(VBString::from(date_serial_to_string(serial)))
}

#[cfg(test)]
mod tests {
    use super::date_dollar;
    use crate::value::VBVariant;

    fn parse_parts(s: &str) -> (i32, i32, i32) {
        let mut it = s.split('/');
        let month = it.next().unwrap().parse().unwrap();
        let day = it.next().unwrap().parse().unwrap();
        let year = it.next().unwrap().parse().unwrap();
        assert!(it.next().is_none(), "unexpected extra parts in {s:?}");
        (month, day, year)
    }

    #[test]
    fn returns_current_date_in_m_d_yyyy() {
        let result = date_dollar().unwrap();
        let s = result.as_str();
        assert_eq!(s.split('/').count(), 3);
        let (month, day, _year) = parse_parts(s);
        assert!((1..=12).contains(&month), "bad month in {s:?}");
        assert!((1..=31).contains(&day), "bad day in {s:?}");
        assert_eq!(s, s.trim(), "unexpected whitespace in {s:?}");
    }

    #[test]
    fn result_round_trips_through_cdate() {
        let s = date_dollar().unwrap();
        let variant = VBVariant::from_string(s.into_inner());
        assert!(variant.as_date_serial().is_ok());
    }

    #[test]
    fn matches_system_date() {
        let before = jiff::Zoned::now().date();
        let s = date_dollar().unwrap();
        let after = jiff::Zoned::now().date();
        let (month, day, year) = parse_parts(s.as_str());
        let parsed = jiff::civil::Date::new(year as i16, month as i8, day as i8).unwrap();
        assert!(parsed >= before && parsed <= after);
    }
}
