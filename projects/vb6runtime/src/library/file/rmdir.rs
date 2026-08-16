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
