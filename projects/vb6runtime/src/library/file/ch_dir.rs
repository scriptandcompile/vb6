//! VB6 ChDir statement syntax:
//! - ChDir path
//!
//! Changes the current directory or folder.
//!
//! The `path` argument can include a drive letter. If no drive is specified,
//! `ChDir` changes the current directory on the current drive.
//!
//! ## Remarks
//!
//! - `ChDir` does not change the current drive. To change the drive as well,
//!   use `ChDrive`.
//! - If the path does not exist, an error occurs (Error 76: Path not found).
//!
//! ## Examples
//!
//! ```vb
//! ChDir "C:\Temp"
//! ChDir "D:\Data"
//! ChDir "\var\log"
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/chdir-statement)

use crate::error::{VBError, VBResult};
use vb6core::error::err_number;
use crate::state::file;
use crate::value::VBVariant;

/// Change the current directory.
///
/// # Arguments
///
/// * `path` - The directory path to change to.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn chdir(path: VBVariant) -> VBResult<()> {
    let path_str = match path {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                err_number::TYPE_MISMATCH,
                "Type mismatch in ChDir",
            ));
        }
    };

    // If path includes a drive letter, change the drive too
    if path_str.len() >= 2 && path_str.as_bytes()[1] == b':' {
        let drive = path_str.as_bytes()[0] as char;
        file::set_current_drive(drive).map_err(|e| {
            VBError::with_description(
                match e.kind() {
                    std::io::ErrorKind::NotFound => err_number::DEVICE_UNAVAILABLE,
                    _ => err_number::DEVICE_IO_ERROR,
                },
                e.to_string(),
            )
        })?;
    }

    // Resolve the path against the root/current dir
    let target = file::resolve_path(std::path::Path::new(&path_str));

    file::set_current_dir(&target).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::NotFound => err_number::PATH_NOT_FOUND,
                std::io::ErrorKind::PermissionDenied => err_number::PERMISSION_DENIED,
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
    fn chdir_changes_directory() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        std::fs::create_dir(&subdir).unwrap();
        file::set_root(dir.path());

        chdir(VBVariant::from_string("subdir")).unwrap();

        let cwd = file::current_dir().unwrap();
        assert_eq!(cwd, subdir);
    }

    #[test]
    fn chdir_rejects_nonexistent_directory() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let result = chdir(VBVariant::from_string("nonexistent"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::PATH_NOT_FOUND);
    }

    #[test]
    fn chdir_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = chdir(VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }
}
