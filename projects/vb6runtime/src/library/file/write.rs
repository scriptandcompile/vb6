//! # Write Statement
//!
//! Writes data to a sequential file.
//!
//! ## Syntax
//!
//! ```vb
//! Write #filenumber, [outputlist]
//! ```
//!
//! ## Parts
//!
//! - **filenumber**: Required. Any valid file number.
//! - **outputlist**: Optional. One or more comma-delimited numeric expressions or string expressions
//!   to write to a file.
//!
//! ## Remarks
//!
//! - **Data Formatting**: Data written with Write # is usually read from a file with Input #.
//! - **Delimiters**: The Write # statement inserts commas between items and quotation marks around
//!   strings as they are written to the file. You don't have to put explicit delimiters in the list.
//! - **Universal Data**: Write # writes data in a universal format that can be read by Input # regardless
//!   of the locale settings.
//! - **Numeric Data**: Numeric data is written with a period (.) as the decimal separator.
//! - **Boolean Values**: Boolean data is written as #TRUE# or #FALSE#.
//! - **Date Values**: Date data is written using the universal date format: #yyyy-mm-dd hh:mm:ss#
//! - **Empty Values**: If outputlist data is Empty, nothing is written. However, if outputlist data is
//!   Null, #NULL# is written.
//! - **Error Data**: Error values are written as #ERROR errorcode#. The number sign (#) ensures the keyword
//!   is not confused with a variable name.
//! - **Comparison with Print #**: Unlike Print #, Write # inserts commas between items and quotes around
//!   strings automatically.
//!
//! ## Examples
//!
//! ### Write Simple Data
//!
//! ```vb
//! Open "test.txt" For Output As #1
//! Write #1, "Hello", 42, True
//! Close #1
//! ' File contents: "Hello",42,#TRUE#
//! ```
//!
//! ### Write Multiple Lines
//!
//! ```vb
//! Open "data.txt" For Output As #1
//! For i = 1 To 10
//!     Write #1, i, i * i, i * i * i
//! Next i
//! Close #1
//! ```
//!
//! ### Write Mixed Data Types
//!
//! ```vb
//! Open "record.txt" For Output As #1
//! Write #1, "John Doe", 30, #1/1/1995#, True
//! Close #1
//! ```
//!
//! ### Write Without Data (New Line)
//!
//! ```vb
//! Open "output.txt" For Output As #1
//! Write #1, "First line"
//! Write #1
//! Write #1, "Third line"
//! Close #1
//! ```
//!
//! ### Write Null and Empty
//!
//! ```vb
//! Open "test.txt" For Output As #1
//! Write #1, Null, Empty, "data"
//! Close #1
//! ' File contents: #NULL#,,"data"
//! ```
//!
//! ### Write Error Values
//!
//! ```vb
//! Open "errors.txt" For Output As #1
//! Write #1, CVErr(2007)
//! Close #1
//! ' File contents: #ERROR 2007#
//! ```
//!
//! ## Common Patterns
//!
//! ### Export Data to CSV-like Format
//!
//! ```vb
//! Sub ExportData()
//!     Open "export.txt" For Output As #1
//!     
//!     ' Write header
//!     Write #1, "Name", "Age", "City"
//!     
//!     ' Write data rows
//!     For i = 0 To UBound(employees)
//!         Write #1, employees(i).Name, employees(i).Age, employees(i).City
//!     Next i
//!     
//!     Close #1
//! End Sub
//! ```
//!
//! ### Write Database Records
//!
//! ```vb
//! Sub SaveRecords()
//!     Open "records.dat" For Output As #1
//!     
//!     Do Until rs.EOF
//!         Write #1, rs!ID, rs!Name, rs!Date, rs!Active
//!         rs.MoveNext
//!     Loop
//!     
//!     Close #1
//! End Sub
//! ```
//!
//! ### Write Configuration Data
//!
//! ```vb
//! Sub SaveConfig()
//!     Open "config.dat" For Output As #1
//!     Write #1, appName, version, lastRun, isRegistered
//!     Close #1
//! End Sub
//! ```
//!
//! ### Write Array Data
//!
//! ```vb
//! Sub WriteArray()
//!     Open "array.dat" For Output As #1
//!     
//!     For i = LBound(data) To UBound(data)
//!         Write #1, data(i)
//!     Next i
//!     
//!     Close #1
//! End Sub
//! ```
//!
//! ### Append Data to Existing File
//!
//! ```vb
//! Sub AppendRecord()
//!     Open "log.txt" For Append As #1
//!     Write #1, Now(), userName, action, details
//!     Close #1
//! End Sub
//! ```

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::value::VBVariant;

/// Write expression list to a sequential file.
///
/// # Arguments
///
/// * `file_number` - The file number to write to.
/// * `values` - The values to write.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn write_statement(file_number: i16, values: &[VBVariant]) -> VBResult<()> {
    // Check file number is valid
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

    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            output.push(',');
        }

        match value {
            VBVariant::Empty => {
                // Empty values write nothing (but still have comma separator)
            }
            VBVariant::Null => {
                output.push_str("#NULL#");
            }
            VBVariant::Boolean(b) => {
                if *b {
                    output.push_str("#TRUE#");
                } else {
                    output.push_str("#FALSE#");
                }
            }
            VBVariant::Long(v) => {
                output.push_str(&v.to_string());
            }
            VBVariant::Integer(v) => {
                output.push_str(&v.to_string());
            }
            VBVariant::Byte(v) => {
                output.push_str(&v.to_string());
            }
            VBVariant::Double(v) => {
                output.push_str(&format!("{}", v));
            }
            VBVariant::Single(v) => {
                output.push_str(&format!("{}", v));
            }
            VBVariant::Currency(v) => {
                let formatted = format_currency(*v);
                output.push_str(&formatted);
            }
            VBVariant::Date(v) => {
                let formatted = crate::value::date_serial_to_string(*v);
                output.push_str(&format!("#{}#", formatted));
            }
            VBVariant::String(s) => {
                // Strings are enclosed in quotes
                output.push('"');
                // Escape any embedded quotes
                for ch in s.as_str().chars() {
                    if ch == '"' {
                        output.push_str("\"\"");
                    } else {
                        output.push(ch);
                    }
                }
                output.push('"');
            }
            VBVariant::Error(e) => {
                output.push_str(&format!("#ERROR {}#", e.number));
            }
            _ => {
                // Objects and arrays can't be written
                return Err(VBError::with_description(
                    13, // Type mismatch
                    "Type mismatch in Write #",
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

    // Write # always adds a newline
    output.push('\r');
    output.push('\n');

    // Write to file
    file::write_file(file_number, output.as_bytes()).map_err(|e| {
        VBError::with_description(
            57, // Device I/O error
            e.to_string(),
        )
    })?;

    Ok(())
}

/// Format a currency value for VB6 Write # output.
fn format_currency(v: i64) -> String {
    let sign = if v < 0 { "-" } else { "" };
    let abs_val = v.unsigned_abs();
    let dollars = abs_val / 10000;
    let cents = abs_val % 10000;
    format!("{}{}.{:04}", sign, dollars, cents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self, AccessMode, LockMode, OpenMode};
    use vb6core::error::err_number;

    #[test]
    fn write_string_with_quotes() {
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

        write_statement(1, &[VBVariant::from_string("Hello")]).unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "\"Hello\"\r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn write_multiple_values() {
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

        write_statement(
            1,
            &[
                VBVariant::from_string("Name"),
                VBVariant::Long(42),
                VBVariant::Double(std::f64::consts::PI),
            ],
        )
        .unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "\"Name\",42,3.141592653589793\r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn write_booleans() {
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

        write_statement(1, &[VBVariant::Boolean(true)]).unwrap();
        write_statement(1, &[VBVariant::Boolean(false)]).unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "#TRUE#\r\n#FALSE#\r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn write_escapes_quotes() {
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

        write_statement(1, &[VBVariant::from_string("He said \"Hello\"")]).unwrap();
        file::close_file(1).unwrap();

        let content = std::fs::read_to_string(dir.path().join("test.txt")).unwrap();
        assert_eq!(content, "\"He said \"\"Hello\"\"\"\r\n");

        let _ = file::close_all_files();
    }

    #[test]
    fn write_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = write_statement(0, &[VBVariant::from_string("test")]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }

    #[test]
    fn write_rejects_closed_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let result = write_statement(1, &[VBVariant::from_string("test")]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }
}
