use crate::parsers::SyntaxKind;

use crate::Token;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    // VB6 Kill statement syntax:
    // - Kill pathname
    //
    // Deletes files from a disk.
    //
    // The Kill statement syntax has this part:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | pathname      | Required. String expression that specifies one or more file names to be deleted. May include directory or folder, and drive. |
    //
    // Remarks:
    // - Kill supports the use of multiple-character (*) and single-character (?) wildcards to specify multiple files.
    // - An error occurs if you try to use Kill to delete an open file.
    // - To remove a directory or folder, use the RmDir statement.
    //
    // Examples:
    // ```vb
    // Kill "C:\DATA.TXT"
    // Kill "C:\*.TXT"           ' Delete all .txt files
    // Kill "C:\TEST?.TXT"       ' Delete TEST1.TXT, TESTA.TXT, etc.
    // Kill App.Path & "\temp.dat"
    // Kill myFileName
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/kill-statement)
    pub(crate) fn parse_kill_statement(&mut self) {
        self.builder.start_node(SyntaxKind::KillStatement.to_raw());

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
