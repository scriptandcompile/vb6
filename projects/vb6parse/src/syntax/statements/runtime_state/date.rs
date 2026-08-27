//! VB6 Date statement syntax:
//! - Date = dateexpression
//!
//! Sets the current system date.
//!
//! dateexpression: Required. Any expression that can represent a date.
//!
//! Note: The Date statement is used to set the date. To retrieve the current date,
//! use the Date function.
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/date-statement)

use crate::parsers::SyntaxKind;

use crate::parsers::cst::Parser;

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
