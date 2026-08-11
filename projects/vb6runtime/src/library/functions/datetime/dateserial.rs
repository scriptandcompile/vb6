//! # `DateSerial` Function
//!
//! Returns a `Variant` (`Date`) for a specified year, month, and day.
//!
//! ## Syntax
//!
//! ```vb
//! DateSerial(year, month, day)
//! ```
//!
//! ## Parameters
//!
//! - **year**: Required. `Integer` expression between 100 and 9999, inclusive, or a numeric
//!   expression. Values from 0 to 29 are interpreted as 2000-2029; values from 30 to 99
//!   are interpreted as 1930-1999.
//! - **month**: Required. `Integer` expression from 1 to 12, but can be any numeric expression
//!   representing months from -32,768 to 32,767. Month values outside 1-12 adjust the year
//!   accordingly.
//! - **day**: Required. `Integer` expression from 1 to 31, but can be any numeric expression
//!   representing days from -32,768 to 32,767. Day values outside the valid range adjust
//!   the month and year accordingly.
//!
//! ## Return Value
//!
//! Returns a `Variant` of subtype `Date` representing the specified date. The time portion
//! is set to midnight (00:00:00).
//!
//! ## Remarks
//!
//! The `DateSerial` function is used to construct a date value from individual year, month,
//! and day components. It's particularly useful for date calculations and building dates
//! programmatically.
//!
//! **Important Characteristics:**
//!
//! - Accepts values outside normal ranges and adjusts automatically
//! - Month values > 12 or < 1 adjust the year
//! - Day values outside valid range adjust the month
//! - Can use 0 or negative values for relative date calculations
//! - Two-digit years: 0-29 → 2000-2029, 30-99 → 1930-1999
//! - Always returns midnight (00:00:00) for time portion
//! - Invalid combinations return compile-time or runtime errors
//!
//! ## Range Adjustment Examples
//!
//! ```vb
//! ' Month adjustment
//! DateSerial(2025, 13, 1)    ' Returns 1/1/2026 (13th month = Jan next year)
//! DateSerial(2025, 0, 1)     ' Returns 12/1/2024 (0th month = Dec previous year)
//! DateSerial(2025, -1, 1)    ' Returns 11/1/2024 (month -1 = Nov previous year)
//!
//! ' Day adjustment
//! DateSerial(2025, 1, 32)    ' Returns 2/1/2025 (32nd day = Feb 1)
//! DateSerial(2025, 1, 0)     ' Returns 12/31/2024 (0th day = last day of prev month)
//! DateSerial(2025, 1, -1)    ' Returns 12/30/2024 (day -1)
//!
//! ' Combined adjustment
//! DateSerial(2025, 13, 32)   ' Returns 2/1/2026
//! ```
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! ' Create a specific date
//! Dim birthday As Date
//! birthday = DateSerial(1990, 5, 15)  ' May 15, 1990
//!
//! ' Create date from variables
//! Dim y As Integer, m As Integer, d As Integer
//! y = 2025
//! m = 12
//! d = 25
//! Dim christmas As Date
//! christmas = DateSerial(y, m, d)
//!
//! ' Current year's date
//! Dim thisYear As Date
//! thisYear = DateSerial(Year(Date), 1, 1)  ' January 1 of current year
//! ```
//!
//! ### Last Day of Month
//!
//! ```vb
//! Function GetLastDayOfMonth(year As Integer, month As Integer) As Date
//!     ' Use day 0 of next month to get last day of current month
//!     GetLastDayOfMonth = DateSerial(year, month + 1, 0)
//! End Function
//!
//! ' Usage
//! Dim lastDay As Date
//! lastDay = GetLastDayOfMonth(2025, 2)  ' Feb 28, 2025 (or 29 in leap year)
//! ```
//!
//! ### First Day of Month
//!
//! ```vb
//! Function GetFirstDayOfMonth(someDate As Date) As Date
//!     GetFirstDayOfMonth = DateSerial(Year(someDate), Month(someDate), 1)
//! End Function
//! ```
//!
//! ## Common Patterns
//!
//! ### Month Boundaries
//!
//! ```vb
//! Function GetMonthStart(someDate As Date) As Date
//!     GetMonthStart = DateSerial(Year(someDate), Month(someDate), 1)
//! End Function
//!
//! Function GetMonthEnd(someDate As Date) As Date
//!     GetMonthEnd = DateSerial(Year(someDate), Month(someDate) + 1, 0)
//! End Function
//!
//! ' Get entire month range
//! Dim startDate As Date
//! Dim endDate As Date
//! startDate = GetMonthStart(Date)
//! endDate = GetMonthEnd(Date)
//! ```
//!
//! ### Year Boundaries
//!
//! ```vb
//! Function GetYearStart(someDate As Date) As Date
//!     GetYearStart = DateSerial(Year(someDate), 1, 1)
//! End Function
//!
//! Function GetYearEnd(someDate As Date) As Date
//!     GetYearEnd = DateSerial(Year(someDate), 12, 31)
//! End Function
//! ```
//!
//! ### Quarter Boundaries
//!
//! ```vb
//! Function GetQuarterStart(year As Integer, quarter As Integer) As Date
//!     Dim month As Integer
//!     month = (quarter - 1) * 3 + 1
//!     GetQuarterStart = DateSerial(year, month, 1)
//! End Function
//!
//! Function GetQuarterEnd(year As Integer, quarter As Integer) As Date
//!     Dim month As Integer
//!     month = quarter * 3
//!     GetQuarterEnd = DateSerial(year, month + 1, 0)
//! End Function
//! ```
//!
//! ### Add Months Correctly
//!
//! ```vb
//! Function AddMonths(startDate As Date, months As Integer) As Date
//!     Dim y As Integer, m As Integer, d As Integer
//!     
//!     y = Year(startDate)
//!     m = Month(startDate)
//!     d = Day(startDate)
//!     
//!     ' Add months (DateSerial handles overflow)
//!     AddMonths = DateSerial(y, m + months, d)
//! End Function
//!
//! ' Handle day overflow gracefully
//! Function AddMonthsSafe(startDate As Date, months As Integer) As Date
//!     Dim y As Integer, m As Integer, d As Integer
//!     Dim lastDay As Date
//!     
//!     y = Year(startDate)
//!     m = Month(startDate)
//!     d = Day(startDate)
//!     
//!     ' Get last day of target month
//!     lastDay = DateSerial(y, m + months + 1, 0)
//!     
//!     ' Use smaller of original day or last day of month
//!     If d > Day(lastDay) Then
//!         d = Day(lastDay)
//!     End If
//!     
//!     AddMonthsSafe = DateSerial(y, m + months, d)
//! End Function
//! ```
//!
//! ### Leap Year Detection
//!
//! ```vb
//! Function IsLeapYear(year As Integer) As Boolean
//!     Dim feb29 As Date
//!     On Error Resume Next
//!     feb29 = DateSerial(year, 2, 29)
//!     IsLeapYear = (Err.Number = 0)
//! End Function
//! ```
//!
//! ### Days in Month
//!
//! ```vb
//! Function DaysInMonth(year As Integer, month As Integer) As Integer
//!     Dim lastDay As Date
//!     lastDay = DateSerial(year, month + 1, 0)
//!     DaysInMonth = Day(lastDay)
//! End Function
//! ```
//!
//! ### Birthday This Year
//!
//! ```vb
//! Function GetBirthdayThisYear(birthDate As Date) As Date
//!     GetBirthdayThisYear = DateSerial(Year(Date), Month(birthDate), Day(birthDate))
//! End Function
//!
//! Function HasBirthdayPassed(birthDate As Date) As Boolean
//!     HasBirthdayPassed = (GetBirthdayThisYear(birthDate) <= Date)
//! End Function
//! ```
//!
//! ### Week Start (Monday)
//!
//! ```vb
//! Function GetWeekStart(someDate As Date) As Date
//!     Dim offset As Integer
//!     offset = Weekday(someDate, vbMonday) - 1
//!     GetWeekStart = DateSerial(Year(someDate), Month(someDate), Day(someDate) - offset)
//! End Function
//! ```
//!
//! ### Generate Date Range
//!
//! ```vb
//! Function GenerateMonthStarts(year As Integer) As Variant
//!     Dim dates(1 To 12) As Date
//!     Dim i As Integer
//!     
//!     For i = 1 To 12
//!         dates(i) = DateSerial(year, i, 1)
//!     Next i
//!     
//!     GenerateMonthStarts = dates
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Fiscal Year Calculations
//!
//! ```vb
//! Function GetFiscalYearStart(calendarYear As Integer, fiscalStartMonth As Integer) As Date
//!     GetFiscalYearStart = DateSerial(calendarYear, fiscalStartMonth, 1)
//! End Function
//!
//! Function GetFiscalYearEnd(calendarYear As Integer, fiscalStartMonth As Integer) As Date
//!     ' Fiscal year end is day before next fiscal year starts
//!     GetFiscalYearEnd = DateSerial(calendarYear + 1, fiscalStartMonth, 0)
//! End Function
//!
//! Function GetCurrentFiscalYear(fiscalStartMonth As Integer) As Integer
//!     Dim currentMonth As Integer
//!     currentMonth = Month(Date)
//!     
//!     If currentMonth >= fiscalStartMonth Then
//!         GetCurrentFiscalYear = Year(Date)
//!     Else
//!         GetCurrentFiscalYear = Year(Date) - 1
//!     End If
//! End Function
//! ```
//!
//! ### Date Table Generator
//!
//! ```vb
//! Sub PopulateDateDimension(startYear As Integer, endYear As Integer)
//!     Dim y As Integer, m As Integer, d As Integer
//!     Dim currentDate As Date
//!     Dim rs As ADODB.Recordset
//!     
//!     Set rs = New ADODB.Recordset
//!     ' Open recordset...
//!     
//!     For y = startYear To endYear
//!         For m = 1 To 12
//!             Dim daysInMonth As Integer
//!             daysInMonth = Day(DateSerial(y, m + 1, 0))
//!             
//!             For d = 1 To daysInMonth
//!                 currentDate = DateSerial(y, m, d)
//!                 
//!                 rs.AddNew
//!                 rs("DateKey") = Format(currentDate, "yyyymmdd")
//!                 rs("FullDate") = currentDate
//!                 rs("Year") = y
//!                 rs("Quarter") = DatePart("q", currentDate)
//!                 rs("Month") = m
//!                 rs("Day") = d
//!                 rs("DayOfWeek") = Weekday(currentDate)
//!                 rs.Update
//!             Next d
//!         Next m
//!     Next y
//! End Sub
//! ```
//!
//! ### Anniversary Calculator
//!
//! ```vb
//! Function GetAnniversaryDate(originalDate As Date, yearsLater As Integer) As Date
//!     Dim y As Integer, m As Integer, d As Integer
//!     
//!     y = Year(originalDate)
//!     m = Month(originalDate)
//!     d = Day(originalDate)
//!     
//!     GetAnniversaryDate = DateSerial(y + yearsLater, m, d)
//! End Function
//!
//! ' Handle Feb 29 anniversaries
//! Function GetAnniversaryDateSafe(originalDate As Date, yearsLater As Integer) As Date
//!     Dim y As Integer, m As Integer, d As Integer
//!     
//!     y = Year(originalDate) + yearsLater
//!     m = Month(originalDate)
//!     d = Day(originalDate)
//!     
//!     ' For Feb 29, use Feb 28 in non-leap years
//!     If m = 2 And d = 29 Then
//!         If Not IsLeapYear(y) Then
//!             d = 28
//!         End If
//!     End If
//!     
//!     GetAnniversaryDateSafe = DateSerial(y, m, d)
//! End Function
//! ```
//!
//! ### Relative Date Builder
//!
//! ```vb
//! Function BuildRelativeDate(baseDate As Date, yearOffset As Integer, _
//!                          monthOffset As Integer, dayOffset As Integer) As Date
//!     BuildRelativeDate = DateSerial(Year(baseDate) + yearOffset, _
//!                                   Month(baseDate) + monthOffset, _
//!                                   Day(baseDate) + dayOffset)
//! End Function
//!
//! ' Get date 2 years, 3 months, and 5 days from now
//! Dim futureDate As Date
//! futureDate = BuildRelativeDate(Date, 2, 3, 5)
//! ```
//!
//! ### Easter Calculation (Simplified)
//!
//! ```vb
//! Function GetEasterSunday(year As Integer) As Date
//!     ' Simplified Meeus/Jones/Butcher algorithm
//!     Dim a As Integer, b As Integer, c As Integer
//!     Dim d As Integer, e As Integer, f As Integer
//!     Dim g As Integer, h As Integer, i As Integer
//!     Dim k As Integer, l As Integer, m As Integer
//!     Dim month As Integer, day As Integer
//!     
//!     a = year Mod 19
//!     b = year \ 100
//!     c = year Mod 100
//!     d = b \ 4
//!     e = b Mod 4
//!     f = (b + 8) \ 25
//!     g = (b - f + 1) \ 3
//!     h = (19 * a + b - d - g + 15) Mod 30
//!     i = c \ 4
//!     k = c Mod 4
//!     l = (32 + 2 * e + 2 * i - h - k) Mod 7
//!     m = (a + 11 * h + 22 * l) \ 451
//!     month = (h + l - 7 * m + 114) \ 31
//!     day = ((h + l - 7 * m + 114) Mod 31) + 1
//!     
//!     GetEasterSunday = DateSerial(year, month, day)
//! End Function
//! ```
//!
//! ### Business Month-End Handler
//!
//! ```vb
//! Function GetBusinessMonthEnd(year As Integer, month As Integer) As Date
//!     Dim lastDay As Date
//!     Dim dayOfWeek As Integer
//!     
//!     lastDay = DateSerial(year, month + 1, 0)
//!     dayOfWeek = Weekday(lastDay)
//!     
//!     ' If weekend, back up to Friday
//!     If dayOfWeek = vbSaturday Then
//!         lastDay = DateSerial(year, month + 1, -1)  ' Friday
//!     ElseIf dayOfWeek = vbSunday Then
//!         lastDay = DateSerial(year, month + 1, -2)  ' Friday
//!     End If
//!     
//!     GetBusinessMonthEnd = lastDay
//! End Function
//! ```
//!
//! ### Date Validator
//!
//! ```vb
//! Function IsValidDate(year As Integer, month As Integer, day As Integer) As Boolean
//!     On Error Resume Next
//!     Dim testDate As Date
//!     testDate = DateSerial(year, month, day)
//!     
//!     IsValidDate = (Err.Number = 0) And _
//!                   (Year(testDate) = year) And _
//!                   (Month(testDate) = month) And _
//!                   (Day(testDate) = day)
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeDateSerial(year As Integer, month As Integer, day As Integer) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     ' Validate ranges
//!     If year < 100 Or year > 9999 Then
//!         SafeDateSerial = Null
//!         Exit Function
//!     End If
//!     
//!     SafeDateSerial = DateSerial(year, month, day)
//!     Exit Function
//!     
//! ErrorHandler:
//!     SafeDateSerial = Null
//! End Function
//! ```
//!
//! ### Common Errors
//!
//! - **Error 5** (Invalid procedure call): Year outside 100-9999 range
//! - **Error 13** (Type mismatch): Non-numeric arguments
//! - **Error 6** (Overflow): Result date outside valid range
//!
//! ## Performance Considerations
//!
//! - `DateSerial` is very fast for date construction
//! - More efficient than parsing date strings
//! - Automatic range adjustment is performant
//! - No string formatting overhead
//! - Ideal for loop-based date generation
//!
//! ## Best Practices
//!
//! ### Use for Date Construction
//!
//! ```vb
//! ' Good - Clear and unambiguous
//! deadline = DateSerial(2025, 12, 31)
//!
//! ' Avoid - Locale-dependent
//! deadline = CDate("12/31/2025")  ' May fail in different locales
//! ```
//!
//! ### Leverage Range Adjustment
//!
//! ```vb
//! ' Use day 0 for last day of previous month
//! lastDayPrevMonth = DateSerial(year, month, 0)
//!
//! ' Use month 0 for last month of previous year
//! dec31 = DateSerial(year, 0, 31)
//! ```
//!
//! ### Validate Before Critical Operations
//!
//! ```vb
//! If IsValidDate(y, m, d) Then
//!     result = DateSerial(y, m, d)
//! Else
//!     MsgBox "Invalid date components"
//! End If
//! ```
//!
//! ### Extract Components for Manipulation
//!
//! ```vb
//! ' Extract, modify, rebuild
//! y = Year(someDate)
//! m = Month(someDate)
//! d = 1  ' First of month
//! newDate = DateSerial(y, m, d)
//! ```
//!
//! ## Comparison with Other Functions
//!
//! ### `DateSerial` vs Date Literals
//!
//! ```vb
//! ' DateSerial - Dynamic, programmatic
//! dt = DateSerial(Year(Date), 12, 25)
//!
//! ' Date Literal - Static, hardcoded
//! dt = #12/25/2025#
//! ```
//!
//! ### `DateSerial` vs `DateValue`
//!
//! ```vb
//! ' `DateSerial` - From numeric components
//! dt = DateSerial(2025, 12, 25)
//!
//! ' `DateValue` - From string representation
//! dt = DateValue("December 25, 2025")
//! ```
//!
//! ### `DateSerial` vs `DateAdd`
//!
//! ```vb
//! ' DateSerial - Absolute date construction
//! nextMonth = DateSerial(Year(Date), Month(Date) + 1, 1)
//!
//! ' DateAdd - Relative date calculation
//! nextMonth = DateAdd("m", 1, Date)
//! ```
//!
//! ## Limitations
//!
//! - Year must be between 100 and 9999
//! - Two-digit year interpretation fixed (0-29=2000-2029, 30-99=1930-1999)
//! - Always returns midnight (no time component)
//! - Cannot directly specify time components
//! - Invalid dates may raise runtime errors
//!
//! ## Related Functions
//!
//! - `DateValue`: Converts a string to a date
//! - `TimeSerial`: Creates a time from hour, minute, and second
//! - `DateAdd`: Adds a time interval to a date
//! - `Year`, `Month`, `Day`: Extract date components
//! - `Date`: Returns current system date
//! - `Now`: Returns current date and time
//! - `IsDate`: Tests if a value can be converted to a date
//! - `CDate`: Converts an expression to a Date

use crate::error::{VBError, VBResult};
use crate::value::{date_serial_to_datetime, VBVariant};

/// Implementation of the `DateSerial` function.
///
/// VB6 behavior:
/// - two-digit years 0-29 map to 2000-2029 and 30-99 map to 1930-1999
/// - month values outside 1-12 adjust the year (13 = January of next year)
/// - day values outside the valid range adjust the month and year, with no
///   clamping (Feb 29 2025 is March 1 2025; Feb 30 2025 is March 2 2025)
/// - the time portion is always midnight
/// - a year outside 100-9999 (after the two-digit mapping) raises error 5
///   (invalid procedure call); a result outside the supported range raises
///   error 6 (overflow); non-numeric arguments raise error 13 (type mismatch)
pub fn date_serial(year: &VBVariant, month: &VBVariant, day: &VBVariant) -> VBResult<VBVariant> {
    let year = year.as_i32()?;
    let month = month.as_i32()?;
    let day = day.as_i32()?;

    let year = if (0..=99).contains(&year) {
        if year <= 29 {
            year + 2000
        } else {
            year + 1900
        }
    } else {
        year
    };

    if !(100..=9999).contains(&year) {
        return Err(VBError::invalid_procedure_call());
    }

    let total_months = year as i64 * 12 + month as i64 - 1;
    let y = total_months.div_euclid(12);
    let m = total_months.rem_euclid(12) + 1;

    let serial = days_from_civil(y, m, 1) as f64 + 25_569.0 + day as f64 - 1.0;

    let dt = date_serial_to_datetime(serial).ok_or_else(VBError::overflow)?;
    if !(100..=9999).contains(&dt.year()) {
        return Err(VBError::overflow());
    }

    Ok(VBVariant::from_date_serial(serial))
}

/// Days since 1970-01-01 for a proleptic Gregorian date (Howard Hinnant
/// algorithm). Supports any `i64` year, so month/day rollover outside the
/// `jiff` year range still computes the correct serial.
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = y - if m <= 2 { 1 } else { 0 };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::date_serial;
    use crate::error::err_number;
    use crate::value::VBVariant;

    fn ds(y: i32, m: i32, d: i32) -> f64 {
        let result = date_serial(
            &VBVariant::from_long(y),
            &VBVariant::from_long(m),
            &VBVariant::from_long(d),
        )
        .unwrap();
        let VBVariant::Date(serial) = result else {
            panic!("expected a Date variant");
        };
        serial
    }

    fn parts(serial: f64) -> (i16, i8, i8) {
        let dt = crate::value::date_serial_to_datetime(serial).unwrap();
        (dt.year(), dt.month(), dt.day())
    }

    #[test]
    fn basic_construction() {
        assert_eq!(parts(ds(2025, 3, 15)), (2025, 3, 15));
        assert_eq!(parts(ds(1990, 5, 15)), (1990, 5, 15));
        assert_eq!(parts(ds(1899, 12, 30)), (1899, 12, 30));
    }

    #[test]
    fn epoch_serials() {
        assert_eq!(ds(1899, 12, 30), 0.0);
        assert_eq!(ds(1899, 12, 31), 1.0);
        assert_eq!(ds(1900, 1, 1), 2.0);
    }

    #[test]
    fn month_rollover() {
        assert_eq!(parts(ds(2025, 13, 1)), (2026, 1, 1));
        assert_eq!(parts(ds(2025, 0, 1)), (2024, 12, 1));
        assert_eq!(parts(ds(2025, -1, 1)), (2024, 11, 1));
        assert_eq!(parts(ds(2025, 24, 1)), (2026, 12, 1));
    }

    #[test]
    fn day_rollover() {
        assert_eq!(parts(ds(2025, 1, 32)), (2025, 2, 1));
        assert_eq!(parts(ds(2025, 1, 0)), (2024, 12, 31));
        assert_eq!(parts(ds(2025, 1, -1)), (2024, 12, 30));
        assert_eq!(parts(ds(2025, 13, 32)), (2026, 2, 1));
    }

    #[test]
    fn no_clamping_for_invalid_days() {
        assert_eq!(parts(ds(2025, 2, 29)), (2025, 3, 1));
        assert_eq!(parts(ds(2025, 2, 30)), (2025, 3, 2));
        assert_eq!(parts(ds(2025, 2, 0)), (2025, 1, 31));
        assert_eq!(parts(ds(2024, 2, 29)), (2024, 2, 29));
    }

    #[test]
    fn two_digit_years() {
        assert_eq!(parts(ds(25, 1, 1)), (2025, 1, 1));
        assert_eq!(parts(ds(0, 1, 1)), (2000, 1, 1));
        assert_eq!(parts(ds(29, 1, 1)), (2029, 1, 1));
        assert_eq!(parts(ds(30, 1, 1)), (1930, 1, 1));
        assert_eq!(parts(ds(99, 1, 1)), (1999, 1, 1));
    }

    #[test]
    fn midnight_time_portion() {
        assert_eq!(ds(2025, 3, 15).fract(), 0.0);
    }

    #[test]
    fn year_outside_range_is_error_5() {
        let err = date_serial(
            &VBVariant::from_long(10_000),
            &VBVariant::from_long(1),
            &VBVariant::from_long(1),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        let err = date_serial(
            &VBVariant::from_long(-1),
            &VBVariant::from_long(1),
            &VBVariant::from_long(1),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn result_outside_range_is_overflow() {
        let err = date_serial(
            &VBVariant::from_long(100),
            &VBVariant::from_long(1),
            &VBVariant::from_long(0),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::OVERFLOW);
        let err = date_serial(
            &VBVariant::from_long(9999),
            &VBVariant::from_long(12),
            &VBVariant::from_long(32),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::OVERFLOW);
    }

    #[test]
    fn boundary_rollover_stays_valid() {
        assert_eq!(parts(ds(9999, 13, 0)), (9999, 12, 31));
        assert_eq!(parts(ds(100, 1, 1)), (100, 1, 1));
    }

    #[test]
    fn non_numeric_argument_is_error_13() {
        let err = date_serial(
            &VBVariant::from_string("abc"),
            &VBVariant::from_long(1),
            &VBVariant::from_long(1),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn null_argument_is_error_94() {
        let err = date_serial(
            &VBVariant::Null,
            &VBVariant::from_long(1),
            &VBVariant::from_long(1),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn negative_day_within_year_range() {
        assert_eq!(parts(ds(2025, 1, -365)), (2024, 1, 1));
        assert_eq!(parts(ds(2025, 1, -364)), (2024, 1, 2));
    }
}
