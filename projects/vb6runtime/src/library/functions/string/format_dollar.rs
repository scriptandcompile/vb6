//! # `Format$` Function
//!
//! Returns a `String` formatted according to instructions contained in a format expression.
//!
//! ## Syntax
//!
//! ```vb6
//! Format$(expression[, format[, firstdayofweek[, firstweekofyear]]])
//! ```
//!
//! ## Parameters
//!
//! - `expression`: Required. Any valid expression.
//! - `format`: Optional. A valid named or user-defined format expression.
//! - `firstdayofweek`: Optional. A constant that specifies the first day of the week.
//! - `firstweekofyear`: Optional. A constant that specifies the first week of the year.
//!
//! ## Return Value
//!
//! Returns a `String` containing the formatted representation of the expression. If `format` is omitted, `Format$` returns a string similar to `Str$`.
//!
//! ## Remarks
//!
//! The `Format$` function is one of the most versatile functions in VB6, allowing you to format numbers, dates, times, and strings according to predefined or custom format expressions.
//!
//! You can use one of the predefined named formats or create user-defined formats with special characters that specify how the value should be displayed.
//!
//! ### Named Numeric Formats
//! - `General Number`: Display number with no thousand separator
//! - `Currency`: Display number with thousand separator and two decimal places
//! - `Fixed`: Display at least one digit to the left and two digits to the right of decimal
//! - `Standard`: Display number with thousand separator
//! - `Percent`: Display number multiplied by 100 with percent sign
//! - `Scientific`: Use standard scientific notation
//! - `Yes/No`: Display No if number is 0; otherwise display Yes
//! - `True/False`: Display False if number is 0; otherwise display True
//! - `On/Off`: Display Off if number is 0; otherwise display On
//!
//! ### Named Date/Time Formats
//! - `General Date`: Display date and/or time
//! - `Long Date`: Display date according to long date format
//! - `Medium Date`: Display date using medium date format
//! - `Short Date`: Display date using short date format
//! - `Long Time`: Display time using long time format (includes hours, minutes, seconds)
//! - `Medium Time`: Display time in 12-hour format using hours and minutes and AM/PM
//! - `Short Time`: Display time using 24-hour format (hh:mm)
//!
//! ### User-Defined Number Format Characters
//! - `0`: Digit placeholder. Display digit or zero
//! - `#`: Digit placeholder. Display digit or nothing
//! - `.`: Decimal placeholder
//! - `%`: Percentage placeholder
//! - `,`: Thousand separator
//! - `E- E+ e- e+`: Scientific notation
//! - `- + $ ( )`: Display literal character
//! - `\`: Display next character as literal
//!
//! ### User-Defined Date/Time Format Characters
//! - `c`: Display date as `ddddd` and time as `ttttt`
//! - `d`: Display day as number without leading zero (1-31)
//! - `dd`: Display day as number with leading zero (01-31)
//! - `ddd`: Display day as abbreviation (Sun-Sat)
//! - `dddd`: Display day as full name (Sunday-Saturday)
//! - `m`: Display month as number without leading zero (1-12)
//! - `mm`: Display month as number with leading zero (01-12)
//! - `mmm`: Display month as abbreviation (Jan-Dec)
//! - `mmmm`: Display month as full name (January-December)
//! - `yy`: Display year as 2-digit number (00-99)
//! - `yyyy`: Display year as 4-digit number (100-9999)
//! - `h`: Display hour as number without leading zero (0-23)
//! - `hh`: Display hour as number with leading zero (00-23)
//! - `n`: Display minute as number without leading zero (0-59)
//! - `nn`: Display minute as number with leading zero (00-59)
//! - `s`: Display second as number without leading zero (0-59)
//! - `ss`: Display second as number with leading zero (00-59)
//! - `AM/PM`: Use 12-hour clock and display uppercase AM/PM
//!
//! ### User-Defined String Format Characters
//! - `@`: Character placeholder. Display character or space
//! - `&`: Character placeholder. Display character or nothing
//! - `<`: Force lowercase
//! - `>`: Force uppercase
//!
//! ## Typical Uses
//!
//! ### Example 1: Formatting Currency
//! ```vb6
//! Dim amount As Double
//! amount = 1234.56
//! Text1.Text = Format$(amount, "Currency")  ' "$1,234.56"
//! ```
//!
//! ### Example 2: Custom Number Format
//! ```vb6
//! Dim value As Double
//! value = 1234.5
//! result = Format$(value, "0000.00")  ' "1234.50"
//! ```
//!
//! ### Example 3: Date Formatting
//! ```vb6
//! Dim today As Date
//! today = Now
//! dateStr = Format$(today, "Long Date")
//! ```
//!
//! ### Example 4: Custom Date Format
//! ```vb6
//! dateStr = Format$(Now, "yyyy-mm-dd")  ' "2024-01-15"
//! ```
//!
//! ## Common Usage Patterns
//!
//! ### Formatting as Percentage
//! ```vb6
//! Dim rate As Double
//! rate = 0.075
//! display = Format$(rate, "0.00%")  ' "7.50%"
//! ```
//!
//! ### Zero-Padded Numbers
//! ```vb6
//! Dim id As Integer
//! id = 42
//! idStr = Format$(id, "000000")  ' "000042"
//! ```
//!
//! ### Phone Number Formatting
//! ```vb6
//! Dim phone As String
//! phone = "5551234567"
//! formatted = Format$(phone, "(@@@) @@@-@@@@")  ' "(555) 123-4567"
//! ```
//!
//! ### Time Formatting
//! ```vb6
//! Dim currentTime As Date
//! currentTime = Now
//! timeStr = Format$(currentTime, "hh:nn:ss AM/PM")
//! ```
//!
//! ### Scientific Notation
//! ```vb6
//! Dim bigNum As Double
//! bigNum = 12345678
//! sciStr = Format$(bigNum, "0.00E+00")  ' "1.23E+07"
//! ```
//!
//! ### File Timestamp
//! ```vb6
//! filename = "backup_" & Format$(Now, "yyyymmdd_hhnnss") & ".dat"
//! ```
//!
//! ### Accounting Format
//! ```vb6
//! balance = Format$(amount, "#,##0.00;(#,##0.00)")
//! ' Positive: "1,234.56"
//! ' Negative: "(1,234.56)"
//! ```
//!
//! ### Leading Zeros for Dates
//! ```vb6
//! monthStr = Format$(Month(Date), "00")  ' "01" to "12"
//! dayStr = Format$(Day(Date), "00")      ' "01" to "31"
//! ```
//!
//! ### Conditional Formatting
//! ```vb6
//! ' Format: positive;negative;zero
//! result = Format$(value, "+0.00;-0.00;Zero")
//! ```
//!
//! ### Uppercase/Lowercase Conversion
//! ```vb6
//! upperName = Format$("john doe", ">")      ' "JOHN DOE"
//! lowerName = Format$("JOHN DOE", "<")      ' "john doe"
//! ```
//!
//! ## Related Functions
//!
//! - `Format`: Variant version of `Format$`
//! - `Str$`: Converts a number to a string
//! - `CStr`: Converts an expression to a string
//! - `FormatNumber`: Formats a number with specific options
//! - `FormatCurrency`: Formats a number as currency
//! - `FormatDateTime`: Formats a date/time value
//! - `FormatPercent`: Formats a number as a percentage
//!
//! ## Best Practices
//!
//! 1. Use named formats for common formatting tasks (clearer intent)
//! 2. Cache format strings if using the same format repeatedly
//! 3. Test custom format strings with edge cases (zero, negative, very large/small)
//! 4. Use `@` instead of `&` in string formats when you want spaces preserved
//! 5. Remember that `m` vs `mm` depends on context (month vs minute)
//! 6. Use four-digit years (`yyyy`) to avoid Y2K-style issues
//! 7. Consider locale settings when using named formats
//! 8. Use semicolons to specify different formats for positive, negative, and zero
//! 9. Escape literal characters with backslash or quotes when needed
//! 10. Be aware that `Format$` returns a string - convert back if needed
//!
//! ## Performance Considerations
//!
//! - Named formats are slightly faster than complex user-defined formats
//! - Avoid calling `Format$` in tight loops if possible (cache results)
//! - For simple zero-padding, `String$` + `Right$` may be faster
//! - `Format$` is slower than simple string concatenation
//! - Consider using `FormatNumber`, `FormatCurrency`, etc. for specific tasks
//!
//! ## Locale Considerations
//!
//! | Aspect | Behavior |
//! |--------|----------|
//! | Currency Symbol | Uses system locale currency symbol |
//! | Decimal Separator | Uses locale decimal separator (. or ,) |
//! | Thousand Separator | Uses locale thousand separator |
//! | Date Format | Named date formats use locale settings |
//! | Day/Month Names | Uses locale language for names |
//! | AM/PM Designators | Uses locale AM/PM strings |
//! | First Day of Week | Can be overridden with parameter |
//! | First Week of Year | Can be overridden with parameter |
//!
//! ## Common Pitfalls
//!
//! - Using `m` for minutes instead of `n` (m means month)
//! - Forgetting that `Format$` always returns a string
//! - Not escaping literal characters in format strings
//! - Assuming `#` and `0` behave the same (they don't)
//! - Using comma as decimal separator in code (always use period)
//! - Not handling empty strings or null values
//! - Forgetting that format strings are case-sensitive
//! - Using named formats that don't exist (causes error)
//!
//! ## Limitations
//!
//! - Cannot create truly custom named formats
//! - Limited control over locale-specific formatting
//! - No built-in format for ISO 8601 dates (must use `yyyy-mm-ddThh:nn:ss`)
//! - Cannot format arrays or objects directly
//! - Some format combinations may produce unexpected results
//! - Maximum string length limitations apply to output
//! - Cannot use for binary or hexadecimal display (use `Hex$` or `Oct$`)

use crate::{
    error::{VBError, VBResult},
    value::{VBLong, VBString, Value},
};

/// Returns a `String` containing the formatted representation of `expression`
/// according to the instructions in `format`.
///
/// `format` may be a named format (`"Currency"`, `"Short Date"`, ...) or a
/// user-defined format expression. When `format` is `None` the value is
/// converted to its natural string representation (like `Str$`, but without
/// the leading space for positive numbers). A `Null` expression always yields
/// the empty string, matching `Format$`.
///
/// `firstdayofweek` and `firstweekofyear` select the week start and first-week
/// rule used by the `w`/`ww` date tokens; when omitted, `vbSunday` (1) and
/// `vbFirstJan1` (1) are used.
///
/// # Errors
///
/// Returns error 13 (`Type mismatch`) when the expression cannot be coerced to
/// the type implied by the format string (e.g. a date format applied to a
/// non-numeric string).
pub fn format_dollar(
    expression: &Value,
    format: Option<&VBString>,
    firstdayofweek: Option<&VBLong>,
    firstweekofyear: Option<&VBLong>,
) -> VBResult<VBString> {
    if expression.is_null() {
        return Ok(VBString::from(""));
    }
    let first_day_of_week = firstdayofweek.map_or(1, |v| {
        let n = v.as_i32();
        if (1..=7).contains(&n) {
            n
        } else {
            1
        }
    });
    let first_week_of_year = firstweekofyear.map_or(1, |v| v.as_i32());

    let Some(fmt) = format else {
        let result = if expression.is_date() {
            match date_parts(expression.as_date_serial()?) {
                Some(parts) => general_date_string(&parts),
                None => return Err(VBError::type_mismatch()),
            }
        } else {
            expression.as_string()?
        };
        return Ok(VBString::from(result));
    };
    let fmt_str = fmt.as_str();
    if fmt_str.trim().is_empty() {
        return Ok(VBString::from(expression.as_string()?));
    }

    if let Some(name) = named_numeric_format_name(fmt_str) {
        let value = expression.as_f64()?;
        return Ok(VBString::from(format_named_numeric(name, value)?));
    }
    if let Some(name) = named_date_format_name(fmt_str) {
        let serial = expression.as_date_serial()?;
        let parts = date_parts(serial).ok_or_else(VBError::type_mismatch)?;
        return Ok(VBString::from(format_named_date(name, &parts)));
    }
    if has_string_placeholder(fmt_str) {
        let input = expression.as_string()?;
        return Ok(VBString::from(format_string_custom(&input, fmt_str)));
    }
    if has_date_letters(fmt_str) {
        let serial = expression.as_date_serial()?;
        return Ok(VBString::from(format_date_custom(
            serial,
            fmt_str,
            first_day_of_week,
            first_week_of_year,
        )?));
    }
    let value = expression.as_f64()?;
    Ok(VBString::from(format_number_custom(value, fmt_str)?))
}

// ---------------------------------------------------------------------------
// Named formats
// ---------------------------------------------------------------------------

/// Maps a case-insensitive named numeric format to its canonical name.
fn named_numeric_format_name(fmt: &str) -> Option<&'static str> {
    match fmt.trim().to_ascii_lowercase().as_str() {
        "general number" => Some("general number"),
        "currency" => Some("currency"),
        "fixed" => Some("fixed"),
        "standard" => Some("standard"),
        "percent" => Some("percent"),
        "scientific" => Some("scientific"),
        "yes/no" => Some("yes/no"),
        "true/false" => Some("true/false"),
        "on/off" => Some("on/off"),
        _ => None,
    }
}

/// Formats `value` using one of the named numeric formats.
fn format_named_numeric(name: &str, value: f64) -> VBResult<String> {
    match name {
        "general number" => Ok(general_number(value)),
        "currency" => format_number_custom(value, "$#,##0.00;($#,##0.00)"),
        "fixed" => format_number_custom(value, "0.00"),
        "standard" => format_number_custom(value, "#,##0.00"),
        "percent" => format_number_custom(value, "0.00%"),
        "scientific" => format_number_custom(value, "0.00E+00"),
        "yes/no" => Ok(if value == 0.0 { "No" } else { "Yes" }.to_string()),
        "true/false" => Ok(if value == 0.0 { "False" } else { "True" }.to_string()),
        "on/off" => Ok(if value == 0.0 { "Off" } else { "On" }.to_string()),
        _ => Err(VBError::invalid_procedure_call()),
    }
}

/// Maps a case-insensitive named date format to its canonical name.
fn named_date_format_name(fmt: &str) -> Option<&'static str> {
    match fmt.trim().to_ascii_lowercase().as_str() {
        "general date" => Some("general date"),
        "long date" => Some("long date"),
        "medium date" => Some("medium date"),
        "short date" => Some("short date"),
        "long time" => Some("long time"),
        "medium time" => Some("medium time"),
        "short time" => Some("short time"),
        _ => None,
    }
}

/// Formats `parts` using one of the named date/time formats.
fn format_named_date(name: &str, parts: &DateParts) -> String {
    match name {
        "general date" => general_date_string(parts),
        "long date" => format!(
            "{}, {} {}, {}",
            DAY_NAMES[(parts.weekday - 1) as usize],
            MONTH_NAMES[(parts.month - 1) as usize],
            parts.day,
            parts.year
        ),
        "medium date" => format!(
            "{:02}-{}-{:02}",
            parts.day,
            &MONTH_NAMES[(parts.month - 1) as usize][..3],
            parts.year % 100
        ),
        "short date" => format!("{}/{}/{}", parts.month, parts.day, parts.year),
        "long time" => format!(
            "{}:{:02}:{:02} {}",
            hour12(parts.hour),
            parts.minute,
            parts.second,
            ampm(parts.hour)
        ),
        "medium time" => format!(
            "{:02}:{:02} {}",
            hour12(parts.hour),
            parts.minute,
            ampm(parts.hour)
        ),
        "short time" => format!("{:02}:{:02}", parts.hour, parts.minute),
        _ => unreachable!("unknown named date format"),
    }
}

// ---------------------------------------------------------------------------
// String formatting (`@`, `&`, `<`, `>`, `!`)
// ---------------------------------------------------------------------------

/// Whether `format` uses string-formatting placeholders/modifiers.
fn has_string_placeholder(fmt: &str) -> bool {
    let mut in_quote = false;
    let mut chars = fmt.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => in_quote = !in_quote,
            '\\' => {
                chars.next();
            }
            c if !in_quote && matches!(c, '@' | '&' | '<' | '>' | '!') => return true,
            _ => {}
        }
    }
    false
}

/// Formats `input` using a user-defined string format expression.
///
/// `@` displays a character or a space, `&` a character or nothing, `<` forces
/// lowercase, `>` forces uppercase, and `!` left-aligns the input.
fn format_string_custom(input: &str, format: &str) -> String {
    let chars: Vec<char> = format.chars().collect();
    let mut has_gt = false;
    let mut has_lt = false;
    let mut left_align = false;
    let mut placeholders = 0usize;
    let mut in_quote = false;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '"' => in_quote = !in_quote,
            '\\' => i += 1,
            c if !in_quote => match c {
                '@' | '&' => placeholders += 1,
                '>' => has_gt = true,
                '<' => has_lt = true,
                '!' => left_align = true,
                _ => {}
            },
            _ => {}
        }
        i += 1;
    }

    let apply_case = |s: String| -> String {
        if has_lt {
            s.to_lowercase()
        } else if has_gt {
            s.to_uppercase()
        } else {
            s
        }
    };
    if placeholders == 0 {
        return apply_case(input.to_string());
    }

    let input_chars: Vec<char> = input.chars().collect();
    let used = placeholders.min(input_chars.len());
    let leading_pad = if left_align { 0 } else { placeholders - used };

    let mut out = String::new();
    let mut consumed = 0usize;
    let mut ph_index = 0usize;
    i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '@' | '&' => {
                if ph_index < leading_pad {
                    if c == '@' {
                        out.push(' ');
                    }
                } else if consumed < used {
                    out.push(input_chars[consumed]);
                    consumed += 1;
                } else if c == '@' {
                    out.push(' ');
                }
                ph_index += 1;
                i += 1;
            }
            '<' | '>' | '!' => i += 1,
            '"' => {
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    out.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
            }
            '\\' => {
                i += 1;
                if i < chars.len() {
                    out.push(chars[i]);
                    i += 1;
                }
            }
            _ => {
                out.push(c);
                i += 1;
            }
        }
    }
    apply_case(out)
}

// ---------------------------------------------------------------------------
// Numeric formatting
// ---------------------------------------------------------------------------

/// A parsed single-section numeric format expression.
struct NumericSection {
    prefix: String,
    int_placeholders: Vec<char>,
    frac_placeholders: Vec<char>,
    has_thousands: bool,
    scaling: u32,
    percent: u32,
    exponent: Option<(char, bool, usize)>,
    suffix: String,
}

/// Splits a format expression into its `;`-separated sections, honoring
/// quoted literals and backslash escapes.
fn split_sections(format: &str) -> Vec<String> {
    let mut sections = vec![String::new()];
    let mut in_quote = false;
    let mut chars = format.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quote = !in_quote;
                sections.last_mut().unwrap().push(c);
            }
            '\\' => {
                sections.last_mut().unwrap().push(c);
                if let Some(next) = chars.next() {
                    sections.last_mut().unwrap().push(next);
                }
            }
            ';' if !in_quote => sections.push(String::new()),
            _ => sections.last_mut().unwrap().push(c),
        }
    }
    sections
}

/// Whether `c` starts the numeric body of a section.
fn is_num_body_start(c: char) -> bool {
    matches!(c, '0' | '#' | '.' | ',' | '%' | 'E' | 'e')
}

/// Reads a quoted literal (`"..."`) or backslash escape (`\x`) starting at `*i`
/// and returns the literal text, advancing `*i` past it.
fn read_literal(chars: &[char], i: &mut usize) -> String {
    let mut out = String::new();
    while *i < chars.len() {
        let c = chars[*i];
        if c == '"' {
            *i += 1;
            while *i < chars.len() && chars[*i] != '"' {
                out.push(chars[*i]);
                *i += 1;
            }
            if *i < chars.len() {
                *i += 1;
            }
        } else if c == '\\' {
            *i += 1;
            if *i < chars.len() {
                out.push(chars[*i]);
                *i += 1;
            }
        } else {
            break;
        }
    }
    out
}

/// Parses a scientific-notation exponent spec starting at index `i` (where
/// `chars[i]` is `E` or `e`), returning the letter, whether the sign is forced
/// positive, the digit count, and the index just past the spec.
fn parse_exponent(chars: &[char], i: usize) -> Option<(char, bool, usize, usize)> {
    let letter = chars[i];
    let mut j = i + 1;
    if j >= chars.len() {
        return None;
    }
    let forced_plus = match chars[j] {
        '+' => true,
        '-' => false,
        _ => return None,
    };
    j += 1;
    let mut digits = 0usize;
    while j < chars.len() && (chars[j] == '0' || chars[j] == '#') {
        digits += 1;
        j += 1;
    }
    if digits == 0 {
        return None;
    }
    Some((letter, forced_plus, digits, j))
}

/// Parses a single numeric format section into its components.
fn parse_numeric_section(section: &str) -> NumericSection {
    let chars: Vec<char> = section.chars().collect();
    let n = chars.len();
    let mut parsed = NumericSection {
        prefix: String::new(),
        int_placeholders: Vec::new(),
        frac_placeholders: Vec::new(),
        has_thousands: false,
        scaling: 0,
        percent: 0,
        exponent: None,
        suffix: String::new(),
    };
    let mut i = 0;
    while i < n && !is_num_body_start(chars[i]) {
        let mut idx = i;
        let literal = read_literal(&chars, &mut idx);
        if idx > i {
            parsed.prefix.push_str(&literal);
            i = idx;
        } else {
            parsed.prefix.push(chars[i]);
            i += 1;
        }
    }

    let mut is_frac = false;
    let mut pending_comma = false;
    while i < n {
        let c = chars[i];
        match c {
            '0' | '#' => {
                if pending_comma {
                    parsed.has_thousands = true;
                    pending_comma = false;
                }
                if is_frac {
                    parsed.frac_placeholders.push(c);
                } else {
                    parsed.int_placeholders.push(c);
                }
                i += 1;
            }
            '.' => {
                if pending_comma {
                    parsed.scaling += 1;
                    pending_comma = false;
                }
                if is_frac {
                    parsed.suffix.push(c);
                } else {
                    is_frac = true;
                }
                i += 1;
            }
            ',' => {
                if parsed.int_placeholders.is_empty() && !is_frac {
                    parsed.suffix.push(c);
                } else {
                    pending_comma = true;
                }
                i += 1;
            }
            '%' => {
                if pending_comma {
                    parsed.scaling += 1;
                    pending_comma = false;
                }
                parsed.percent += 1;
                i += 1;
            }
            'E' | 'e' => {
                if let Some((letter, forced_plus, digits, next)) = parse_exponent(&chars, i) {
                    parsed.exponent = Some((letter, forced_plus, digits));
                    i = next;
                } else {
                    if pending_comma {
                        parsed.scaling += 1;
                        pending_comma = false;
                    }
                    parsed.suffix.push(c);
                    i += 1;
                }
            }
            '"' | '\\' => {
                if pending_comma {
                    parsed.scaling += 1;
                    pending_comma = false;
                }
                let mut idx = i;
                parsed.suffix.push_str(&read_literal(&chars, &mut idx));
                i = idx;
            }
            _ => {
                if pending_comma {
                    parsed.scaling += 1;
                    pending_comma = false;
                }
                parsed.suffix.push(c);
                i += 1;
            }
        }
    }
    if pending_comma {
        parsed.scaling += 1;
    }
    parsed
}

/// Rounds a non-negative `f64` half away from zero (VB6 `Format` rounding).
fn round_half_away(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// Groups an integer digit string into 3-digit clusters from the right.
fn group_thousands(s: &str) -> String {
    let bytes: Vec<char> = s.chars().collect();
    let len = bytes.len();
    let mut out = String::new();
    for (i, c) in bytes.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*c);
    }
    out
}

/// Formats the integer and fractional digit body of `value` according to
/// `parsed`, including zero-fill, thousand separators, and rounding.
fn format_digits(value: f64, parsed: &NumericSection) -> String {
    let frac_count = parsed.frac_placeholders.len();
    let factor = 10.0f64.powi(frac_count as i32);
    let scaled = round_half_away(value * factor);
    let scaled_i = scaled as i128;
    let factor_i = 10i128.pow(frac_count as u32);
    let int_val = scaled_i / factor_i;
    let frac_val = (scaled_i % factor_i).unsigned_abs();
    let int_digits: Vec<char> = int_val.to_string().chars().collect();
    let frac_digits: Vec<char> = if frac_count > 0 {
        format!("{:0>width$}", frac_val, width = frac_count)
            .chars()
            .collect()
    } else {
        Vec::new()
    };

    let mut int_out = String::new();
    let ph = parsed.int_placeholders.len();
    if ph == 0 {
        int_out.push('0');
    } else if int_digits.len() >= ph {
        int_out = int_digits.iter().collect();
    } else {
        let pad = ph - int_digits.len();
        for (k, &p) in parsed.int_placeholders.iter().enumerate() {
            if k < pad {
                if p == '0' {
                    int_out.push('0');
                }
            } else {
                int_out.push(int_digits[k - pad]);
            }
        }
    }
    if parsed.has_thousands {
        int_out = group_thousands(&int_out);
    }

    let mut frac_out = String::new();
    for (k, &p) in parsed.frac_placeholders.iter().enumerate() {
        frac_out.push(frac_digits[k]);
        if p == '#' && k + 1 == parsed.frac_placeholders.len() {
            // Trailing `#` placeholders may drop zero digits; handled below.
            let _ = ();
        }
    }
    let mut frac_ph = parsed.frac_placeholders.clone();
    while let Some(&p) = frac_ph.last() {
        if p == '#' && frac_out.ends_with('0') {
            frac_out.pop();
            frac_ph.pop();
        } else {
            break;
        }
    }

    let mut body = int_out;
    if !frac_out.is_empty() {
        body.push('.');
        body.push_str(&frac_out);
    }
    body
}

/// Formats `value` with a scientific-notation format expression.
fn format_exponent_body(
    value: f64,
    parsed: &NumericSection,
    letter: char,
    forced_plus: bool,
    digits: usize,
) -> String {
    let (mantissa, exponent) = if value == 0.0 {
        (0.0, 0)
    } else {
        let e = value.abs().log10().floor() as i32;
        (value / 10.0f64.powi(e), e)
    };
    let mantissa_body = format_digits(mantissa, parsed);
    let mut exp_str = String::new();
    exp_str.push(letter);
    if forced_plus && exponent >= 0 {
        exp_str.push('+');
    } else if exponent < 0 {
        exp_str.push('-');
    }
    let magnitude = exponent.abs().to_string();
    if magnitude.len() < digits {
        exp_str.push_str(&"0".repeat(digits - magnitude.len()));
    }
    exp_str.push_str(&magnitude);
    format!("{mantissa_body}{exp_str}")
}

/// Formats `value` using a user-defined numeric format expression.
fn format_number_custom(value: f64, format: &str) -> VBResult<String> {
    let sections = split_sections(format);
    let negative = value < 0.0;
    let zero = value == 0.0;
    let index = if negative {
        if sections.len() > 1 {
            1
        } else {
            0
        }
    } else if zero && sections.len() > 2 {
        2
    } else {
        0
    };
    let section = &sections[index.min(sections.len() - 1)];
    let parsed = parse_numeric_section(section);

    if parsed.int_placeholders.is_empty()
        && parsed.frac_placeholders.is_empty()
        && parsed.exponent.is_none()
    {
        let mut literal = parsed.prefix;
        literal.push_str(&parsed.suffix);
        return Ok(literal);
    }

    let mut v = value.abs();
    if parsed.percent > 0 {
        v *= 100.0f64.powi(parsed.percent as i32);
    }
    if parsed.scaling > 0 {
        v /= 1000.0f64.powi(parsed.scaling as i32);
    }

    let body = if let Some((letter, forced_plus, digits)) = parsed.exponent {
        format_exponent_body(v, &parsed, letter, forced_plus, digits)
    } else {
        format_digits(v, &parsed)
    };

    let mut out = String::new();
    out.push_str(&parsed.prefix);
    if negative && sections.len() == 1 && !parsed.prefix.contains('-') {
        out.push('-');
    }
    out.push_str(&body);
    for _ in 0..parsed.percent {
        out.push('%');
    }
    out.push_str(&parsed.suffix);
    Ok(out)
}

/// The `General Number` named format.
fn general_number(value: f64) -> String {
    if value.is_finite() && value == value.trunc() && value.abs() < 1.0e15 {
        format!("{}", value as i64)
    } else {
        format!("{}", value)
    }
}

// ---------------------------------------------------------------------------
// Date/time formatting
// ---------------------------------------------------------------------------

const DAY_NAMES: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// The decomposed civil date/time components of an OLE automation serial.
struct DateParts {
    year: i16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    /// Day of the week, 1 = Sunday .. 7 = Saturday.
    weekday: i8,
    /// Day of the year, 1-based.
    day_of_year: i16,
}

fn date_parts(serial: f64) -> Option<DateParts> {
    let dt = crate::value::date_serial_to_datetime(serial)?;
    Some(DateParts {
        year: dt.year(),
        month: dt.month() as u8,
        day: dt.day() as u8,
        hour: dt.hour() as u8,
        minute: dt.minute() as u8,
        second: dt.second() as u8,
        weekday: dt.date().weekday().to_sunday_one_offset(),
        day_of_year: dt.date().day_of_year(),
    })
}

/// The 12-hour clock hour for a 0-23 hour.
fn hour12(hour: u8) -> u8 {
    let r = hour % 12;
    if r == 0 {
        12
    } else {
        r
    }
}

fn ampm(hour: u8) -> &'static str {
    if hour < 12 {
        "AM"
    } else {
        "PM"
    }
}

fn pad2(v: i32) -> String {
    format!("{v:02}")
}

/// The `General Date` representation: short date, plus time when not midnight.
fn general_date_string(parts: &DateParts) -> String {
    let date = format!("{}/{}/{}", parts.month, parts.day, parts.year);
    if parts.hour == 0 && parts.minute == 0 && parts.second == 0 {
        date
    } else {
        format!(
            "{date} {}:{:02}:{:02} {}",
            hour12(parts.hour),
            parts.minute,
            parts.second,
            ampm(parts.hour)
        )
    }
}

/// VB6 `Weekday(date, first_day_of_week)`: 1-based index relative to `fdow`.
fn vb_weekday(sunday_based: i32, fdow: i32) -> i32 {
    ((sunday_based - fdow).rem_euclid(7)) + 1
}

/// The weekday of January 1 of the year containing `parts` (1 = Sunday).
fn jan1_weekday(parts: &DateParts) -> i32 {
    let offset = (parts.day_of_year as i32 - 1) % 7;
    (((parts.weekday as i32 - 1 - offset) % 7) + 7) % 7 + 1
}

/// VB6 `DatePart("ww", ...)` week-of-year (assuming the first week starts on
/// January 1, i.e. `vbFirstJan1`).
fn week_of_year(parts: &DateParts, fdow: i32) -> i32 {
    let jan1_wd = vb_weekday(jan1_weekday(parts), fdow);
    ((parts.day_of_year as i32 - 1 + jan1_wd - 1) / 7) + 1
}

/// Whether `format` uses any date/time token letters outside quotes/escapes.
fn has_date_letters(fmt: &str) -> bool {
    let mut in_quote = false;
    let mut chars = fmt.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => in_quote = !in_quote,
            '\\' => {
                chars.next();
            }
            c if !in_quote && "dmyhnsqwc".contains(c.to_ascii_lowercase()) => return true,
            _ => {}
        }
    }
    false
}

/// Whether the text at `i` starts with `token`, ignoring case.
fn starts_ci(chars: &[char], i: usize, token: &str) -> bool {
    let t: Vec<char> = token.chars().collect();
    i + t.len() <= chars.len()
        && t.iter()
            .zip(&chars[i..i + t.len()])
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
}

/// Formats `serial` using a user-defined date/time format expression.
fn format_date_custom(serial: f64, format: &str, fdow: i32, _fwoy: i32) -> VBResult<String> {
    let parts = date_parts(serial).ok_or_else(VBError::type_mismatch)?;
    let chars: Vec<char> = format.chars().collect();
    let lower_format = format.to_ascii_lowercase();
    let has_ampm = lower_format.contains("am/pm") || lower_format.contains("a/p");

    let mut out = String::new();
    let mut i = 0;
    let mut last_was_hour = false;
    while i < chars.len() {
        let c = chars[i];
        if c == '"' || c == '\\' {
            let mut idx = i;
            out.push_str(&read_literal(&chars, &mut idx));
            i = idx;
            continue;
        }
        let c_lower = c.to_ascii_lowercase();
        if (c_lower == 'a' || c_lower == 'p') && starts_ci(&chars, i, "am/pm") {
            out.push_str(ampm(parts.hour));
            i += 5;
            last_was_hour = false;
            continue;
        }
        if (c_lower == 'a' || c_lower == 'p') && starts_ci(&chars, i, "a/p") {
            out.push_str(if parts.hour < 12 { "A" } else { "P" });
            i += 3;
            last_was_hour = false;
            continue;
        }

        let mut j = i;
        while j < chars.len() && chars[j].eq_ignore_ascii_case(&c) {
            j += 1;
        }
        let run_len = j - i;
        let lower = c.to_ascii_lowercase();
        let mut handled = false;
        let mut sets_hour = false;
        match lower {
            'c' if run_len == 1 => {
                out.push_str(&general_date_string(&parts));
                handled = true;
            }
            'd' => {
                let text = match run_len {
                    4 => DAY_NAMES[(parts.weekday - 1) as usize].to_string(),
                    3 => DAY_NAMES[(parts.weekday - 1) as usize][..3].to_string(),
                    2 => pad2(parts.day as i32),
                    _ => parts.day.to_string(),
                };
                out.push_str(&text);
                handled = true;
            }
            'm' => {
                let text = if last_was_hour {
                    match run_len {
                        2 => pad2(parts.minute as i32),
                        _ => parts.minute.to_string(),
                    }
                } else {
                    match run_len {
                        4 => MONTH_NAMES[(parts.month - 1) as usize].to_string(),
                        3 => MONTH_NAMES[(parts.month - 1) as usize][..3].to_string(),
                        2 => pad2(parts.month as i32),
                        _ => parts.month.to_string(),
                    }
                };
                out.push_str(&text);
                handled = true;
            }
            'y' => {
                let text = match run_len {
                    2 => pad2((parts.year % 100) as i32),
                    _ => parts.year.to_string(),
                };
                out.push_str(&text);
                handled = true;
            }
            'h' => {
                let hour = if has_ampm {
                    hour12(parts.hour)
                } else {
                    parts.hour
                };
                let text = if run_len >= 2 {
                    pad2(hour as i32)
                } else {
                    hour.to_string()
                };
                out.push_str(&text);
                handled = true;
                sets_hour = true;
            }
            'n' => {
                let text = if run_len >= 2 {
                    pad2(parts.minute as i32)
                } else {
                    parts.minute.to_string()
                };
                out.push_str(&text);
                handled = true;
            }
            's' => {
                let text = if run_len >= 2 {
                    pad2(parts.second as i32)
                } else {
                    parts.second.to_string()
                };
                out.push_str(&text);
                handled = true;
            }
            'q' => {
                out.push_str(&((parts.month as i32 - 1) / 3 + 1).to_string());
                handled = true;
            }
            'w' => {
                let text = if run_len >= 2 {
                    week_of_year(&parts, fdow).to_string()
                } else {
                    vb_weekday(parts.weekday as i32, fdow).to_string()
                };
                out.push_str(&text);
                handled = true;
            }
            _ => {}
        }
        if !handled {
            out.extend(chars[i..j].iter());
        } else {
            last_was_hour = sets_hour;
        }
        i = j;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;

    fn fmt(expr: Value, format: &str) -> String {
        format_dollar(&expr, Some(&VBString::from(format)), None, None)
            .unwrap()
            .into_inner()
    }

    fn fmt_opt(expr: Value, format: Option<&str>) -> String {
        format_dollar(&expr, format.map(VBString::from).as_ref(), None, None)
            .unwrap()
            .into_inner()
    }

    #[test]
    fn no_format_converts_like_str() {
        assert_eq!(fmt_opt(Value::Long(1234), None), "1234");
        assert_eq!(fmt_opt(Value::Long(-1234), None), "-1234");
        assert_eq!(fmt_opt(Value::Double(1234.5), None), "1234.5");
        assert_eq!(fmt_opt(Value::from_string("hello"), None), "hello");
        assert_eq!(fmt_opt(Value::Boolean(true), None), "True");
        assert_eq!(fmt_opt(Value::from_date_serial(45_662.0), None), "1/5/2025");
    }

    #[test]
    fn null_returns_empty_string() {
        assert_eq!(fmt_opt(Value::Null, None), "");
        assert_eq!(fmt(Value::Null, "0.00"), "");
    }

    #[test]
    fn named_numeric_formats() {
        assert_eq!(fmt(Value::Double(1234.5), "General Number"), "1234.5");
        assert_eq!(fmt(Value::Double(1234.5), "Currency"), "$1,234.50");
        assert_eq!(fmt(Value::Double(-1234.5), "Currency"), "($1,234.50)");
        assert_eq!(fmt(Value::Double(1234.5), "Fixed"), "1234.50");
        assert_eq!(fmt(Value::Double(1234.5), "Standard"), "1,234.50");
        assert_eq!(fmt(Value::Double(0.075), "Percent"), "7.50%");
        assert_eq!(fmt(Value::Double(12_345_678.0), "Scientific"), "1.23E+07");
        assert_eq!(fmt(Value::Double(5.0), "Yes/No"), "Yes");
        assert_eq!(fmt(Value::Double(0.0), "Yes/No"), "No");
        assert_eq!(fmt(Value::Double(5.0), "True/False"), "True");
        assert_eq!(fmt(Value::Double(0.0), "On/Off"), "Off");
    }

    #[test]
    fn custom_numeric_formats() {
        assert_eq!(fmt(Value::Double(1234.5), "0000.00"), "1234.50");
        assert_eq!(fmt(Value::Long(42), "000000"), "000042");
        assert_eq!(fmt(Value::Long(7), "00"), "07");
        assert_eq!(fmt(Value::Double(0.075), "0.00%"), "7.50%");
        assert_eq!(fmt(Value::Double(12_345_678.0), "0.00E+00"), "1.23E+07");
        assert_eq!(fmt(Value::Double(0.000_012_3), "0.00E-00"), "1.23E-05");
        assert_eq!(
            fmt(Value::Double(1234.56), "#,##0.00;(#,##0.00)"),
            "1,234.56"
        );
        assert_eq!(
            fmt(Value::Double(-1234.56), "#,##0.00;(#,##0.00)"),
            "(1,234.56)"
        );
    }

    #[test]
    fn custom_numeric_sections() {
        assert_eq!(fmt(Value::Double(1.5), "+0.00;-0.00;Zero"), "+1.50");
        assert_eq!(fmt(Value::Double(-1.5), "+0.00;-0.00;Zero"), "-1.50");
        assert_eq!(fmt(Value::Double(0.0), "+0.00;-0.00;Zero"), "Zero");
    }

    #[test]
    fn custom_numeric_rounding() {
        assert_eq!(fmt(Value::Double(2.5), "0"), "3");
        assert_eq!(fmt(Value::Double(2.4), "0"), "2");
        assert_eq!(fmt(Value::Double(-2.5), "0.0"), "-2.5");
    }

    #[test]
    fn named_date_formats() {
        assert_eq!(
            fmt(Value::from_date_serial(45_662.0), "Long Date"),
            "Sunday, January 5, 2025"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.0), "Medium Date"),
            "05-Jan-25"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.0), "Short Date"),
            "1/5/2025"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.656_597_222), "Long Time"),
            "3:45:30 PM"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.656_597_222), "Medium Time"),
            "03:45 PM"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.656_597_222), "Short Time"),
            "15:45"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.656_597_222), "General Date"),
            "1/5/2025 3:45:30 PM"
        );
    }

    #[test]
    fn custom_date_formats() {
        assert_eq!(
            fmt(Value::from_date_serial(45_662.0), "yyyy-mm-dd"),
            "2025-01-05"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.0), "dddd, mmmm d, yyyy"),
            "Sunday, January 5, 2025"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.0), "dd-mmm-yy"),
            "05-Jan-25"
        );
        assert_eq!(
            fmt(
                Value::from_date_serial(45_662.656_597_222),
                "yyyymmdd_hhnnss"
            ),
            "20250105_154530"
        );
        assert_eq!(
            fmt(
                Value::from_date_serial(45_662.656_597_222),
                "hh:nn:ss AM/PM"
            ),
            "03:45:30 PM"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.656_597_222), "hh:mm:ss"),
            "15:45:30"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.656_597_222), "mm/dd/yyyy"),
            "01/05/2025"
        );
        assert_eq!(
            fmt(Value::from_date_serial(45_662.656_597_222), "h:mm AM/PM"),
            "3:45 PM"
        );
    }

    #[test]
    fn m_following_h_is_minutes() {
        assert_eq!(
            fmt(Value::from_date_serial(45_662.656_597_222), "hh:mm"),
            "15:45"
        );
        assert_eq!(fmt(Value::from_date_serial(45_662.656_597_222), "mm"), "01");
    }

    #[test]
    fn string_formats() {
        assert_eq!(fmt(Value::from_string("john doe"), ">"), "JOHN DOE");
        assert_eq!(fmt(Value::from_string("JOHN DOE"), "<"), "john doe");
        assert_eq!(
            fmt(Value::from_string("5551234567"), "(@@@) @@@-@@@@"),
            "(555) 123-4567"
        );
        assert_eq!(fmt(Value::from_string("hello"), "@@@@@@@@@@"), "     hello");
    }

    #[test]
    fn type_mismatch_for_incompatible_values() {
        assert_eq!(
            format_dollar(
                &Value::from_string("abc"),
                Some(&VBString::from("Currency")),
                None,
                None
            )
            .unwrap_err()
            .number,
            err_number::TYPE_MISMATCH
        );
        assert_eq!(
            format_dollar(
                &Value::from_string("abc"),
                Some(&VBString::from("0.00")),
                None,
                None
            )
            .unwrap_err()
            .number,
            err_number::TYPE_MISMATCH
        );
        assert_eq!(
            format_dollar(
                &Value::from_string("abc"),
                Some(&VBString::from("yyyy-mm-dd")),
                None,
                None
            )
            .unwrap_err()
            .number,
            err_number::TYPE_MISMATCH
        );
    }

    #[test]
    fn empty_format_behaves_like_no_format() {
        assert_eq!(fmt(Value::Double(1234.5), ""), "1234.5");
    }
}
