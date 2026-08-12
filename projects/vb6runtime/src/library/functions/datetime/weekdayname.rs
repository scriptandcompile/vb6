//! VB6 `WeekdayName` Function
//!
//! The `WeekdayName` function returns a string indicating the specified day of the week.
//!
//! ## Syntax
//! ```vb6
//! WeekdayName(weekday[, abbreviate[, firstdayofweek]])
//! ```
//!
//! ## Parameters
//! - `weekday`: Required. The numeric designation for the day of the week. Numeric value of each day depends on the `firstdayofweek` setting.
//! - `abbreviate`: Optional. Boolean value that indicates if the weekday name is to be abbreviated. If omitted, the default is False (not abbreviated).
//! - `firstdayofweek`: Optional. Numeric value indicating the first day of the week. See Settings section for values.
//!
//! ### `FirstDayOfWeek` Constants
//! - `vbUseSystemDayOfWeek` (0): Use National Language Support (NLS) API setting
//! - `vbSunday` (1): Sunday (default)
//! - `vbMonday` (2): Monday
//! - `vbTuesday` (3): Tuesday
//! - `vbWednesday` (4): Wednesday
//! - `vbThursday` (5): Thursday
//! - `vbFriday` (6): Friday
//! - `vbSaturday` (7): Saturday
//!
//! ## Returns
//! Returns a `String` containing the name of the specified day of the week. The string is localized based on the system's regional settings.
//!
//! ## Remarks
//! The `WeekdayName` function provides localized day names:
//!
//! - **Localization**: Returns day names according to system locale (e.g., "Monday" in English, "Lundi" in French)
//! - **Abbreviation**: When `abbreviate` is True, returns shortened form (e.g., "Mon" instead of "Monday")
//! - **Weekday parameter**: Numeric value from 1 to 7
//! - **Default first day**: Sunday (vbSunday = 1) if `firstdayofweek` not specified
//! - **Consistency with Weekday**: Use same `firstdayofweek` value as Weekday function for consistency
//! - **System locale**: Output language depends on Windows regional settings
//! - **Case sensitivity**: Returned string typically has proper capitalization
//! - **Abbreviation length**: Typically 3 characters in English, varies by locale
//!
//! ### Understanding Weekday Parameter
//! The `weekday` parameter's meaning depends on `firstdayofweek`:
//! - If `firstdayofweek` is `vbSunday` (default): 1=Sunday, 2=Monday, 3=Tuesday, etc.
//! - If `firstdayofweek` is `vbMonday`: 1=Monday, 2=Tuesday, 3=Wednesday, etc.
//! - The number always refers to the position in the week, starting from the specified first day
//!
//! ### Abbreviation Examples (English locale)
//! - Full: "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
//! - Abbreviated: "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
//!
//! ### Combining with Weekday Function
//! ```vb6
//! ' Get the name of today's day
//! dayName = WeekdayName(Weekday(Date))
//! ```
//!
//! ## Typical Uses
//! 1. **Display Day Names**: Show user-friendly day names in UI
//! 2. **Report Headers**: Label columns or sections with day names
//! 3. **Calendar Applications**: Display calendar grid headers
//! 4. **Localized Applications**: Provide day names in user's language
//! 5. **Schedule Display**: Show schedule with day names
//! 6. **Date Formatting**: Create custom date format strings
//! 7. **Dropdown Lists**: Populate day selection dropdowns
//! 8. **Log Files**: Human-readable date information in logs
//!
//! ## Basic Examples
//!
//! ### Example 1: Get Today's Day Name
//! ```vb6
//! Sub ShowTodayName()
//!     Dim todayName As String
//!     todayName = WeekdayName(Weekday(Date))
//!     MsgBox "Today is " & todayName
//! End Sub
//! ```
//!
//! ### Example 2: Display All Day Names
//! ```vb6
//! Sub ListAllDays()
//!     Dim i As Integer
//!     For i = 1 To 7
//!         Debug.Print WeekdayName(i)
//!     Next i
//! End Sub
//! ```
//!
//! ### Example 3: Abbreviated Day Names
//! ```vb6
//! Function GetAbbreviatedDayName(dayNumber As Integer) As String
//!     GetAbbreviatedDayName = WeekdayName(dayNumber, True)
//! End Function
//!
//! ' Usage:
//! Debug.Print GetAbbreviatedDayName(1) ' Prints "Sun" (if vbSunday is first day)
//! ```
//!
//! ### Example 4: Create Calendar Header
//! ```vb6
//! Function CreateCalendarHeader(Optional abbreviated As Boolean = True) As String
//!     Dim i As Integer
//!     Dim header As String
//!     
//!     header = ""
//!     For i = 1 To 7
//!         header = header & WeekdayName(i, abbreviated, vbSunday) & vbTab
//!     Next i
//!     
//!     CreateCalendarHeader = header
//! End Function
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: Format Date with Day Name
//! ```vb6
//! Function FormatDateWithDayName(dt As Date) As String
//!     FormatDateWithDayName = WeekdayName(Weekday(dt)) & ", " & Format$(dt, "mmmm d, yyyy")
//! End Function
//! ```
//!
//! ### Pattern 2: Get All Day Names Array
//! ```vb6
//! Function GetDayNamesArray(Optional abbreviated As Boolean = False) As String()
//!     Dim days(1 To 7) As String
//!     Dim i As Integer
//!     
//!     For i = 1 To 7
//!         days(i) = WeekdayName(i, abbreviated)
//!     Next i
//!     
//!     GetDayNamesArray = days
//! End Function
//! ```
//!
//! ### Pattern 3: ISO Week Day Names (Monday First)
//! ```vb6
//! Function GetISODayName(dayNumber As Integer, Optional abbreviated As Boolean = False) As String
//!     GetISODayName = WeekdayName(dayNumber, abbreviated, vbMonday)
//! End Function
//! ```
//!
//! ### Pattern 4: Populate `ComboBox` with Days
//! ```vb6
//! Sub PopulateDayCombo(combo As ComboBox)
//!     Dim i As Integer
//!     combo.Clear
//!     For i = 1 To 7
//!         combo.AddItem WeekdayName(i)
//!     Next i
//! End Sub
//! ```
//!
//! ### Pattern 5: Get Day Initials
//! ```vb6
//! Function GetDayInitial(dayNumber As Integer) As String
//!     GetDayInitial = Left$(WeekdayName(dayNumber, True), 1)
//! End Function
//! ```
//!
//! ### Pattern 6: Create Week Schedule Header
//! ```vb6
//! Function CreateWeekSchedule() As String
//!     Dim i As Integer
//!     Dim schedule As String
//!     
//!     schedule = "Week Schedule:" & vbCrLf
//!     For i = 1 To 7
//!         schedule = schedule & WeekdayName(i) & ": _____" & vbCrLf
//!     Next i
//!     
//!     CreateWeekSchedule = schedule
//! End Function
//! ```
//!
//! ### Pattern 7: Format Event Description
//! ```vb6
//! Function FormatEventDescription(eventDate As Date, eventName As String) As String
//!     FormatEventDescription = eventName & " on " & _
//!                             WeekdayName(Weekday(eventDate)) & ", " & _
//!                             Format$(eventDate, "mmmm d")
//! End Function
//! ```
//!
//! ### Pattern 8: Get Weekday vs Weekend Label
//! ```vb6
//! Function GetDayTypeLabel(dt As Date) As String
//!     Dim dayNum As Integer
//!     dayNum = Weekday(dt)
//!     
//!     If dayNum = vbSaturday Or dayNum = vbSunday Then
//!         GetDayTypeLabel = WeekdayName(dayNum) & " (Weekend)"
//!     Else
//!         GetDayTypeLabel = WeekdayName(dayNum) & " (Weekday)"
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 9: Create Pivot Headers
//! ```vb6
//! Function CreatePivotDayHeaders() As Variant
//!     Dim headers(1 To 7) As String
//!     Dim i As Integer
//!     
//!     For i = 1 To 7
//!         headers(i) = WeekdayName(i, True, vbMonday)
//!     Next i
//!     
//!     CreatePivotDayHeaders = headers
//! End Function
//! ```
//!
//! ### Pattern 10: Conditional Day Name Display
//! ```vb6
//! Function GetDisplayDayName(dt As Date, useAbbreviation As Boolean) As String
//!     GetDisplayDayName = WeekdayName(Weekday(dt), useAbbreviation)
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Calendar Header Generator Class
//! ```vb6
//! ' Class: CalendarHeaderGenerator
//! ' Generates calendar headers with configurable options
//! Option Explicit
//!
//! Private m_FirstDayOfWeek As VbDayOfWeek
//! Private m_Abbreviated As Boolean
//! Private m_Separator As String
//!
//! Public Sub Initialize(Optional firstDay As VbDayOfWeek = vbSunday, _
//!                      Optional abbreviated As Boolean = True, _
//!                      Optional separator As String = " ")
//!     m_FirstDayOfWeek = firstDay
//!     m_Abbreviated = abbreviated
//!     m_Separator = separator
//! End Sub
//!
//! Public Function GenerateHeader() As String
//!     Dim i As Integer
//!     Dim header As String
//!     
//!     header = ""
//!     For i = 1 To 7
//!         If i > 1 Then header = header & m_Separator
//!         header = header & WeekdayName(i, m_Abbreviated, m_FirstDayOfWeek)
//!     Next i
//!     
//!     GenerateHeader = header
//! End Function
//!
//! Public Function GenerateHeaderArray() As String()
//!     Dim headers(1 To 7) As String
//!     Dim i As Integer
//!     
//!     For i = 1 To 7
//!         headers(i) = WeekdayName(i, m_Abbreviated, m_FirstDayOfWeek)
//!     Next i
//!     
//!     GenerateHeaderArray = headers
//! End Function
//!
//! Public Function GetDayName(dayNumber As Integer) As String
//!     If dayNumber < 1 Or dayNumber > 7 Then
//!         Err.Raise 5, , "Day number must be between 1 and 7"
//!     End If
//!     GetDayName = WeekdayName(dayNumber, m_Abbreviated, m_FirstDayOfWeek)
//! End Function
//! ```
//!
//! ### Example 2: Date Formatter Module
//! ```vb6
//! ' Module: DateFormatter
//! ' Advanced date formatting with day names
//! Option Explicit
//!
//! Public Function FormatLongDate(dt As Date) As String
//!     FormatLongDate = WeekdayName(Weekday(dt)) & ", " & _
//!                     Format$(dt, "mmmm d, yyyy")
//! End Function
//!
//! Public Function FormatShortDate(dt As Date) As String
//!     FormatShortDate = WeekdayName(Weekday(dt), True) & " " & _
//!                      Format$(dt, "mm/dd/yyyy")
//! End Function
//!
//! Public Function FormatScheduleDate(dt As Date) As String
//!     FormatScheduleDate = WeekdayName(Weekday(dt), True) & ", " & _
//!                         Format$(dt, "mmm d")
//! End Function
//!
//! Public Function FormatCalendarDate(dt As Date) As String
//!     FormatCalendarDate = WeekdayName(Weekday(dt)) & vbCrLf & _
//!                         Format$(dt, "d")
//! End Function
//!
//! Public Function GetDayWithOrdinal(dt As Date) As String
//!     Dim dayNum As Integer
//!     Dim suffix As String
//!     
//!     dayNum = Day(dt)
//!     
//!     Select Case dayNum
//!         Case 1, 21, 31
//!             suffix = "st"
//!         Case 2, 22
//!             suffix = "nd"
//!         Case 3, 23
//!             suffix = "rd"
//!         Case Else
//!             suffix = "th"
//!     End Select
//!     
//!     GetDayWithOrdinal = WeekdayName(Weekday(dt)) & ", " & _
//!                        MonthName(Month(dt)) & " " & dayNum & suffix
//! End Function
//! ```
//!
//! ### Example 3: Schedule Analyzer Class
//! ```vb6
//! ' Class: ScheduleAnalyzer
//! ' Analyzes schedules and provides day-based insights
//! Option Explicit
//!
//! Public Function GetDayDistributionReport(dates() As Date) As String
//!     Dim dayCounts(1 To 7) As Integer
//!     Dim i As Long
//!     Dim report As String
//!     Dim dayNum As Integer
//!     
//!     ' Count occurrences
//!     For i = LBound(dates) To UBound(dates)
//!         dayNum = Weekday(dates(i))
//!         dayCounts(dayNum) = dayCounts(dayNum) + 1
//!     Next i
//!     
//!     ' Build report
//!     report = "Day Distribution:" & vbCrLf
//!     For i = 1 To 7
//!         report = report & WeekdayName(i) & ": " & dayCounts(i) & vbCrLf
//!     Next i
//!     
//!     GetDayDistributionReport = report
//! End Function
//!
//! Public Function GetMostCommonDay(dates() As Date) As String
//!     Dim dayCounts(1 To 7) As Integer
//!     Dim i As Long
//!     Dim maxCount As Integer
//!     Dim maxDay As Integer
//!     Dim dayNum As Integer
//!     
//!     For i = LBound(dates) To UBound(dates)
//!         dayNum = Weekday(dates(i))
//!         dayCounts(dayNum) = dayCounts(dayNum) + 1
//!     Next i
//!     
//!     maxCount = 0
//!     maxDay = 1
//!     For i = 1 To 7
//!         If dayCounts(i) > maxCount Then
//!             maxCount = dayCounts(i)
//!             maxDay = i
//!         End If
//!     Next i
//!     
//!     GetMostCommonDay = WeekdayName(maxDay)
//! End Function
//!
//! Public Function CreateDaySummary(dates() As Date) As Collection
//!     Dim summary As New Collection
//!     Dim i As Integer
//!     
//!     For i = 1 To 7
//!         summary.Add 0, WeekdayName(i)
//!     Next i
//!     
//!     Dim dt As Variant
//!     For Each dt In dates
//!         Dim dayName As String
//!         dayName = WeekdayName(Weekday(dt))
//!         summary.Remove dayName
//!         summary.Add summary(dayName) + 1, dayName
//!     Next dt
//!     
//!     Set CreateDaySummary = summary
//! End Function
//! ```
//!
//! ### Example 4: Localization Helper Module
//! ```vb6
//! ' Module: LocalizationHelper
//! ' Helps with localized day name handling
//! Option Explicit
//!
//! Public Function GetLocalizedDayNames(Optional abbreviated As Boolean = False, _
//!                                     Optional firstDay As VbDayOfWeek = vbSunday) As String()
//!     Dim names(1 To 7) As String
//!     Dim i As Integer
//!     
//!     For i = 1 To 7
//!         names(i) = WeekdayName(i, abbreviated, firstDay)
//!     Next i
//!     
//!     GetLocalizedDayNames = names
//! End Function
//!
//! Public Function CreateDayNameLookup(Optional abbreviated As Boolean = False) As Collection
//!     Dim lookup As New Collection
//!     Dim i As Integer
//!     
//!     For i = 1 To 7
//!         lookup.Add WeekdayName(i, abbreviated), CStr(i)
//!     Next i
//!     
//!     Set CreateDayNameLookup = lookup
//! End Function
//!
//! Public Function FindDayNumber(dayName As String) As Integer
//!     Dim i As Integer
//!     
//!     For i = 1 To 7
//!         If UCase$(WeekdayName(i)) = UCase$(dayName) Then
//!             FindDayNumber = i
//!             Exit Function
//!         End If
//!         If UCase$(WeekdayName(i, True)) = UCase$(dayName) Then
//!             FindDayNumber = i
//!             Exit Function
//!         End If
//!     Next i
//!     
//!     FindDayNumber = 0 ' Not found
//! End Function
//!
//! Public Function IsDayNameValid(dayName As String) As Boolean
//!     IsDayNameValid = (FindDayNumber(dayName) > 0)
//! End Function
//!
//! Public Function NormalizeDayName(dayName As String) As String
//!     Dim dayNum As Integer
//!     dayNum = FindDayNumber(dayName)
//!     
//!     If dayNum > 0 Then
//!         NormalizeDayName = WeekdayName(dayNum)
//!     Else
//!         NormalizeDayName = ""
//!     End If
//! End Function
//! ```
//!
//! ## Error Handling
//! The `WeekdayName` function can raise the following errors:
//!
//! - **Error 5 (Invalid procedure call or argument)**: If `weekday` is less than 1 or greater than 7
//! - **Error 5 (Invalid procedure call or argument)**: If `firstdayofweek` is not between 0 and 7
//! - **Error 13 (Type mismatch)**: If arguments are not of correct type
//!
//! ## Performance Notes
//! - Very fast operation - simple lookup/formatting
//! - Constant time O(1) complexity
//! - No significant performance difference between abbreviated and full forms
//! - Can be called repeatedly without performance concerns
//! - Consider caching day name arrays if used frequently in loops
//!
//! ## Best Practices
//! 1. **Use consistent firstdayofweek** with Weekday function to avoid confusion
//! 2. **Cache day name arrays** if populating lists or grids repeatedly
//! 3. **Handle localization** - day names will differ based on system locale
//! 4. **Document expectations** about which day is first in week
//! 5. **Use abbreviation** parameter for space-constrained displays
//! 6. **Validate weekday range** (1-7) before calling
//! 7. **Consider `MonthName`** for consistent date formatting patterns
//! 8. **Test with different locales** if application is internationalized
//! 9. **Use named constants** (vbMonday, etc.) for clarity
//! 10. **Combine with Format$** for custom date displays
//!
//! ## Comparison Table
//!
//! | Function | Purpose | Returns | Localized |
//! |----------|---------|---------|-----------|
//! | `WeekdayName` | Get day name | String | Yes |
//! | `Weekday` | Get day number | Integer (1-7) | No |
//! | `MonthName` | Get month name | String | Yes |
//! | `Format$` | Format date | String | Partially |
//!
//! ## Platform Notes
//! - Available in VB6, VBA, and `VBScript`
//! - Behavior consistent across platforms
//! - Output localized based on system regional settings
//! - Abbreviation format varies by locale
//! - English (US): "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
//! - Other languages will return appropriate translations
//!
//! ## Limitations
//! - Cannot customize day name output (uses system locale)
//! - Abbreviation length not configurable (determined by locale)
//! - No way to get day name in specific language (uses system setting)
//! - Cannot get day names for custom calendars (e.g., Hebrew, Islamic)
//! - No built-in way to get single-letter day abbreviations
//! - Cannot specify case (capitalization) of returned string

use crate::error::{VBError, VBResult};
use crate::library::functions::datetime::datepart::first_day;
use crate::value::{VBString, VBVariant};

const FULL_NAMES: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

const ABBREVIATED_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// Implementation of the `WeekdayName` function.
///
/// VB6 behavior:
/// - returns a string with the day name based on `weekday` (1-7) and `firstdayofweek`
/// - `abbreviate` is optional, defaults to False (full name)
/// - `firstdayofweek` accepts vbUseSystem (0) through vbSaturday (7); any other
///   value raises error 5 (invalid procedure call)
/// - a `weekday` outside 1..=7 raises error 5
/// - a non-numeric weekday raises error 13 (type mismatch); Null raises
///   error 94 (invalid use of Null)
pub fn weekday_name(
    weekday: &VBVariant,
    abbreviate: Option<&VBVariant>,
    firstdayofweek: Option<&VBVariant>,
) -> VBResult<VBString> {
    let weekday = weekday.as_i32()?;
    if !(1..=7).contains(&weekday) {
        return Err(VBError::invalid_procedure_call());
    }

    let fdow = first_day(firstdayofweek)?;
    let abbreviate = match abbreviate {
        None => false,
        Some(v) => v.as_bool()?,
    };

    // Convert weekday number (relative to fdow) to 0-indexed day where 0=Sunday
    let sunday_based = ((weekday - 1) + (fdow - 1)).rem_euclid(7) as usize;
    let name = if abbreviate {
        ABBREVIATED_NAMES[sunday_based]
    } else {
        FULL_NAMES[sunday_based]
    };

    Ok(VBString::from(name))
}

#[cfg(test)]
mod tests {
    use super::{weekday_name, ABBREVIATED_NAMES, FULL_NAMES};
    use crate::error::err_number;
    use crate::value::VBVariant;

    fn wn(input: &VBVariant) -> String {
        weekday_name(input, None, None)
            .unwrap()
            .as_str()
            .to_string()
    }

    fn wn_opts(
        input: &VBVariant,
        abbreviate: Option<&VBVariant>,
        fdow: Option<&VBVariant>,
    ) -> String {
        weekday_name(input, abbreviate, fdow)
            .unwrap()
            .as_str()
            .to_string()
    }

    #[test]
    fn full_names_sunday_first() {
        for (i, name) in FULL_NAMES.iter().enumerate() {
            let day_num = i as i32 + 1;
            assert_eq!(wn(&VBVariant::from_long(day_num)), *name);
        }
    }

    #[test]
    fn abbreviated_names_sunday_first() {
        for (i, name) in ABBREVIATED_NAMES.iter().enumerate() {
            let day_num = i as i32 + 1;
            assert_eq!(
                wn_opts(
                    &VBVariant::from_long(day_num),
                    Some(&VBVariant::from_bool(true)),
                    None
                ),
                *name
            );
        }
    }

    #[test]
    fn abbreviate_defaults_to_false() {
        assert_eq!(wn(&VBVariant::from_long(1)), "Sunday");
    }

    #[test]
    fn honors_first_day_of_week_monday() {
        // With vbMonday (2): 1=Monday, 2=Tuesday, ..., 7=Sunday
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(1),
                None,
                Some(&VBVariant::from_long(2))
            ),
            "Monday"
        );
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(2),
                None,
                Some(&VBVariant::from_long(2))
            ),
            "Tuesday"
        );
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(7),
                None,
                Some(&VBVariant::from_long(2))
            ),
            "Sunday"
        );
    }

    #[test]
    fn honors_first_day_of_week_tuesday() {
        // With vbTuesday (3): 1=Tuesday, ..., 7=Monday
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(1),
                None,
                Some(&VBVariant::from_long(3))
            ),
            "Tuesday"
        );
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(7),
                None,
                Some(&VBVariant::from_long(3))
            ),
            "Monday"
        );
    }

    #[test]
    fn abbreviated_with_first_day_monday() {
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(1),
                Some(&VBVariant::from_bool(true)),
                Some(&VBVariant::from_long(2))
            ),
            "Mon"
        );
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(7),
                Some(&VBVariant::from_bool(true)),
                Some(&VBVariant::from_long(2))
            ),
            "Sun"
        );
    }

    #[test]
    fn use_system_defaults_to_sunday() {
        // vbUseSystem (0) defaults to Sunday
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(1),
                None,
                Some(&VBVariant::from_long(0))
            ),
            "Sunday"
        );
    }

    #[test]
    fn weekday_out_of_range_is_error_5() {
        for bad in [0, -1, 8] {
            let err = weekday_name(&VBVariant::from_long(bad), None, None).unwrap_err();
            assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        }
    }

    #[test]
    fn invalid_first_day_of_week_is_error_5() {
        for fdow in [8, -1] {
            let err = weekday_name(
                &VBVariant::from_long(1),
                None,
                Some(&VBVariant::from_long(fdow)),
            )
            .unwrap_err();
            assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        }
    }

    #[test]
    fn null_weekday_is_error_94() {
        let err = weekday_name(&VBVariant::Null, None, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn non_numeric_weekday_is_error_13() {
        let err = weekday_name(&VBVariant::from_string("abc"), None, None).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn returns_vbstring() {
        let result = weekday_name(&VBVariant::from_long(1), None, None).unwrap();
        assert_eq!(result.as_str(), "Sunday");
    }

    #[test]
    fn abbreviate_accepts_numeric_values() {
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(2),
                Some(&VBVariant::from_long(1)),
                None
            ),
            "Mon"
        );
        assert_eq!(
            wn_opts(
                &VBVariant::from_long(2),
                Some(&VBVariant::from_long(0)),
                None
            ),
            "Monday"
        );
    }
}
