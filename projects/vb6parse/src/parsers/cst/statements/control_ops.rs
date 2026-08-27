use super::Parser;
use crate::language::Token;
use crate::parsers::SyntaxKind;

// Extracted from: control_flow/resume.rs
impl Parser<'_> {
    /// Parse a Resume statement.
    ///
    /// VB6 Resume statement syntax:
    /// - `Resume`
    /// - `Resume Next`
    /// - `Resume Label`
    ///
    /// Resumes execution after an error-handling routine is finished.
    ///
    /// # Syntax
    ///
    /// The `Resume` statement has these forms:
    ///
    /// | Form | Description |
    /// |------|-------------|
    /// | `Resume` | If the error occurred in the same procedure as the error handler, execution resumes with the statement that caused the error. If the error occurred in a called procedure, execution resumes at the statement that last called out of the procedure containing the error-handling routine. |
    /// | `Resume Next` | If the error occurred in the same procedure as the error handler, execution resumes with the statement immediately following the statement that caused the error. If the error occurred in a called procedure, execution resumes with the statement immediately following the statement that last called out of the procedure containing the error-handling routine (or On Error Resume Next statement). |
    /// | `Resume Label` | Execution resumes at the line specified by the label argument. The label argument can be a line label or line number. |
    ///
    /// # Remarks
    ///
    /// - The `Resume` statement can be used only in an error-handling routine.
    /// - Using `Resume` without specifying a label causes execution to resume at the statement that caused the error.
    /// - `Resume Next` is useful when you want to continue execution despite an error.
    /// - `Resume Label` is useful when you want to continue execution at a specific location after handling an error.
    /// - If you use a `Resume` statement anywhere except in an error-handling routine, an error occurs.
    /// - `Resume` cannot be used in any procedure that contains an On Error `Resume Next` statement.
    ///
    /// # Examples
    ///
    /// ```vb
    /// Sub Test()
    ///     On Error GoTo ErrorHandler
    ///     ' Code that might cause error
    ///     x = 1 / 0
    ///     Exit Sub
    /// ErrorHandler:
    ///     MsgBox "Error occurred"
    ///     Resume Next
    /// End Sub
    /// ```
    ///
    /// ```vb
    /// Sub Test2()
    ///     On Error GoTo ErrorHandler
    ///     ' Code that might cause error
    ///     Exit Sub
    /// ErrorHandler:
    ///     If Err.Number = 11 Then
    ///         Resume
    ///     Else
    ///         Resume CleanUp
    ///     End If
    /// CleanUp:
    ///     ' Cleanup code
    /// End Sub
    /// ```
    ///
    /// # References
    ///
    /// [Microsoft VBA Language Reference - Resume Statement](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/resume-statement)
    pub(crate) fn parse_resume_statement(&mut self) {
        // if we are now parsing a resume statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::ResumeStatement.to_raw());
        self.consume_whitespace();

        // Consume "Resume" keyword
        self.consume_token();

        // Consume everything until newline (Next keyword or label)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // ResumeStatement
    }
}

// Extracted from: control_flow/exit.rs
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

// Extracted from: control_flow/jump.rs
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

// Extracted from: control_flow/on_statements.rs
impl Parser<'_> {
    /// Parse an `On Error` statement.
    ///
    /// VB6 `On Error` statement syntax:
    /// - `On Error GoTo label`
    /// - `On Error GoTo 0`
    /// - `On Error Resume Next`
    ///
    /// Enables an error-handling routine and specifies the location of the routine within a procedure.
    ///
    /// The `On Error` statement syntax has these forms:
    ///
    /// | Form | Description |
    /// |------|-------------|
    /// | `On Error GoTo line` | Enables the error-handling routine that starts at line. The line argument is any line label or line number. If a run-time error occurs, control branches to line, making the error handler active. |
    /// | `On Error Resume Next` | Specifies that when a run-time error occurs, control goes to the statement immediately following the statement where the error occurred, and execution continues from that point. |
    /// | `On Error GoTo 0` | Disables any enabled error handler in the current procedure. |
    ///
    /// Remarks:
    /// - If you don't use an `On Error` statement, any run-time error that occurs is fatal; that is, an error message is displayed and execution stops.
    /// - An "enabled" error handler is one that is turned on by an `On Error` statement. An "active" error handler is an enabled handler that is in the process of handling an error.
    /// - If an error occurs while an error handler is active (between the occurrence of the error and a `Resume`, `Exit Sub`, `Exit Function`, or `Exit Property` statement), the current procedure's error handler can't handle the error.
    /// - Control returns to the calling procedure. If the calling procedure has an enabled error handler, it is activated to handle the error.
    /// - If the calling procedure's error handler is also active, control passes back through previous calling procedures until an enabled, but inactive, error handler is found.
    /// - If no inactive, enabled error handler is found, the error is fatal at the point at which it actually occurred.
    /// - Each time the error handler passes control back to a calling procedure, that procedure becomes the current procedure. Once an error is handled in any procedure, execution resumes in the current procedure at the point designated by the `Resume` statement.
    ///
    /// Examples:
    /// ```vb
    /// Sub Test()
    ///     On Error GoTo ErrorHandler
    ///     ' Code that might cause an error
    ///     Exit Sub
    /// ErrorHandler:
    ///     MsgBox "An error occurred: " & Err.Description
    /// End Sub
    ///
    /// Sub Test2()
    ///     On Error Resume Next
    ///     ' Code continues even if errors occur
    ///     MkDir "C:\Temp"  ' Won't stop if directory exists
    /// End Sub
    ///
    /// Sub Test3()
    ///     On Error GoTo 0  ' Disable error handling
    ///     ' Normal error behavior
    /// End Sub
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/on-error-statement)
    pub(crate) fn parse_on_error_statement(&mut self) {
        // if we are now parsing an on error statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::OnErrorStatement.to_raw());
        self.consume_whitespace();

        // Consume "On" keyword
        self.consume_token();

        // Consume "Error" keyword
        if self.at_token(Token::ErrorKeyword) {
            self.consume_token();
        }

        // Consume everything until newline (GoTo label, Resume Next, GoTo 0, etc.)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // OnErrorStatement
    }

    /// Parse an `On GoTo` statement.
    ///
    /// VB6 `On GoTo` statement syntax:
    /// - `On expression GoTo label1[, label2, ...]`
    ///
    /// Branches to one of several specified labels, depending on the value of an expression.
    ///
    /// The `On...GoTo` statement syntax has these parts:
    ///
    /// | Part | Description |
    /// |------|-------------|
    /// | expression | Required. Any numeric expression that evaluates to a whole number between 0 and 255, inclusive. If expression is any number other than a whole number, it is rounded before it is evaluated. |
    /// | labellist | Required. List of line labels or line numbers separated by commas. |
    ///
    /// Remarks:
    /// - The value of expression determines which line is branched to in the list of labels. If the value of expression is less than 1 or greater than the number of items in the list, one of the following results occurs:
    ///   - If expression equals 0, execution continues with the statement following `On...GoTo`.
    ///   - If expression is greater than the number of labels in the list, execution continues with the statement following `On...GoTo`.
    ///   - If expression is negative or greater than 255, an error occurs.
    /// - The `On...GoTo` statement is useful for branching to one of several different labels based on a value.
    /// - Using `On...GoTo` is considered obsolete. Modern VB6 code should use `Select Case` instead.
    ///
    /// Examples:
    /// ```vb
    /// Sub Test()
    ///     Dim choice As Integer
    ///     choice = 2
    ///     On choice GoTo Label1, Label2, Label3
    ///     Exit Sub
    /// Label1:
    ///     MsgBox "Choice 1"
    ///     Exit Sub
    /// Label2:
    ///     MsgBox "Choice 2"
    ///     Exit Sub
    /// Label3:
    ///     MsgBox "Choice 3"
    /// End Sub
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/ongoto-and-ongosub-statements)
    pub(crate) fn parse_on_goto_statement(&mut self) {
        // if we are now parsing an on goto statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::OnGoToStatement.to_raw());
        self.consume_whitespace();

        // Consume "On" keyword
        self.consume_token();

        // Consume everything until newline (expression GoTo labels)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // OnGoToStatement
    }

    /// Parse an `On GoSub` statement.
    ///
    /// VB6 `On GoSub` statement syntax:
    /// - `On expression GoSub label1[, label2, ...]`
    ///
    /// Branches to one of several specified subroutines, depending on the value of an expression.
    ///
    /// The `On...GoSub` statement syntax has these parts:
    ///
    /// | Part | Description |
    /// |------|-------------|
    /// | expression | Required. Any numeric expression that evaluates to a whole number between 0 and 255, inclusive. If expression is any number other than a whole number, it is rounded before it is evaluated. |
    /// | labellist | Required. List of line labels or line numbers separated by commas. |
    ///
    /// Remarks:
    /// - The value of expression determines which subroutine is called in the list of labels. If the value of expression is less than 1 or greater than the number of items in the list, one of the following results occurs:
    ///   - If expression equals 0, execution continues with the statement following `On...GoSub`.
    ///   - If expression is greater than the number of labels in the list, execution continues with the statement following `On...GoSub`.
    ///   - If expression is negative or greater than 255, an error occurs.
    /// - The `On...GoSub` statement is useful for branching to one of several different subroutines based on a value.
    /// - Each subroutine must end with a Return statement to return to the statement following the `On...GoSub`.
    /// - Using `On...GoSub` is considered obsolete. Modern VB6 code should use `Select Case` with Sub procedure calls instead.
    ///
    /// Examples:
    /// ```vb
    /// Sub Test()
    ///     Dim menuChoice As Integer
    ///     menuChoice = 1
    ///     On menuChoice GoSub Menu1, Menu2, Menu3
    ///     Exit Sub
    /// Menu1:
    ///     MsgBox "Menu 1 selected"
    ///     Return
    /// Menu2:
    ///     MsgBox "Menu 2 selected"
    ///     Return
    /// Menu3:
    ///     MsgBox "Menu 3 selected"
    ///     Return
    /// End Sub
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/ongoto-and-ongosub-statements)
    pub(crate) fn parse_on_gosub_statement(&mut self) {
        // if we are now parsing an on gosub statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::OnGoSubStatement.to_raw());
        self.consume_whitespace();

        // Consume "On" keyword
        self.consume_token();

        // Consume everything until newline (expression GoSub labels)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // OnGoSubStatement
    }
}

// Extracted from: control_flow/end.rs
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
