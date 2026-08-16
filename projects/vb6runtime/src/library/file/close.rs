//! VB6 Close statement syntax:
//! - Close [filenumberlist]
//!
//! Closes input or output files opened using the Open statement.
//!
//! ## Parameters
//!
//! - `filenumberlist` - Optional. One or more file numbers using the syntax:
//!   [[#]filenumber] [, [#]filenumber] ...
//!
//! If `filenumberlist` is omitted, all active files opened by the Open statement are closed.
//!
//! ## Reference
//!
//! [Close Statement](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/close-statement)
