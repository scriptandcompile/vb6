use crate::parsers::SyntaxKind;

use crate::Token;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    // VB6 Unlock statement syntax:
    // - Unlock [#]filenumber[, recordrange]
    //
    // Removes access restrictions on all or part of an open file.
    //
    // The Unlock statement syntax has these parts:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | filenumber    | Required. Any valid file number. |
    // | recordrange   | Optional. Range of records to unlock. Can be: record, start To end, or omitted for entire file. |
    //
    // Remarks:
    // - Unlock is used to remove locks placed on a file with the Lock statement.
    // - The Unlock statement allows other processes to access the unlocked portions of the file.
    // - The arguments to Unlock must exactly match those used with the corresponding Lock statement.
    // - The first record or byte in a file is at position 1, the second at position 2, and so on.
    // - If you specify just one record number, only that record is unlocked.
    // - If you specify a range, all records in that range are unlocked.
    // - For files opened in Binary, Input, or Output mode, Unlock always unlocks the entire file,
    //   regardless of the recordrange argument.
    // - For files opened in Random mode, Unlock unlocks the specified record or range of records.
    // - Each Lock statement must have a corresponding Unlock statement with the same file number
    //   and record range.
    //
    // Examples:
    // ```vb
    // Unlock #1
    // Unlock #1, 5
    // Unlock #1, 10 To 20
    // Unlock fileNum, recordNum
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/unlock-statement)
    pub(crate) fn parse_unlock_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::UnlockStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse filenumber expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse optional comma and recordrange
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();

            if !self.is_at_end() && !self.at_token(Token::Newline) {
                // Parse start expression
                self.parse_expression();
                self.consume_whitespace();

                // Optional To ... end range
                if self.at_token(Token::ToKeyword) {
                    self.consume_token();
                    self.consume_whitespace();

                    // Parse end expression
                    self.parse_expression();
                    self.consume_whitespace();
                }
            }
        }

        self.builder.finish_node();
    }
}
