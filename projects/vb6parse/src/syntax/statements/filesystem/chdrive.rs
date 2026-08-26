use crate::parsers::SyntaxKind;

use crate::Token;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parses a `ChDrive` statement.
    ///
    /// `ChDrive` statement syntax:
    /// ```vb
    /// ChDrive drive
    /// ```
    ///
    /// - **drive**: Required. String expression that specifies the drive.
    pub(crate) fn parse_ch_drive_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::ChDriveStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
            self.consume_whitespace();
        }

        self.builder.finish_node();
    }
}
