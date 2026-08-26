use crate::parsers::SyntaxKind;

use crate::parsers::cst::Parser;

impl Parser<'_> {
    // VB6 Reset statement syntax:
    // - Reset
    //
    // Closes all disk files opened using the Open statement.
    //
    // The Reset statement closes all active files opened by the Open statement
    // and writes the contents of all file buffers to disk.
    //
    // Use Reset to ensure all file data is written to disk before ending your program.
    // This is particularly important in programs that may terminate abnormally.
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/reset-statement)
    pub(crate) fn parse_reset_statement(&mut self) {
        self.builder.start_node(SyntaxKind::ResetStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        self.builder.finish_node();
    }
}

