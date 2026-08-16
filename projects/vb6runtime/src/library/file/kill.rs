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
