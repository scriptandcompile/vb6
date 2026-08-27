//! Attribute statement parsing for VB6 CST.
//!
//! This module handles parsing of Attribute statements like:
//! - `Attribute VB_Name = "modTest"`
//! - `Attribute VB_GlobalNameSpace = False`

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse an Attribute statement: `Attribute VB_Name = "value"`
    pub(crate) fn parse_attribute_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::AttributeStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume "Attribute" keyword
        self.consume_token();

        // Consume everything until newline (preserving all tokens)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // AttributeStatement
    }
}
