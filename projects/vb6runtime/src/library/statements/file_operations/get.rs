//! VB6 Get statement syntax:
//! - Get [#]filenumber, [recnumber], varname
//!
//! Reads data from an open disk file into a variable.
//!
//! ## Syntax
//!
//! ```vb
//! Get [#]filenumber, [recnumber], varname
//! ```
//!
//! ## Parameters
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | recnumber     | Optional. Variant (Long). Record number (Random mode files) or byte number (Binary mode files) at which reading begins. |
//! | varname       | Required. Valid variable name into which data is read. |
//!
//! ## Remarks
//! - Get is used with files opened in Binary or Random mode.
//! - For files opened in Random mode, the record length specified in the Open statement determines the number of bytes read.
//! - For files opened in Binary mode, Get reads any number of bytes.
//! - The first record or byte in a file is at position 1, the second at position 2, and so on.
//! - If you omit recnumber, the next record or byte following the last Get or Put statement (or pointed to by the last Seek function) is read.
//! - You must include delimiting commas, for example: Get #1, , myVariable
//! - For files opened in Random mode, the following rules apply:
//!   * If the length of the data being read is less than the length specified in the Len clause, subsequent records on disk are aligned on record-length boundaries.
//!   * The space between the end of one record and the beginning of the next is padded with existing file contents.
//!   * If the variable being read is a variable-length string, Get reads a 2-byte descriptor containing the string length and then reads the string data.
//! - For files opened in Binary mode, all the Random rules apply, except:
//!   * The Len clause in the Open statement has no effect.
//!   * Get reads the data contiguously, with no padding between records.
//!
//! ## Examples
//!
//! ```vb
//! Get #1, , myRecord
//! Get #1, recordNumber, customerData
//! Get fileNum, , buffer
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/get-statement)
