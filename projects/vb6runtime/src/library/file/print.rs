//! VB6 Print # statement syntax:
//! - Print #filenumber, [outputlist]
//!
//! Writes display-formatted data to a sequential file.
//!
//! The Print # statement syntax has these parts:
//!
//! | Part        | Description |
//! |-------------|-------------|
//! | filenumber  | Required. Any valid file number. |
//! | outputlist  | Optional. Expression or list of expressions to print. |
//!
//! ## Remarks
//!
//! - Data written with Print # is usually read from a file with Line Input # or Input.
//! - If you omit outputlist and include only a list separator after filenumber, a blank line is printed to the file.
//! - Multiple expressions can be separated with either a space or a semicolon.
//! - A space has the same effect as a semicolon.
//! - For Boolean data, either True or False is printed.
//! - The True and False keywords are not translated, regardless of locale.
//! - Date data is written to the file using the standard short date format recognized by your system.
//! - When either the date or the time component is missing or zero, only the part provided gets written to the file.
//! - Nothing is written to the file if outputlist data is Empty. However, if outputlist data is Null, Null is output to the file.
//! - For error data, the output appears as Error errorcode. The Error keyword is not translated, regardless of locale.
//! - All data written to the file using Print # is internationally aware; that is, the data is properly formatted using the appropriate decimal separator and thousands separator.
//! - When data is written to a file, several universal assumptions are followed:
//!   * Numeric data is always written using the period as the decimal separator.
//!   * For numeric data, a leading space is always reserved for the sign of the number.
//!   * A trailing space is included after each number.
//! - Unlike the Print method, the Print # statement doesn't insert commas or spaces between items as they are written to the file.
//! - When you use the Print # statement, you insert explicit delimiters in your output list when you want to add commas or spaces.
//! - The Print # statement usually writes Variant data to a file the same way it writes other data types.
//! - However, there are some exceptions:
//!   * If the data being written is a Variant of VarType vbError, an error message string is not written to the file.
//!   * Only the word Error and the error code are written.
//!   * If the data being written is a Variant of VarType vbEmpty, nothing is written to the file.
//!
//! ## Examples
//!
//! ```vb
//! ' Basic usage
//! Print #1, "Hello World"
//!
//! ' Multiple items
//! Print #1, x, y, z
//!
//! ' With semicolon separator
//! Print #1, "Name: "; userName; " Age: "; userAge
//!
//! ' Blank line
//! Print #1,
//!
//! ' Variable file number
//! Dim fileNum As Integer
//! fileNum = FreeFile
//! Print #fileNum, data
//!
//! ' Complex expressions
//! Print #1, Format$(Now, "yyyy-mm-dd"), totalAmount
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/print-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::value::VBVariant;

/// Write display-formatted data to a sequential file.
///
/// # Arguments
///
/// * `file_number` - The file number to write to.
/// * `values` - The values to write.
/// * `newline` - Whether to append a newline at the end.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn print_statement(file_number: i16, values: &[VBVariant], newline: bool) -> VBResult<()> {
    // Check file number is valid
    // Validate file number range
    if !(file::MIN_FILE_NUMBER..=file::MAX_FILE_NUMBER).contains(&file_number) {
        return Err(VBError::with_description(
            52, // Bad file name or number
            format!("Bad file name or number: {}", file_number),
        ));
    }

    // Check file is open
    if !file::is_file_open(file_number) {
        return Err(VBError::with_description(
            52, // Bad file name or number
            format!("File not open: #{}", file_number),
        ));
    }

    // Get the current width for this file (0 means no limit)
    let width = file::with_file_mut(file_number, |file| file.width).unwrap_or(0);

    // Build the output string
    let mut output = String::new();

    for value in values.iter() {
        match value {
            VBVariant::Empty => {
                // Empty values print nothing
            }
            VBVariant::Null => {
                output.push_str("Null");
            }
            VBVariant::Boolean(b) => {
                output.push_str(if *b { "True" } else { "False" });
            }
            VBVariant::Long(v) => {
                // Leading space for positive numbers
                if *v >= 0 {
                    output.push(' ');
                }
                output.push_str(&v.to_string());
                output.push(' ');
            }
            VBVariant::Integer(v) => {
                if *v >= 0 {
                    output.push(' ');
                }
                output.push_str(&v.to_string());
                output.push(' ');
            }
            VBVariant::Byte(v) => {
                output.push(' ');
                output.push_str(&v.to_string());
                output.push(' ');
            }
            VBVariant::Double(v) => {
                if *v >= 0.0 {
                    output.push(' ');
                }
                output.push_str(&format_f64(*v));
                output.push(' ');
            }
            VBVariant::Single(v) => {
                if *v >= 0.0 {
                    output.push(' ');
                }
                output.push_str(&format_f64(*v as f64));
                output.push(' ');
            }
            VBVariant::Currency(v) => {
                let formatted = format_currency(*v);
                if !formatted.starts_with('-') {
                    output.push(' ');
                }
                output.push_str(&formatted);
                output.push(' ');
            }
            VBVariant::Date(v) => {
                let formatted = crate::value::date_serial_to_string(*v);
                output.push_str(&formatted);
            }
            VBVariant::String(s) => {
                output.push_str(s.as_str());
            }
            VBVariant::Error(e) => {
                output.push_str(&format!("Error {}", e.number));
            }
            _ => {
                // Objects and arrays can't be printed
                return Err(VBError::with_description(
                    13, // Type mismatch
                    "Type mismatch in Print #",
                ));
            }
        }
    }

    // Check width and add newline if output would exceed the width limit
    if width > 0 && !output.is_empty() {
        // If output would exceed the width limit, start a new line
        if output.len() > width as usize {
            output.push('\n');
        }
    }

    // Track the column position of what we're about to write
    let char_count = output.chars().count();

    if newline {
        output.push('\r');
        output.push('\n');
    }

    // Write to file
    file::write_file(file_number, output.as_bytes()).map_err(|e| {
        VBError::with_description(
            57, // Device I/O error
            e.to_string(),
        )
    })?;

    // Update the print column tracking
    if newline {
        // After newline, reset to column 1
        file::reset_print_column(file_number);
    } else {
        // Advance by the number of characters written
        file::advance_print_column(file_number, char_count);
    }

    Ok(())
}

/// Format an f64 for VB6 Print # output.
fn format_f64(v: f64) -> String {
    if v == v.floor() && v.abs() < 1e15 {
        // Integer-like value
        format!("{}", v as i64)
    } else {
        format!("{}", v)
    }
}

/// Format a currency value for VB6 Print # output.
fn format_currency(v: i64) -> String {
    let sign = if v < 0 { "-" } else { "" };
    let abs_val = v.unsigned_abs();
    let dollars = abs_val / 10000;
    let cents = abs_val % 10000;
    format!("{}{}.{:04}", sign, dollars, cents)
}

/// Write display-formatted data with a trailing newline (Print # with ,).
pub fn print_statement_with_newline(file_number: i16, values: &[VBVariant]) -> VBResult<()> {
    print_statement(file_number, values, true)
}

/// Write display-formatted data without a trailing newline (Print # with ;).
pub fn print_statement_without_newline(file_number: i16, values: &[VBVariant]) -> VBResult<()> {
    print_statement(file_number, values, false)
}

/// Write a blank line (Print # with just ,).
pub fn print_blank_line(file_number: i16) -> VBResult<()> {
    print_statement(file_number, &[], true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self, AccessMode, LockMode, OpenMode};
    use vb6core::error::err_number;

    #[test]
    fn print_writes_string() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        print_statement(1, &[VBVariant::from_string("Hello")], true).unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "Hello\r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn print_writes_number() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        print_statement(1, &[VBVariant::Long(42)], true).unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, " 42 \r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn print_writes_multiple_values() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        print_statement(
            1,
            &[
                VBVariant::from_string("Name:"),
                VBVariant::from_string("John"),
                VBVariant::Long(25),
            ],
            true,
        )
        .unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "Name:John 25 \r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn print_without_newline() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        print_statement(1, &[VBVariant::from_string("Hello")], false).unwrap();
        print_statement(1, &[VBVariant::from_string(" World")], true).unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "Hello World\r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn print_blank_line_test() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        print_statement(1, &[VBVariant::from_string("Line 1")], true).unwrap();
        super::print_blank_line(1).unwrap();
        print_statement(1, &[VBVariant::from_string("Line 3")], true).unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "Line 1\r\n\r\nLine 3\r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn print_writes_boolean() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        print_statement(1, &[VBVariant::Boolean(true)], true).unwrap();
        print_statement(1, &[VBVariant::Boolean(false)], true).unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "True\r\nFalse\r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn print_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = print_statement(0, &[VBVariant::from_string("test")], true);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }

    #[test]
    fn print_rejects_closed_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let result = print_statement(1, &[VBVariant::from_string("test")], true);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }
}
