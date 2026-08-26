use crate::parsers::SyntaxKind;

use crate::Token;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    // VB6 FileCopy statement syntax:
    // - FileCopy source, destination
    //
    // Copies a file.
    //
    // The FileCopy statement syntax has these named arguments:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | source        | Required. String expression that specifies a file name. May include directory or folder, and drive. |
    // | destination   | Required. String expression that specifies a file name. May include directory or folder, and drive. |
    //
    // Remarks:
    // - If you try to use the FileCopy statement on a currently open file, an error occurs.
    // - FileCopy can copy files between directories/folders and between drives.
    // - Both source and destination can include path information (drive and directory/folder).
    // - If destination specifies a directory/folder that doesn't exist, FileCopy creates it.
    //
    // Examples:
    // ```vb
    // FileCopy "C:\SOURCE.TXT", "C:\DEST.TXT"
    // FileCopy oldFile, newFile
    // FileCopy App.Path & "\data.dat", "C:\Backup\data.dat"
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/filecopy-statement)
    pub(crate) fn parse_file_copy_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::FileCopyStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse source expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse comma
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();
        }

        // Parse destination expression
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
            self.consume_whitespace();
        }

        self.builder.finish_node();
    }
}
