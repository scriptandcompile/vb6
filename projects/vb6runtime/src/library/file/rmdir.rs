//! # `RmDir` Statement
//!
//! Removes an empty directory or folder.
//!
//! ## Syntax
//!
//! ```vb
//! RmDir path
//! ```
//!
//! - `path`: Required. String expression that identifies the directory or folder to be removed. May include drive.
//!   If no drive is specified, `RmDir` removes the directory or folder on the current drive.
//!
//! ## Remarks
//!
//! - An error occurs if you try to use `RmDir` on a directory containing files. Use the Kill statement to delete all files before attempting to remove a directory.
//! - An error also occurs if you try to remove a directory that doesn't exist.
//! - The directory must be empty (contain no files or subdirectories) before it can be removed.
//! - The `path` argument can include absolute or relative paths.
//! - On Windows systems, both forward slashes (/) and backslashes (\) can be used as path separators.
//! - The directory name can include the drive letter.
//! - You cannot remove the current directory. You must change to a parent or different directory first.
//! - UNC paths are supported on network drives.
//! - To remove a directory tree, you must remove all subdirectories first (working from innermost to outermost).
//!
//! ## Examples
//!
//! ```vb
//! ' Remove a directory in the current directory
//! RmDir "OldFolder"
//!
//! ' Remove a directory with full path
//! RmDir "C:\Temp\TempFiles"
//!
//! ' Remove a directory on another drive
//! RmDir "D:\Data\Archive"
//!
//! ' Remove nested directories (must remove innermost first)
//! RmDir "C:\Temp\Logs\Archive"
//! RmDir "C:\Temp\Logs"
//! RmDir "C:\Temp"
//!
//! ' Remove directory on network drive
//! RmDir "\\Server\Share\OldFolder"
//!
//! ' Safe removal with error handling
//! On Error Resume Next
//! RmDir "C:\Temp\ToDelete"
//! If Err.Number <> 0 Then
//!     MsgBox "Could not remove directory"
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Common Errors
//!
//! - **Error 75**: Path/File access error - directory contains files or subdirectories
//! - **Error 76**: Path not found - directory doesn't exist
//! - **Error 5**: Invalid procedure call - trying to remove current directory
//!
//! ## Reference
//!
//! [RmDir Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/rmdir-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::value::VBVariant;

/// Remove an existing directory.
///
/// # Arguments
///
/// * `path` - The directory path to remove.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn rmdir(path: VBVariant) -> VBResult<()> {
    // Get the path
    let path_str = match path {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in RmDir",
            ));
        }
    };

    // Remove the directory through the backend
    file::remove_dir(std::path::Path::new(&path_str)).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::NotFound => 76,         // Path not found
                std::io::ErrorKind::PermissionDenied => 70, // Permission denied
                _ => 75,                                    // Path/File access error
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
    fn rmdir_removes_directory() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create directory
        std::fs::create_dir(dir.path().join("toremove")).unwrap();
        assert!(dir.path().join("toremove").exists());

        // Remove it
        rmdir(VBVariant::from_string("toremove")).unwrap();

        // Verify
        assert!(!dir.path().join("toremove").exists());
    }

    #[test]
    fn rmdir_rejects_nonexistent_directory() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let result = rmdir(VBVariant::from_string("nonexistent"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 76);
    }

    #[test]
    fn rmdir_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = rmdir(VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 13);
    }
}
