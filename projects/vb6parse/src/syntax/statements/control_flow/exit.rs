//! Exit statement parsing for VB6.

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse an Exit statement.
    ///
    /// VB6 Exit statement syntax:
    /// - Exit Do
    /// - Exit For
    /// - Exit Function
    /// - Exit Property
    /// - Exit Sub
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/exit-statement)
    pub(crate) fn parse_exit_statement(&mut self) {
        // if we are now parsing an exit statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::ExitStatement.to_raw());
        self.consume_whitespace();

        // Consume "Exit" keyword
        self.consume_token();

        // Consume whitespace after Exit
        self.consume_whitespace();

        // Consume the exit type (Do, For, Function, Property, Sub)
        if self.at_token(Token::DoKeyword)
            || self.at_token(Token::ForKeyword)
            || self.at_token(Token::FunctionKeyword)
            || self.at_token(Token::PropertyKeyword)
            || self.at_token(Token::SubKeyword)
        {
            self.consume_token();
        }

        self.builder.finish_node(); // ExitStatement
    }
}
