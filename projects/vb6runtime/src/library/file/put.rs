//! Parse a Put statement.
//!
//! VB6 Put statement syntax:
//! - Put [#]filenumber, [recnumber], varname
//!
//! Writes data from a variable to a disk file.
//!
//! The Put statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | recnumber     | Optional. Variant (Long). Record number (Random mode files) or byte number (Binary mode files) at which writing begins. |
//! | varname       | Required. Valid variable name containing data to be written to disk. |
//!
//! ## Remarks
//! - Put is used with files opened in Binary or Random mode.
//! - For files opened in Random mode, the record length specified in the Open statement determines the number of bytes written.
//! - For files opened in Binary mode, Put writes any number of bytes.
//! - The first record or byte in a file is at position 1, the second at position 2, and so on.
//! - If you omit recnumber, the next record or byte following the last Put or Get statement (or pointed to by the last Seek function) is written.
//! - You must include delimiting commas, for example: Put #1, , myVariable
//! - For files opened in Random mode, the following rules apply:
//!   * If the length of the data being written is less than the length specified in the Len clause, subsequent records on disk are aligned on record-length boundaries.
//!   * The space between the end of one record and the beginning of the next is padded with the existing file contents.
//!   * If the variable being written is a variable-length string, Put writes a 2-byte descriptor containing the string length and then writes the string data.
//! - For files opened in Binary mode, all the Random rules apply, except:
//!   * The Len clause in the Open statement has no effect.
//!   * Put writes the data contiguously, with no padding between records.
//! - Put statements usually mirror Get statements. That is, data written with Put is typically read with Get.
//!
//! ## Examples
//!
//! ```vb
//! Put #1, , myRecord
//! Put #1, recordNumber, customerData
//! Put fileNum, , buffer
//! Put #1, filePosition, userData
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/put-statement)
