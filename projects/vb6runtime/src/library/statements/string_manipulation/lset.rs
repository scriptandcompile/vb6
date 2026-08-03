//! VB6 LSet statement syntax:
//! - LSet stringvar = string
//! - LSet varname1 = varname2 (for user-defined types)
//!
//! Left-aligns a string within a string variable, or copies a variable of one user-defined type
//! to another variable of a different user-defined type.
//!
//! The LSet statement syntax has these parts:
//!
//! | Part          | Description |
//! |---------------|-------------|
//! | stringvar     | Required. Name of string variable. |
//! | string        | Required. String expression to be left-aligned within stringvar. |
//! | varname1      | Required. Variable name of the user-defined type being copied to. |
//! | varname2      | Required. Variable name of the user-defined type being copied from. |
//!
//! ## Remarks
//!
//! - LSet left-aligns strings within string variables.
//! - If string is shorter than stringvar, LSet left-aligns the string in stringvar and pads
//!   remaining characters with spaces.
//! - If string is longer than stringvar, LSet places only the leftmost characters that fit into
//!   stringvar.
//! - Warning: Using LSet to copy variables of different user-defined types is not recommended.
//!   Copying variables of one user-defined type into variables of a different user-defined type
//!   can produce unpredictable results.
//! - When copying between variables of user-defined types, the memory assigned to one variable is
//!   copied byte-for-byte to the memory assigned to the other variable.
//! - LSet is commonly used with fixed-length strings.
//! - LSet can be used with variant variables that contain strings.
//!
//! ## Examples
//!
//! ```vb
//! LSet MyString = "Left"
//! LSet FixedString = userName
//! LSet myRecord = sourceRecord
//! ```
//!
//! ## Reference
//!
//! [LSet Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/lset-statement)
