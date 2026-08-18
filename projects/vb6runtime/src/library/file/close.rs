//! VB6 Close statement syntax:
//! - Close [filenumberlist]
//!
//! Closes input or output files opened using the Open statement.
//!
//! ## Parameters
//!
//! - `filenumberlist` - Optional. One or more file numbers using the syntax:
//!   [[#]filenumber] [, [#]filenumber] ...
//!
//! If `filenumberlist` is omitted, all active files opened by the Open statement are closed.
//!
//! ## Reference
//!
//! [Close Statement](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/close-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::state::file::{MAX_FILE_NUMBER, MIN_FILE_NUMBER};

/// Close one or more open files.
///
/// # Arguments
///
/// * `file_numbers` - A list of file numbers to close. If empty, all files are closed.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn close_files(file_numbers: &[i16]) -> VBResult<()> {
    if file_numbers.is_empty() {
        // Close all files
        file::close_all_files().map_err(|e| {
            VBError::with_description(
                57, // Device I/O error
                e.to_string(),
            )
        })?;
    } else {
        // Close specified files
        for &num in file_numbers {
            if !(MIN_FILE_NUMBER..=MAX_FILE_NUMBER).contains(&num) {
                return Err(VBError::with_description(
                    52, // Bad file name or number
                    format!("Bad file name or number: {}", num),
                ));
            }
            file::close_file(num).map_err(|e| {
                VBError::with_description(
                    57, // Device I/O error
                    e.to_string(),
                )
            })?;
        }
    }
    Ok(())
}

/// Close a single file by number.
///
/// # Arguments
///
/// * `file_number` - The file number to close (1-511).
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn close_file(file_number: i16) -> VBResult<()> {
    if !(MIN_FILE_NUMBER..=MAX_FILE_NUMBER).contains(&file_number) {
        return Err(VBError::with_description(
            52, // Bad file name or number
            format!("Bad file name or number: {}", file_number),
        ));
    }

    file::close_file(file_number).map_err(|e| {
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
    use vb6core::error::err_number;

    #[test]
    fn close_single_file() {
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
        assert!(file::is_file_open(1));

        close_file(1).unwrap();
        assert!(!file::is_file_open(1));

        let _ = file::close_all_files();
    }

    #[test]
    fn close_multiple_files() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path1 = std::path::PathBuf::from("test1.txt");
        let path2 = std::path::PathBuf::from("test2.txt");
        file::open_file(
            &path1,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();
        file::open_file(
            &path2,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            2,
        )
        .unwrap();

        assert!(file::is_file_open(1));
        assert!(file::is_file_open(2));

        close_files(&[1, 2]).unwrap();

        assert!(!file::is_file_open(1));
        assert!(!file::is_file_open(2));

        let _ = file::close_all_files();
    }

    #[test]
    fn close_all_files() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path1 = std::path::PathBuf::from("test1.txt");
        let path2 = std::path::PathBuf::from("test2.txt");
        file::open_file(
            &path1,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();
        file::open_file(
            &path2,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            2,
        )
        .unwrap();

        assert!(file::is_file_open(1));
        assert!(file::is_file_open(2));

        close_files(&[]).unwrap(); // Empty list = close all

        assert!(!file::is_file_open(1));
        assert!(!file::is_file_open(2));
    }

    #[test]
    fn close_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = close_file(0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let result = close_file(512);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().number,
            err_number::BAD_FILE_NAME_OR_NUMBER
        );

        let _ = file::close_all_files();
    }
}
