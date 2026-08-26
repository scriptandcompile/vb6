//! Variable and constant declaration parsing for VB6 CST.
//!
//! This module handles parsing of VB6 variable and constant declarations:
//! - `Dim` - Declare variables at procedure or module level
//! - `Private` - Declare private module-level variables
//! - `Public` - Declare public module-level variables
//! - `Const` - Declare named constants
//! - `Static` - Declare static variables that retain values between procedure calls
//! - `WithEvents` - Declare object variables capable of responding to events
//!
//! # Variables with `WithEvents`
//!
//! The `WithEvents` keyword is used with `Private`, `Public`, or `Dim` to declare object variables
//! that can respond to events raised by the object. This is commonly used in class modules
//! and form modules.
//!
//! ## Syntax
//! ```vb
//! Private WithEvents variablename As objecttype
//! Public WithEvents variablename As objecttype
//! Dim WithEvents variablename As objecttype
//! ```
//!
//! ## Examples
//! ```vb
//! Dim x As Integer
//! Private m_value As Long
//! Private WithEvents m_button As CommandButton
//! Public g_config As String
//! Public WithEvents g_app As Application
//! Const MAX_SIZE = 100
//! Static counter As Long
//! ```
//!
//! ## Remarks
//! - `WithEvents` can only be used with object variables
//! - `WithEvents` variables must be declared as a specific class type, not `As Object`
//! - Events are accessible through the object's event procedures (`objectname_eventname`)
//! - Public `WithEvents` variables are accessible from other modules
//! - Commonly used with form controls, `ActiveX` objects, and custom classes that raise events
//!
//! [WithEvents Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa243352(v=vs.60))

use crate::language::Token;
use crate::parsers::SyntaxKind;
use crate::parsers::cst::Parser;

impl Parser<'_> {
    /// Parse a Dim statement: Dim/Private/Public/Const/Static x As Type
    ///
    /// VB6 variable declaration statement syntax:
    /// - Dim varname [As type]
    /// - Private varname [As type]
    /// - Private `WithEvents` varname As objecttype
    /// - Public varname [As type]
    /// - Public `WithEvents` varname As objecttype
    /// - Const constname = expression
    /// - Static varname [As type]
    ///
    /// Used to declare variables and allocate storage space.
    ///
    /// The `WithEvents` keyword can be used with `Private`, `Public`, or `Dim` to declare
    /// object variables that can respond to events raised by the object.
    ///
    /// Examples:
    /// ```vb
    /// Dim x As Integer
    /// Private m_value As Long
    /// Private WithEvents m_button As CommandButton
    /// Public g_config As String
    /// Public WithEvents g_app As Application
    /// Const MAX_SIZE = 100
    /// Static counter As Long
    /// ```
    ///
    /// [Dim Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/dim-statement)
    /// [WithEvents Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa243352(v=vs.60))
    pub(crate) fn parse_dim(&mut self) {
        // if we are now parsing a dim statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::DimStatement.to_raw());

        // Consume any leading whitespace, then the primary declaration keyword
        // (Dim, Private, Public, Const, Static, etc.). The whitespace is consumed
        // first so the keyword is never mistaken for a variable name below.
        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Consume a secondary declaration keyword, e.g. `Const` in
        // `Private Const x = 1`, so it is not parsed as a variable name.
        if self.at_token(Token::ConstKeyword) || self.at_token(Token::StaticKeyword) {
            self.consume_token();
            self.consume_whitespace();
        }

        loop {
            self.consume_whitespace();

            if self.at_token(Token::Newline)
                || self.at_token(Token::ColonOperator)
                || self.is_at_end()
            {
                break;
            }

            // WithEvents
            if self.at_token(Token::WithEventsKeyword) {
                self.consume_token();
                self.consume_whitespace();
            }

            // Variable name (keywords can be used as variable names in VB6, e.g. `Name`)
            if self.at_token(Token::Identifier) {
                self.consume_token();
            } else if self.at_keyword() {
                self.consume_token_as_identifier();
            } else {
                // Error recovery: consume until comma or newline
                while !self.is_at_end()
                    && !self.at_token(Token::Comma)
                    && !self.at_token(Token::Newline)
                {
                    self.consume_token();
                }
            }

            self.consume_whitespace();

            // Array bounds: (1 To 10)
            if self.at_token(Token::LeftParenthesis) {
                self.consume_token();
                // Parse bounds list
                loop {
                    self.consume_whitespace();
                    if self.at_token(Token::RightParenthesis) {
                        break;
                    }
                    self.parse_expression(); // lower or upper
                    self.consume_whitespace();
                    if self.at_token(Token::ToKeyword) {
                        self.consume_token();
                        self.consume_whitespace();
                        self.parse_expression(); // upper
                    }

                    if self.at_token(Token::Comma) {
                        self.consume_token();
                    } else {
                        break;
                    }
                }
                if self.at_token(Token::RightParenthesis) {
                    self.consume_token();
                }
            }

            self.consume_whitespace();

            // As Type
            if self.at_token(Token::AsKeyword) {
                self.consume_token();
                self.consume_whitespace();
                if self.at_token(Token::NewKeyword) {
                    self.consume_token();
                    self.consume_whitespace();
                }
                // Type name (identifier or keyword)
                self.consume_token();
                // Handle complex types like ADODB.Connection
                while self.at_token(Token::PeriodOperator) {
                    self.consume_token();
                    self.consume_token();
                }
            }

            self.consume_whitespace();

            // Initializer (for Const or optional initialization)
            if self.at_token(Token::EqualityOperator) {
                self.consume_token();
                self.consume_whitespace();
                self.parse_expression();
            }

            self.consume_whitespace();

            if self.at_token(Token::Comma) {
                self.consume_token();
            } else {
                break;
            }
        }

        // Consume everything until newline (preserving all tokens)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // DimStatement
    }
}
