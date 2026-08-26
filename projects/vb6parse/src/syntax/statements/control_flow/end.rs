//! End statement parsing for VB6.

use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse a standalone `End` statement.
    ///
    /// The `End` statement terminates program execution immediately.
    /// It closes all files opened using the `Open` statement and clears all variables.
    ///
    /// Syntax:
    ///   `End`
    ///
    /// Note: This is distinct from compound `End` keywords like `End If`, `End Sub`,
    /// `End Function`, etc., which are block terminators handled by their respective parsers.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/end-statement)
    pub(crate) fn parse_end_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::EndStatement.to_raw());
        self.consume_whitespace();

        // Consume "End" keyword
        self.consume_token();

        self.builder.finish_node(); // EndStatement
    }
}
