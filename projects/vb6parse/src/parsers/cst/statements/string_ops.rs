use super::Parser;
use crate::language::Token;
use crate::parsers::SyntaxKind;

// Extracted from: string_manipulation/lset.rs
impl Parser<'_> {
    /// Parses an `LSet` statement: `LSet target = value`
    pub(crate) fn parse_lset_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::LSetStatement.to_raw());

        self.consume_whitespace();

        if let Some((text, _)) = self.tokens.get(self.pos) {
            self.builder.token(SyntaxKind::LSetKeyword.to_raw(), text);
            self.pos += 1;
        }

        self.consume_whitespace();

        self.parse_expression();

        self.consume_whitespace();

        if let Some((text, token)) = self.tokens.get(self.pos) {
            let kind = SyntaxKind::from(*token);
            if kind == SyntaxKind::EqualityOperator {
                self.builder.token(kind.to_raw(), text);
                self.pos += 1;
            }
        }

        self.consume_whitespace();

        self.parse_expression();

        self.builder.finish_node();
    }
}

// Extracted from: string_manipulation/midb.rs
impl Parser<'_> {
    /// Parses a `MidB` statement: `MidB(stringvar, start[, length]) = string`
    pub(crate) fn parse_midb_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::MidBStatement.to_raw());

        self.consume_whitespace();

        if let Some((text, _)) = self.tokens.get(self.pos) {
            self.builder.token(SyntaxKind::MidBKeyword.to_raw(), text);
            self.pos += 1;
        }

        self.consume_whitespace();

        if self.at_token(Token::LeftParenthesis) {
            self.consume_token();
        }

        self.builder.start_node(SyntaxKind::ArgumentList.to_raw());

        if self.is_at_end() || self.at_token(Token::RightParenthesis) {
            // No arguments, just closing paren
        } else {
            // Parse first argument
            self.builder.start_node(SyntaxKind::Argument.to_raw());
            self.parse_expression();
            self.builder.finish_node();

            // Parse additional arguments separated by commas
            while !self.is_at_end() && !self.at_token(Token::RightParenthesis) {
                self.consume_whitespace();
                if !self.is_at_end() && self.at_token(Token::Comma) {
                    self.consume_token();
                }
                self.consume_whitespace();
                if !self.is_at_end() && !self.at_token(Token::RightParenthesis) {
                    self.builder.start_node(SyntaxKind::Argument.to_raw());
                    self.parse_expression();
                    self.builder.finish_node();
                }
            }
        }

        self.builder.finish_node();

        if self.at_token(Token::RightParenthesis) {
            self.consume_token();
        }

        self.consume_whitespace();

        if let Some((text, token)) = self.tokens.get(self.pos) {
            let kind = SyntaxKind::from(*token);
            if kind == SyntaxKind::EqualityOperator {
                self.builder.token(kind.to_raw(), text);
                self.pos += 1;
            }
        }

        self.parse_expression();

        self.builder.finish_node();
    }
}

// Extracted from: string_manipulation/rset.rs
impl Parser<'_> {
    pub(crate) fn parse_rset_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::RSetStatement.to_raw());

        self.consume_whitespace();

        if let Some((text, _)) = self.tokens.get(self.pos) {
            self.builder.token(SyntaxKind::RSetKeyword.to_raw(), text);
            self.pos += 1;
        }

        self.consume_whitespace();

        self.parse_expression();

        self.consume_whitespace();

        if let Some((text, token)) = self.tokens.get(self.pos) {
            let kind = SyntaxKind::from(*token);
            if kind == SyntaxKind::EqualityOperator {
                self.builder.token(kind.to_raw(), text);
                self.pos += 1;
            }
        }

        self.consume_whitespace();

        self.parse_expression();

        self.builder.finish_node();
    }
}

// Extracted from: string_manipulation/mid.rs
impl Parser<'_> {
    /// Parses a Mid statement: `Mid(stringvar, start[, length]) = string`
    pub(crate) fn parse_mid_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::MidStatement.to_raw());

        self.consume_whitespace();

        if let Some((text, _)) = self.tokens.get(self.pos) {
            self.builder.token(SyntaxKind::MidKeyword.to_raw(), text);
            self.pos += 1;
        }

        self.consume_whitespace();

        if self.at_token(Token::LeftParenthesis) {
            self.consume_token();
        }

        self.builder.start_node(SyntaxKind::ArgumentList.to_raw());

        if self.is_at_end() || self.at_token(Token::RightParenthesis) {
            // No arguments, just closing paren
        } else {
            // Parse first argument
            self.builder.start_node(SyntaxKind::Argument.to_raw());
            self.parse_expression();
            self.builder.finish_node();

            // Parse additional arguments separated by commas
            while !self.is_at_end() && !self.at_token(Token::RightParenthesis) {
                self.consume_whitespace();
                if !self.is_at_end() && self.at_token(Token::Comma) {
                    self.consume_token();
                }
                self.consume_whitespace();
                if !self.is_at_end() && !self.at_token(Token::RightParenthesis) {
                    self.builder.start_node(SyntaxKind::Argument.to_raw());
                    self.parse_expression();
                    self.builder.finish_node();
                }
            }
        }

        self.builder.finish_node();

        if self.at_token(Token::RightParenthesis) {
            self.consume_token();
        }

        self.consume_whitespace();

        if let Some((text, token)) = self.tokens.get(self.pos) {
            let kind = SyntaxKind::from(*token);
            if kind == SyntaxKind::EqualityOperator {
                self.builder.token(kind.to_raw(), text);
                self.pos += 1;
            }
        }

        self.parse_expression();

        self.builder.finish_node();
    }
}
