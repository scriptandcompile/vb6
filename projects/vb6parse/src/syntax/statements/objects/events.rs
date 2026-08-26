//! `RaiseEvent` statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 `RaiseEvent` statements for firing custom events:
//! - `RaiseEvent` - Fire a custom event declared in a class or form
//!
//! # `RaiseEvent` Statement
//!
//! The `RaiseEvent` statement fires an event that has been declared within a class, form,
//! or document using the `Event` statement. Events can be raised with or without arguments.
//!
//! ## Syntax
//! ```vb
//! RaiseEvent eventName [(argumentList)]
//! ```
//!
//! ## Examples
//! ```vb
//! ' Event declaration (in class declarations)
//! Event DataReceived(data As String)
//! Event StatusChanged(oldStatus As Integer, newStatus As Integer)
//! Event ProcessComplete()
//!
//! ' Raising events
//! RaiseEvent ProcessComplete
//! RaiseEvent DataReceived("Test data")
//! RaiseEvent StatusChanged(0, 1)
//! ```
//!
//! ## Remarks
//! - Events must be declared with the `Event` statement before they can be raised
//! - `RaiseEvent` can only be used in the module where the event is declared
//! - Arguments passed must match the event declaration
//! - Events are consumed by objects that declare variables `WithEvents`
//! - Events cannot be raised recursively (no re-entrancy)
//! - Events raised in forms and controls are handled by the container
//!
//! ## Related Declarations
//! ```vb
//! ' Declaring events
//! Public Event StatusChange(status As Integer)
//!
//! ' Handling events (in consumer code)
//! Dim WithEvents obj As MyClass
//!
//! Private Sub obj_StatusChange(status As Integer)
//!     ' Handle the event
//! End Sub
//! ```
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/raiseevent-statement)

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse a `RaiseEvent` statement.
    ///
    /// VB6 `RaiseEvent` statement syntax:
    ///
    /// ```text
    /// RaiseEvent eventName [(argumentList)]
    /// ```
    ///
    /// ## Examples
    /// ```vb
    /// Sub ProcessData()
    ///     RaiseEvent DataReceived("Test data")
    ///     RaiseEvent StatusChanged(0, 1)
    ///     RaiseEvent ProcessComplete
    /// End Sub
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/raiseevent-statement)
    pub(crate) fn parse_raiseevent_statement(&mut self) {
        // if we are now parsing a raiseevent statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::RaiseEventStatement.to_raw());
        self.consume_whitespace();

        // Consume "RaiseEvent" keyword
        self.consume_token();

        // Consume everything until newline (event name and arguments)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // RaiseEventStatement
    }
}
