use crate::parsers::SyntaxKind;

use crate::Token;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parses a `ChDir` statement.
    ///
    /// `ChDir` statement syntax:
    /// ```vb
    /// ChDir path
    /// ```
    ///
    /// - **path**: Required. String expression that specifies the directory path.
    pub(crate) fn parse_ch_dir_statement(&mut self) {
        self.builder.start_node(SyntaxKind::ChDirStatement.to_raw());

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
