//! # `MkDir` Statement
//!
//! Creates a new directory or folder.
//!
//! ## Syntax
//!
//! ```vb
//! MkDir path
//! ```
//!
//! - `path`: Required. String expression that identifies the directory or folder to be created. May include drive.
//!   If no drive is specified, `MkDir` creates the new directory or folder on the current drive.
//!
//! ## Remarks
//!
//! - An error occurs if you try to create a directory or folder that already exists
//! - The `path` argument can include absolute or relative paths
//! - You can use `MkDir` to create nested directories by creating parent directories first
//! - On Windows systems, both forward slashes (/) and backslashes (\) can be used as path separators
//! - The directory name can include the drive letter
//! - UNC paths are supported on network drives
//!
//! ## Examples
//!
//! ```vb
//! ' Create a directory in the current directory
//! MkDir "MyNewFolder"
//!
//! ' Create a directory with full path
//! MkDir "C:\Program Files\MyApp"
//!
//! ' Create a directory on another drive
//! MkDir "D:\Data\Reports"
//!
//! ' Create nested directories (parent must exist first)
//! MkDir "C:\Temp"
//! MkDir "C:\Temp\Logs"
//! MkDir "C:\Temp\Logs\Archive"
//!
//! ' Create directory on network drive
//! MkDir "\\Server\Share\NewFolder"
//! ```
//!
//! ## Reference
//!
//! [MkDir Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/mkdir-statement)

use crate::error::{VBError, VBResult};
use vb6core::error::err_number;
use crate::state::file;
use crate::value::VBVariant;

/// Create a new directory.
///
/// # Arguments
///
/// * `path` - The directory path to create.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn mkdir(path: VBVariant) -> VBResult<()> {
    // Get the path
    let path_str = match path {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in MkDir",
            ));
        }
    };

    // Create the directory through the backend
    file::create_dir(std::path::Path::new(&path_str)).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => err_number::PATH_FILE_ACCESS_ERROR, // Path/File access error
                std::io::ErrorKind::NotFound => err_number::PATH_NOT_FOUND,      // Path not found
                std::io::ErrorKind::PermissionDenied => err_number::PERMISSION_DENIED, // Permission denied
                _ => err_number::DEVICE_IO_ERROR,                                 // Device I/O error
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
    fn mkdir_creates_directory() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create directory
        mkdir(VBVariant::from_string("newdir")).unwrap();

        // Verify
        assert!(dir.path().join("newdir").is_dir());
    }

    #[test]
    fn mkdir_rejects_existing_directory() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create directory
        std::fs::create_dir(dir.path().join("existing")).unwrap();

        let result = mkdir(VBVariant::from_string("existing"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::PATH_FILE_ACCESS_ERROR);
    }

    #[test]
    fn mkdir_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = mkdir(VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }
}
