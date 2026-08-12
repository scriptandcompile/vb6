//! # `DatePart` Function
//!
//! Returns a `Variant` (`Integer`) containing the specified part of a given date.
//!
//! ## Syntax
//!
//! ```vb
//! DatePart(interval, date[, firstdayofweek[, firstweekofyear]])
//! ```
//!
//! ## Parameters
//!
//! - **interval**: Required. `String` expression that is the interval of time you want to return.
//!   See the Interval Settings section for valid values.
//! - **date**: Required. `Variant` (`Date`) value that you want to evaluate.
//! - **firstdayofweek**: Optional. Constant that specifies the first day of the week.
//!   If not specified, Sunday is assumed. See `FirstDayOfWeek` Constants.
//! - **firstweekofyear**: Optional. Constant that specifies the first week of the year.
//!   If not specified, the first week is assumed to be the week containing January 1.
//!   See `FirstWeekOfYear` Constants.
//!
//! ## Interval Settings
//!
//! The `interval` parameter can have the following values:
//!
//! | Setting | Description | Return Range |
//! |---------|-------------|--------------|
//! | "yyyy" | Year | 100-9999 |
//! | "q" | Quarter | 1-4 |
//! | "m" | Month | 1-12 |
//! | "y" | Day of year | 1-366 |
//! | "d" | Day | 1-31 |
//! | "w" | Weekday | 1-7 (Sunday=1) |
//! | "ww" | Week of year | 1-53 |
//! | "h" | Hour | 0-23 |
//! | "n" | Minute | 0-59 |
//! | "s" | Second | 0-59 |
//!
//! ## `FirstDayOfWeek` Constants
//!
//! | Constant | Value | Description |
//! |----------|-------|-------------|
//! | vbUseSystem | 0 | Use system setting |
//! | vbSunday | 1 | Sunday (default) |
//! | vbMonday | 2 | Monday |
//! | vbTuesday | 3 | Tuesday |
//! | vbWednesday | 4 | Wednesday |
//! | vbThursday | 5 | Thursday |
//! | vbFriday | 6 | Friday |
//! | vbSaturday | 7 | Saturday |
//!
//! ## `FirstWeekOfYear` Constants
//!
//! | Constant | Value | Description |
//! |----------|-------|-------------|
//! | vbUseSystem | 0 | Use system setting |
//! | vbFirstJan1 | 1 | Start with week containing January 1 (default) |
//! | vbFirstFourDays | 2 | Start with week having at least 4 days in new year |
//! | vbFirstFullWeek | 3 | Start with first full week of the year |
//!
//! ## Return Value
//!
//! Returns an `Integer` representing the specified part of the date. Returns `Null` if the date is `Null`.
//!
//! ## Remarks
//!
//! The `DatePart` function is used to extract a specific component from a date value.
//! It's particularly useful for date-based calculations, filtering, and grouping operations.
//!
//! **Important Characteristics:**
//!
//! - More flexible than `Year()`, `Month()`, or `Day()` functions.
//! - Can extract quarter, week, and day of year.
//! - Weekday numbering depends on `firstdayofweek` parameter.
//! - Week numbering depends on `firstweekofyear` parameter.
//! - Hours use 24-hour format (0-23).
//! - Sunday is 1 by default for weekday ("w").
//! - Compatible with SQL Server's `DATEPART` function
//!
//! ## Equivalent Simple Functions
//!
//! Some intervals have equivalent dedicated functions:
//! - `DatePart("yyyy", date)` = `Year(date)`
//! - `DatePart("m", date)` = `Month(date)`
//! - `DatePart("d", date)` = `Day(date)`
//! - `DatePart("w", date)` = `Weekday(date)`
//! - `DatePart("h", date)` = `Hour(date)`
//! - `DatePart("n", date)` = `Minute(date)`
//! - `DatePart("s", date)` = `Second(date)`
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! Dim testDate As Date
//! testDate = #3/15/2025 14:30:45#
//!
//! ' Extract various parts
//! MsgBox "Year: " & DatePart("yyyy", testDate)      ' 2025
//! MsgBox "Quarter: " & DatePart("q", testDate)      ' 1
//! MsgBox "Month: " & DatePart("m", testDate)        ' 3
//! MsgBox "Day: " & DatePart("d", testDate)          ' 15
//! MsgBox "Day of Year: " & DatePart("y", testDate)  ' 74
//! MsgBox "Weekday: " & DatePart("w", testDate)      ' Varies by day
//! MsgBox "Week: " & DatePart("ww", testDate)        ' Week number
//! MsgBox "Hour: " & DatePart("h", testDate)         ' 14
//! MsgBox "Minute: " & DatePart("n", testDate)       ' 30
//! MsgBox "Second: " & DatePart("s", testDate)       ' 45
//! ```
//!
//! ### Quarter Calculation
//!
//! ```vb
//! Function GetQuarter(dateValue As Date) As Integer
//!     GetQuarter = DatePart("q", dateValue)
//! End Function
//!
//! ' Usage
//! Dim currentQuarter As Integer
//! currentQuarter = GetQuarter(Date)
//! MsgBox "We are in Q" & currentQuarter
//! ```
//!
//! ### Week Number
//!
//! ```vb
//! Function GetWeekNumber(dateValue As Date) As Integer
//!     ' ISO week number (Monday start, 4-day rule)
//!     GetWeekNumber = DatePart("ww", dateValue, vbMonday, vbFirstFourDays)
//! End Function
//! ```
//!
//! ## Common Patterns
//!
//! ### Fiscal Quarter Determination
//!
//! ```vb
//! Function GetFiscalQuarter(dateValue As Date, fiscalYearStart As Integer) As Integer
//!     ' fiscalYearStart is the month number (e.g., 4 for April)
//!     Dim currentMonth As Integer
//!     Dim adjustedMonth As Integer
//!     
//!     currentMonth = DatePart("m", dateValue)
//!     adjustedMonth = currentMonth - fiscalYearStart + 1
//!     
//!     If adjustedMonth <= 0 Then
//!         adjustedMonth = adjustedMonth + 12
//!     End If
//!     
//!     GetFiscalQuarter = Int((adjustedMonth - 1) / 3) + 1
//! End Function
//! ```
//!
//! ### Group By Time Period
//!
//! ```vb
//! Function GroupByPeriod(dateValue As Date, period As String) As String
//!     Select Case LCase(period)
//!         Case "year"
//!             GroupByPeriod = CStr(DatePart("yyyy", dateValue))
//!         Case "quarter"
//!             GroupByPeriod = DatePart("yyyy", dateValue) & "-Q" & DatePart("q", dateValue)
//!         Case "month"
//!             GroupByPeriod = DatePart("yyyy", dateValue) & "-" & Format(DatePart("m", dateValue), "00")
//!         Case "week"
//!             GroupByPeriod = DatePart("yyyy", dateValue) & "-W" & Format(DatePart("ww", dateValue), "00")
//!         Case Else
//!             GroupByPeriod = Format(dateValue, "yyyy-mm-dd")
//!     End Select
//! End Function
//! ```
//!
//! ### Day Name from Weekday
//!
//! ```vb
//! Function GetDayName(dateValue As Date) As String
//!     Select Case DatePart("w", dateValue)
//!         Case 1: GetDayName = "Sunday"
//!         Case 2: GetDayName = "Monday"
//!         Case 3: GetDayName = "Tuesday"
//!         Case 4: GetDayName = "Wednesday"
//!         Case 5: GetDayName = "Thursday"
//!         Case 6: GetDayName = "Friday"
//!         Case 7: GetDayName = "Saturday"
//!     End Select
//! End Function
//! ```
//!
//! ### Time of Day Category
//!
//! ```vb
//! Function GetTimeOfDay(dateValue As Date) As String
//!     Dim hour As Integer
//!     hour = DatePart("h", dateValue)
//!     
//!     Select Case hour
//!         Case 0 To 5
//!             GetTimeOfDay = "Night"
//!         Case 6 To 11
//!             GetTimeOfDay = "Morning"
//!         Case 12 To 17
//!             GetTimeOfDay = "Afternoon"
//!         Case 18 To 23
//!             GetTimeOfDay = "Evening"
//!     End Select
//! End Function
//! ```
//!
//! ### Business Hour Check
//!
//! ```vb
//! Function IsBusinessHours(checkTime As Date) As Boolean
//!     Dim hour As Integer
//!     Dim weekday As Integer
//!     
//!     hour = DatePart("h", checkTime)
//!     weekday = DatePart("w", checkTime)
//!     
//!     ' Monday-Friday, 9 AM - 5 PM
//!     If weekday >= 2 And weekday <= 6 Then  ' Mon-Fri
//!         If hour >= 9 And hour < 17 Then
//!             IsBusinessHours = True
//!         End If
//!     End If
//! End Function
//! ```
//!
//! ### Month Name Lookup
//!
//! ```vb
//! Function GetMonthName(dateValue As Date) As String
//!     Dim monthNames As Variant
//!     Dim monthNum As Integer
//!     
//!     monthNames = Array("January", "February", "March", "April", "May", "June", _
//!                       "July", "August", "September", "October", "November", "December")
//!     
//!     monthNum = DatePart("m", dateValue)
//!     GetMonthName = monthNames(monthNum - 1)
//! End Function
//! ```
//!
//! ### Quarter End Date
//!
//! ```vb
//! Function GetQuarterEnd(dateValue As Date) As Date
//!     Dim quarter As Integer
//!     Dim year As Integer
//!     Dim endMonth As Integer
//!     
//!     quarter = DatePart("q", dateValue)
//!     year = DatePart("yyyy", dateValue)
//!     endMonth = quarter * 3
//!     
//!     GetQuarterEnd = DateSerial(year, endMonth + 1, 0)  ' Last day of quarter
//! End Function
//! ```
//!
//! ### Data Binning by Hour
//!
//! ```vb
//! Function GetHourBucket(timestamp As Date) As String
//!     Dim hour As Integer
//!     hour = DatePart("h", timestamp)
//!     GetHourBucket = Format(hour, "00") & ":00"
//! End Function
//!
//! ' Use for grouping log entries
//! Sub AnalyzeLogs()
//!     Dim entry As Date
//!     Dim bucket As String
//!     
//!     For Each entry In logEntries
//!         bucket = GetHourBucket(entry)
//!         hourCounts(bucket) = hourCounts(bucket) + 1
//!     Next
//! End Sub
//! ```
//!
//! ## Advanced Usage
//!
//! ### ISO 8601 Week Number
//!
//! ```vb
//! Function GetISOWeekNumber(dateValue As Date) As Integer
//!     ' ISO 8601 week number: Monday start, 4-day rule
//!     GetISOWeekNumber = DatePart("ww", dateValue, vbMonday, vbFirstFourDays)
//! End Function
//!
//! Function GetISOYear(dateValue As Date) As Integer
//!     ' Year for ISO week (may differ from calendar year)
//!     Dim weekNum As Integer
//!     Dim month As Integer
//!     
//!     weekNum = GetISOWeekNumber(dateValue)
//!     month = DatePart("m", dateValue)
//!     
//!     If month = 1 And weekNum > 51 Then
//!         GetISOYear = DatePart("yyyy", dateValue) - 1
//!     ElseIf month = 12 And weekNum = 1 Then
//!         GetISOYear = DatePart("yyyy", dateValue) + 1
//!     Else
//!         GetISOYear = DatePart("yyyy", dateValue)
//!     End If
//! End Function
//! ```
//!
//! ### Dynamic Date Grouping
//!
//! ```vb
//! Function GetDateKey(dateValue As Date, granularity As String) As String
//!     Dim year As Integer
//!     Dim month As Integer
//!     Dim day As Integer
//!     Dim week As Integer
//!     Dim quarter As Integer
//!     
//!     year = DatePart("yyyy", dateValue)
//!     
//!     Select Case LCase(granularity)
//!         Case "year"
//!             GetDateKey = CStr(year)
//!         
//!         Case "quarter"
//!             quarter = DatePart("q", dateValue)
//!             GetDateKey = year & "Q" & quarter
//!         
//!         Case "month"
//!             month = DatePart("m", dateValue)
//!             GetDateKey = year & Format(month, "00")
//!         
//!         Case "week"
//!             week = DatePart("ww", dateValue, vbMonday)
//!             GetDateKey = year & "W" & Format(week, "00")
//!         
//!         Case "day"
//!             month = DatePart("m", dateValue)
//!             day = DatePart("d", dateValue)
//!             GetDateKey = year & Format(month, "00") & Format(day, "00")
//!         
//!         Case Else
//!             GetDateKey = Format(dateValue, "yyyymmdd")
//!     End Select
//! End Function
//! ```
//!
//! ### Custom Calendar System
//!
//! ```vb
//! Type CustomCalendar
//!     Year As Integer
//!     Period As Integer
//!     Week As Integer
//!     Day As Integer
//! End Type
//!
//! Function ConvertToCustomCalendar(dateValue As Date) As CustomCalendar
//!     Dim cal As CustomCalendar
//!     Dim yearStart As Date
//!     Dim dayOfYear As Integer
//!     
//!     cal.Year = DatePart("yyyy", dateValue)
//!     
//!     ' 13 periods of 4 weeks each
//!     yearStart = DateSerial(cal.Year, 1, 1)
//!     dayOfYear = DatePart("y", dateValue)
//!     
//!     cal.Week = Int((dayOfYear - 1) / 7) + 1
//!     cal.Period = Int((cal.Week - 1) / 4) + 1
//!     cal.Day = DatePart("w", dateValue, vbMonday)
//!     
//!     ConvertToCustomCalendar = cal
//! End Function
//! ```
//!
//! ### Time Series Aggregation
//!
//! ```vb
//! Function AggregateByInterval(dates() As Date, values() As Double, _
//!                             interval As String) As Collection
//!     Dim results As New Collection
//!     Dim i As Long
//!     Dim key As String
//!     Dim total As Double
//!     Dim count As Long
//!     
//!     For i = LBound(dates) To UBound(dates)
//!         key = GetDateKey(dates(i), interval)
//!         
//!         On Error Resume Next
//!         total = results(key)
//!         If Err.Number <> 0 Then
//!             results.Add values(i), key
//!         Else
//!             results.Remove key
//!             results.Add total + values(i), key
//!         End If
//!         On Error GoTo 0
//!     Next i
//!     
//!     Set AggregateByInterval = results
//! End Function
//! ```
//!
//! ### Shift Schedule Detector
//!
//! ```vb
//! Function GetShift(timestamp As Date) As String
//!     Dim hour As Integer
//!     Dim weekday As Integer
//!     
//!     hour = DatePart("h", timestamp)
//!     weekday = DatePart("w", timestamp)
//!     
//!     ' Weekend check
//!     If weekday = 1 Or weekday = 7 Then
//!         GetShift = "Weekend"
//!         Exit Function
//!     End If
//!     
//!     ' Shift determination
//!     Select Case hour
//!         Case 6 To 13
//!             GetShift = "Morning Shift"
//!         Case 14 To 21
//!             GetShift = "Afternoon Shift"
//!         Case Else
//!             GetShift = "Night Shift"
//!     End Select
//! End Function
//! ```
//!
//! ### Calendar Week Display
//!
//! ```vb
//! Function FormatCalendarWeek(dateValue As Date, Optional useISO As Boolean = False) As String
//!     Dim year As Integer
//!     Dim week As Integer
//!     
//!     If useISO Then
//!         year = GetISOYear(dateValue)
//!         week = GetISOWeekNumber(dateValue)
//!     Else
//!         year = DatePart("yyyy", dateValue)
//!         week = DatePart("ww", dateValue)
//!     End If
//!     
//!     FormatCalendarWeek = year & "-W" & Format(week, "00")
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeDatePart(interval As String, dateValue As Variant) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     ' Validate date
//!     If Not IsDate(dateValue) Then
//!         SafeDatePart = Null
//!         Exit Function
//!     End If
//!     
//!     ' Validate interval
//!     Select Case LCase(interval)
//!         Case "yyyy", "q", "m", "y", "d", "w", "ww", "h", "n", "s"
//!             SafeDatePart = DatePart(interval, CDate(dateValue))
//!         Case Else
//!             SafeDatePart = Null
//!     End Select
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     SafeDatePart = Null
//! End Function
//! ```
//!
//! ### Common Errors
//!
//! - **Error 5** (Invalid procedure call): Invalid interval string
//! - **Error 13** (Type mismatch): Non-date value passed as date parameter
//!
//! ## Performance Considerations
//!
//! - `DatePart` is efficient for single extractions
//! - For multiple parts from same date, consider using dedicated functions:
//!   ```vb
//!   ' Less efficient
//!   y = DatePart("yyyy", d)
//!   m = DatePart("m", d)
//!   d = DatePart("d", d)
//!   
//!   ' More efficient
//!   y = Year(d)
//!   m = Month(d)
//!   d = Day(d)
//!   ```
//! - Week calculations are more expensive than other intervals
//! - Cache results when processing large datasets
//!
//! ## Best Practices
//!
//! ### Use Named Constants
//!
//! ```vb
//! ' Define interval constants
//! Const INTERVAL_YEAR As String = "yyyy"
//! Const INTERVAL_QUARTER As String = "q"
//! Const INTERVAL_MONTH As String = "m"
//! Const INTERVAL_WEEK As String = "ww"
//!
//! ' Use in code
//! quarter = DatePart(INTERVAL_QUARTER, Date)
//! ```
//!
//! ### Prefer Specific Functions for Simple Cases
//!
//! ```vb
//! ' Good - Use specific function
//! y = Year(someDate)
//!
//! ' Less clear - Using DatePart
//! y = DatePart("yyyy", someDate)
//! ```
//!
//! ### Be Aware of Weekday Numbering
//!
//! ```vb
//! ' Default: Sunday = 1
//! day = DatePart("w", Date)
//!
//! ' Explicit: Monday = 1
//! day = DatePart("w", Date, vbMonday)
//! ```
//!
//! ## Comparison with Other Functions
//!
//! ### `DatePart` vs Dedicated Functions
//!
//! ```vb
//! ' DatePart - Flexible, supports all intervals
//! quarter = DatePart("q", Date)
//! dayOfYear = DatePart("y", Date)
//!
//! ' Dedicated - Simpler, more readable for common cases
//! year = Year(Date)
//! month = Month(Date)
//! day = Day(Date)
//! weekday = Weekday(Date)
//! ```
//!
//! ## Limitations
//!
//! - No millisecond support
//! - Week numbering can be confusing with different standards (ISO vs US)
//! - Quarter calculation doesn't support fiscal quarters directly
//! - No built-in locale-aware day/month names
//! - `FirstWeekOfYear` affects week numbering interpretation
//!
//! ## Related Functions
//!
//! - `Year`: Returns the year part of a date
//! - `Month`: Returns the month part of a date
//! - `Day`: Returns the day part of a date
//! - `Weekday`: Returns the day of the week
//! - `Hour`: Returns the hour part of a time
//! - `Minute`: Returns the minute part of a time
//! - `Second`: Returns the second part of a time
//! - `DateAdd`: Adds a time interval to a date
//! - `DateDiff`: Returns the difference between two dates
//! - `DateSerial`: Creates a date from year, month, and day values
//! - `Format`: Formats a date as a string (alternative for custom formatting)

use crate::error::{VBError, VBResult};
use crate::value::{date_serial_to_datetime, VBVariant};

/// Implementation of the `DatePart` function.
///
/// VB6 behavior:
/// - a `Null` date returns `Null`
/// - "w" (weekday) numbers days relative to `firstdayofweek`
///   (Sunday = 1 by default; `vbMonday` makes Monday = 1)
/// - "ww" (week of year) uses `firstdayofweek` and `firstweekofyear`:
///   `vbFirstJan1` starts week 1 on the week containing Jan 1, `vbFirstFourDays`
///   starts week 1 on the first week with at least four days in the new year,
///   and `vbFirstFullWeek` starts week 1 on the first full week (dates before
///   it are week 0, matching VB6); `vbUseSystem` is treated as Sunday /
///   `vbFirstJan1`
/// - an unknown interval raises error 5 (invalid procedure call); a non-date
///   value raises error 13 (type mismatch)
pub fn date_part(
    interval: &VBVariant,
    date: &VBVariant,
    first_day_of_week: Option<&VBVariant>,
    first_week_of_year: Option<&VBVariant>,
) -> VBResult<VBVariant> {
    if date.is_null() {
        return Ok(VBVariant::Null);
    }

    let fdow = first_day(first_day_of_week)?;
    let fwoy = match first_week_of_year {
        None => 1,
        Some(v) => {
            let n = v.as_i32()?;
            if !(0..=3).contains(&n) {
                return Err(VBError::invalid_procedure_call());
            }
            n
        }
    };

    let interval = interval.as_string()?.to_ascii_lowercase();
    let serial = date.as_date_serial()?;
    let dt = date_serial_to_datetime(serial).ok_or_else(VBError::type_mismatch)?;

    let result = match interval.as_str() {
        "yyyy" => dt.year() as i32,
        "q" => (dt.month() as i32 - 1) / 3 + 1,
        "m" => dt.month() as i32,
        "y" => dt.date().day_of_year() as i32,
        "d" => dt.day() as i32,
        "w" => weekday(&dt, fdow),
        "ww" => week_of_year(&dt, fdow, fwoy),
        "h" => dt.hour() as i32,
        "n" => dt.minute() as i32,
        "s" => dt.second() as i32,
        _ => return Err(VBError::invalid_procedure_call()),
    };

    Ok(VBVariant::from_integer(result as i16))
}

/// The `firstdayofweek` constant, normalized to a 1=Sunday..7=Saturday offset.
pub(crate) fn first_day(first_day_of_week: Option<&VBVariant>) -> VBResult<i32> {
    match first_day_of_week {
        None => Ok(1),
        Some(v) => {
            let n = v.as_i32()?;
            match n {
                0 | 1 => Ok(1),
                2..=7 => Ok(n),
                _ => Err(VBError::invalid_procedure_call()),
            }
        }
    }
}

/// Day of the week relative to `fdow` (1 = `fdow` itself .. 7).
pub(crate) fn weekday(dt: &jiff::civil::DateTime, fdow: i32) -> i32 {
    let sunday_based = dt.date().weekday().to_sunday_one_offset() as i32 - 1;
    (sunday_based - (fdow - 1)).rem_euclid(7) + 1
}

/// Week of the year within the calendar year containing `dt`.
fn week_of_year(dt: &jiff::civil::DateTime, fdow: i32, fwoy: i32) -> i32 {
    use jiff::civil::Date;

    let date_serial = day_serial(dt.date());
    let jan1_serial = day_serial(Date::new(dt.year(), 1, 1).expect("valid january 1"));
    let t = fdow as i64;

    // The day serial of the week start that contains Jan 1.
    let prev_fdow = jan1_serial as i64 - (jan1_serial as i64 - t).rem_euclid(7);
    let week1_start = match fwoy {
        2 => {
            let days_in_new_year = 7 - (jan1_serial as i64 - prev_fdow);
            if days_in_new_year >= 4 {
                prev_fdow
            } else {
                prev_fdow + 7
            }
        }
        3 => {
            if (jan1_serial as i64 - t).rem_euclid(7) == 0 {
                jan1_serial as i64
            } else {
                prev_fdow + 7
            }
        }
        _ => prev_fdow,
    };

    (date_serial as i64 - week1_start).div_euclid(7) as i32 + 1
}

/// The day serial of a civil date (1899-12-30 == 0).
fn day_serial(date: jiff::civil::Date) -> f64 {
    use jiff::civil::Date;
    use jiff::{SpanRelativeTo, Unit};

    let base = Date::new(1899, 12, 30).expect("valid epoch");
    date.since(base)
        .ok()
        .and_then(|span| span.total((Unit::Day, SpanRelativeTo::days_are_24_hours())).ok())
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::date_part;
    use crate::error::err_number;
    use crate::value::VBVariant;

    fn part(interval: &str, date: &str) -> i16 {
        let result = date_part(
            &VBVariant::from_string(interval),
            &VBVariant::from_string(date),
            None,
            None,
        )
        .unwrap();
        let VBVariant::Integer(n) = result else {
            panic!("expected an Integer variant");
        };
        n
    }

    fn part_opts(interval: &str, date: &str, fdow: i32, fwoy: i32) -> i16 {
        let result = date_part(
            &VBVariant::from_string(interval),
            &VBVariant::from_string(date),
            Some(&VBVariant::from_long(fdow)),
            Some(&VBVariant::from_long(fwoy)),
        )
        .unwrap();
        let VBVariant::Integer(n) = result else {
            panic!("expected an Integer variant");
        };
        n
    }

    #[test]
    fn basic_parts() {
        assert_eq!(part("yyyy", "3/15/2025 14:30:45"), 2025);
        assert_eq!(part("q", "3/15/2025"), 1);
        assert_eq!(part("m", "3/15/2025"), 3);
        assert_eq!(part("d", "3/15/2025"), 15);
        assert_eq!(part("h", "3/15/2025 2:30 PM"), 14);
        assert_eq!(part("n", "3/15/2025 2:30 PM"), 30);
        assert_eq!(part("s", "3/15/2025 2:30:45 PM"), 45);
    }

    #[test]
    fn quarters() {
        assert_eq!(part("q", "1/15/2025"), 1);
        assert_eq!(part("q", "4/15/2025"), 2);
        assert_eq!(part("q", "7/15/2025"), 3);
        assert_eq!(part("q", "10/15/2025"), 4);
        assert_eq!(part("q", "12/31/2025"), 4);
    }

    #[test]
    fn day_of_year() {
        assert_eq!(part("y", "1/1/2025"), 1);
        assert_eq!(part("y", "3/15/2025"), 74);
        assert_eq!(part("y", "12/31/2025"), 365);
        assert_eq!(part("y", "12/31/2024"), 366);
    }

    #[test]
    fn weekday_defaults_to_sunday_first() {
        assert_eq!(part("w", "1/1/2025"), 4);
        assert_eq!(part("w", "1/5/2025"), 1);
        assert_eq!(part("w", "1/4/2025"), 7);
    }

    #[test]
    fn weekday_honors_first_day_of_week() {
        assert_eq!(part_opts("w", "1/6/2025", 2, 1), 1);
        assert_eq!(part_opts("w", "1/12/2025", 2, 1), 7);
        assert_eq!(part_opts("w", "1/5/2025", 0, 1), 1);
    }

    #[test]
    fn week_of_year_first_jan_1() {
        assert_eq!(part("ww", "1/1/2025"), 1);
        assert_eq!(part("ww", "1/4/2025"), 1);
        assert_eq!(part("ww", "1/5/2025"), 2);
        assert_eq!(part("ww", "12/31/2025"), 53);
    }

    #[test]
    fn week_of_year_first_four_days() {
        assert_eq!(part_opts("ww", "1/1/2025", 1, 2), 1);
        assert_eq!(part_opts("ww", "1/5/2025", 1, 2), 2);
    }

    #[test]
    fn week_of_year_iso() {
        assert_eq!(part_opts("ww", "1/1/2025", 2, 2), 1);
        assert_eq!(part_opts("ww", "12/29/2025", 2, 2), 53);
    }

    #[test]
    fn week_of_year_first_full_week() {
        assert_eq!(part_opts("ww", "1/1/2025", 1, 3), 0);
        assert_eq!(part_opts("ww", "1/5/2025", 1, 3), 1);
        assert_eq!(part_opts("ww", "1/12/2025", 1, 3), 2);
    }

    #[test]
    fn midnight_hour_is_zero() {
        assert_eq!(part("h", "1/15/2025"), 0);
    }

    #[test]
    fn null_date_returns_null() {
        let result = date_part(
            &VBVariant::from_string("yyyy"),
            &VBVariant::Null,
            None,
            None,
        )
        .unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn invalid_interval_is_error_5() {
        let err = date_part(
            &VBVariant::from_string("bogus"),
            &VBVariant::from_string("1/15/2025"),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn non_date_parameter_is_error_13() {
        let err = date_part(
            &VBVariant::from_string("yyyy"),
            &VBVariant::from_string("not a date"),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn invalid_first_day_of_week_is_error_5() {
        let err = date_part(
            &VBVariant::from_string("d"),
            &VBVariant::from_string("1/15/2025"),
            Some(&VBVariant::from_long(9)),
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn invalid_first_week_of_year_is_error_5() {
        let err = date_part(
            &VBVariant::from_string("d"),
            &VBVariant::from_string("1/15/2025"),
            None,
            Some(&VBVariant::from_long(9)),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }
}
