//! Parameter list parsing for VB6 CST.
//!
//! This module handles parsing of parameter lists in VB6 procedures:
//! - Function parameter lists
//! - Sub parameter lists
//! - Property parameter lists

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse a parameter list: (param1 As Type, param2 As Type)
    ///
    /// VB6 parameter list syntax supports:
    /// - [Optional] [`ByVal` | `ByRef`] [`ParamArray`] varname[()] [As type] [= defaultvalue]
    ///
    /// This parser handles nested parentheses for array parameters and default values.
    pub(crate) fn parse_parameter_list(&mut self) {
        self.builder.start_node(SyntaxKind::ParameterList.to_raw());

        // Consume "("
        self.consume_token();

        loop {
            self.consume_whitespace();

            if self.at_token(Token::RightParenthesis) || self.is_at_end() {
                break;
            }

            // Optional
            if self.at_token(Token::OptionalKeyword) {
                self.consume_token();
                self.consume_whitespace();
            }

            // ByVal / ByRef
            if self.at_token(Token::ByValKeyword) || self.at_token(Token::ByRefKeyword) {
                self.consume_token();
                self.consume_whitespace();
            }

            // ParamArray
            if self.at_token(Token::ParamArrayKeyword) {
                self.consume_token();
                self.consume_whitespace();
            }

            // Variable name (keywords can be used as variable names in VB6, e.g. `Name`)
            if self.at_token(Token::Identifier) {
                self.consume_token();
            } else if self.at_keyword() {
                self.consume_token_as_identifier();
            } else {
                // Error recovery
                break;
            }

            self.consume_whitespace();

            // Array parens ()
            if self.at_token(Token::LeftParenthesis) {
                self.consume_token();
                self.consume_whitespace();
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

            // Default value
            if self.at_token(Token::EqualityOperator) {
                self.consume_token();
                self.consume_whitespace();
                self.parse_expression();
            }

            self.consume_whitespace();

            if self.at_token(Token::Comma) {
                self.consume_token();
            } else {
                break;
            }
        }

        // Consume ")"
        if self.at_token(Token::RightParenthesis) {
            self.consume_token();
        }

        self.builder.finish_node(); // ParameterList
    }
}
