//! # `SetAttr` Statement
//!
//! Sets attribute information for a file.
//!
//! ## Syntax
//!
//! ```vb
//! SetAttr pathname, attributes
//! ```
//!
//! ## Parts
//!
//! - **pathname**: Required. String expression that specifies a file name. May include directory or folder, and drive.
//! - **attributes**: Required. Numeric expression or constant specifying the file attributes. Sum of the values of the file attribute constants.
//!
//! ## File Attribute Constants
//!
//! | Constant | Value | Description |
//! |----------|-------|-------------|
//! | vbNormal | 0 | Normal (no attributes set) |
//! | vbReadOnly | 1 | Read-only file attribute |
//! | vbHidden | 2 | Hidden file attribute |
//! | vbSystem | 4 | System file attribute |
//! | vbArchive | 32 | File has changed since last backup |
//!
//! ## Remarks
//!
//! - **Combining Attributes**: You can combine attributes by adding their values together (e.g., `vbReadOnly + vbHidden = 3`).
//! - **File Must Exist**: A run-time error occurs if the file specified by pathname doesn't exist.
//! - **Pathname Validation**: Pathname can be a fully qualified path or a relative path. Wildcard characters (* and ?) are not supported.
//! - **Cannot Set Directory Attribute**: You cannot use `SetAttr` to set the directory (vbDirectory = 16) attribute. Use `MkDir` and `RmDir` instead.
//! - **Volume Label**: You cannot use `SetAttr` to set the volume label (vbVolume = 8) attribute.
//! - **Read-Only Directories**: `SetAttr` cannot change the read-only status of a directory; it only works with files.
//! - **Error Handling**: Use error handling to trap potential errors like file not found, permission denied, or invalid attributes.
//! - **`GetAttr` Function**: Use `GetAttr` to retrieve current file attributes before modifying them with `SetAttr`.
//! - **Clearing Attributes**: To clear an attribute, set the file to vbNormal (0) or use a combination that excludes the unwanted attribute.
//!
//! ## Examples
//!
//! ### Set File to Read-Only
//!
//! ```vb
//! SetAttr "C:\MyFile.txt", vbReadOnly
//! ```
//!
//! ### Set File to Hidden
//!
//! ```vb
//! SetAttr "C:\Data\Secret.dat", vbHidden
//! ```
//!
//! ### Combine Multiple Attributes
//!
//! ```vb
//! ' Set file to read-only and hidden
//! SetAttr "C:\Config.ini", vbReadOnly + vbHidden
//! ```
//!
//! ### Clear All Attributes (Normal)
//!
//! ```vb
//! SetAttr "C:\MyFile.txt", vbNormal
//! ```
//!
//! ### Set Archive Attribute
//!
//! ```vb
//! SetAttr "C:\Backup\Data.dat", vbArchive
//! ```
//!
//! ### Using Variables
//!
//! ```vb
//! Dim fileName As String
//! Dim attrs As Integer
//!
//! fileName = "C:\Data\MyFile.txt"
//! attrs = vbReadOnly + vbArchive
//! SetAttr fileName, attrs
//! ```
//!
//! ### Toggle Read-Only Attribute
//!
//! ```vb
//! Dim currentAttrs As Integer
//! Dim filePath As String
//!
//! filePath = "C:\MyFile.txt"
//! currentAttrs = GetAttr(filePath)
//!
//! If currentAttrs And vbReadOnly Then
//!     ' Remove read-only
//!     SetAttr filePath, currentAttrs And Not vbReadOnly
//! Else
//!     ' Add read-only
//!     SetAttr filePath, currentAttrs Or vbReadOnly
//! End If
//! ```
//!
//! ### Set System File
//!
//! ```vb
//! SetAttr "C:\Windows\system.dat", vbSystem
//! ```
//!
//! ### Set Multiple Files in a Loop
//!
//! ```vb
//! Dim i As Integer
//! For i = 1 To 10
//!     SetAttr "C:\Files\File" & i & ".txt", vbReadOnly
//! Next i
//! ```
//!
//! ### With Error Handling
//!
//! ```vb
//! On Error Resume Next
//! SetAttr "C:\MyFile.txt", vbReadOnly
//! If Err.Number <> 0 Then
//!     MsgBox "Could not set file attributes: " & Err.Description
//! End If
//! On Error GoTo 0
//! ```
//!
//! ### Using App.Path
//!
//! ```vb
//! SetAttr App.Path & "\Config.ini", vbHidden
//! ```
//!
//! ### Preserve Existing Attributes While Adding New Ones
//!
//! ```vb
//! Dim filePath As String
//! Dim currentAttrs As Integer
//!
//! filePath = "C:\MyFile.txt"
//! currentAttrs = GetAttr(filePath)
//!
//! ' Add hidden attribute while preserving others
//! SetAttr filePath, currentAttrs Or vbHidden
//! ```
//!
//! ### Remove Specific Attribute
//!
//! ```vb
//! Dim filePath As String
//! Dim currentAttrs As Integer
//!
//! filePath = "C:\MyFile.txt"
//! currentAttrs = GetAttr(filePath)
//!
//! ' Remove hidden attribute while preserving others
//! SetAttr filePath, currentAttrs And Not vbHidden
//! ```
//!
//! ### Using Numeric Values
//!
//! ```vb
//! SetAttr "C:\MyFile.txt", 1  ' Same as vbReadOnly
//! SetAttr "C:\MyFile.txt", 3  ' Read-only + Hidden (1 + 2)
//! SetAttr "C:\MyFile.txt", 35 ' Read-only + Hidden + Archive (1 + 2 + 32)
//! ```
//!
//! ### Conditional Attribute Setting
//!
//! ```vb
//! If FileIsImportant Then
//!     SetAttr filePath, vbReadOnly + vbArchive
//! Else
//!     SetAttr filePath, vbNormal
//! End If
//! ```
//!
//! ## Common Errors
//!
//! - **Error 53**: File not found - occurs if the pathname doesn't exist
//! - **Error 5**: Invalid procedure call or argument - occurs if attributes value is invalid
//! - **Error 70**: Permission denied - occurs if you don't have write access to the file
//! - **Error 75**: Path/File access error - occurs if the file is open or locked
//!
//! ## Important Notes
//!
//! - **File Must Be Closed**: The file should not be open when you use `SetAttr`.
//! - **Permissions Required**: You must have appropriate permissions to change file attributes.
//! - **Network Files**: `SetAttr` works with network files if you have appropriate permissions.
//! - **UNC Paths**: `SetAttr` supports UNC (Universal Naming Convention) paths like "\\\\Server\\Share\\File.txt".
//! - **Attribute Persistence**: File attributes persist after the application closes; they're stored in the file system.
//! - **Read-Only Files**: To modify a read-only file, you must first remove the read-only attribute, make changes, then restore it.
//! - **`GetAttr` Complement**: Always use `GetAttr` to retrieve current attributes before modifying them to avoid unintentionally removing existing attributes.
//!
//! ## Best Practices
//!
//! - Use error handling when working with `SetAttr` as file operations can fail for many reasons
//! - Use `GetAttr` before `SetAttr` to preserve existing attributes you don't want to change
//! - Use symbolic constants (vbReadOnly, etc.) instead of numeric values for better code readability
//! - Check file existence using `Dir()` before calling `SetAttr`
//! - Be cautious when setting system attributes as they can affect system stability
//! - Document why specific attributes are being set, especially for hidden or system files
//! - Consider user permissions when setting attributes on shared or network files
//!
//! ## See Also
//!
//! - `GetAttr` function (retrieve file attributes)
//! - `Dir` function (check if file exists)
//! - `Kill` statement (delete files)
//! - `Name` statement (rename files)
//! - `FileCopy` statement (copy files)
//!
//! ## References
//!
//! - [SetAttr Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/setattr-statement)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::value::VBVariant;
use vb6core::error::err_number;

/// Set attributes for a file.
///
/// # Arguments
///
/// * `pathname` - The file path.
/// * `attributes` - The attributes to set.
///
/// # Returns
///
/// Returns `Ok(())` on success, or `Err(VBError)` on failure.
pub fn setattr(pathname: VBVariant, attributes: VBVariant) -> VBResult<()> {
    // Get the file path
    let path_str = match pathname {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in SetAttr",
            ));
        }
    };

    // Get attributes
    let attrs = match attributes {
        VBVariant::Long(v) => v as i16,
        VBVariant::Integer(v) => v,
        VBVariant::Byte(v) => v as i16,
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in SetAttr",
            ));
        }
    };

    // Set attributes through the backend
    file::set_attrs(std::path::Path::new(&path_str), attrs).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::NotFound => err_number::FILE_NOT_FOUND, // File not found
                std::io::ErrorKind::PermissionDenied => err_number::PERMISSION_DENIED, // Permission denied
                _ => err_number::DEVICE_IO_ERROR, // Device I/O error
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
    use vb6core::error::err_number;

    #[test]
    fn setattr_sets_readonly() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a file
        std::fs::write(dir.path().join("test.txt"), "Hello").unwrap();

        // Set readonly (VB_READ_ONLY = 1)
        setattr(VBVariant::from_string("test.txt"), VBVariant::Long(1)).unwrap();

        // Verify
        let metadata = std::fs::metadata(dir.path().join("test.txt")).unwrap();
        assert!(metadata.permissions().readonly());
    }

    #[test]
    fn setattr_rejects_nonexistent_file() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let result = setattr(
            VBVariant::from_string("nonexistent.txt"),
            VBVariant::Long(0),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::FILE_NOT_FOUND);
    }

    #[test]
    fn setattr_rejects_non_string_path() {
        let _guard = crate::state::test_support::lock_test();

        let result = setattr(VBVariant::Long(42), VBVariant::Long(0));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn setattr_rejects_non_numeric_attrs() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        std::fs::write(dir.path().join("test.txt"), "Hello").unwrap();

        let result = setattr(
            VBVariant::from_string("test.txt"),
            VBVariant::from_string("invalid"),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, err_number::TYPE_MISMATCH);
    }
}
