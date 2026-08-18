//! Parse a Put statement.
//!
//! VB6 Put statement syntax:
//! - Put [#]filenumber, [recnumber], varname
//!
//! Writes data from a variable to a disk file.
//!
//! The Put statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | recnumber     | Optional. Variant (Long). Record number (Random mode files) or byte number (Binary mode files) at which writing begins. |
//! | varname       | Required. Valid variable name containing data to be written to disk. |
//!
//! ## Remarks
//! - Put is used with files opened in Binary or Random mode.
//! - For files opened in Random mode, the record length specified in the Open statement determines the number of bytes written.
//! - For files opened in Binary mode, Put writes any number of bytes.
//! - The first record or byte in a file is at position 1, the second at position 2, and so on.
//! - If you omit recnumber, the next record or byte following the last Put or Get statement (or pointed to by the last Seek function) is written.
//! - You must include delimiting commas, for example: Put #1, , myVariable
//! - For files opened in Random mode, the following rules apply:
//!   * If the length of the data being written is less than the length specified in the Len clause, subsequent records on disk are aligned on record-length boundaries.
//!   * The space between the end of one record and the beginning of the next is padded with the existing file contents.
//!   * If the variable being written is a variable-length string, Put writes a 2-byte descriptor containing the string length and then writes the string data.
//! - For files opened in Binary mode, all the Random rules apply, except:
//!   * The Len clause in the Open statement has no effect.
//!   * Put writes the data contiguously, with no padding between records.
//! - Put statements usually mirror Get statements. That is, data written with Put is typically read with Get.
//!
//! ## Examples
//!
//! ```vb
//! Put #1, , myRecord
//! Put #1, recordNumber, customerData
//! Put fileNum, , buffer
//! Put #1, filePosition, userData
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/put-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use vb6core::error::err_number;

/// Write data from a variable to an open file.
///
/// # Arguments
///
/// * `file_number` - The file number to write to.
/// * `record_number` - Optional record/byte number (1-based). If 0, uses current position.
/// * `varname` - The data to write.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn put_statement(
    file_number: i16,
    record_number: Option<i64>,
    varname: &crate::value::VBVariant,
) -> VBResult<()> {
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

    // Convert the value to bytes
    let bytes = match varname {
        crate::value::VBVariant::Long(v) => v.to_le_bytes().to_vec(),
        crate::value::VBVariant::Integer(v) => v.to_le_bytes().to_vec(),
        crate::value::VBVariant::Byte(v) => vec![*v],
        crate::value::VBVariant::Double(v) => v.to_le_bytes().to_vec(),
        crate::value::VBVariant::Single(v) => v.to_le_bytes().to_vec(),
        crate::value::VBVariant::Currency(v) => v.to_le_bytes().to_vec(),
        crate::value::VBVariant::Boolean(v) => {
            if *v {
                1i16.to_le_bytes().to_vec()
            } else {
                0i16.to_le_bytes().to_vec()
            }
        }
        crate::value::VBVariant::Date(v) => v.to_le_bytes().to_vec(),
        crate::value::VBVariant::String(s) => {
            // Write length-prefixed string
            let mut result = Vec::new();
            let len = s.as_str().len() as u16;
            result.extend_from_slice(&len.to_le_bytes());
            result.extend_from_slice(s.as_str().as_bytes());
            result
        }
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in Put statement",
            ));
        }
    };

    // Seek to the position and write
    file::seek_file(file_number, position)?;
    file::write_file(file_number, &bytes).map_err(|e| {
        VBError::with_description(
            57, // Device I/O error
            e.to_string(),
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self, AccessMode, LockMode, OpenMode};
    use crate::value::VBVariant;
    use vb6core::error::err_number;

    #[test]
    fn put_long_to_binary_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

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

        // Write a long value
        put_statement(1, Some(1), &VBVariant::Long(42)).unwrap();

        // Read it back
        let result =
            crate::library::file::get::get_statement(1, Some(1), VBVariant::Long(0)).unwrap();
        assert_eq!(result, VBVariant::Long(42));

        let _ = file::close_all_files();
    }

    #[test]
    fn put_string_to_binary_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

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

        // Write a string
        put_statement(1, Some(1), &VBVariant::from_string("Hello")).unwrap();

        // Read it back
        let result =
            crate::library::file::get::get_statement(1, Some(1), VBVariant::from_string(""))
                .unwrap();
        assert_eq!(result, VBVariant::from_string("Hello"));

        let _ = file::close_all_files();
    }

    #[test]
    fn put_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = put_statement(0, Some(1), &VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }

    #[test]
    fn put_rejects_closed_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let result = put_statement(1, Some(1), &VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }
}
