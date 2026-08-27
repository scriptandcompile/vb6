//! Do/Loop statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 loop statements:
//! - Do While...Loop
//! - Do Until...Loop
//! - Do...Loop While
//! - Do...Loop Until
//! - Do...Loop (infinite loop)
//! - While...Wend

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse a Do...Loop statement.
    ///
    /// VB6 supports several forms of Do loops:
    /// - Do While condition...Loop
    /// - Do Until condition...Loop
    /// - Do...Loop While condition
    /// - Do...Loop Until condition
    /// - Do...Loop (infinite loop, requires Exit Do)
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/doloop-statement)
    pub(crate) fn parse_do_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::DoStatement.to_raw());
        self.consume_whitespace();
        self.consume_token(); // Do
        self.consume_whitespace();

        // Check for While/Until after Do
        if self.at_token(Token::WhileKeyword) || self.at_token(Token::UntilKeyword) {
            self.consume_token();
            self.consume_whitespace();
            self.parse_expression();
        }

        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.parse_statement_list(|parser| parser.at_token(Token::LoopKeyword));

        if self.at_token(Token::LoopKeyword) {
            self.consume_token();
            self.consume_whitespace();

            if self.at_token(Token::WhileKeyword) || self.at_token(Token::UntilKeyword) {
                self.consume_token();
                self.consume_whitespace();
                self.parse_expression();
            }

            if self.at_token(Token::Newline) {
                self.consume_token();
            }
        }

        self.builder.finish_node(); // DoStatement
    }

    /// Parse a While...Wend statement.
    ///
    /// VB6 While...Wend loop syntax:
    /// - While condition
    ///   ...statements...
    ///   Wend
    ///
    /// While...Wend statement syntax:
    ///
    /// | Part      | Description |
    /// |-----------|-------------|
    /// | condition | Required. Numeric or String expression that evaluates to True or False. If condition is Null, condition is treated as False. |
    /// | statements| Optional. One or more statements executed while condition is True. |
    ///
    /// Remarks:
    /// - If condition is True, all statements are executed until the Wend statement is encountered.
    /// - Control then returns to the While statement and condition is again checked.
    /// - If condition is still True, the process is repeated. If it's not True, execution resumes with the statement following the Wend statement.
    /// - While...Wend loops can be nested to any level. Each Wend matches the most recent While.
    /// - Note: The Do...Loop statement provides a more structured and flexible way to perform looping.
    /// - Tip: While...Wend is provided for compatibility with earlier versions of Visual Basic. Consider using Do...Loop instead for new code.
    ///
    /// Examples:
    /// ```vb
    /// Dim counter As Integer
    /// counter = 0
    /// While counter < 20
    ///     counter = counter + 1
    ///     Debug.Print counter
    /// Wend
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/whilewend-statement)
    /// Parse a While...Wend statement.
    ///
    /// While...Wend is a legacy VB6 loop construct that executes a block of
    /// statements while a condition is true. It has been superseded by Do While...Loop
    /// but is still supported for backward compatibility.
    ///
    /// Syntax:
    /// ```vb6
    /// While condition
    ///     statements
    /// Wend
    /// ```
    ///
    /// Example:
    /// ```vb6
    /// While x < 10
    ///     x = x + 1
    /// Wend
    /// ```
    ///
    /// The condition is evaluated before each iteration. If the condition is
    /// initially false, the loop body will not execute at all.
    pub(crate) fn parse_while_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::WhileStatement.to_raw());
        self.consume_whitespace();
        self.consume_token(); // While
        self.consume_whitespace();
        self.parse_expression();

        // Consume newline after While line
        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.parse_statement_list(|parser| parser.at_token(Token::WendKeyword));

        if self.at_token(Token::WendKeyword) {
            self.consume_token();

            if self.at_token(Token::Newline) {
                self.consume_token();
            }
        }

        self.builder.finish_node(); // WhileStatement
    }
}
