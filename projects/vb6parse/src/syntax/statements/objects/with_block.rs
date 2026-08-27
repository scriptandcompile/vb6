//! With statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 With blocks for simplified object property access:
//! - `With...End With` - Reference an object multiple times without repeating its name
//!
//! # With Statement
//!
//! The With statement allows you to perform a series of statements on a specified object
//! without requalifying the name of the object. You can use `.PropertyName` or `.MethodName`
//! within the With block to access members of the object.
//!
//! ## Syntax
//! ```vb
//! With object
//!     .Property1 = value1
//!     .Property2 = value2
//!     .Method arg1, arg2
//! End With
//! ```
//!
//! ## Examples
//! ```vb
//! With myObject
//!     .Property = "value"
//!     .NestedObject.Property = 123
//!     .Method "arg"
//! End With
//!
//! ' Nested With blocks
//! With obj1
//!     With .NestedObj
//!         .Value = 10
//!     End With
//! End With
//!
//! ' With New keyword
//! With New MyClass
//!     .Initialize
//! End With
//! ```
//!
//! ## Remarks
//! - Statements in the With block access members using the dot prefix (`.`)
//! - With blocks can be nested
//! - The object reference is evaluated once at the beginning
//! - Can be used with the New keyword to create and initialize objects
//! - Improves code readability and can improve performance
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/with-statement)

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse a With statement.
    ///
    /// VB6 With statement syntax:
    ///
    /// ```vb
    /// With object
    ///     .Property1 = value1
    ///     .Property2 = value2
    /// End With
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/with-statement)
    pub(crate) fn parse_with_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::WithStatement.to_raw());
        self.consume_whitespace();
        self.consume_token(); // With
        self.consume_whitespace();
        self.parse_expression();
        self.consume_until_after(Token::Newline);

        self.parse_statement_list(|parser| {
            parser.at_token(Token::EndKeyword)
                && parser.peek_next_keyword() == Some(Token::WithKeyword)
        });

        if self.at_token(Token::EndKeyword) {
            self.consume_token();
            self.consume_whitespace();
            self.consume_token(); // With
            self.consume_until_after(Token::Newline);
        }

        self.builder.finish_node(); // WithStatement
    }
}
