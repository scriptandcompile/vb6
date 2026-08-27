use super::Parser;
use crate::language::Token;
use crate::parsers::SyntaxKind;

// Extracted from: runtime_state/randomize.rs
impl Parser<'_> {
    pub(crate) fn parse_randomize_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::RandomizeStatement.to_raw());
        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }
        self.builder.finish_node();
    }
}

// Extracted from: runtime_state/time.rs
impl Parser<'_> {
    pub(crate) fn parse_time_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::TimeStatement.to_raw());

        self.consume_whitespace();

        if let Some((text, token)) = self.tokens.get(self.pos) {
            let kind = SyntaxKind::from(*token);
            if kind == SyntaxKind::TimeKeyword {
                self.builder.token(SyntaxKind::TimeKeyword.to_raw(), text);
                self.pos += 1;
            }
        }

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

// Extracted from: runtime_state/error.rs
impl Parser<'_> {
    pub(crate) fn parse_error_statement(&mut self) {
        self.builder.start_node(SyntaxKind::ErrorStatement.to_raw());
        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }
        self.builder.finish_node();
    }
}

// Extracted from: runtime_state/date.rs
impl Parser<'_> {
    pub(crate) fn parse_date_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::DateStatement.to_raw());

        self.consume_whitespace();

        if let Some((text, token)) = self.tokens.get(self.pos) {
            let kind = SyntaxKind::from(*token);
            if kind == SyntaxKind::DateKeyword {
                self.builder.token(SyntaxKind::DateKeyword.to_raw(), text);
                self.pos += 1;
            }
        }

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
