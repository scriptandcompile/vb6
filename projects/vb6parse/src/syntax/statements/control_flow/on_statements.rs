//! On-prefixed statement parsing for VB6 (`On Error`, `On GoTo`, `On GoSub`).

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

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
