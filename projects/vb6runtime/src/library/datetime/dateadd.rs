//! # `DateAdd` Function
//!
//! Returns a `Variant` (`Date`) containing a date to which a specified time interval has been added.
//!
//! ## Syntax
//!
//! ```vb
//! DateAdd(interval, number, date)
//! ```
//!
//! ## Parameters
//!
//! - **`interval`**: Required. `String` expression that is the interval of time you want to add.
//!   See the Interval Settings section for valid values.
//! - **`number`**: Required. `Numeric` expression that is the number of intervals you want to add.
//!   Can be positive (to get dates in the future) or negative (to get dates in the past).
//! - **`date`**: Required. `Variant` (`Date`) or literal representing the date to which the interval is added.
//!
//! ## Interval Settings
//!
//! The `interval` parameter can have the following values:
//!
//! | Setting | Description |
//! |---------|-------------|
//! | "yyyy" | Year |
//! | "q" | Quarter |
//! | "m" | Month |
//! | "y" | Day of year |
//! | "d" | Day |
//! | "w" | Weekday |
//! | "ww" | Week of year |
//! | "h" | Hour |
//! | "n" | Minute |
//! | "s" | Second |
//!
//! ## Return Value
//!
//! Returns a `Variant` of subtype `Date` containing the result of adding the specified interval
//! to the given date. Returns Null if any parameter is Null.
//!
//! ## Remarks
//!
//! The `DateAdd` function is used to add or subtract a specified time interval from a date.
//! You can use it to calculate future or past dates relative to a known date.
//!
//! **Important Characteristics:**
//!
//! - Negative numbers subtract intervals (dates in the past)
//! - Positive numbers add intervals (dates in the future)
//! - Handles month-end dates intelligently (e.g., adding 1 month to Jan 31 gives Feb 28/29)
//! - When adding months, if the resulting day doesn't exist, uses last day of month
//! - Respects daylight saving time transitions
//! - Week ("ww") interval treats Sunday as the first day of the week
//! - Weekday ("w") interval is equivalent to day ("d") interval
//! - Day of year ("y") interval is equivalent to day ("d") interval
//!
//! ## Month and Year Calculations
//!
//! When adding months or years, `DateAdd` ensures the result is valid:
//! - Jan 31 + 1 month = Feb 28 (or 29 in leap year)
//! - Jan 31 + 2 months = Mar 31
//! - Aug 31 - 3 months = May 31
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! ' Add days to a date
//! Dim futureDate As Date
//! futureDate = DateAdd("d", 30, Date)
//! MsgBox "30 days from now: " & futureDate
//!
//! ' Subtract days from a date
//! Dim pastDate As Date
//! pastDate = DateAdd("d", -7, Date)
//! MsgBox "A week ago: " & pastDate
//!
//! ' Add months
//! Dim nextMonth As Date
//! nextMonth = DateAdd("m", 1, Date)
//! MsgBox "One month from now: " & nextMonth
//! ```
//!
//! ### Different Time Intervals
//!
//! ```vb
//! Dim startDate As Date
//! startDate = #1/15/2025#
//!
//! ' Add years
//! MsgBox "Next year: " & DateAdd("yyyy", 1, startDate)
//!
//! ' Add quarters
//! MsgBox "Next quarter: " & DateAdd("q", 1, startDate)
//!
//! ' Add weeks
//! MsgBox "Next week: " & DateAdd("ww", 1, startDate)
//!
//! ' Add hours
//! MsgBox "In 6 hours: " & DateAdd("h", 6, startDate)
//!
//! ' Add minutes
//! MsgBox "In 90 minutes: " & DateAdd("n", 90, startDate)
//!
//! ' Add seconds
//! MsgBox "In 3600 seconds: " & DateAdd("s", 3600, startDate)
//! ```
//!
//! ### Working with Past Dates
//!
//! ```vb
//! ' Calculate date 90 days ago
//! Dim quarterAgo As Date
//! quarterAgo = DateAdd("d", -90, Date)
//!
//! ' Calculate date 1 year ago
//! Dim yearAgo As Date
//! yearAgo = DateAdd("yyyy", -1, Date)
//!
//! ' Calculate date 3 months ago
//! Dim threeMonthsAgo As Date
//! threeMonthsAgo = DateAdd("m", -3, Date)
//! ```
//!
//! ## Common Patterns
//!
//! ### Due Date Calculation
//!
//! ```vb
//! Function CalculateDueDate(invoiceDate As Date, terms As Integer) As Date
//!     ' NET 30, NET 60, etc.
//!     CalculateDueDate = DateAdd("d", terms, invoiceDate)
//! End Function
//!
//! ' Usage
//! Dim invoice As Date
//! Dim dueDate As Date
//! invoice = Date
//! dueDate = CalculateDueDate(invoice, 30)  ' Due in 30 days
//! ```
//!
//! ### Age-Based Eligibility
//!
//! ```vb
//! Function IsOldEnough(birthDate As Date, requiredAge As Integer) As Boolean
//!     Dim eligibilityDate As Date
//!     eligibilityDate = DateAdd("yyyy", requiredAge, birthDate)
//!     IsOldEnough = (Date >= eligibilityDate)
//! End Function
//!
//! ' Usage
//! If IsOldEnough(#5/10/2005#, 18) Then
//!     MsgBox "Eligible"
//! End If
//! ```
//!
//! ### Expiration Date Setting
//!
//! ```vb
//! Function SetExpirationDate(startDate As Date, months As Integer) As Date
//!     SetExpirationDate = DateAdd("m", months, startDate)
//! End Function
//!
//! ' Set license to expire in 12 months
//! Dim license As Date
//! license = Date
//! Dim expires As Date
//! expires = SetExpirationDate(license, 12)
//! ```
//!
//! ### Meeting Schedule
//!
//! ```vb
//! Function GetNextMeeting(lastMeeting As Date, interval As String, count As Integer) As Date
//!     GetNextMeeting = DateAdd(interval, count, lastMeeting)
//! End Function
//!
//! ' Weekly meeting
//! Dim nextWeekly As Date
//! nextWeekly = GetNextMeeting(#1/15/2025#, "ww", 1)
//!
//! ' Monthly meeting
//! Dim nextMonthly As Date
//! nextMonthly = GetNextMeeting(#1/15/2025#, "m", 1)
//! ```
//!
//! ### Subscription Renewal
//!
//! ```vb
//! Sub CalculateRenewalDates()
//!     Dim startDate As Date
//!     Dim firstRenewal As Date
//!     Dim secondRenewal As Date
//!     
//!     startDate = Date
//!     firstRenewal = DateAdd("m", 12, startDate)   ' Annual renewal
//!     secondRenewal = DateAdd("m", 24, startDate)  ' Second year
//!     
//!     MsgBox "Start: " & startDate & vbCrLf & _
//!            "First renewal: " & firstRenewal & vbCrLf & _
//!            "Second renewal: " & secondRenewal
//! End Sub
//! ```
//!
//! ### Trial Period End
//!
//! ```vb
//! Function GetTrialEndDate(startDate As Date, trialDays As Integer) As Date
//!     GetTrialEndDate = DateAdd("d", trialDays, startDate)
//! End Function
//!
//! ' 30-day trial
//! Dim trialStart As Date
//! Dim trialEnd As Date
//! trialStart = Date
//! trialEnd = GetTrialEndDate(trialStart, 30)
//! ```
//!
//! ### Report Period Calculation
//!
//! ```vb
//! Function GetReportingPeriod(endDate As Date, months As Integer) As Date
//!     ' Calculate start date by going back specified months
//!     GetReportingPeriod = DateAdd("m", -months, endDate)
//! End Function
//!
//! ' Get start of 6-month period ending today
//! Dim periodStart As Date
//! periodStart = GetReportingPeriod(Date, 6)
//! ```
//!
//! ### Reminder Dates
//!
//! ```vb
//! Sub SetReminders(eventDate As Date)
//!     Dim oneWeekBefore As Date
//!     Dim oneDayBefore As Date
//!     Dim oneHourBefore As Date
//!     
//!     oneWeekBefore = DateAdd("d", -7, eventDate)
//!     oneDayBefore = DateAdd("d", -1, eventDate)
//!     oneHourBefore = DateAdd("h", -1, eventDate)
//!     
//!     ' Schedule reminders...
//! End Sub
//! ```
//!
//! ## Advanced Usage
//!
//! ### Business Days Calculation
//!
//! ```vb
//! Function AddBusinessDays(startDate As Date, days As Integer) As Date
//!     Dim result As Date
//!     Dim daysAdded As Integer
//!     Dim direction As Integer
//!     
//!     result = startDate
//!     direction = Sgn(days)
//!     daysAdded = 0
//!     
//!     Do While Abs(daysAdded) < Abs(days)
//!         result = DateAdd("d", direction, result)
//!         
//!         ' Skip weekends
//!         If Weekday(result) <> vbSaturday And Weekday(result) <> vbSunday Then
//!             daysAdded = daysAdded + direction
//!         End If
//!     Loop
//!     
//!     AddBusinessDays = result
//! End Function
//! ```
//!
//! ### Date Range Generator
//!
//! ```vb
//! Function GenerateDateSeries(startDate As Date, interval As String, _
//!                            count As Integer, step As Integer) As Variant
//!     Dim dates() As Date
//!     Dim i As Integer
//!     
//!     ReDim dates(0 To count - 1)
//!     
//!     For i = 0 To count - 1
//!         dates(i) = DateAdd(interval, i * step, startDate)
//!     Next i
//!     
//!     GenerateDateSeries = dates
//! End Function
//!
//! ' Generate 12 month-end dates
//! Dim monthEnds As Variant
//! monthEnds = GenerateDateSeries(#1/31/2025#, "m", 12, 1)
//! ```
//!
//! ### Fiscal Period Calculator
//!
//! ```vb
//! Function GetFiscalQuarterEnd(fiscalYearStart As Date, quarter As Integer) As Date
//!     Dim quarterStart As Date
//!     Dim quarterEnd As Date
//!     
//!     ' Calculate start of quarter
//!     quarterStart = DateAdd("m", (quarter - 1) * 3, fiscalYearStart)
//!     
//!     ' End is 3 months later minus 1 day
//!     quarterEnd = DateAdd("d", -1, DateAdd("m", 3, quarterStart))
//!     
//!     GetFiscalQuarterEnd = quarterEnd
//! End Function
//! ```
//!
//! ### Recurring Event Calculator
//!
//! ```vb
//! Function GetNextOccurrence(lastOccurrence As Date, frequency As String) As Date
//!     Select Case LCase(frequency)
//!         Case "daily"
//!             GetNextOccurrence = DateAdd("d", 1, lastOccurrence)
//!         Case "weekly"
//!             GetNextOccurrence = DateAdd("ww", 1, lastOccurrence)
//!         Case "biweekly"
//!             GetNextOccurrence = DateAdd("ww", 2, lastOccurrence)
//!         Case "monthly"
//!             GetNextOccurrence = DateAdd("m", 1, lastOccurrence)
//!         Case "quarterly"
//!             GetNextOccurrence = DateAdd("q", 1, lastOccurrence)
//!         Case "annually"
//!             GetNextOccurrence = DateAdd("yyyy", 1, lastOccurrence)
//!         Case Else
//!             GetNextOccurrence = lastOccurrence
//!     End Select
//! End Function
//! ```
//!
//! ### Time Zone Offset (Simple)
//!
//! ```vb
//! Function ConvertToTimeZone(localTime As Date, hourOffset As Integer) As Date
//!     ' Simple timezone conversion (doesn't account for DST)
//!     ConvertToTimeZone = DateAdd("h", hourOffset, localTime)
//! End Function
//!
//! ' Convert EST to PST (3 hours earlier)
//! Dim estTime As Date
//! Dim pstTime As Date
//! estTime = Now
//! pstTime = ConvertToTimeZone(estTime, -3)
//! ```
//!
//! ### Age Calculator with Precision
//!
//! ```vb
//! Function GetExactAge(birthDate As Date) As String
//!     Dim years As Integer
//!     Dim months As Integer
//!     Dim days As Integer
//!     Dim tempDate As Date
//!     
//!     ' Calculate years
//!     tempDate = birthDate
//!     years = 0
//!     Do While DateAdd("yyyy", years + 1, tempDate) <= Date
//!         years = years + 1
//!     Loop
//!     
//!     ' Calculate remaining months
//!     tempDate = DateAdd("yyyy", years, birthDate)
//!     months = 0
//!     Do While DateAdd("m", months + 1, tempDate) <= Date
//!         months = months + 1
//!     Loop
//!     
//!     ' Calculate remaining days
//!     tempDate = DateAdd("m", months, DateAdd("yyyy", years, birthDate))
//!     days = DateDiff("d", tempDate, Date)
//!     
//!     GetExactAge = years & " years, " & months & " months, " & days & " days"
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeDateAdd(interval As String, number As Long, _
//!                     dateValue As Date) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     ' Validate interval
//!     Select Case LCase(interval)
//!         Case "yyyy", "q", "m", "y", "d", "w", "ww", "h", "n", "s"
//!             SafeDateAdd = DateAdd(interval, number, dateValue)
//!         Case Else
//!             SafeDateAdd = Null  ' Invalid interval
//!     End Select
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     SafeDateAdd = Null  ' Return Null on error
//! End Function
//! ```
//!
//! ### Common Errors
//!
//! - **Error 5** (Invalid procedure call): Invalid interval string
//! - **Error 13** (Type mismatch): Non-numeric number or non-date date parameter
//! - **Error 6** (Overflow): Result date is outside valid range (100-9999 AD)
//!
//! ## Performance Considerations
//!
//! - `DateAdd` is efficient for single date calculations
//! - For large date ranges, consider pre-calculating frequently used values
//! - Month and year additions are slightly slower than day additions
//! - Interval string comparison is case-insensitive but exact strings are faster
//!
//! ## Best Practices
//!
//! ### Use Named Constants for Intervals
//!
//! ```vb
//! ' Define constants for clarity
//! Const INTERVAL_YEAR As String = "yyyy"
//! Const INTERVAL_MONTH As String = "m"
//! Const INTERVAL_DAY As String = "d"
//! Const INTERVAL_HOUR As String = "h"
//!
//! ' Use in code
//! nextYear = DateAdd(INTERVAL_YEAR, 1, Date)
//! ```
//!
//! ### Validate Input Dates
//!
//! ```vb
//! Function AddDaysToDate(startDate As Variant, days As Integer) As Date
//!     If Not IsDate(startDate) Then
//!         Err.Raise 13, , "Invalid date"
//!     End If
//!     
//!     AddDaysToDate = DateAdd("d", days, CDate(startDate))
//! End Function
//! ```
//!
//! ### Handle Month-End Edge Cases
//!
//! ```vb
//! ' Be aware of month-end behavior
//! Dim jan31 As Date
//! jan31 = #1/31/2025#
//!
//! ' Adding 1 month gives Feb 28 (or 29)
//! Dim result As Date
//! result = DateAdd("m", 1, jan31)  ' Feb 28, 2025
//!
//! ' Adding 2 months gives Mar 31
//! result = DateAdd("m", 2, jan31)  ' Mar 31, 2025
//! ```
//!
//! ## Comparison with Other Date Functions
//!
//! ### `DateAdd` vs `DateDiff`
//!
//! ```vb
//! ' DateAdd - Adds interval to date, returns new date
//! Dim future As Date
//! future = DateAdd("d", 30, Date)
//!
//! ' DateDiff - Calculates interval between dates, returns number
//! Dim difference As Long
//! difference = DateDiff("d", Date, future)  ' Returns 30
//! ```
//!
//! ### `DateAdd` vs Simple Arithmetic
//!
//! ```vb
//! ' Simple arithmetic works for days
//! Dim tomorrow As Date
//! tomorrow = Date + 1  ' Same as DateAdd("d", 1, Date)
//!
//! ' But DateAdd is better for months/years
//! Dim nextMonth As Date
//! nextMonth = DateAdd("m", 1, Date)  ' Handles month-end correctly
//! ```
//!
//! ## Limitations
//!
//! - Date range limited to January 1, 100 through December 31, 9999
//! - No built-in support for business day calculations
//! - Doesn't handle holidays automatically
//! - Week starts on Sunday (cannot be customized)
//! - No built-in timezone support
//! - Daylight saving time handled by system, results may vary
//!
//! ## Related Functions
//!
//! - `DateDiff`: Returns the number of intervals between two dates
//! - `DatePart`: Returns a specified part of a date
//! - `DateSerial`: Creates a date from year, month, and day values
//! - `DateValue`: Converts a string to a date
//! - `Year`, `Month`, `Day`: Extract date components
//! - `Hour`, `Minute`, `Second`: Extract time components
//! - `Now`: Returns current date and time
//! - `Date`: Returns current date
//! - `Time`: Returns current time

use crate::error::{VBError, VBResult};
use crate::value::{date_serial_to_datetime, VBVariant};

/// Implementation of the `DateAdd` function.
///
/// VB6 behavior:
/// - a `Null` parameter returns `Null`
/// - month/quarter/year additions clamp to the last day of the target month
///   (Jan 31 + 1 month = Feb 28/29; Jan 31 + 2 months = Mar 31)
/// - "y" (day of year) and "w" (weekday) are equivalent to "d" (day)
/// - fractional month/quarter/year counts are rounded to the nearest integer
/// - the time portion is preserved for month/quarter/year additions
/// - results outside the 100-9999 AD range raise error 6 (overflow)
/// - an unknown interval raises error 5 (invalid procedure call)
pub fn date_add(interval: &VBVariant, number: &VBVariant, date: &VBVariant) -> VBResult<VBVariant> {
    if interval.is_null() || number.is_null() || date.is_null() {
        return Ok(VBVariant::Null);
    }

    let interval = interval.as_string()?.to_ascii_lowercase();
    let number = number.as_f64()?;
    let serial = date.as_date_serial()?;

    let result = match interval.as_str() {
        "yyyy" => add_months(serial, number * 12.0)?,
        "q" => add_months(serial, number * 3.0)?,
        "m" => add_months(serial, number)?,
        "y" | "d" | "w" => serial + number,
        "ww" => serial + number * 7.0,
        "h" => serial + number / 24.0,
        "n" => serial + number / 1_440.0,
        "s" => serial + number / 86_400.0,
        _ => return Err(VBError::invalid_procedure_call()),
    };

    Ok(VBVariant::from_date_serial(validated(result)?))
}

/// Add a number of months to a date serial, preserving the time portion and
/// clamping the day to the last day of the target month.
fn add_months(serial: f64, months: f64) -> VBResult<f64> {
    use jiff::civil::Date;
    use jiff::{SpanRelativeTo, Unit};

    let dt = date_serial_to_datetime(serial).ok_or_else(VBError::type_mismatch)?;
    let months = months.round() as i64;
    let total = dt.year() as i64 * 12 + dt.month() as i64 - 1 + months;
    let year = total.div_euclid(12);
    let month = total.rem_euclid(12) + 1;
    let day = (dt.day() as i64).min(days_in_month(year, month)) as i8;

    let date = Date::new(year as i16, month as i8, day).map_err(|_| VBError::overflow())?;
    let base = Date::new(1899, 12, 30).expect("valid epoch");
    let days = date
        .since(base)
        .map_err(|_| VBError::overflow())?
        .total((Unit::Day, SpanRelativeTo::days_are_24_hours()))
        .map_err(|_| VBError::overflow())?;
    let frac =
        (dt.hour() as f64 * 3600.0 + dt.minute() as f64 * 60.0 + dt.second() as f64) / 86_400.0;
    Ok(days + frac)
}

/// Number of days in `month` of `year` (proleptic Gregorian).
fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
            if leap {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Reject date serials outside the VB6-supported 100-9999 AD range.
fn validated(serial: f64) -> VBResult<f64> {
    let dt = date_serial_to_datetime(serial).ok_or_else(VBError::overflow)?;
    if (100..=9999).contains(&dt.year()) {
        Ok(serial)
    } else {
        Err(VBError::overflow())
    }
}

#[cfg(test)]
mod tests {
    use super::date_add;
    use crate::error::err_number;
    use crate::value::VBVariant;

    fn add(interval: &str, number: f64, date: &str) -> f64 {
        let result = date_add(
            &VBVariant::from_string(interval),
            &VBVariant::from_double(number),
            &VBVariant::from_string(date),
        )
        .unwrap();
        let VBVariant::Date(serial) = result else {
            panic!("expected a Date variant");
        };
        serial
    }

    fn parts(serial: f64) -> (i16, i8, i8, i8, i8, i8) {
        let dt = crate::value::date_serial_to_datetime(serial).unwrap();
        (
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second(),
        )
    }

    #[test]
    fn adds_days() {
        assert_eq!(parts(add("d", 30.0, "1/15/2025")), (2025, 2, 14, 0, 0, 0));
    }

    #[test]
    fn subtracts_days() {
        assert_eq!(parts(add("d", -7.0, "1/15/2025")), (2025, 1, 8, 0, 0, 0));
    }

    #[test]
    fn day_of_year_and_weekday_match_days() {
        assert_eq!(add("y", 30.0, "1/15/2025"), add("d", 30.0, "1/15/2025"));
        assert_eq!(add("w", 30.0, "1/15/2025"), add("d", 30.0, "1/15/2025"));
    }

    #[test]
    fn adds_weeks() {
        assert_eq!(parts(add("ww", 1.0, "1/15/2025")), (2025, 1, 22, 0, 0, 0));
    }

    #[test]
    fn adds_months() {
        assert_eq!(parts(add("m", 1.0, "1/15/2025")), (2025, 2, 15, 0, 0, 0));
        assert_eq!(parts(add("m", 3.0, "1/15/2025")), (2025, 4, 15, 0, 0, 0));
    }

    #[test]
    fn adds_quarters() {
        assert_eq!(parts(add("q", 1.0, "1/15/2025")), (2025, 4, 15, 0, 0, 0));
        assert_eq!(parts(add("q", -1.0, "1/15/2025")), (2024, 10, 15, 0, 0, 0));
    }

    #[test]
    fn adds_years() {
        assert_eq!(parts(add("yyyy", 1.0, "1/15/2025")), (2026, 1, 15, 0, 0, 0));
        assert_eq!(
            parts(add("yyyy", -1.0, "1/15/2025")),
            (2024, 1, 15, 0, 0, 0)
        );
    }

    #[test]
    fn month_end_clamps_to_last_day() {
        assert_eq!(parts(add("m", 1.0, "1/31/2025")), (2025, 2, 28, 0, 0, 0));
        assert_eq!(parts(add("m", 2.0, "1/31/2025")), (2025, 3, 31, 0, 0, 0));
        assert_eq!(parts(add("m", -3.0, "8/31/2025")), (2025, 5, 31, 0, 0, 0));
    }

    #[test]
    fn leap_year_additions() {
        assert_eq!(parts(add("yyyy", 1.0, "2/29/2024")), (2025, 2, 28, 0, 0, 0));
        assert_eq!(parts(add("yyyy", 4.0, "2/29/2024")), (2028, 2, 29, 0, 0, 0));
    }

    #[test]
    fn adds_time_intervals() {
        assert_eq!(
            parts(add("h", 6.0, "1/15/2025 12:00 PM")),
            (2025, 1, 15, 18, 0, 0)
        );
        assert_eq!(parts(add("n", 90.0, "1/15/2025")), (2025, 1, 15, 1, 30, 0));
        assert_eq!(parts(add("s", 3600.0, "1/15/2025")), (2025, 1, 15, 1, 0, 0));
    }

    #[test]
    fn time_rolls_over_midnight() {
        assert_eq!(
            parts(add("h", 12.0, "1/15/2025 6:00 PM")),
            (2025, 1, 16, 6, 0, 0)
        );
    }

    #[test]
    fn month_add_preserves_time() {
        assert_eq!(
            parts(add("m", 1.0, "1/15/2025 3:30 PM")),
            (2025, 2, 15, 15, 30, 0)
        );
    }

    #[test]
    fn fractional_months_round() {
        assert_eq!(parts(add("m", 1.5, "1/15/2025")), (2025, 3, 15, 0, 0, 0));
    }

    #[test]
    fn fractional_days_keep_time() {
        assert_eq!(
            parts(add("d", 0.5, "1/15/2025 12:00 AM")),
            (2025, 1, 15, 12, 0, 0)
        );
    }

    #[test]
    fn null_parameter_returns_null() {
        let result = date_add(
            &VBVariant::Null,
            &VBVariant::from_double(1.0),
            &VBVariant::from_string("1/15/2025"),
        )
        .unwrap();
        assert!(result.is_null());
        let result = date_add(
            &VBVariant::from_string("d"),
            &VBVariant::Null,
            &VBVariant::from_string("1/15/2025"),
        )
        .unwrap();
        assert!(result.is_null());
        let result = date_add(
            &VBVariant::from_string("d"),
            &VBVariant::from_double(1.0),
            &VBVariant::Null,
        )
        .unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn invalid_interval_is_error_5() {
        let err = date_add(
            &VBVariant::from_string("bogus"),
            &VBVariant::from_double(1.0),
            &VBVariant::from_string("1/15/2025"),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn out_of_range_result_is_overflow() {
        let err = date_add(
            &VBVariant::from_string("yyyy"),
            &VBVariant::from_double(10_000.0),
            &VBVariant::from_string("1/15/2025"),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::OVERFLOW);
    }

    #[test]
    fn interval_match_is_case_insensitive() {
        assert_eq!(parts(add("YYYY", 1.0, "1/15/2025")), (2026, 1, 15, 0, 0, 0));
    }
}
