//! Erase statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 Erase statements:
//! - `Erase` - Reinitialize fixed-size array elements and deallocate dynamic arrays
//!
//! # Erase Statement
//!
//! The Erase statement is used to reinitialize the elements of fixed-size arrays
//! and to release storage space used by dynamic arrays.
//!
//! ## Syntax
//! ```vb
//! Erase arraylist
//! ```
//!
//! ## Behavior
//! - For fixed-size arrays: Reinitializes the elements to their default values
//!   (0 for numeric types, "" for strings, Nothing for objects)
//! - For dynamic arrays: Deallocates the memory used by the array
//!
//! ## Examples
//! ```vb
//! Erase myArray
//! Erase array1, array2, array3
//! ```
//!
//! ## Remarks
//! - The arraylist argument is a list of one or more comma-delimited array variable names
//! - After erasing a dynamic array, you must use `ReDim` to reallocate it before using again
//! - Erasing a fixed-size array does not deallocate memory, just resets values
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/erase-statement)

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse an Erase statement: Erase array1 [, array2] ...
    ///
    /// VB6 Erase statement syntax:
    /// - Erase arraylist
    ///
    /// The Erase statement is used to reinitialize the elements of fixed-size arrays
    /// and to release storage space used by dynamic arrays.
    ///
    /// The arraylist argument is a list of one or more comma-delimited array variable names.
    ///
    /// Behavior:
    /// - For fixed-size arrays: Reinitializes the elements to their default values
    ///   (0 for numeric types, "" for strings, Nothing for objects)
    /// - For dynamic arrays: Deallocates the memory used by the array
    ///
    /// Examples:
    /// ```vb
    /// Erase myArray
    /// Erase array1, array2, array3
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/erase-statement)
    pub(crate) fn parse_erase_statement(&mut self) {
        // if we are now parsing an erase statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::EraseStatement.to_raw());

        // Consume "Erase" keyword
        self.consume_token();

        // Consume everything until newline (array names, commas, etc.)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // EraseStatement
    }
}
