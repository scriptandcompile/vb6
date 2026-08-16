//! VB6 Lock statement syntax:
//! - Lock [#]filenumber[, recordrange]
//!
//! Controls access to all or part of an open file.
//!
//! The Lock statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | recordrange   | Optional. Range of records to lock. Can be: record, start To end, or omitted for entire file. |
//!
//! ## Remarks
//!
//! - Lock and Unlock are used in environments where multiple processes might need access to the same file.
//! - Lock and Unlock statements are always used in pairs.
//! - The Lock statement locks all or part of a file opened using the Open statement.
//! - The first record or byte in a file is at position 1, the second at position 2, and so on.
//! - If you specify just one record number, only that record is locked.
//! - If you specify a range, all records in that range are locked.
//! - For files opened in Binary, Input, or Output mode, Lock always locks the entire file,
//!   regardless of the recordrange argument.
//! - For files opened in Random mode, Lock locks the specified record or range of records.
//! - Locked portions of a file can't be accessed by other processes until unlocked with Unlock.
//! - Use Unlock to remove the lock from a portion of a file.
//!
//! ## Examples
//!
//! ```vb
//! Lock #1
//! Lock #1, 5
//! Lock #1, 10 To 20
//! Lock fileNum, recordNum
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/lock-statement)
