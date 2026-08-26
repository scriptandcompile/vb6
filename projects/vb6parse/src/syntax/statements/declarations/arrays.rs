//! `ReDim` statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 `ReDim` (reallocate dimension) statements:
//! - `ReDim` - Reallocate storage space for dynamic array variables
//! - `ReDim Preserve` - Reallocate while preserving existing data
//!
//! # `ReDim` Statement
//!
//! The `ReDim` statement is used at procedure level to reallocate storage space
//! for dynamic array variables. The optional `Preserve` keyword preserves the data
//! in the existing array when you change the size of the last dimension.
//!
//! ## Syntax
//! ```vb
//! ReDim [Preserve] varname(subscripts) [As type] [, varname(subscripts) [As type]] ...
//! ```
//!
//! ## Examples
//! ```vb
//! ReDim myArray(10)
//! ReDim Preserve argv(argc - 1)
//! ReDim ICI(1 To num) As ImageCodecInfo
//! ReDim Buffer(1 To Size) As Byte
//! ReDim arr1(10), arr2(20), arr3(30)
//! ```
//!
//! ## Remarks
//! - Can be used only at procedure level
//! - Can change the number of dimensions, size of each dimension, and data type
//! - Preserve keyword keeps existing data but only allows resizing the last dimension
//! - Can reallocate multiple arrays in a single statement
//!
//! [Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa266231(v=vs.60))

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse a `ReDim` statement.
    ///
    /// VB6 `ReDim` statement syntax:
    /// - `ReDim` [Preserve] varname(subscripts) [As type] [, varname(subscripts) [As type]] ...
    ///
    /// Used at procedure level to reallocate storage space for dynamic array variables.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa266231(v=vs.60))
    pub(crate) fn parse_redim_statement(&mut self) {
        // if we are now parsing a ReDim statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::ReDimStatement.to_raw());

        // This parser may be entered while positioned on leading whitespace
        // (e.g. an indented statement), so consume it before the keyword to
        // ensure "ReDim" is never parsed as a variable name below.
        self.consume_whitespace();

        // Consume "ReDim" keyword
        self.consume_token();
        self.consume_whitespace();

        // Optional Preserve
        if self.at_token(Token::PreserveKeyword) {
            self.consume_token();
            self.consume_whitespace();
        }

        loop {
            self.consume_whitespace();

            if self.at_token(Token::Newline)
                || self.at_token(Token::ColonOperator)
                || self.is_at_end()
            {
                break;
            }

            // Variable name (keywords can be used as variable names in VB6, e.g. `Name`)
            if self.at_token(Token::Identifier) {
                self.consume_token();
            } else if self.at_keyword() {
                self.consume_token_as_identifier();
            } else {
                // Error recovery
                while !self.is_at_end()
                    && !self.at_token(Token::Comma)
                    && !self.at_token(Token::Newline)
                {
                    self.consume_token();
                }
            }

            self.consume_whitespace();

            // Array bounds: (1 To 10)
            if self.at_token(Token::LeftParenthesis) {
                self.consume_token();
                // Parse bounds list
                loop {
                    self.consume_whitespace();
                    if self.at_token(Token::RightParenthesis) {
                        break;
                    }
                    self.parse_expression(); // lower or upper
                    self.consume_whitespace();
                    if self.at_token(Token::ToKeyword) {
                        self.consume_token();
                        self.consume_whitespace();
                        self.parse_expression(); // upper
                    }

                    if self.at_token(Token::Comma) {
                        self.consume_token();
                    } else {
                        break;
                    }
                }
                if self.at_token(Token::RightParenthesis) {
                    self.consume_token();
                }
            }

            self.consume_whitespace();

            // As Type
            if self.at_token(Token::AsKeyword) {
                self.consume_token();
                self.consume_whitespace();
                // Type name
                self.consume_token();
                while self.at_token(Token::PeriodOperator) {
                    self.consume_token();
                    self.consume_token();
                }
            }

            self.consume_whitespace();

            if self.at_token(Token::Comma) {
                self.consume_token();
            } else {
                break;
            }
        }

        // Consume everything until newline (Preserve, variable declarations, etc.)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // ReDimStatement
    }
}
