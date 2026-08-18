//! VB6 Reset statement syntax:
//! - Reset
//!
//! Closes all disk files opened using the Open statement.
//!
//! The Reset statement closes all active files opened by the Open statement
//! and writes the contents of all file buffers to disk.
//!
//! Use Reset to ensure all file data is written to disk before ending your program.
//! This is particularly important in programs that may terminate abnormally.
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/reset-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;

/// Close all open files and flush their buffers.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn reset_statement() -> VBResult<()> {
    file::close_all_files().map_err(|e| {
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

    #[test]
    fn reset_closes_all_files() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Open multiple files
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

        // Reset
        reset_statement().unwrap();

        assert!(!file::is_file_open(1));
        assert!(!file::is_file_open(2));
    }

    #[test]
    fn reset_succeeds_with_no_open_files() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        // Reset with no files open
        let result = reset_statement();
        assert!(result.is_ok());
    }
}
