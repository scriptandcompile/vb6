//! VB6 Input statement syntax:
//! - Input #filenumber, varlist
//!
//! Reads data from an open sequential file and assigns the data to variables.
//!
//! The Input # statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | filenumber    | Required. Any valid file number. |
//! | varlist       | Required. Comma-delimited list of variables that are assigned values read from the file. Variables can't be arrays or object variables. However, variables that describe an element of an array or user-defined type may be used. |
//!
//! ## Remarks
//!
//! - Data read with Input # is usually written to a file with Write #.
//! - Use this statement only with files opened in Input or Binary mode.
//! - The Input # statement reads data items from a sequential file and assigns them to variables.
//! - Data items in the file must appear in the same order as the variables in varlist and be separated by commas.
//! - If the data item to be read is a quoted string, Input # strips the quotation marks.
//! - Input # is typically used to read data that was written to a file using the Write # statement.
//! - For files opened for Binary access, Input # reads all the bytes it needs to complete the varlist.
//! - If end of file is reached before all variables are filled, an error occurs.
//!
//! ## Examples
//!
//! ```vb
//! Input #1, name, age
//! Input #fileNum, x, y, z
//! Input #1, firstName, lastName, address
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/input-statement)
