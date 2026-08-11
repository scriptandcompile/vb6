//! # `DateDiff` Function
//!
//! Returns a `Variant` (`Long`) specifying the number of time intervals between two specified dates.
//!
//! ## Syntax
//!
//! ```vb
//! DateDiff(interval, date1, date2[, firstdayofweek[, firstweekofyear]])
//! ```
//!
//! ## Parameters
//!
//! - **interval**: Required. `String` expression that is the interval of time you want to use
//!   to calculate the difference between date1 and date2. See Interval Settings for values.
//! - **date1**, **date2**: Required. `Variant` (`Date`) values that you want to use in the calculation.
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
//! Returns a `Long` integer representing the number of intervals between the two dates.
//! The result is positive if date2 is later than date1, negative if date2 is earlier than date1,
//! and zero if they are equal.
//!
//! ## Remarks
//!
//! The `DateDiff` function is used to calculate the difference between two dates in the
//! specified time interval. The function counts the number of interval boundaries crossed
//! between the two dates.
//!
//! **Important Characteristics:**
//!
//! - Returns positive number if date2 > date1 (future date)
//! - Returns negative number if date2 < date1 (past date)
//! - Returns zero if date2 = date1 (same date/time)
//! - Counts interval boundaries, not elapsed time
//! - For "yyyy", crossing from Dec 31 to Jan 1 counts as 1 year
//! - For "m", crossing from Jan 31 to Feb 1 counts as 1 month
//! - For "ww", counts week boundaries (Sunday to Sunday by default)
//! - Day of year ("y") is equivalent to day ("d")
//! - Weekday ("w") is equivalent to day ("d")
//!
//! ## Boundary Counting vs Elapsed Time
//!
//! `DateDiff` counts boundaries crossed, not elapsed time:
//!
//! ```vb
//! ' Year example
//! DateDiff("yyyy", #12/31/2024#, #1/1/2025#)  ' Returns 1 (crossed 1 year boundary)
//! ' But only 1 day elapsed!
//!
//! ' Month example
//! DateDiff("m", #1/31/2025#, #2/1/2025#)  ' Returns 1 (crossed 1 month boundary)
//! ' But only 1 day elapsed!
//!
//! ' Day example (actual elapsed time)
//! DateDiff("d", #1/1/2025#, #1/31/2025#)  ' Returns 30 (30 days elapsed)
//! ```
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! ' Calculate days between dates
//! Dim days As Long
//! days = DateDiff("d", #1/1/2025#, #1/31/2025#)
//! MsgBox "Days: " & days  ' Shows 30
//!
//! ' Calculate months between dates
//! Dim months As Long
//! months = DateDiff("m", #1/15/2025#, #6/15/2025#)
//! MsgBox "Months: " & months  ' Shows 5
//!
//! ' Calculate years between dates
//! Dim years As Long
//! years = DateDiff("yyyy", #1/1/2000#, #1/1/2025#)
//! MsgBox "Years: " & years  ' Shows 25
//! ```
//!
//! ### Age Calculation
//!
//! ```vb
//! Function CalculateAge(birthDate As Date) As Integer
//!     Dim age As Integer
//!     age = DateDiff("yyyy", birthDate, Date)
//!     
//!     ' Adjust if birthday hasn't occurred this year
//!     If DateSerial(Year(Date), Month(birthDate), Day(birthDate)) > Date Then
//!         age = age - 1
//!     End If
//!     
//!     CalculateAge = age
//! End Function
//! ```
//!
//! ### Days Until/Since Event
//!
//! ```vb
//! Function DaysUntilEvent(eventDate As Date) As Long
//!     DaysUntilEvent = DateDiff("d", Date, eventDate)
//! End Function
//!
//! ' Usage
//! Dim daysLeft As Long
//! daysLeft = DaysUntilEvent(#12/25/2025#)
//! If daysLeft > 0 Then
//!     MsgBox daysLeft & " days until Christmas"
//! ElseIf daysLeft < 0 Then
//!     MsgBox "Christmas was " & Abs(daysLeft) & " days ago"
//! Else
//!     MsgBox "Today is Christmas!"
//! End If
//! ```
//!
//! ## Common Patterns
//!
//! ### Elapsed Time Display
//!
//! ```vb
//! Function FormatElapsedTime(startTime As Date, endTime As Date) As String
//!     Dim hours As Long
//!     Dim minutes As Long
//!     Dim seconds As Long
//!     
//!     hours = DateDiff("h", startTime, endTime)
//!     minutes = DateDiff("n", startTime, endTime) Mod 60
//!     seconds = DateDiff("s", startTime, endTime) Mod 60
//!     
//!     FormatElapsedTime = hours & ":" & Format(minutes, "00") & ":" & Format(seconds, "00")
//! End Function
//! ```
//!
//! ### Working Days Calculator
//!
//! ```vb
//! Function CountWorkingDays(startDate As Date, endDate As Date) As Long
//!     Dim dayCount As Long
//!     Dim workDays As Long
//!     Dim currentDate As Date
//!     
//!     dayCount = DateDiff("d", startDate, endDate)
//!     workDays = 0
//!     
//!     For i = 0 To dayCount
//!         currentDate = DateAdd("d", i, startDate)
//!         If Weekday(currentDate) <> vbSaturday And Weekday(currentDate) <> vbSunday Then
//!             workDays = workDays + 1
//!         End If
//!     Next i
//!     
//!     CountWorkingDays = workDays
//! End Function
//! ```
//!
//! ### Overdue Indicator
//!
//! ```vb
//! Function GetOverdueDays(dueDate As Date) As Long
//!     Dim days As Long
//!     days = DateDiff("d", dueDate, Date)
//!     
//!     If days > 0 Then
//!         GetOverdueDays = days  ' Positive = overdue
//!     Else
//!         GetOverdueDays = 0     ' Not overdue
//!     End If
//! End Function
//! ```
//!
//! ### Subscription Status
//!
//! ```vb
//! Function GetSubscriptionStatus(startDate As Date, endDate As Date) As String
//!     Dim daysRemaining As Long
//!     
//!     daysRemaining = DateDiff("d", Date, endDate)
//!     
//!     Select Case daysRemaining
//!         Case Is < 0
//!             GetSubscriptionStatus = "Expired"
//!         Case 0 To 7
//!             GetSubscriptionStatus = "Expiring Soon (" & daysRemaining & " days)"
//!         Case 8 To 30
//!             GetSubscriptionStatus = "Active (" & daysRemaining & " days left)"
//!         Case Else
//!             GetSubscriptionStatus = "Active"
//!     End Select
//! End Function
//! ```
//!
//! ### Quarterly Report Period
//!
//! ```vb
//! Function GetQuartersBetween(startDate As Date, endDate As Date) As Integer
//!     GetQuartersBetween = DateDiff("q", startDate, endDate)
//! End Function
//!
//! ' Check if in same quarter
//! Function InSameQuarter(date1 As Date, date2 As Date) As Boolean
//!     InSameQuarter = (DateDiff("q", date1, date2) = 0)
//! End Function
//! ```
//!
//! ### Meeting Interval Tracker
//!
//! ```vb
//! Function WeeksSinceLastMeeting(lastMeeting As Date) As Long
//!     WeeksSinceLastMeeting = DateDiff("ww", lastMeeting, Date)
//! End Function
//!
//! Function IsMeetingDue(lastMeeting As Date, interval As Integer) As Boolean
//!     IsMeetingDue = (DateDiff("ww", lastMeeting, Date) >= interval)
//! End Function
//! ```
//!
//! ### Time Tracking
//!
//! ```vb
//! Sub LogSessionDuration(startTime As Date, endTime As Date)
//!     Dim hours As Long
//!     Dim minutes As Long
//!     
//!     hours = DateDiff("h", startTime, endTime)
//!     minutes = DateDiff("n", startTime, endTime) - (hours * 60)
//!     
//!     Debug.Print "Session duration: " & hours & "h " & minutes & "m"
//! End Sub
//! ```
//!
//! ### Age Range Categorization
//!
//! ```vb
//! Function GetAgeCategory(birthDate As Date) As String
//!     Dim age As Integer
//!     age = DateDiff("yyyy", birthDate, Date)
//!     
//!     ' Adjust for birthday not yet occurred
//!     If Month(Date) < Month(birthDate) Or _
//!        (Month(Date) = Month(birthDate) And Day(Date) < Day(birthDate)) Then
//!         age = age - 1
//!     End If
//!     
//!     Select Case age
//!         Case 0 To 12
//!             GetAgeCategory = "Child"
//!         Case 13 To 19
//!             GetAgeCategory = "Teenager"
//!         Case 20 To 64
//!             GetAgeCategory = "Adult"
//!         Case Else
//!             GetAgeCategory = "Senior"
//!     End Select
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Complete Time Breakdown
//!
//! ```vb
//! Type TimeBreakdown
//!     Years As Long
//!     Months As Long
//!     Days As Long
//!     Hours As Long
//!     Minutes As Long
//!     Seconds As Long
//! End Type
//!
//! Function GetDetailedDifference(startDate As Date, endDate As Date) As TimeBreakdown
//!     Dim result As TimeBreakdown
//!     Dim tempDate As Date
//!     
//!     ' Calculate years
//!     result.Years = DateDiff("yyyy", startDate, endDate)
//!     tempDate = DateAdd("yyyy", result.Years, startDate)
//!     If tempDate > endDate Then
//!         result.Years = result.Years - 1
//!         tempDate = DateAdd("yyyy", result.Years, startDate)
//!     End If
//!     
//!     ' Calculate months
//!     result.Months = DateDiff("m", tempDate, endDate)
//!     tempDate = DateAdd("m", result.Months, tempDate)
//!     If tempDate > endDate Then
//!         result.Months = result.Months - 1
//!         tempDate = DateAdd("m", result.Months, DateAdd("yyyy", result.Years, startDate))
//!     End If
//!     
//!     ' Calculate remaining time
//!     result.Days = DateDiff("d", tempDate, endDate)
//!     result.Hours = DateDiff("h", tempDate, endDate) Mod 24
//!     result.Minutes = DateDiff("n", tempDate, endDate) Mod 60
//!     result.Seconds = DateDiff("s", tempDate, endDate) Mod 60
//!     
//!     GetDetailedDifference = result
//! End Function
//! ```
//!
//! ### Week Number with Custom First Day
//!
//! ```vb
//! Function GetWeekNumber(dateValue As Date, startDay As VbDayOfWeek) As Long
//!     Dim yearStart As Date
//!     yearStart = DateSerial(Year(dateValue), 1, 1)
//!     GetWeekNumber = DateDiff("ww", yearStart, dateValue, startDay, vbFirstFourDays)
//! End Function
//!
//! ' Usage
//! Dim weekNum As Long
//! weekNum = GetWeekNumber(Date, vbMonday)  ' ISO week number (Monday start)
//! ```
//!
//! ### Performance Timer
//!
//! ```vb
//! Private m_startTime As Date
//!
//! Sub StartTimer()
//!     m_startTime = Now
//! End Sub
//!
//! Function GetElapsedMilliseconds() As Double
//!     Dim seconds As Long
//!     seconds = DateDiff("s", m_startTime, Now)
//!     
//!     ' VB6 doesn't support milliseconds directly
//!     ' This gives seconds as closest approximation
//!     GetElapsedMilliseconds = seconds * 1000
//! End Function
//! ```
//!
//! ### Date Range Validator
//!
//! ```vb
//! Function ValidateDateRange(startDate As Date, endDate As Date, _
//!                          maxDays As Long) As Boolean
//!     Dim daysDiff As Long
//!     
//!     ' Check date order
//!     If startDate > endDate Then
//!         ValidateDateRange = False
//!         Exit Function
//!     End If
//!     
//!     ' Check range limit
//!     daysDiff = DateDiff("d", startDate, endDate)
//!     ValidateDateRange = (daysDiff <= maxDays)
//! End Function
//! ```
//!
//! ### Fiscal Period Calculator
//!
//! ```vb
//! Function GetFiscalPeriodDifference(date1 As Date, date2 As Date, _
//!                                   fiscalYearStart As Integer) As Long
//!     ' Calculate fiscal months between dates
//!     ' fiscalYearStart = month number (e.g., 4 for April)
//!     
//!     Dim adjustedDate1 As Date
//!     Dim adjustedDate2 As Date
//!     
//!     ' Adjust dates to fiscal year basis
//!     adjustedDate1 = DateSerial(Year(date1), Month(date1) - fiscalYearStart + 1, Day(date1))
//!     adjustedDate2 = DateSerial(Year(date2), Month(date2) - fiscalYearStart + 1, Day(date2))
//!     
//!     GetFiscalPeriodDifference = DateDiff("m", adjustedDate1, adjustedDate2)
//! End Function
//! ```
//!
//! ### Batch Date Comparison
//!
//! ```vb
//! Function FindOldestDate(dates() As Date) As Date
//!     Dim i As Integer
//!     Dim oldest As Date
//!     
//!     oldest = dates(LBound(dates))
//!     
//!     For i = LBound(dates) + 1 To UBound(dates)
//!         If DateDiff("d", dates(i), oldest) > 0 Then
//!             oldest = dates(i)
//!         End If
//!     Next i
//!     
//!     FindOldestDate = oldest
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeDateDiff(interval As String, date1 As Variant, _
//!                      date2 As Variant) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     ' Validate dates
//!     If Not IsDate(date1) Or Not IsDate(date2) Then
//!         SafeDateDiff = Null
//!         Exit Function
//!     End If
//!     
//!     ' Validate interval
//!     Select Case LCase(interval)
//!         Case "yyyy", "q", "m", "y", "d", "w", "ww", "h", "n", "s"
//!             SafeDateDiff = DateDiff(interval, CDate(date1), CDate(date2))
//!         Case Else
//!             SafeDateDiff = Null
//!     End Select
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     SafeDateDiff = Null
//! End Function
//! ```
//!
//! ### Common Errors
//!
//! - **Error 5** (Invalid procedure call): Invalid interval string
//! - **Error 13** (Type mismatch): Non-date values passed as date parameters
//! - **Error 6** (Overflow): Result exceeds Long integer range
//!
//! ## Performance Considerations
//!
//! - `DateDiff` is very fast for simple interval calculations
//! - Day ("d") calculations are fastest
//! - Month ("m") and year ("yyyy") require more computation
//! - Week calculations depend on `FirstDayOfWeek` and `FirstWeekOfYear` parameters
//! - For large datasets, cache `DateDiff` results when possible
//!
//! ## Best Practices
//!
//! ### Use Appropriate Intervals
//!
//! ```vb
//! ' Good - Use "d" for exact day count
//! days = DateDiff("d", startDate, endDate)
//!
//! ' Be careful - "yyyy" counts year boundaries, not elapsed years
//! years = DateDiff("yyyy", #12/31/2024#, #1/1/2025#)  ' Returns 1, but only 1 day!
//! ```
//!
//! ### Order Matters
//!
//! ```vb
//! ' Positive result - date2 is in future
//! diff = DateDiff("d", #1/1/2025#, #1/31/2025#)  ' Returns 30
//!
//! ' Negative result - date2 is in past
//! diff = DateDiff("d", #1/31/2025#, #1/1/2025#)  ' Returns -30
//! ```
//!
//! ### Handle Negative Results
//!
//! ```vb
//! Function GetAbsoluteDaysDifference(date1 As Date, date2 As Date) As Long
//!     GetAbsoluteDaysDifference = Abs(DateDiff("d", date1, date2))
//! End Function
//! ```
//!
//! ### Validate Date Order
//!
//! ```vb
//! Function CalculateDuration(startDate As Date, endDate As Date) As Long
//!     If startDate > endDate Then
//!         Err.Raise 5, , "Start date must be before end date"
//!     End If
//!     
//!     CalculateDuration = DateDiff("d", startDate, endDate)
//! End Function
//! ```
//!
//! ## Comparison with Other Functions
//!
//! ### `DateDiff` vs `DateAdd`
//!
//! ```vb
//! ' `DateDiff` - Calculate interval between dates (returns Long)
//! diff = DateDiff("d", #1/1/2025#, #1/31/2025#)  ' Returns 30
//!
//! ' `DateAdd` - Add interval to date (returns Date)
//! newDate = DateAdd("d", 30, #1/1/2025#)  ' Returns #1/31/2025#
//! ```
//!
//! ### `DateDiff` vs Subtraction
//!
//! ```vb
//! ' Subtraction gives days as Double
//! diff = #1/31/2025# - #1/1/2025#  ' Returns 30.0
//!
//! ' DateDiff gives days as Long
//! diff = DateDiff("d", #1/1/2025#, #1/31/2025#)  ' Returns 30
//!
//! ' DateDiff supports other intervals
//! months = DateDiff("m", #1/1/2025#, #6/1/2025#)  ' Returns 5
//! ```
//!
//! ## Limitations
//!
//! - Result must fit in Long integer range (-2,147,483,648 to 2,147,483,647)
//! - Week calculations depend on system or specified first day of week
//! - Counts boundaries crossed, not elapsed time (except for "d", "h", "n", "s")
//! - No built-in support for milliseconds
//! - No built-in support for business day calculations
//! - Cannot directly exclude holidays
//!
//! ## Related Functions
//!
//! - `DateAdd`: Adds a time interval to a date
//! - `DatePart`: Returns a specified part of a date
//! - `DateSerial`: Creates a date from year, month, and day values
//! - `Year`, `Month`, `Day`: Extract date components
//! - `Hour`, `Minute`, `Second`: Extract time components
//! - `Weekday`: Returns the day of the week
//! - `Now`: Returns current date and time
//! - `Date`: Returns current date
//! - `Time`: Returns current time

use crate::error::{VBError, VBResult};
use crate::value::{date_serial_to_datetime, VBVariant};

/// Implementation of the `DateDiff` function.
///
/// VB6 behavior:
/// - counts interval boundaries crossed, not elapsed time (except
///   `d`/`h`/`n`/`s`, which return whole elapsed units truncated toward zero)
/// - `yyyy`, `q`, `m` count year/quarter/month boundaries and ignore the time
///   portion (Dec 31 to Jan 1 is 1 year; Jan 31 to Feb 1 is 1 month)
/// - `y` (day of year) and `w` (weekday) are equivalent to `d` (day)
/// - `ww` counts week boundaries, aligned to `firstdayofweek` (Sunday by
///   default; `vbUseSystem` is treated as Sunday)
/// - `firstweekofyear` is validated but does not affect the result
/// - a `Null` parameter raises error 94 (invalid use of null); a non-date
///   value raises error 13 (type mismatch); an unknown interval raises error
///   5 (invalid procedure call)
/// - results outside the Long range raise error 6 (overflow)
pub fn date_diff(
    interval: &VBVariant,
    date1: &VBVariant,
    date2: &VBVariant,
    first_day_of_week: Option<&VBVariant>,
    first_week_of_year: Option<&VBVariant>,
) -> VBResult<VBVariant> {
    if let Some(v) = first_week_of_year {
        let n = v.as_i32()?;
        if !(0..=3).contains(&n) {
            return Err(VBError::invalid_procedure_call());
        }
    }

    let interval = interval.as_string()?.to_ascii_lowercase();
    let serial1 = date1.as_date_serial()?;
    let serial2 = date2.as_date_serial()?;
    let fdow = first_day(first_day_of_week)?;

    let result = match interval.as_str() {
        "yyyy" => year_diff(serial1, serial2)?,
        "q" => quarter_diff(serial1, serial2)?,
        "m" => month_diff(serial1, serial2)?,
        "d" | "y" | "w" => to_long(serial2.floor() - serial1.floor())?,
        "ww" => week_diff(serial1, serial2, fdow)?,
        "h" => to_long((serial2 - serial1) * 24.0)?,
        "n" => to_long((serial2 - serial1) * 1_440.0)?,
        "s" => to_long((serial2 - serial1) * 86_400.0)?,
        _ => return Err(VBError::invalid_procedure_call()),
    };

    Ok(VBVariant::from_long(result))
}

/// The day serial of a date, erroring with type mismatch when it cannot be
/// represented as a civil datetime.
fn civil(serial: f64) -> VBResult<jiff::civil::DateTime> {
    date_serial_to_datetime(serial).ok_or_else(VBError::type_mismatch)
}

/// Whole years between two date serials (year-boundary count).
fn year_diff(serial1: f64, serial2: f64) -> VBResult<i32> {
    let d1 = civil(serial1)?;
    let d2 = civil(serial2)?;
    Ok(d2.year() as i32 - d1.year() as i32)
}

/// Whole months between two date serials (month-boundary count).
fn month_diff(serial1: f64, serial2: f64) -> VBResult<i32> {
    let d1 = civil(serial1)?;
    let d2 = civil(serial2)?;
    Ok((d2.year() as i32 - d1.year() as i32) * 12 + d2.month() as i32 - d1.month() as i32)
}

/// Whole quarters between two date serials (quarter-boundary count).
fn quarter_diff(serial1: f64, serial2: f64) -> VBResult<i32> {
    let d1 = civil(serial1)?;
    let d2 = civil(serial2)?;
    let q = |dt: &jiff::civil::DateTime| dt.year() as i32 * 4 + (dt.month() as i32 - 1) / 3;
    Ok(q(&d2) - q(&d1))
}

/// Week boundaries crossed between two date serials, aligned to `fdow`
/// (1 = Sunday .. 7 = Saturday, matching the day-serial modulo 7).
fn week_diff(serial1: f64, serial2: f64, fdow: i64) -> VBResult<i32> {
    let d1 = serial1.floor() as i64;
    let d2 = serial2.floor() as i64;
    let count = (d2 - fdow).div_euclid(7) - (d1 - fdow).div_euclid(7);
    to_long(count as f64)
}

/// The `firstdayofweek` constant as a day-serial modulo offset.
fn first_day(first_day_of_week: Option<&VBVariant>) -> VBResult<i64> {
    match first_day_of_week {
        None => Ok(1),
        Some(v) => {
            let n = v.as_i32()?;
            match n {
                0 | 1 => Ok(1),
                2..=7 => Ok(n as i64),
                _ => Err(VBError::invalid_procedure_call()),
            }
        }
    }
}

/// Truncate toward zero and check the result fits in a VB6 `Long`.
fn to_long(v: f64) -> VBResult<i32> {
    let t = v.trunc();
    if !t.is_finite() || t > i32::MAX as f64 || t < i32::MIN as f64 {
        return Err(VBError::overflow());
    }
    Ok(t as i32)
}

#[cfg(test)]
mod tests {
    use super::date_diff;
    use crate::error::err_number;
    use crate::value::VBVariant;

    fn diff(interval: &str, date1: &str, date2: &str) -> i32 {
        let result = date_diff(
            &VBVariant::from_string(interval),
            &VBVariant::from_string(date1),
            &VBVariant::from_string(date2),
            None,
            None,
        )
        .unwrap();
        let VBVariant::Long(n) = result else {
            panic!("expected a Long variant");
        };
        n
    }

    fn diff_fdow(interval: &str, date1: &str, date2: &str, fdow: i32) -> i32 {
        let result = date_diff(
            &VBVariant::from_string(interval),
            &VBVariant::from_string(date1),
            &VBVariant::from_string(date2),
            Some(&VBVariant::from_long(fdow)),
            None,
        )
        .unwrap();
        let VBVariant::Long(n) = result else {
            panic!("expected a Long variant");
        };
        n
    }

    #[test]
    fn days_between_dates() {
        assert_eq!(diff("d", "1/1/2025", "1/31/2025"), 30);
    }

    #[test]
    fn negative_days() {
        assert_eq!(diff("d", "1/31/2025", "1/1/2025"), -30);
    }

    #[test]
    fn same_date_is_zero() {
        assert_eq!(diff("d", "1/15/2025", "1/15/2025"), 0);
    }

    #[test]
    fn day_difference_ignores_time() {
        assert_eq!(diff("d", "1/1/2025 11:00 PM", "1/2/2025 1:00 AM"), 1);
    }

    #[test]
    fn year_boundary_counts_as_one() {
        assert_eq!(diff("yyyy", "12/31/2024", "1/1/2025"), 1);
        assert_eq!(diff("yyyy", "1/1/2025", "12/31/2025"), 0);
    }

    #[test]
    fn years_between() {
        assert_eq!(diff("yyyy", "1/1/2000", "1/1/2025"), 25);
    }

    #[test]
    fn months_between() {
        assert_eq!(diff("m", "1/15/2025", "6/15/2025"), 5);
        assert_eq!(diff("m", "1/31/2025", "2/1/2025"), 1);
    }

    #[test]
    fn negative_months() {
        assert_eq!(diff("m", "2/1/2025", "1/31/2025"), -1);
    }

    #[test]
    fn quarters_between() {
        assert_eq!(diff("q", "1/1/2025", "4/1/2025"), 1);
        assert_eq!(diff("q", "1/1/2025", "1/1/2026"), 4);
        assert_eq!(diff("q", "2/15/2025", "4/1/2025"), 1);
    }

    #[test]
    fn day_of_year_and_weekday_are_day() {
        assert_eq!(diff("y", "1/1/2025", "1/31/2025"), 30);
        assert_eq!(diff("w", "1/1/2025", "1/31/2025"), 30);
    }

    #[test]
    fn time_intervals() {
        assert_eq!(diff("h", "1/15/2025 10:30 AM", "1/15/2025 12:00 PM"), 1);
        assert_eq!(diff("n", "1/15/2025 10:30 AM", "1/15/2025 12:00 PM"), 90);
        assert_eq!(diff("s", "1/15/2025 10:30:00 AM", "1/15/2025 10:31:30 AM"), 90);
    }

    #[test]
    fn negative_time_truncates_toward_zero() {
        assert_eq!(diff("h", "1/15/2025 12:00 PM", "1/15/2025 10:30 AM"), -1);
    }

    #[test]
    fn week_boundaries() {
        assert_eq!(diff("ww", "1/1/2025", "1/5/2025"), 1);
        assert_eq!(diff("ww", "1/1/2025", "1/4/2025"), 0);
        assert_eq!(diff("ww", "1/5/2025", "1/12/2025"), 1);
        assert_eq!(diff("ww", "1/5/2025", "1/1/2025"), -1);
    }

    #[test]
    fn week_boundaries_honor_first_day_of_week() {
        assert_eq!(diff_fdow("ww", "1/4/2025", "1/5/2025", 2), 0);
        assert_eq!(diff_fdow("ww", "1/4/2025", "1/6/2025", 2), 1);
        assert_eq!(diff_fdow("ww", "1/4/2025", "1/5/2025", 0), 1);
    }

    #[test]
    fn first_week_of_year_is_validated() {
        let result = date_diff(
            &VBVariant::from_string("d"),
            &VBVariant::from_string("1/1/2025"),
            &VBVariant::from_string("1/2/2025"),
            None,
            Some(&VBVariant::from_long(3)),
        )
        .unwrap();
        let VBVariant::Long(n) = result else {
            panic!("expected a Long variant");
        };
        assert_eq!(n, 1);

        let err = date_diff(
            &VBVariant::from_string("d"),
            &VBVariant::from_string("1/1/2025"),
            &VBVariant::from_string("1/2/2025"),
            None,
            Some(&VBVariant::from_long(9)),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn invalid_first_day_of_week_is_error_5() {
        let err = date_diff(
            &VBVariant::from_string("d"),
            &VBVariant::from_string("1/1/2025"),
            &VBVariant::from_string("1/2/2025"),
            Some(&VBVariant::from_long(9)),
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn null_date_is_error_94() {
        let err = date_diff(
            &VBVariant::from_string("d"),
            &VBVariant::Null,
            &VBVariant::from_string("1/2/2025"),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn invalid_interval_is_error_5() {
        let err = date_diff(
            &VBVariant::from_string("bogus"),
            &VBVariant::from_string("1/1/2025"),
            &VBVariant::from_string("1/2/2025"),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn non_date_parameter_is_error_13() {
        let err = date_diff(
            &VBVariant::from_string("d"),
            &VBVariant::from_string("not a date"),
            &VBVariant::from_string("1/2/2025"),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn result_outside_long_range_is_overflow() {
        let err = date_diff(
            &VBVariant::from_string("s"),
            &VBVariant::from_string("1/1/2025"),
            &VBVariant::from_string("1/1/9999"),
            None,
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::OVERFLOW);
    }
}
