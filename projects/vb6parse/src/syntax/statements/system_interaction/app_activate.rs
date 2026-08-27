use crate::parsers::SyntaxKind;

use crate::Token;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    // VB6 AppActivate statement syntax:
    // - AppActivate title[, wait]
    //
    // Activates an application window.
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/appactivate-statement)
    pub(crate) fn parse_app_activate_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::AppActivateStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.at_token(Token::Newline) && !self.is_at_end() {
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}
