//! # `Date` Function
//!
//! Returns the current system date as a `Variant` of subtype `Date`.
//!
//! ## Syntax
//!
//! ```vb
//! Date
//! ```
//!
//! ## Parameters
//!
//! None. The `Date` function takes no parameters.
//!
//! ## Return Value
//!
//! Returns a `Variant` of subtype `Date` (`VarType = 7`) containing the current system date.
//! The time portion is set to midnight (00:00:00).
//!
//! ## Remarks
//!
//! The `Date` function returns the current date from the system clock. Unlike `Now`,
//! which returns both date and time, `Date` returns only the date portion with the
//! time set to midnight.
//!
//! **Important Characteristics:**
//!
//! - Returns only the date portion (time is midnight)
//! - Uses system date from computer's clock
//! - Can also be used as a statement to set the system date: `Date = #1/1/2025#`
//! - Date values are stored internally as Double (days since Dec 30, 1899)
//! - `VarType` of result is 7 (vbDate)
//! - Locale-aware for display formatting
//! - Date range: January 1, 100 to December 31, 9999
//!
//! ## Date Storage Format
//!
//! Internally, dates are stored as Double precision floating-point numbers:
//! - Integer part: Number of days since December 30, 1899
//! - Fractional part: Time of day (0.0 = midnight, 0.5 = noon, etc.)
//! - Date function always returns fractional part as 0.0
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! ' Get current date
//! Dim today As Date
//! today = Date
//! MsgBox "Today is: " & today
//!
//! ' Display in message box
//! MsgBox "Current date: " & Date
//!
//! ' Store in Variant
//! Dim currentDate As Variant
//! currentDate = Date
//! ```
//!
//! ### Date Calculations
//!
//! ```vb
//! ' Calculate days until end of year
//! Dim daysLeft As Long
//! daysLeft = DateSerial(Year(Date), 12, 31) - Date
//! MsgBox daysLeft & " days left in the year"
//!
//! ' Add 30 days to current date
//! Dim futureDate As Date
//! futureDate = Date + 30
//! MsgBox "30 days from now: " & futureDate
//!
//! ' Subtract dates to get difference
//! Dim startDate As Date
//! Dim daysPassed As Long
//! startDate = #1/1/2025#
//! daysPassed = Date - startDate
//! ```
//!
//! ### Date Comparison
//!
//! ```vb
//! ' Check if date is in the past
//! Dim deadline As Date
//! deadline = #12/31/2025#
//!
//! If Date > deadline Then
//!     MsgBox "Deadline has passed!"
//! Else
//!     MsgBox "Still time remaining"
//! End If
//!
//! ' Compare with specific date
//! If Date = #1/1/2025# Then
//!     MsgBox "Happy New Year!"
//! End If
//! ```
//!
//! ## Common Patterns
//!
//! ### Date Stamping Records
//!
//! ```vb
//! ' Add timestamp to database record
//! rs.AddNew
//! rs("CustomerName") = txtName.Text
//! rs("OrderDate") = Date
//! rs("OrderTime") = Time
//! rs.Update
//!
//! ' File naming with date
//! Dim fileName As String
//! fileName = "Report_" & Format(Date, "yyyymmdd") & ".txt"
//! ```
//!
//! ### Date Validation
//!
//! ```vb
//! Function IsDateInRange(checkDate As Date, startDate As Date, endDate As Date) As Boolean
//!     IsDateInRange = (checkDate >= startDate And checkDate <= endDate)
//! End Function
//!
//! ' Check if today is within range
//! If IsDateInRange(Date, #1/1/2025#, #12/31/2025#) Then
//!     MsgBox "Date is within 2025"
//! End If
//! ```
//!
//! ### Age Calculation
//!
//! ```vb
//! Function CalculateAge(birthDate As Date) As Integer
//!     Dim age As Integer
//!     age = Year(Date) - Year(birthDate)
//!     
//!     ' Adjust if birthday hasn't occurred yet this year
//!     If Month(Date) < Month(birthDate) Or _
//!        (Month(Date) = Month(birthDate) And Day(Date) < Day(birthDate)) Then
//!         age = age - 1
//!     End If
//!     
//!     CalculateAge = age
//! End Function
//! ```
//!
//! ### Business Days Calculation
//!
//! ```vb
//! Function IsWeekday(checkDate As Date) As Boolean
//!     Dim dayOfWeek As Integer
//!     dayOfWeek = Weekday(checkDate)
//!     IsWeekday = (dayOfWeek > 1 And dayOfWeek < 7)  ' Not Sunday(1) or Saturday(7)
//! End Function
//!
//! Function AddBusinessDays(startDate As Date, days As Integer) As Date
//!     Dim currentDate As Date
//!     Dim daysAdded As Integer
//!     
//!     currentDate = startDate
//!     daysAdded = 0
//!     
//!     Do While daysAdded < days
//!         currentDate = currentDate + 1
//!         If IsWeekday(currentDate) Then
//!             daysAdded = daysAdded + 1
//!         End If
//!     Loop
//!     
//!     AddBusinessDays = currentDate
//! End Function
//! ```
//!
//! ### Date Range Reporting
//!
//! ```vb
//! Sub GenerateMonthlyReport()
//!     Dim firstDay As Date
//!     Dim lastDay As Date
//!     
//!     ' Get first day of current month
//!     firstDay = DateSerial(Year(Date), Month(Date), 1)
//!     
//!     ' Get last day of current month
//!     lastDay = DateSerial(Year(Date), Month(Date) + 1, 0)
//!     
//!     ' Generate report for date range
//!     MsgBox "Report period: " & firstDay & " to " & lastDay
//! End Sub
//! ```
//!
//! ### Date-Based File Organization
//!
//! ```vb
//! Function GetArchiveFolder() As String
//!     Dim folderPath As String
//!     folderPath = "C:\Archive\" & Year(Date) & "\" & Format(Date, "mm")
//!     
//!     ' Create folder if it doesn't exist
//!     If Dir(folderPath, vbDirectory) = "" Then
//!         MkDir folderPath
//!     End If
//!     
//!     GetArchiveFolder = folderPath
//! End Function
//! ```
//!
//! ### Expiration Checking
//!
//! ```vb
//! Function IsExpired(expirationDate As Date) As Boolean
//!     IsExpired = (Date > expirationDate)
//! End Function
//!
//! Function DaysUntilExpiration(expirationDate As Date) As Long
//!     If IsExpired(expirationDate) Then
//!         DaysUntilExpiration = 0
//!     Else
//!         DaysUntilExpiration = expirationDate - Date
//!     End If
//! End Function
//! ```
//!
//! ### Logging with Timestamps
//!
//! ```vb
//! Sub LogMessage(message As String)
//!     Dim logFile As Integer
//!     Dim logFileName As String
//!     
//!     ' Create daily log file
//!     logFileName = "Log_" & Format(Date, "yyyy-mm-dd") & ".txt"
//!     
//!     logFile = FreeFile
//!     Open logFileName For Append As logFile
//!     Print #logFile, Date & " " & Time & ": " & message
//!     Close logFile
//! End Sub
//! ```
//!
//! ## Advanced Usage
//!
//! ### Date Cache for Performance
//!
//! ```vb
//! ' Module-level variables
//! Private m_cachedDate As Date
//! Private m_cacheValid As Boolean
//!
//! Function GetTodaysDate() As Date
//!     ' Cache date to avoid repeated system calls in tight loops
//!     Static lastCheck As Date
//!     
//!     If Not m_cacheValid Or lastCheck <> Date Then
//!         m_cachedDate = Date
//!         lastCheck = m_cachedDate
//!         m_cacheValid = True
//!     End If
//!     
//!     GetTodaysDate = m_cachedDate
//! End Function
//! ```
//!
//! ### Fiscal Year Calculations
//!
//! ```vb
//! Function GetFiscalYear(Optional checkDate As Variant) As Integer
//!     Dim workDate As Date
//!     
//!     ' Use current date if not specified
//!     If IsMissing(checkDate) Then
//!         workDate = Date
//!     Else
//!         workDate = checkDate
//!     End If
//!     
//!     ' Fiscal year starts April 1
//!     If Month(workDate) < 4 Then
//!         GetFiscalYear = Year(workDate) - 1
//!     Else
//!         GetFiscalYear = Year(workDate)
//!     End If
//! End Function
//!
//! Function GetFiscalQuarter(Optional checkDate As Variant) As Integer
//!     Dim workDate As Date
//!     Dim fiscalMonth As Integer
//!     
//!     If IsMissing(checkDate) Then
//!         workDate = Date
//!     Else
//!         workDate = checkDate
//!     End If
//!     
//!     ' Calculate month in fiscal year (April = 1)
//!     fiscalMonth = Month(workDate) - 3
//!     If fiscalMonth <= 0 Then fiscalMonth = fiscalMonth + 12
//!     
//!     ' Determine quarter
//!     GetFiscalQuarter = Int((fiscalMonth - 1) / 3) + 1
//! End Function
//! ```
//!
//! ### Date Sequence Generator
//!
//! ```vb
//! Function GenerateDateRange(startDate As Date, endDate As Date) As Variant
//!     Dim dates() As Date
//!     Dim currentDate As Date
//!     Dim index As Long
//!     Dim dayCount As Long
//!     
//!     dayCount = endDate - startDate + 1
//!     ReDim dates(0 To dayCount - 1)
//!     
//!     currentDate = startDate
//!     For index = 0 To dayCount - 1
//!         dates(index) = currentDate
//!         currentDate = currentDate + 1
//!     Next index
//!     
//!     GenerateDateRange = dates
//! End Function
//! ```
//!
//! ### Date-Based Conditional Logic
//!
//! ```vb
//! Function GetDiscountRate() As Double
//!     Dim dayOfMonth As Integer
//!     dayOfMonth = Day(Date)
//!     
//!     Select Case dayOfMonth
//!         Case 1 To 10
//!             GetDiscountRate = 0.1   ' 10% early month discount
//!         Case 11 To 20
//!             GetDiscountRate = 0.05  ' 5% mid-month discount
//!         Case Else
//!             GetDiscountRate = 0     ' No discount
//!     End Select
//! End Function
//! ```
//!
//! ## Date vs Now vs Time
//!
//! ```vb
//! ' Date - Returns only date portion (time = midnight)
//! Dim d As Date
//! d = Date  ' Example: 1/15/2025 12:00:00 AM
//!
//! ' Now - Returns date and time
//! Dim n As Date
//! n = Now   ' Example: 1/15/2025 2:30:45 PM
//!
//! ' Time - Returns only time portion (date = Dec 30, 1899)
//! Dim t As Date
//! t = Time  ' Example: 12/30/1899 2:30:45 PM
//!
//! ' Extract components
//! Dim today As Date
//! today = Date  ' Gets just the date
//! Dim dateTime As Date
//! dateTime = today + Time  ' Combines current date and time
//! ```
//!
//! ## Setting the System Date
//!
//! ```vb
//! ' Date can also be used as a statement (requires admin rights)
//! Date = #1/1/2025#  ' Sets system date
//!
//! ' More common: just read the date
//! Dim currentDate As Date
//! currentDate = Date  ' Read-only operation
//! ```
//!
//! ## Performance Considerations
//!
//! - `Date` is a system call that reads the system clock
//! - In tight loops, consider caching if the date won't change during execution
//! - Faster than `Now` when you only need the date portion
//! - Date comparisons are fast (numeric comparison of Double values)
//! - Format conversions can be slower than raw date operations
//!
//! ## Best Practices
//!
//! ### Cache in Long-Running Loops
//!
//! ```vb
//! ' Inefficient - calls Date repeatedly
//! For i = 1 To 10000
//!     If records(i).ExpiryDate < Date Then
//!         ' Process expired record
//!     End If
//! Next i
//!
//! ' Better - cache the date
//! Dim today As Date
//! today = Date
//! For i = 1 To 10000
//!     If records(i).ExpiryDate < today Then
//!         ' Process expired record
//!     End If
//! Next i
//! ```
//!
//! ### Use Appropriate Date Functions
//!
//! ```vb
//! ' Good - Use Date for date-only operations
//! Dim orderDate As Date
//! orderDate = Date
//!
//! ' Good - Use Now for timestamps
//! Dim timestamp As Date
//! timestamp = Now
//!
//! ' Avoid - Don't extract date from Now if you just need date
//! Dim today As Date
//! today = Int(Now)  ' Works but less clear than Date
//! ```
//!
//! ### Store Dates as Date Type
//!
//! ```vb
//! ' Good - Proper type usage
//! Dim birthDate As Date
//! birthDate = Date
//!
//! ' Avoid - String storage loses type safety and comparison capability
//! Dim birthDateStr As String
//! birthDateStr = CStr(Date)  ' Comparison becomes string-based
//! ```
//!
//! ## Locale Considerations
//!
//! - Display format depends on system locale settings
//! - Internal storage is always the same (Double)
//! - Use `Format` function for explicit formatting
//! - Date literals use US format (#MM/DD/YYYY#) regardless of locale
//!
//! ```vb
//! ' Display varies by locale
//! MsgBox Date  ' US: 1/15/2025, UK: 15/01/2025
//!
//! ' Explicit formatting
//! MsgBox Format(Date, "yyyy-mm-dd")  ' Always: 2025-01-15
//! ```
//!
//! ## Limitations
//!
//! - Cannot directly extract error information from invalid dates
//! - System date setting requires administrative privileges
//! - Date range limited to January 1, 100 through December 31, 9999
//! - Accuracy depends on system clock
//! - No built-in timezone support
//! - Daylight saving time handled by system, not VB6
//!
//! ## Related Functions
//!
//! - `Now`: Returns current date and time
//! - `Time`: Returns current time (date portion is Dec 30, 1899)
//! - `DateSerial`: Creates a date from year, month, and day values
//! - `DateValue`: Converts a string to a date
//! - `Year`, `Month`, `Day`: Extract components from a date
//! - `Weekday`: Returns day of week (1=Sunday, 7=Saturday)
//! - `DateAdd`: Adds a time interval to a date
//! - `DateDiff`: Returns the difference between two dates
//! - `DatePart`: Returns a specific part of a date
//! - `Format`: Formats a date as a string
//! - `IsDate`: Tests if a value can be converted to a date
//! - `CDate`: Converts an expression to a Date

use crate::{error::VBResult, value::VBVariant};

/// Implementation of the `Date` function.
///
/// VB6 behavior:
/// - returns the current system date as a `Date` variant with the time
///   portion set to midnight
/// - never raises an error
pub fn date() -> VBResult<VBVariant> {
    use jiff::civil::Date;
    use jiff::{SpanRelativeTo, Unit};

    let instant = crate::state::clock::get();
    let zoned = jiff::Zoned::new(instant, jiff::tz::TimeZone::UTC);
    let today = zoned.date();
    let base = Date::new(1899, 12, 30).expect("valid epoch");
    let serial = today
        .since(base)
        .ok()
        .and_then(|span| {
            span.total((Unit::Day, SpanRelativeTo::days_are_24_hours()))
                .ok()
        })
        .unwrap_or(0.0);
    Ok(VBVariant::from_date_serial(serial))
}

#[cfg(test)]
mod tests {
    use super::date;
    use crate::state::test_support::TEST_LOCK;
    use crate::VBVariant;

    #[test]
    fn returns_date_variant() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        let value = date().unwrap();
        assert_eq!(value.var_type(), 7);
        assert_eq!(value.type_of(), crate::VBType::Date);
        crate::state::clock::reset();
    }

    #[test]
    fn time_portion_is_midnight() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        let value = date().unwrap();
        let VBVariant::Date(serial) = value else {
            panic!("expected a Date variant");
        };
        assert_eq!(serial.fract(), 0.0, "time must be midnight");
        crate::state::clock::reset();
    }

    #[test]
    fn matches_system_date() {
        let _guard = TEST_LOCK.lock().unwrap();
        crate::state::clock::reset();
        let before = {
            let ts = crate::state::clock::get();
            jiff::Zoned::new(ts, jiff::tz::TimeZone::UTC).date()
        };
        let value = date().unwrap();
        let after = {
            let ts = crate::state::clock::get();
            jiff::Zoned::new(ts, jiff::tz::TimeZone::UTC).date()
        };
        let VBVariant::Date(serial) = value else {
            panic!("expected a Date variant");
        };
        let parsed = crate::value::date_serial_to_datetime(serial).expect("valid serial");
        let parsed = jiff::civil::Date::new(parsed.year(), parsed.month(), parsed.day())
            .expect("valid civil date");
        assert!(parsed >= before && parsed <= after);
        crate::state::clock::reset();
    }

    #[test]
    fn reflects_mock_clock() {
        let _guard = TEST_LOCK.lock().unwrap();
        let target = jiff::civil::Date::new(2025, 6, 15).unwrap();
        crate::state::clock::set_date(target);
        let value = date().unwrap();
        let VBVariant::Date(serial) = value else {
            panic!("expected a Date variant");
        };
        let parsed = crate::value::date_serial_to_datetime(serial).expect("valid serial");
        let parsed_date =
            jiff::civil::Date::new(parsed.year(), parsed.month(), parsed.day()).unwrap();
        assert_eq!(parsed_date, target);
        crate::state::clock::reset();
    }
}
