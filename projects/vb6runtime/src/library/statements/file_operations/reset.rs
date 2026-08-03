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
