//! VB6 Unlock statement syntax:
//! - Unlock [#]filenumber[, recordrange]
//!
//! Removes access restrictions on all or part of an open file.
//!
//! The Unlock statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | recordrange   | Optional. Range of records to unlock. Can be: record, start To end, or omitted for entire file. |
//!
//! ## Remarks
//!
//! - Unlock is used to remove locks placed on a file with the Lock statement.
//! - The arguments to Unlock must exactly match those used with the corresponding Lock statement.
//! - For Binary, Input, or Output mode, Unlock always unlocks the entire file regardless of recordrange.
//! - For Random mode, Unlock unlocks the specified record or range of records.
//!
//! ## Examples
//!
//! ```vb
//! Unlock #1
//! Unlock #1, 5
//! Unlock #1, 10 To 20
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/unlock-statement)

use std::io;
use crate::error::{VBError, VBResult};
use vb6core::error::err_number;
use crate::state::file;
use crate::value::VBVariant;

/// Unlock all or part of an open file.
///
/// # Arguments
///
/// * `file_number` - The VB6 file number of the open file.
/// * `record_range` - Optional. Must match a prior `Lock` call.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn unlock_file(file_number: VBVariant, record_range: VBVariant) -> VBResult<()> {
    let file_num = match file_number {
        VBVariant::Integer(n) => n,
        VBVariant::Long(n) => n as i16,
        VBVariant::Byte(n) => n as i16,
        _ => {
            return Err(VBError::with_description(
                err_number::TYPE_MISMATCH,
                "Type mismatch in Unlock statement",
            ));
        }
    };

    if !(file::MIN_FILE_NUMBER..=file::MAX_FILE_NUMBER).contains(&file_num) {
        return Err(VBError::with_description(
            err_number::BAD_FILE_NAME_OR_NUMBER,
            "Bad file name or number",
        ));
    }

    if !file::is_file_open(file_num) {
        return Err(VBError::with_description(
            err_number::BAD_FILE_NAME_OR_NUMBER,
            "File not open",
        ));
    }

    let range = super::lock::parse_record_range(&record_range)?;

    file::unlock_file(file_num, range).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                io::ErrorKind::NotFound => err_number::BAD_FILE_NAME_OR_NUMBER,
                _ => err_number::DEVICE_IO_ERROR,
            },
            e.to_string(),
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use vb6core::error::err_number;
    use super::*;
    use crate::state::file::{self};

    #[test]
    fn unlock_entire_file() {
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

        // Lock then unlock
        let _ = file::lock_file(1, None);
        unlock_file(VBVariant::Integer(1), VBVariant::Empty).unwrap();

        let _ = file::close_all_files();
    }

    #[test]
    fn unlock_rejects_bad_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = unlock_file(VBVariant::Integer(999), VBVariant::Empty);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );
    }

    #[test]
    fn unlock_rejects_non_numeric() {
        let _guard = crate::state::test_support::lock_test();

        let result = unlock_file(VBVariant::from_string("abc"), VBVariant::Empty);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }
}
