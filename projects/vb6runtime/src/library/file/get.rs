//! VB6 Get statement syntax:
//! - Get [#]filenumber, [recnumber], varname
//!
//! Reads data from an open disk file into a variable.
//!
//! ## Syntax
//!
//! ```vb
//! Get [#]filenumber, [recnumber], varname
//! ```
//!
//! ## Parameters
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | recnumber     | Optional. Variant (Long). Record number (Random mode files) or byte number (Binary mode files) at which reading begins. |
//! | varname       | Required. Valid variable name into which data is read. |
//!
//! ## Remarks
//! - Get is used with files opened in Binary or Random mode.
//! - For files opened in Random mode, the record length specified in the Open statement determines the number of bytes read.
//! - For files opened in Binary mode, Get reads any number of bytes.
//! - The first record or byte in a file is at position 1, the second at position 2, and so on.
//! - If you omit recnumber, the next record or byte following the last Get or Put statement (or pointed to by the last Seek function) is read.
//! - You must include delimiting commas, for example: Get #1, , myVariable
//! - For files opened in Random mode, the following rules apply:
//!   * If the length of the data being read is less than the length specified in the Len clause, subsequent records on disk are aligned on record-length boundaries.
//!   * The space between the end of one record and the beginning of the next is padded with existing file contents.
//!   * If the variable being read is a variable-length string, Get reads a 2-byte descriptor containing the string length and then reads the string data.
//! - For files opened in Binary mode, all the Random rules apply, except:
//!   * The Len clause in the Open statement has no effect.
//!   * Get reads the data contiguously, with no padding between records.
//!
//! ## Examples
//!
//! ```vb
//! Get #1, , myRecord
//! Get #1, recordNumber, customerData
//! Get fileNum, , buffer
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/get-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::state::file::{MAX_FILE_NUMBER, MIN_FILE_NUMBER};
use crate::value::VBVariant;
use vb6core::error::err_number;

/// Read data from an open file into a variable.
///
/// # Arguments
///
/// * `file_number` - The file number to read from.
/// * `record_number` - Optional record/byte number (1-based). If 0, uses current position.
/// * `varname` - The variable to read into.
///
/// # Returns
///
/// Returns the data read as a VBVariant.
pub fn get_statement(
    file_number: i16,
    record_number: Option<i64>,
    varname: VBVariant,
) -> VBResult<VBVariant> {
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

    // Get file info
    let file = file::get_file(file_number).ok_or_else(|| {
        VBError::with_description(err_number::BAD_FILE_NAME_OR_NUMBER, "File not open")
    })?;

    // Determine position
    let position = if let Some(rec) = record_number {
        if rec <= 0 {
            return Err(VBError::with_description(
                63, // Bad record number
                "Bad record number",
            ));
        }
        rec
    } else {
        // Use current position
        file.position + 1
    };

    // Determine how many bytes to read based on the variable type
    let bytes_to_read = match &varname {
        VBVariant::Long(_) => 4,
        VBVariant::Integer(_) => 2,
        VBVariant::Byte(_) => 1,
        VBVariant::Double(_) => 8,
        VBVariant::Single(_) => 4,
        VBVariant::Currency(_) => 8,
        VBVariant::Boolean(_) => 2,
        VBVariant::Date(_) => 8,
        VBVariant::String(_s) => {
            // For strings, read length-prefixed
            // First, read the 2-byte length prefix
            let mut len_buf = [0u8; 2];
            file::seek_file(file_number, position)?;
            file::read_file(file_number, &mut len_buf)?;
            let str_len = u16::from_le_bytes(len_buf) as usize;

            // Now read the string data
            let mut str_buf = vec![0u8; str_len];
            file::read_file(file_number, &mut str_buf)?;

            // Convert to VBVariant
            let s = String::from_utf8_lossy(&str_buf).to_string();
            return Ok(VBVariant::from_string(s));
        }
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in Get statement",
            ));
        }
    };

    // Read the data
    let mut buf = vec![0u8; bytes_to_read];
    file::seek_file(file_number, position)?;
    let bytes_read = file::read_file(file_number, &mut buf)?;

    if bytes_read < bytes_to_read {
        return Err(VBError::with_description(
            62, // Input past end of file
            "Input past end of file",
        ));
    }

    // Convert bytes to the appropriate type
    let result = match &varname {
        VBVariant::Long(_) => {
            let val = i32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
            VBVariant::Long(val)
        }
        VBVariant::Integer(_) => {
            let val = i16::from_le_bytes([buf[0], buf[1]]);
            VBVariant::Integer(val)
        }
        VBVariant::Byte(_) => VBVariant::Byte(buf[0]),
        VBVariant::Double(_) => {
            let bits = u64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]);
            VBVariant::Double(f64::from_bits(bits))
        }
        VBVariant::Single(_) => {
            let bits = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
            VBVariant::Single(f32::from_bits(bits))
        }
        VBVariant::Currency(_) => {
            let val = i64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]);
            VBVariant::Currency(val)
        }
        VBVariant::Boolean(_) => {
            let val = i16::from_le_bytes([buf[0], buf[1]]);
            VBVariant::Boolean(val != 0)
        }
        VBVariant::Date(_) => {
            let bits = u64::from_le_bytes([
                buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
            ]);
            VBVariant::Date(f64::from_bits(bits))
        }
        _ => unreachable!(),
    };

    Ok(result)
}

/// Get the record length for a file.
pub fn get_record_length(file_number: i16) -> VBResult<i32> {
    let file = file::get_file(file_number).ok_or_else(|| {
        VBError::with_description(err_number::BAD_FILE_NAME_OR_NUMBER, "File not open")
    })?;
    Ok(file.record_length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self, AccessMode, LockMode, OpenMode};
    use vb6core::error::err_number;

    #[test]
    fn get_long_from_binary_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a test file with a long value
        let path = std::path::PathBuf::from("test.bin");
        file::open_file(
            &path,
            OpenMode::Binary,
            AccessMode::ReadWrite,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        // Write a long value (42)
        let data = 42i32.to_le_bytes();
        file::write_file(1, &data).unwrap();

        // Read it back
        let result = get_statement(1, Some(1), VBVariant::Long(0)).unwrap();
        assert_eq!(result, VBVariant::Long(42));

        let _ = file::close_all_files();
    }

    #[test]
    fn get_string_from_binary_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a test file with a string
        let path = std::path::PathBuf::from("test.bin");
        file::open_file(
            &path,
            OpenMode::Binary,
            AccessMode::ReadWrite,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        // Write a length-prefixed string
        let s = "Hello";
        let len = (s.len() as u16).to_le_bytes();
        file::write_file(1, &len).unwrap();
        file::write_file(1, s.as_bytes()).unwrap();

        // Read it back
        let result = get_statement(1, Some(1), VBVariant::from_string("")).unwrap();
        assert_eq!(result, VBVariant::from_string("Hello"));

        let _ = file::close_all_files();
    }

    #[test]
    fn get_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = get_statement(0, Some(1), VBVariant::Long(0));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }

    #[test]
    fn get_rejects_closed_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let result = get_statement(1, Some(1), VBVariant::Long(0));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }
}
