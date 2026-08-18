//! VB6 Kill statement syntax:
//! - Kill pathname
//!
//! Deletes files from a disk.
//!
//! The Kill statement syntax has this part:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | pathname      | Required. String expression that specifies one or more file names to be deleted. May include directory or folder, and drive. |
//!
//! Remarks:
//! - Kill supports the use of multiple-character (*) and single-character (?) wildcards to specify multiple files.
//! - An error occurs if you try to use Kill to delete an open file.
//! - To remove a directory or folder, use the RmDir statement.
//!
//! ## Examples
//!
//! ```vb
//! Kill "C:\DATA.TXT"
//! Kill "C:\*.TXT"           ' Delete all .txt files
//! Kill "C:\TEST?.TXT"       ' Delete TEST1.TXT, TESTA.TXT, etc.
//! Kill App.Path & "\temp.dat"
//! Kill myFileName
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/kill-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::value::VBVariant;

/// Delete a file from disk.
///
/// # Arguments
///
/// * `pathname` - The file path to delete.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn kill(pathname: VBVariant) -> VBResult<()> {
    // Get the file path
    let path_str = match pathname {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in Kill",
            ));
        }
    };

    // Delete the file through the backend
    file::remove_file(std::path::Path::new(&path_str)).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::NotFound => 53,         // File not found
                std::io::ErrorKind::PermissionDenied => 70, // Permission denied
                _ => 57,                                    // Device I/O error
            },
            e.to_string(),
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self};

    #[test]
    fn kill_deletes_file() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a file
        std::fs::write(dir.path().join("test.txt"), "Hello").unwrap();
        assert!(dir.path().join("test.txt").exists());

        // Kill it
        kill(VBVariant::from_string("test.txt")).unwrap();
        assert!(!dir.path().join("test.txt").exists());
    }

    #[test]
    fn kill_rejects_nonexistent_file() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let result = kill(VBVariant::from_string("nonexistent.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 53);
    }

    #[test]
    fn kill_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = kill(VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 13);
    }
}
