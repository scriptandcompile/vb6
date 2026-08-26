use crate::Token;
use crate::parsers::SyntaxKind;

use crate::parsers::cst::Parser;

impl Parser<'_> {
    // VB6 Print # statement syntax:
    // - Print #filenumber, [outputlist]
    //
    // Writes display-formatted data to a sequential file.
    //
    // The Print # statement syntax has these parts:
    //
    // | Part        | Description |
    // |-------------|-------------|
    // | filenumber  | Required. Any valid file number. |
    // | outputlist  | Optional. Expression or list of expressions to print. |
    //
    // Remarks:
    // - Data written with Print # is usually read from a file with Line Input # or Input.
    // - If you omit outputlist and include only a list separator after filenumber, a blank line is printed to the file.
    // - Multiple expressions can be separated with either a space or a semicolon.
    // - A space has the same effect as a semicolon.
    // - For Boolean data, either True or False is printed.
    // - The True and False keywords are not translated, regardless of locale.
    // - Date data is written to the file using the standard short date format recognized by your system.
    // - When either the date or the time component is missing or zero, only the part provided gets written to the file.
    // - Nothing is written to the file if outputlist data is Empty. However, if outputlist data is Null, Null is output to the file.
    // - For error data, the output appears as Error errorcode. The Error keyword is not translated, regardless of locale.
    // - All data written to the file using Print # is internationally aware; that is, the data is properly formatted using the appropriate decimal separator and thousands separator.
    // - When data is written to a file, several universal assumptions are followed:
    //   * Numeric data is always written using the period as the decimal separator.
    //   * For numeric data, a leading space is always reserved for the sign of the number.
    //   * A trailing space is included after each number.
    // - Unlike the Print method, the Print # statement doesn't insert commas or spaces between items as they are written to the file.
    // - When you use the Print # statement, you insert explicit delimiters in your output list when you want to add commas or spaces.
    // - The Print # statement usually writes Variant data to a file the same way it writes other data types.
    // - However, there are some exceptions:
    //   * If the data being written is a Variant of VarType vbError, an error message string is not written to the file.
    //   * Only the word Error and the error code are written.
    //   * If the data being written is a Variant of VarType vbEmpty, nothing is written to the file.
    //
    // Examples:
    // ```vb
    // ' Basic usage
    // Print #1, "Hello World"
    //
    // ' Multiple items
    // Print #1, x, y, z
    //
    // ' With semicolon separator
    // Print #1, "Name: "; userName; " Age: "; userAge
    //
    // ' Blank line
    // Print #1,
    //
    // ' Variable file number
    // Dim fileNum As Integer
    // fileNum = FreeFile
    // Print #fileNum, data
    //
    // ' Complex expressions
    // Print #1, Format$(Now, "yyyy-mm-dd"), totalAmount
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/print-statement)
    pub(crate) fn parse_print_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::PrintStatement.to_raw());

        // Consume any leading whitespace and the Print keyword.
        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if self.at_token(Token::Octothorpe) {
            // `Print #filenumber, outputlist`: file output. Parse the file
            // number and the output list structurally so the interpreter can
            // evaluate it (it treats every `Print` as console output).
            self.consume_token();
            self.consume_whitespace();
            if !self.at_token(Token::Newline) && !self.at_token(Token::Comma) && !self.is_at_end() {
                self.parse_expression();
                self.consume_whitespace();
            }
            if self.at_token(Token::Comma) {
                self.consume_token();
                self.consume_whitespace();
                self.parse_print_output_list();
            }
            self.consume_whitespace();
            self.consume_until_after(Token::Newline);
        } else if self.at_token(Token::Newline) || self.is_at_end() {
            // Bare `Print` with no output list.
            self.consume_until_after(Token::Newline);
        } else {
            // Bare `Print [outputlist]`: parse the output list as expressions
            // so the interpreter can evaluate them. (Real VB6 requires an
            // object qualifier for `Print` in a standard module; this is a
            // console-output extension shared by the interpreter and compiler.)
            self.parse_print_output_list();
            self.consume_whitespace();
            self.consume_until_after(Token::Newline);
        }

        self.builder.finish_node();
    }

    /// Parse the output list of a bare `Print` statement, mirroring
    /// `parse_unparenthesized_arguments` with semicolon separators enabled.
    fn parse_print_output_list(&mut self) {
        self.builder.start_node(SyntaxKind::ArgumentList.to_raw());

        loop {
            if self.at_token(Token::Newline) || self.is_at_end() {
                break;
            }

            self.builder.start_node(SyntaxKind::Argument.to_raw());

            // Empty arguments (an immediate separator) print nothing.
            if !self.at_token(Token::Comma) && !self.at_token(Token::Semicolon) {
                self.parse_expression();
            }

            self.builder.finish_node();

            self.consume_whitespace();

            if self.at_token(Token::Comma) || self.at_token(Token::Semicolon) {
                self.consume_token();
                self.consume_whitespace();
            } else {
                break;
            }
        }

        self.builder.finish_node();
    }
}
