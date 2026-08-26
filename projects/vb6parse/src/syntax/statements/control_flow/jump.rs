//! Jump statement parsing for VB6 (`GoTo`, `GoSub`, `Return`, `Labels`).

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse a `GoSub` statement.
    ///
    /// VB6 `GoSub` statement syntax:
    /// - `GoSub` label
    ///
    /// Branches to and returns from a subroutine within a procedure.
    ///
    /// The `GoSub`...`Return` statement syntax has these parts:
    ///
    /// | Part   | Description |
    /// |--------|-------------|
    /// | label  | Required. A line label or line number. |
    ///
    /// Remarks:
    /// - You can use `GoSub` and `Return` anywhere in a procedure, but `GoSub` and the corresponding `Return` statement must be in the same procedure.
    /// - A subroutine can contain more than one `Return` statement, but the first one encountered causes the flow of execution to branch back to the statement immediately following the most recently executed `GoSub` statement.
    /// - You can't enter or exit `Sub` procedures with `GoSub`...`Return`.
    /// - Using `GoSub` and `Return` is considered obsolete. Modern VB6 code should use `Sub` or `Function` procedures instead.
    ///
    /// Examples:
    /// ```vb
    /// Sub Test()
    ///     GoSub ErrorHandler
    ///     Exit Sub
    /// ErrorHandler:
    ///     MsgBox "Error"
    ///     Return
    /// End Sub
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/gosubreturn-statement)
    pub(crate) fn parse_gosub_statement(&mut self) {
        // if we are now parsing a gosub statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::GoSubStatement.to_raw());
        self.consume_whitespace();

        // Consume "GoSub" keyword
        self.consume_token();

        // Consume everything until newline (the label name)
        self.consume_until(Token::Newline);

        self.builder.finish_node(); // GoSubStatement
    }

    /// Parse a Return statement.
    ///
    /// VB6 Return statement syntax:
    /// - Return
    ///
    /// Returns from a subroutine within a procedure.
    ///
    /// Remarks:
    /// - `Return` must be used with `GoSub` to return to the statement following the `GoSub` call.
    /// - You can use `GoSub` and `Return` anywhere in a procedure, but `GoSub` and the corresponding `Return` statement must be in the same procedure.
    /// - A subroutine can contain more than one `Return` statement, but the first one encountered causes the flow of execution to branch back to the statement immediately following the most recently executed `GoSub` statement.
    /// - Using `GoSub` and `Return` is considered obsolete. Modern VB6 code should use `Sub` or `Function` procedures instead.
    ///
    /// Examples:
    /// ```vb
    /// Sub Test()
    ///     GoSub Cleanup
    ///     Exit Sub
    /// Cleanup:
    ///     Set obj = Nothing
    ///     Return
    /// End Sub
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/gosubreturn-statement)
    pub(crate) fn parse_return_statement(&mut self) {
        // if we are now parsing a return statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::ReturnStatement.to_raw());
        self.consume_whitespace();

        // Consume "Return" keyword
        self.consume_token();

        self.builder.finish_node(); // ReturnStatement
    }

    /// Parse a `GoTo` statement.
    ///
    /// Syntax:
    ///   `GoTo` label
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/goto-statement)
    pub(crate) fn parse_goto_statement(&mut self) {
        // if we are now parsing a `GoTo` statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::GotoStatement.to_raw());
        self.consume_whitespace();

        // Consume "`GoTo`" keyword
        self.consume_token();

        // Consume everything until newline (the label name)
        self.consume_until(Token::Newline);

        self.builder.finish_node(); // GotoStatement
    }

    /// Parse a label statement.
    ///
    /// VB6 label syntax:
    /// - `LabelName:`
    ///
    /// `Labels` are used as targets for `GoTo` and `GoSub` statements.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/goto-statement)
    pub(crate) fn parse_label_statement(&mut self) {
        // if we are now parsing a label statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::LabelStatement.to_raw());
        self.consume_whitespace();

        // Consume the label identifier
        self.consume_token();

        // Consume optional whitespace
        self.consume_whitespace();

        // Consume the colon
        if self.at_token(Token::ColonOperator) {
            self.consume_token();
        }

        // Consume the newline if present
        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.builder.finish_node(); // LabelStatement
    }

    /// Check if the current position is at a label.
    /// A label is an identifier followed by a colon, or a numeric line label.
    pub(crate) fn is_at_label(&self) -> bool {
        let next_token_is_colon = matches!(self.peek_next_token(), Some(Token::ColonOperator));

        // If we are not parsing the header, then some keywords are valid identifiers (like "Begin")
        // TODO: Consider adding a list of keywords that can be used as labels.
        // TODO: Also consider modifying tokenizer to recognize when inside header to more easily identify Identifiers vs header only keywords.
        if next_token_is_colon
            && !self.parsing_header
            && matches!(self.current_token(), Some(Token::BeginKeyword))
        {
            return true;
        }

        (next_token_is_colon && (self.is_identifier() || self.is_number()))
            || self.is_at_numeric_line_label()
    }

    #[allow(clippy::needless_continue)]
    fn is_at_numeric_line_label(&self) -> bool {
        if !self.is_number() {
            return false;
        }

        if !matches!(
            self.peek_next_token(),
            Some(Token::Whitespace | Token::Newline)
        ) {
            return false;
        }

        let mut index = self.pos;
        while index > 0 {
            index -= 1;
            match self.tokens[index].1 {
                // The continue arm actually is needed, but couldn't convince clippy
                // so I just pushed it to ignore. Oh well.
                Token::Whitespace => continue,
                Token::Newline => return true,
                _ => return false,
            }
        }

        true
    }
}
