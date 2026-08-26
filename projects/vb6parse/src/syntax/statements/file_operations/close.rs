use crate::parsers::SyntaxKind;

use crate::Token;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    // VB6 Close statement syntax:
    // - Close [filenumberlist]
    //
    // Closes input or output files opened using the Open statement.
    //
    // filenumberlist: Optional. One or more file numbers using the syntax:
    // [[#]filenumber] [, [#]filenumber] ...
    //
    // If filenumberlist is omitted, all active files opened by the Open statement are closed.
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/close-statement)
    pub(crate) fn parse_close_statement(&mut self) {
        self.builder.start_node(SyntaxKind::CloseStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.at_token(Token::Newline) && !self.is_at_end() {
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}
