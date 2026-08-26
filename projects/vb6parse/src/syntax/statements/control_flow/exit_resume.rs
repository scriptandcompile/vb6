//! Exit and Resume statement parsing for VB6.

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

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
