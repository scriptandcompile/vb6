//! Set statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 Set statements for object reference assignment:
//! - `Set` - Assign an object reference to a variable
//!
//! # Set Statement
//!
//! The Set statement assigns an object reference to a variable or property.
//! In VB6, object variables must be assigned using Set (unlike value types which use `=` alone).
//!
//! ## Syntax
//! ```vb
//! Set objectVar = [New] objectExpression
//! Set objectVar = Nothing
//! ```
//!
//! ## Examples
//! ```vb
//! Set obj = myObject
//! Set obj = New MyClass
//! Set obj = Nothing
//! Set myObj.Property = otherObj
//! Set result = GetObject("WinMgmts:")
//! Set item = collection.Item(1)
//! ```
//!
//! ## Remarks
//! - Required for assigning object references (not for value types)
//! - Can use `New` keyword to create a new instance
//! - Use `Nothing` to release an object reference
//! - Assignment operator `=` follows the Set keyword
//! - Works with properties and collection items
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/set-statement)

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse a Set statement.
    ///
    /// VB6 Set statement syntax:
    /// - Set objectVar = [New] objectExpression
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/set-statement)
    pub(crate) fn parse_set_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::SetStatement.to_raw());
        self.consume_whitespace();

        // Consume "Set" keyword
        self.consume_token();
        self.consume_whitespace();

        // Parse left-hand side (identifier or member access like Form1.Picture)
        self.parse_lvalue();

        // Skip whitespace
        self.consume_whitespace();

        // Consume "="
        if self.at_token(Token::EqualityOperator) {
            self.consume_token();
        }
        self.consume_whitespace();

        // Parse right-hand side as a proper expression tree
        // parse_expression handles:
        // - "Set obj = Nothing" → IdentifierExpression for Nothing
        // - "Set obj = New MyClass" → NewExpression via parse_prefix_expression_frame
        // - "Set obj = GetObject(...)" → CallExpression
        // - "Set obj = collection.Item(1)" → MemberAccessExpression + CallExpression
        // - "Set obj = someVar" → IdentifierExpression
        // - "Set obj = a + b" → BinaryExpression
        self.parse_expression();

        // Consume newline
        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.builder.finish_node();
    }
}
