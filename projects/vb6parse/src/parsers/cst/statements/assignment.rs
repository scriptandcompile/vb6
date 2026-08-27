//! Assignment statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 assignment statements:
//! - Let statement: `Let x = 5` (optional keyword)
//! - Simple variable assignment: `x = 5`
//! - Property assignment: `obj.property = value`
//! - Array assignment: `arr(index) = value`

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse an assignment statement.
    ///
    /// VB6 assignment statement syntax:
    /// - variableName = expression
    /// - object.property = expression
    /// - array(index) = expression
    ///
    pub(crate) fn parse_assignment_statement(&mut self) {
        // Assignments can appear in both header and body, so we do not modify parsing_header here.

        self.builder
            .start_node(SyntaxKind::AssignmentStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Parse left-hand side - use parse_lvalue which stops before =
        self.parse_lvalue();

        // Skip whitespace
        self.consume_whitespace();

        // Consume the equals sign
        if self.at_token(Token::EqualityOperator) {
            self.consume_token();
        }

        // Skip whitespace after =
        self.consume_whitespace();

        // Parse right-hand side (value expression)
        self.parse_expression();

        // Consume the newline if present (but not colon - that's handled by caller)
        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.builder.finish_node(); // AssignmentStatement
    }

    /// Parse a Let statement.
    ///
    /// VB6 Let statement syntax:
    /// - Let variableName = expression
    ///
    /// The Let keyword is optional in VB6 and is provided for backward compatibility.
    /// Most modern VB6 code omits the Let keyword.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/let-statement)
    pub(crate) fn parse_let_statement(&mut self) {
        // Let statements can appear in both header and body, so we do not modify parsing_header here.

        self.builder.start_node(SyntaxKind::LetStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume "Let" keyword
        self.consume_token();

        // Parse left-hand side
        self.parse_lvalue();

        // Skip whitespace
        self.consume_whitespace();

        // Consume "="
        if self.at_token(Token::EqualityOperator) {
            self.consume_token();
        }

        // Skip whitespace
        self.consume_whitespace();

        // Parse right-hand side
        self.parse_expression();

        // Consume the newline if present (but not colon - that's handled by caller)
        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.builder.finish_node(); // LetStatement
    }

    /// Check if the current position is at the start of an assignment statement.
    /// This looks ahead to see if there's an `=` operator (not part of a comparison).
    /// Note: Let statements are handled separately and should be checked first.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn is_at_assignment(&self) -> bool {
        // Assignment statements must start with an assignable token.
        if !(self.at_token(Token::Identifier)
            || self.at_keyword()
            || self.at_token(Token::PeriodOperator)
            || self.at_token(Token::Octothorpe))
        {
            return false;
        }

        // Let statements are handled separately
        if self.at_token(Token::LetKeyword) {
            return false;
        }

        // Look ahead through the tokens to find an = operator at depth 0 before a newline
        // We need to skip: identifiers, periods, parentheses, array indices, etc.
        // Note: In VB6, keywords can be used as property/member names (e.g., obj.Property = value)
        // and also as variable names (e.g., text = "hello")
        let mut last_was_period = false;
        let mut at_start = true;
        let mut paren_depth: i32 = 0;
        let mut seen_other_operator = false;
        let mut seen_top_level_comma = false;

        for (_text, token) in self.tokens.iter().skip(self.pos) {
            match token {
                Token::Newline | Token::EndOfLineComment | Token::RemComment => {
                    // Reached end of line without finding assignment
                    return false;
                }
                Token::ColonOperator if paren_depth == 0 => {
                    // Colon separates statements on the same line in VB6.
                    // Stop lookahead at the current statement boundary.
                    return false;
                }
                Token::LeftParenthesis => {
                    paren_depth += 1;
                    last_was_period = false;
                    at_start = false;
                }
                Token::RightParenthesis => {
                    paren_depth = paren_depth.saturating_sub(1);
                    last_was_period = false;
                    at_start = false;
                }
                Token::EqualityOperator if paren_depth == 0 => {
                    // A top-level comma before '=' indicates we're already in
                    // argument-list style syntax (procedure call), not an assignment.
                    if seen_top_level_comma {
                        return false;
                    }
                    // Found an = operator at depth 0
                    // If we've seen other operators (like >=, And, Or), this is part of an expression, not assignment
                    return !seen_other_operator;
                }
                Token::PeriodOperator => {
                    last_was_period = true;
                    at_start = false;
                }
                // Skip tokens that could appear in the left-hand side of an assignment
                Token::Whitespace => {}
                Token::Identifier
                | Token::DollarSign
                | Token::Percent
                | Token::AtSign
                | Token::IntegerLiteral
                | Token::LongLiteral
                | Token::SingleLiteral
                | Token::DoubleLiteral
                | Token::DecimalLiteral
                | Token::StringLiteral
                | Token::DateTimeLiteral
                | Token::TrueKeyword
                | Token::FalseKeyword
                | Token::NullKeyword
                | Token::EmptyKeyword
                | Token::ExclamationMark
                | Token::Octothorpe
                | Token::Comma => {
                    if *token == Token::Comma && paren_depth == 0 {
                        seen_top_level_comma = true;
                    }
                    last_was_period = false;
                    at_start = false;
                }
                // If we're inside parentheses, = operators are part of expressions, not assignments
                Token::EqualityOperator if paren_depth > 0 => {
                    // This is part of an expression, not an assignment
                    last_was_period = false;
                    at_start = false;
                }
                // Track operators that indicate we're in an expression context
                Token::AndKeyword
                | Token::OrKeyword
                | Token::XorKeyword
                | Token::EqvKeyword
                | Token::ImpKeyword
                | Token::ModKeyword
                | Token::NotKeyword
                | Token::LessThanOperator
                | Token::GreaterThanOperator
                | Token::LessThanOrEqualOperator
                | Token::GreaterThanOrEqualOperator
                | Token::InequalityOperator
                | Token::AdditionOperator
                | Token::SubtractionOperator
                | Token::MultiplicationOperator
                | Token::DivisionOperator
                | Token::BackwardSlashOperator
                | Token::ExponentiationOperator
                | Token::Ampersand => {
                    if paren_depth == 0 {
                        seen_other_operator = true;
                    }
                    last_was_period = false;
                    at_start = false;
                }
                // After a period, keywords can be property names, so skip them
                _ if last_was_period => {
                    last_was_period = false;
                    at_start = false;
                }
                // At the start of a statement, keywords can be used as variable names
                _ if at_start && token.is_keyword() => {
                    at_start = false;
                }
                // Inside indexing/call parentheses, allow expression tokens while looking for
                // top-level assignment operator.
                _ if paren_depth > 0 => {
                    last_was_period = false;
                    at_start = false;
                }
                // If we hit other unexpected tokens, it's not an assignment
                _ => {
                    return false;
                }
            }
        }
        false
    }
}
