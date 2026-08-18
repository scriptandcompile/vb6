//! VB6 Lock statement syntax:
//! - Lock [#]filenumber[, recordrange]
//!
//! Controls access to all or part of an open file.
//!
//! The Lock statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | recordrange   | Optional. Range of records to lock. Can be: record, start To end, or omitted for entire file. |
//!
//! ## Remarks
//!
//! - Lock and Unlock are used in environments where multiple processes might need access to the same file.
//! - Lock and Unlock statements are always used in pairs.
//! - For Binary, Input, or Output mode, Lock always locks the entire file regardless of recordrange.
//! - For Random mode, Lock locks the specified record or range of records.
//!
//! ## Examples
//!
//! ```vb
//! Lock #1
//! Lock #1, 5
//! Lock #1, 10 To 20
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/lock-statement)

use crate::error::{VBError, VBResult};
use vb6core::error::err_number;
use crate::state::file;
use crate::value::VBVariant;

/// Lock all or part of an open file for exclusive access.
///
/// # Arguments
///
/// * `file_number` - The VB6 file number of the open file.
/// * `record_range` - Optional. A `VBVariant` array specifying the record range:
///   - `Empty` or omitted: lock the entire file
///   - Single `Long`: lock that one record
///   - Two `Long` values: lock the range from first to second (inclusive)
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn lock_file(file_number: VBVariant, record_range: VBVariant) -> VBResult<()> {
    let file_num = match file_number {
        VBVariant::Integer(n) => n,
        VBVariant::Long(n) => n as i16,
        VBVariant::Byte(n) => n as i16,
        _ => {
            return Err(VBError::with_description(
                err_number::TYPE_MISMATCH,
                "Type mismatch in Lock statement",
            ));
        }
    };

    // Validate file number
    if !(file::MIN_FILE_NUMBER..=file::MAX_FILE_NUMBER).contains(&file_num) {
        return Err(VBError::with_description(
            err_number::BAD_FILE_NAME_OR_NUMBER,
            "Bad file name or number",
        ));
    }

    // Check file is open
    if !file::is_file_open(file_num) {
        return Err(VBError::with_description(
            err_number::BAD_FILE_NAME_OR_NUMBER,
            "File not open",
        ));
    }

    let range = parse_record_range(&record_range)?;

    file::lock_file(file_num, range).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                io::ErrorKind::PermissionDenied => err_number::FILE_ALREADY_OPEN,
                io::ErrorKind::NotFound => err_number::BAD_FILE_NAME_OR_NUMBER,
                _ => err_number::DEVICE_IO_ERROR,
            },
            e.to_string(),
        )
    })?;

    Ok(())
}

/// Parse the optional record range from a VBVariant.
///
/// Returns `None` for entire-file lock, or `Some((start, end))` for a range.
pub(crate) fn parse_record_range(range: &VBVariant) -> VBResult<Option<(i32, i32)>> {
    match range {
        VBVariant::Empty => Ok(None),
        VBVariant::Long(n) => {
            let n = *n;
            if n < 1 {
                return Err(VBError::with_description(
                    err_number::BAD_RECORD_NUMBER,
                    "Bad record number",
                ));
            }
            Ok(Some((n, n)))
        }
        VBVariant::Integer(n) => {
            let n = *n as i32;
            if n < 1 {
                return Err(VBError::with_description(
                    err_number::BAD_RECORD_NUMBER,
                    "Bad record number",
                ));
            }
            Ok(Some((n, n)))
        }
        _ => Err(VBError::with_description(
            err_number::TYPE_MISMATCH,
            "Type mismatch in Lock statement",
        )),
    }
}

use std::io;

#[cfg(test)]
mod tests {
    use vb6core::error::err_number;
    use super::*;
    use crate::state::file::{self};

    #[test]
    fn lock_entire_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, "Hello").unwrap();

        file::open_file(
            &path,
            file::OpenMode::Random,
            file::AccessMode::ReadWrite,
            file::LockMode::Shared,
            1,
            1,
        )
        .unwrap();

        lock_file(VBVariant::Integer(1), VBVariant::Empty).unwrap();

        // Unlock and close
        let _ = file::unlock_file(1, None);
        let _ = file::close_all_files();
    }

    #[test]
    fn lock_record_range() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.txt");
        std::fs::write(&path, "Hello").unwrap();

        file::open_file(
            &path,
            file::OpenMode::Random,
            file::AccessMode::ReadWrite,
            file::LockMode::Shared,
            1,
            1,
        )
        .unwrap();

        // Lock record 5 (single record)
        lock_file(VBVariant::Integer(1), VBVariant::Long(5)).unwrap();

        let _ = file::unlock_file(1, Some((5, 5)));
        let _ = file::close_all_files();
    }

    #[test]
    fn lock_rejects_bad_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = lock_file(VBVariant::Integer(999), VBVariant::Empty);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );
    }

    #[test]
    fn lock_rejects_non_numeric() {
        let _guard = crate::state::test_support::lock_test();

        let result = lock_file(VBVariant::from_string("abc"), VBVariant::Empty);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }
}
