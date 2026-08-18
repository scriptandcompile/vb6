//! # Name Statement
//!
//! Renames a disk file, directory, or folder.
//!
//! ## Syntax
//!
//! ```vb
//! Name oldpathname As newpathname
//! ```
//!
//! - `oldpathname`: Required. String expression that specifies the existing file name and location. May include directory or folder, and drive.
//! - `newpathname`: Required. String expression that specifies the new file name and location. May include directory or folder, and drive.
//!   Cannot specify a different drive from the one specified in `oldpathname`.
//!
//! ## Remarks
//!
//! - The `Name` statement renames a file and moves it to a different directory or folder, if necessary
//! - `Name` can move a file across directories or folders, but both `oldpathname` and `newpathname` must be on the same drive
//! - Using `Name` on an open file produces an error. You must close an open file before renaming it
//! - `Name` arguments can include relative or absolute paths
//! - The `Name` statement can also rename directories or folders
//! - If `newpathname` already exists, an error occurs
//! - Wildcard characters (* and ?) are not allowed in either `oldpathname` or `newpathname`
//!
//! ## Examples
//!
//! ```vb
//! ' Rename a file
//! Name "OLDFILE.TXT" As "NEWFILE.TXT"
//!
//! ' Move and rename a file
//! Name "C:\Data\Report.doc" As "C:\Archive\OldReport.doc"
//!
//! ' Rename a directory
//! Name "C:\OldFolder" As "C:\NewFolder"
//!
//! ' Move file to different directory (same drive)
//! Name "C:\Temp\Test.dat" As "C:\Data\Test.dat"
//!
//! ' Using variables
//! Dim oldName As String, newName As String
//! oldName = "File1.txt"
//! newName = "File2.txt"
//! Name oldName As newName
//! ```
//!
//! ## Reference
//!
//! [Name Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/name-statement)

use crate::error::{VBError, VBResult};
use vb6core::error::err_number;
use crate::state::file;
use crate::value::VBVariant;

/// Rename a file, directory, or folder.
///
/// # Arguments
///
/// * `old_pathname` - The current file path.
/// * `new_pathname` - The new file path.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn name_statement(old_pathname: VBVariant, new_pathname: VBVariant) -> VBResult<()> {
    // Get old path
    let old_str = match old_pathname {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in Name statement",
            ));
        }
    };

    // Get new path
    let new_str = match new_pathname {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in Name statement",
            ));
        }
    };

    // Check source exists
    if !file::file_exists(std::path::Path::new(&old_str)) {
        return Err(VBError::with_description(
            53, // File not found
            format!("File not found: {}", old_str),
        ));
    }

    // Check destination doesn't exist
    if file::file_exists(std::path::Path::new(&new_str)) {
        return Err(VBError::with_description(
            75, // Path/File access error
            format!("File already exists: {}", new_str),
        ));
    }

    // Rename the file through the backend
    file::rename_file(
        std::path::Path::new(&old_str),
        std::path::Path::new(&new_str),
    )
    .map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::NotFound => err_number::FILE_NOT_FOUND,         // File not found
                std::io::ErrorKind::PermissionDenied => err_number::PERMISSION_DENIED, // Permission denied
                _ => err_number::PATH_FILE_ACCESS_ERROR,                                    // Path/File access error
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
    fn name_renames_file() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a file
        std::fs::write(dir.path().join("old.txt"), "Hello").unwrap();

        // Rename it
        name_statement(
            VBVariant::from_string("old.txt"),
            VBVariant::from_string("new.txt"),
        )
        .unwrap();

        // Verify
        assert!(!dir.path().join("old.txt").exists());
        assert!(dir.path().join("new.txt").exists());
        let content = std::fs::read_to_string(dir.path().join("new.txt")).unwrap();
        assert_eq!(content, "Hello");
    }

    #[test]
    fn name_rejects_nonexistent_source() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let result = name_statement(
            VBVariant::from_string("nonexistent.txt"),
            VBVariant::from_string("new.txt"),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::FILE_NOT_FOUND);
    }

    #[test]
    fn name_rejects_existing_destination() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        std::fs::write(dir.path().join("old.txt"), "Hello").unwrap();
        std::fs::write(dir.path().join("new.txt"), "World").unwrap();

        let result = name_statement(
            VBVariant::from_string("old.txt"),
            VBVariant::from_string("new.txt"),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::PATH_FILE_ACCESS_ERROR);
    }

    #[test]
    fn name_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = name_statement(VBVariant::Long(42), VBVariant::from_string("new.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);

        let result = name_statement(VBVariant::from_string("old.txt"), VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }
}
