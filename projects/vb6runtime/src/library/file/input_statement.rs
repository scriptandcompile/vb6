//! VB6 Input statement syntax:
//! - Input #filenumber, varlist
//!
//! Reads data from an open sequential file and assigns the data to variables.
//!
//! The Input # statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | varlist       | Required. Comma-delimited list of variables that are assigned values read from the file. Variables can't be arrays or object variables. However, variables that describe an element of an array or user-defined type may be used. |
//!
//! ## Remarks
//!
//! - Data read with Input # is usually written to a file with Write #.
//! - Use this statement only with files opened in Input or Binary mode.
//! - The Input # statement reads data items from a sequential file and assigns them to variables.
//! - Data items in the file must appear in the same order as the variables in varlist and be separated by commas.
//! - If the data item to be read is a quoted string, Input # strips the quotation marks.
//! - Input # is typically used to read data that was written to a file using the Write # statement.
//! - For files opened for Binary access, Input # reads all the bytes it needs to complete the varlist.
//! - If end of file is reached before all variables are filled, an error occurs.
//!
//! ## Examples
//!
//! ```vb
//! Input #1, name, age
//! Input #fileNum, x, y, z
//! Input #1, firstName, lastName, address
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/input-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::state::file::{MAX_FILE_NUMBER, MIN_FILE_NUMBER};
use crate::value::VBVariant;

/// Read data from an open sequential file.
///
/// # Arguments
///
/// * `file_number` - The file number to read from.
///
/// # Returns
///
/// Returns a vector of values read from the file.
pub fn input_statement(file_number: i16) -> VBResult<Vec<VBVariant>> {
    // Check file number is valid
    if !(MIN_FILE_NUMBER..=MAX_FILE_NUMBER).contains(&file_number) {
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

    // Read a line from the file
    let mut values = Vec::new();
    let mut in_quotes = false;
    let mut current_value = String::new();

    loop {
        let mut buf = [0u8; 1];
        let bytes_read = file::read_file(file_number, &mut buf)?;

        if bytes_read == 0 {
            // EOF
            if !current_value.is_empty() || in_quotes {
                values.push(parse_value(&current_value));
            }
            break;
        }

        let ch = buf[0] as char;

        if ch == '"' {
            in_quotes = !in_quotes;
            // Don't push here — the next comma or newline will push the value.
        } else if ch == ',' && !in_quotes {
            // Field separator
            values.push(parse_value(&current_value));
            current_value.clear();
        } else if ch == '\n' && !in_quotes {
            // End of line
            let trimmed = current_value.trim_end_matches('\r');
            if !trimmed.is_empty() {
                values.push(parse_value(trimmed));
            }
            break;
        } else {
            current_value.push(ch);
        }
    }

    Ok(values)
}

/// Parse a value from a string, converting to appropriate VB6 type.
fn parse_value(s: &str) -> VBVariant {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return VBVariant::Empty;
    }

    // Try to parse as number
    if let Ok(long_val) = trimmed.parse::<i32>() {
        return VBVariant::Long(long_val);
    }

    if let Ok(double_val) = trimmed.parse::<f64>() {
        return VBVariant::Double(double_val);
    }

    // Try to parse as boolean
    match trimmed.to_uppercase().as_str() {
        "TRUE" => return VBVariant::Boolean(true),
        "FALSE" => return VBVariant::Boolean(false),
        _ => {}
    }

    // Default to string
    VBVariant::from_string(trimmed)
}

#[cfg(test)]
mod tests {
    use vb6core::error::err_number;
    use super::*;
    use crate::state::file::{self, AccessMode, LockMode, OpenMode};
    use std::io::Write;

    #[test]
    fn input_reads_comma_separated_values() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a test file with comma-separated values
        let mut test_file = std::fs::File::create(dir.path().join("test.txt")).unwrap();
        writeln!(test_file, "Hello,42,3.14").unwrap();
        drop(test_file);

        // Open for input
        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Input,
            AccessMode::Read,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        // Read values
        let values = input_statement(1).unwrap();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], VBVariant::from_string("Hello"));
        assert_eq!(values[1], VBVariant::Long(42));
        assert_eq!(values[2], VBVariant::Double(3.14));

        let _ = file::close_all_files();
    }

    #[test]
    fn input_handles_quoted_strings() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a test file with quoted strings
        let mut test_file = std::fs::File::create(dir.path().join("test.txt")).unwrap();
        writeln!(test_file, "\"Hello, World\",42").unwrap();
        drop(test_file);

        // Open for input
        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Input,
            AccessMode::Read,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        // Read values
        let values = input_statement(1).unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], VBVariant::from_string("Hello, World"));
        assert_eq!(values[1], VBVariant::Long(42));

        let _ = file::close_all_files();
    }

    #[test]
    fn input_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = input_statement(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::BAD_FILE_NAME_OR_NUMBER);

        let _ = file::close_all_files();
    }

    #[test]
    fn input_rejects_closed_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let result = input_statement(1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::BAD_FILE_NAME_OR_NUMBER);

        let _ = file::close_all_files();
    }
}
