//! Property statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 Property statements:
//! - Property Get
//! - Property Let
//! - Property Set

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Check if the current position is at a compiler conditional Property statement.
    ///
    /// This includes statements like:
    /// - `#If ... Then`
    /// - `#ElseIf ... Then`
    /// - `#Else`
    /// - `#End If`
    pub(crate) fn is_at_compiler_conditional_property_statement(&self) -> bool {
        if !self.at_compiler_directive_keyword(Token::IfKeyword) {
            return false;
        }

        let mut index = self.pos;

        while let Some((_, token)) = self.tokens.get(index) {
            index += 1;
            if *token == Token::Newline {
                break;
            }
        }

        while let Some((_, token)) = self.tokens.get(index) {
            match token {
                Token::Whitespace
                | Token::Newline
                | Token::PublicKeyword
                | Token::PrivateKeyword
                | Token::FriendKeyword
                | Token::StaticKeyword => {
                    index += 1;
                }
                Token::PropertyKeyword => return true,
                _ => return false,
            }
        }

        false
    }

    /// Parse a Property statement (Property Get, Property Let, or Property Set).
    ///
    /// VB6 Property statement syntax:
    /// - [Public | Private | Friend] [Static] Property Get name [(arglist)] [As type]
    /// - [Public | Private | Friend] [Static] Property Let name ([arglist,] value)
    /// - [Public | Private | Friend] [Static] Property Set name ([arglist,] value)
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/property-get-statement)
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/property-let-statement)
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/property-set-statement)
    pub(crate) fn parse_property_statement(&mut self) {
        // if we are now parsing a property statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::PropertyStatement.to_raw());

        self.consume_property_signature_line();

        // Parse body until "End Property"
        self.parse_statement_list(|parser| {
            (parser.at_token(Token::EndKeyword)
                && parser.peek_next_keyword() == Some(Token::PropertyKeyword))
                || parser.at_component_declaration_start()
        });

        self.consume_property_terminator();

        self.builder.finish_node(); // PropertyStatement
    }

    pub(crate) fn parse_compiler_conditional_property_statement(&mut self) {
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::PropertyStatement.to_raw());

        self.consume_until_after(Token::Newline);
        self.consume_property_signature_line();

        while self.at_compiler_directive_keyword(Token::ElseIfKeyword)
            || self.at_compiler_directive_keyword(Token::ElseKeyword)
        {
            self.consume_until_after(Token::Newline);
            self.consume_property_signature_line();
        }

        if self.at_compiler_end_if_directive() {
            self.consume_until_after(Token::Newline);
        }

        self.parse_statement_list(|parser| {
            (parser.at_token(Token::EndKeyword)
                && parser.peek_next_keyword() == Some(Token::PropertyKeyword))
                || parser.at_component_declaration_start()
        });

        self.consume_property_terminator();

        if self.at_compiler_end_if_directive() {
            self.consume_compiler_directive_prefix();
            self.consume_until_after(Token::Newline);
        }

        self.builder.finish_node(); // PropertyStatement
    }

    fn consume_property_signature_line(&mut self) {
        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume optional Public/Private/Friend keyword
        if self.at_token(Token::PublicKeyword)
            || self.at_token(Token::PrivateKeyword)
            || self.at_token(Token::FriendKeyword)
        {
            self.consume_token();

            // Consume any whitespace after visibility modifier
            self.consume_whitespace();
        }

        // Consume optional Static keyword
        if self.at_token(Token::StaticKeyword) {
            self.consume_token();

            // Consume any whitespace after Static
            self.consume_whitespace();
        }

        // Consume "Property" keyword
        self.consume_token();

        // Consume any whitespace after "Property"
        self.consume_whitespace();

        // Consume Get/Let/Set keyword
        if self.at_token(Token::GetKeyword)
            || self.at_token(Token::LetKeyword)
            || self.at_token(Token::SetKeyword)
        {
            self.consume_token();
        }

        // Consume any whitespace after Get/Let/Set
        self.consume_whitespace();

        // Consume property name (keywords can be used as property names in VB6)
        if self.at_token(Token::Identifier) {
            self.consume_token();
        } else if self.at_keyword() {
            self.consume_token_as_identifier();
        }

        // Consume any whitespace before parameter list
        self.consume_whitespace();

        // Parse parameter list if present
        if self.at_token(Token::LeftParenthesis) {
            self.parse_parameter_list();
        }

        // Consume everything until newline (includes "As Type" if present)
        self.consume_until_after(Token::Newline);
    }

    fn consume_property_terminator(&mut self) {
        // Consume "End Property" and trailing tokens, or report mismatch as recovery.
        self.consume_expected_end_keyword_terminator(Token::PropertyKeyword, "Property");
    }
}
