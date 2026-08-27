//! Select Case statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 Select Case statements:
//! - Select Case statements with multiple Case clauses
//! - Case Else clauses
//! - Case expressions (values, ranges, Is comparisons)

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse a Select Case statement.
    ///
    /// Syntax:
    ///   Select Case testexpression
    ///     Case expression1
    ///       statements1
    ///     Case expression2
    ///       statements2
    ///     Case Else
    ///       statementsElse
    ///   End Select
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/select-case-statement)
    pub(crate) fn parse_select_case_statement(&mut self) {
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::SelectCaseStatement.to_raw());
        self.consume_whitespace();
        self.consume_token(); // Select
        self.consume_whitespace();

        if self.at_token(Token::CaseKeyword) {
            self.consume_token();
        }

        self.consume_whitespace();
        self.parse_expression();
        self.consume_until_after(Token::Newline);

        // Parse Case clauses
        while !self.is_at_end() {
            if self.at_token(Token::EndKeyword)
                && self.peek_next_keyword() == Some(Token::SelectKeyword)
            {
                break;
            }

            if self.at_token(Token::CaseKeyword) {
                // Check for Case Else
                let is_case_else = {
                    let next = self.peek_next_keyword();
                    next == Some(Token::ElseKeyword)
                };

                if is_case_else {
                    self.builder.start_node(SyntaxKind::CaseElseClause.to_raw());
                    self.consume_token(); // Case
                    self.consume_whitespace();
                    self.consume_token(); // Else
                    self.consume_until_after(Token::Newline);

                    self.parse_statement_list(|parser| {
                        parser.at_token(Token::CaseKeyword)
                            || (parser.at_token(Token::EndKeyword)
                                && parser.peek_next_keyword() == Some(Token::SelectKeyword))
                    });

                    self.builder.finish_node(); // CaseElseClause
                } else {
                    self.builder.start_node(SyntaxKind::CaseClause.to_raw());
                    self.consume_token(); // Case
                    self.consume_until_after(Token::Newline);

                    self.parse_statement_list(|parser| {
                        parser.at_token(Token::CaseKeyword)
                            || (parser.at_token(Token::EndKeyword)
                                && parser.peek_next_keyword() == Some(Token::SelectKeyword))
                    });

                    self.builder.finish_node(); // CaseClause
                }
            } else {
                // Consume unknown tokens
                self.consume_token();
            }
        }

        // Consume End Select
        if self.at_token(Token::EndKeyword) {
            self.consume_token();
            self.consume_whitespace();
            self.consume_token(); // Select
            self.consume_until_after(Token::Newline);
        }

        self.builder.finish_node(); // SelectCaseStatement
    }
}
