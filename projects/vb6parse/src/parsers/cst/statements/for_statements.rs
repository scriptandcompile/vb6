//! For/Next and For Each/Next statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 For loop statements:
//! - For...Next loops with counter variables
//! - For Each...In...Next loops for collections
//! - Step clauses
//! - Nested loops

use super::Parser;
use crate::language::Token;
use crate::parsers::SyntaxKind;

use std::num::NonZeroUsize;

impl Parser<'_> {
    pub(crate) fn has_inline_next_before_newline(&self) -> bool {
        let mut index = self.pos;

        while let Some((_, token)) = self.tokens.get(index) {
            match token {
                Token::Newline => return false,
                Token::NextKeyword => return true,
                _ => index += 1,
            }
        }

        false
    }

    pub(crate) fn parse_single_line_for_body_until_next(&mut self) {
        if self.at_token(Token::ColonOperator) {
            self.consume_token();
        }

        self.parse_statement_list(|parser| {
            parser.at_token(Token::NextKeyword) || parser.at_token(Token::Newline)
        });

        if self.at_token(Token::NextKeyword) {
            self.consume_token();
            self.consume_until_after(Token::Newline);
        } else if self.at_token(Token::Newline) {
            self.consume_token();
        }
    }

    /// Parse a For...Next statement.
    ///
    /// VB6 For...Next loop syntax:
    /// - For counter = start To end [Step step]...Next [counter]
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/fornext-statement)
    pub(crate) fn parse_for_statement(&mut self) {
        self.parsing_header = false;

        // Check if For Each
        let is_for_each = {
            let next_kw = if self.at_token(Token::Whitespace) {
                let keyword_count = NonZeroUsize::new(2)
                    .expect("Should be impossible to fail to create NonZeroUsize for 2");
                self.peek_next_count_keywords(keyword_count).nth(1)
            } else {
                self.peek_next_keyword()
            };
            next_kw == Some(Token::EachKeyword)
        };

        if is_for_each {
            self.builder
                .start_node(SyntaxKind::ForEachStatement.to_raw());
            self.consume_whitespace();
            self.consume_token(); // For
            self.consume_whitespace();
            self.consume_token(); // Each
            self.consume_until_after(Token::Newline);

            self.parse_statement_list(|parser| parser.at_token(Token::NextKeyword));

            if self.at_token(Token::NextKeyword) {
                self.consume_token();
                self.consume_until_after(Token::Newline);
            }

            self.builder.finish_node(); // ForEachStatement
        } else {
            self.builder.start_node(SyntaxKind::ForStatement.to_raw());
            self.consume_whitespace();
            self.consume_token(); // For

            // Parse counter variable (lvalue)
            self.parse_lvalue();
            self.consume_whitespace();

            // Consume "="
            if self.at_token(Token::EqualityOperator) {
                self.consume_token();
            }
            self.consume_whitespace();

            // Parse start value
            self.parse_expression();
            self.consume_whitespace();

            // Consume "To" keyword if present
            if self.at_token(Token::ToKeyword) {
                self.consume_token();
                self.consume_whitespace();

                // Parse end value
                self.parse_expression();
                self.consume_whitespace();

                // Consume "Step" keyword if present
                if self.at_token(Token::StepKeyword) {
                    self.consume_token();
                    self.consume_whitespace();

                    // Parse step value
                    self.parse_expression();
                }
            }

            if self.has_inline_next_before_newline() {
                self.parse_single_line_for_body_until_next();
            } else {
                // Consume newline after For line
                self.consume_until_after(Token::Newline);

                self.parse_statement_list(|parser| parser.at_token(Token::NextKeyword));

                if self.at_token(Token::NextKeyword) {
                    self.consume_token();
                    self.consume_until_after(Token::Newline);
                }
            }

            self.builder.finish_node(); // ForStatement
        }
    }

    /// Parse a For Each...Next statement.
    ///
    /// VB6 For Each...Next loop syntax:
    /// - For Each element In collection...Next [element]
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/for-eachnext-statement)
    pub(crate) fn parse_for_each_statement(&mut self) {
        // if we are now parsing a for each statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::ForEachStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume "For" keyword
        self.consume_token();

        // Consume whitespace
        self.consume_whitespace();

        // Consume "Each" keyword
        if self.at_token(Token::EachKeyword) {
            self.consume_token();
        }

        // Consume everything until "In" or newline
        // This includes: element variable name and whitespace
        while !self.is_at_end()
            && !self.at_token(Token::InKeyword)
            && !self.at_token(Token::Newline)
        {
            self.consume_token();
        }

        // Consume "In" keyword if present
        if self.at_token(Token::InKeyword) {
            self.consume_token();

            // Consume everything until newline (the collection)
            self.consume_until(Token::Newline);
        }

        // Consume newline after For Each line
        self.consume_until_after(Token::Newline);

        // Parse the loop body until "Next"
        self.parse_statement_list(|parser| parser.at_token(Token::NextKeyword));

        // Consume "Next" keyword
        if self.at_token(Token::NextKeyword) {
            self.consume_token();

            // Consume everything until newline (optional element variable)
            self.consume_until_after(Token::Newline);
        }

        self.builder.finish_node(); // ForEachStatement
    }
}
