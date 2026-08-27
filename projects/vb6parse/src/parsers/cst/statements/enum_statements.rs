//! Enum statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 Enum (enumeration) statements.
//!
//! Enum statement syntax:
//!
//! \[ Public | Private \] Enum name
//! membername \[= constantexpression\]
//! membername \[= constantexpression\]
//! ...
//! End Enum
//!
//! Enumerations provide a convenient way to work with sets of related constants
//! and to associate constant values with names.
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/enum-statement)

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse a Visual Basic 6 Enum statement with syntax:
    ///
    /// \[ Public | Private \] Enum name
    /// membername \[= constantexpression\]
    /// membername \[= constantexpression\]
    /// ...
    /// End Enum
    ///
    /// The Enum statement syntax has these parts:
    ///
    /// | Part        | Optional / Required | Description |
    /// |-------------|---------------------|-------------|
    /// | Public      | Optional | Indicates that the Enum type is accessible to all other procedures in all modules. If used in a module that contains an Option Private statement, the Enum is not available outside the project. |
    /// | Private     | Optional | Indicates that the Enum type is accessible only to other procedures in the module where it is declared. |
    /// | name        | Required | Name of the Enum type; follows standard variable naming conventions. |
    /// | membername  | Required | Name of the enumeration member; follows standard variable naming conventions. |
    /// | constantexpression | Optional | Value to be assigned to the member (evaluates to a Long). If no constantexpression is specified, the value assigned is either zero (if it is the first membername), or 1 greater than the value of the immediately preceding membername. |
    ///
    /// Remarks:
    /// - Enumeration variables are variables declared with an Enum type.
    /// - Both variables and properties can be declared with an Enum type.
    /// - The values of Enum members are initialized to constant values within the Enum statement.
    /// - Values can't be modified at run time.
    /// - Enum values are Long integers.
    /// - By default, the first member is initialized to 0, and subsequent members are initialized to 1 more than the previous member.
    /// - You can assign specific values to members using the = operator.
    ///
    /// Examples:
    /// ```vb
    /// Public Enum SecurityLevel
    ///     IllegalEntry = -1
    ///     SecurityLevel1 = 0
    ///     SecurityLevel2 = 1
    /// End Enum
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/enum-statement)
    pub(crate) fn parse_enum_statement(&mut self) {
        // if we are now parsing an enum statement, we are no longer in the header.
        self.parsing_header = false;
        self.builder.start_node(SyntaxKind::EnumStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume optional Public/Private keyword
        if self.at_token(Token::PublicKeyword) || self.at_token(Token::PrivateKeyword) {
            self.consume_token();

            // Consume any whitespace after visibility modifier
            self.consume_whitespace();
        }

        // Consume "Enum" keyword
        self.consume_token();

        // Consume any whitespace after "Enum"
        self.consume_whitespace();

        // Consume enum name (keywords can be used as enum names in VB6)
        if self.at_token(Token::Identifier) {
            self.consume_token();
        } else if self.at_keyword() {
            self.consume_token_as_identifier();
        }

        // Consume everything until newline (preserving all tokens)
        self.consume_until_after(Token::Newline);

        // Wrap body in StatementList for formatter indent tracking
        self.builder.start_node(SyntaxKind::StatementList.to_raw());

        // Parse enum members until "End Enum"
        while !self.is_at_end() {
            // Check if we've reached "End Enum"
            if self.at_token(Token::EndKeyword)
                && self.peek_next_keyword() == Some(Token::EnumKeyword)
            {
                break;
            }

            // Consume enum member lines (identifier [= expression])
            // This includes whitespace, comments, identifiers, operators, and newlines
            // Also includes square brackets for VB6 attributes like [Description("...")]
            match self.current_token() {
                Some(
                    Token::Whitespace
                    | Token::Newline
                    | Token::EndOfLineComment
                    | Token::RemComment
                    | Token::Identifier
                    | Token::EqualityOperator
                    | Token::IntegerLiteral
                    | Token::LongLiteral
                    | Token::SingleLiteral
                    | Token::DoubleLiteral
                    | Token::SubtractionOperator
                    | Token::AdditionOperator
                    | Token::MultiplicationOperator
                    | Token::DivisionOperator
                    | Token::ExponentiationOperator
                    | Token::LeftParenthesis
                    | Token::RightParenthesis
                    | Token::Ampersand
                    | Token::Underscore
                    | Token::Comma
                    | Token::LeftSquareBracket
                    | Token::RightSquareBracket,
                ) => {
                    self.consume_token();
                }
                _ if self.at_keyword() => {
                    // Keywords can appear inside square brackets as escaped identifiers
                    self.consume_token();
                }
                _ => {
                    self.consume_error(vec!["enum member name".to_string()]);
                }
            }
        }

        self.builder.finish_node(); // StatementList

        // Consume "End Enum" and trailing tokens
        if self.at_token(Token::EndKeyword) {
            // Consume "End"
            self.consume_token();

            // Consume any whitespace between "End" and "Enum"
            self.consume_whitespace();

            // Consume "Enum"
            self.consume_token();

            // Consume until newline (including it)
            self.consume_until_after(Token::Newline);
        }

        self.builder.finish_node(); // EnumStatement
    }
}
