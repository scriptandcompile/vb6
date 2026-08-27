//! Function statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 Function statements with syntax:
//!
//! \[ Public | Private | Friend \] \[ Static \] Function name \[ ( arglist ) \] \[ As type \]
//! \[ statements \]
//! \[ name = expression \]
//! \[ Exit Function \]
//! \[ statements \]
//! \[ name = expression \]
//! End Function
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/function-statement)

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse a Visual Basic 6 `Function` with syntax:
    ///
    /// `\[ Public | Private | Friend \] \[ Static \] Function name \[ ( arglist ) \] \[ As type \]`
    /// `\[ statements \]`
    /// `\[ name = expression \]`
    /// `\[ Exit Function \]`
    /// `\[ statements \]`
    /// `\[ name = expression \]`
    /// `End Function`
    ///
    /// The `Function` statement syntax has these parts:
    ///
    /// | Part        | Optional / Required | Description |
    /// |-------------|---------------------|-------------|
    /// | Public      | Optional | Indicates that the `Function` procedure is accessible to all other procedures in all modules. If used in a module that contains an Option Private, the procedure is not available outside the project. |
    /// | Private     | Optional | Indicates that the `Function` procedure is accessible only to other procedures in the module where it is declared. |
    /// | Friend      | Optional | Used only in a class module. Indicates that the `Function` procedure is visible throughout the project, but not visible to a controller of an instance of an object. |
    /// | Static      | Optional | Indicates that the `Function` procedure's local variables are preserved between calls. The Static attribute doesn't affect variables that are declared outside the `Function`, even if they are used in the procedure. |
    /// | name        | Required | Name of the `Function`; follows standard variable naming conventions. |
    /// | arglist     | Optional | List of variables representing arguments that are passed to the `Function` procedure when it is called. Multiple variables are separated by commas. |
    /// | type        | Optional | Data type of the value returned by the `Function` procedure; may be `Byte`, `Boolean`, `Integer`, `Long`, `Currency`, `Single`, `Double`, `Decimal` (not currently supported), `Date`, `String` (except fixed length), `Object`, `Variant`, or any user-defined type. |
    /// | statements  | Optional | Any group of statements to be executed within the `Function` procedure.
    /// | expression  | Optional | Return value of the `Function`. |
    ///
    /// The arglist argument has the following syntax and parts:
    ///
    /// `\[ Optional \] \[ ByVal | ByRef \] \[ ParamArray \] varname \[ ( ) \] \[ As type \] \[ = defaultvalue \]`
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/function-statement)
    pub(crate) fn parse_function_statement(&mut self) {
        // if we are now parsing a function statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::FunctionStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume optional Public/Private/Friend keyword
        if self.at_token(Token::PublicKeyword)
            || self.at_token(Token::PrivateKeyword)
            || self.at_token(Token::FriendKeyword)
        {
            self.consume_token();

            // Consume any whitespace after visibility modifier
            self.consume_whitespace();
        }

        // Consume optional Static keyword
        if self.at_token(Token::StaticKeyword) {
            self.consume_token();

            // Consume any whitespace after Static
            self.consume_whitespace();
        }

        // Consume "Function" keyword
        self.consume_token();

        // Consume any whitespace after "Function"
        self.consume_whitespace();

        // Consume function name (keywords can be used as function names in VB6)
        if self.at_token(Token::Identifier) {
            self.consume_token();
        } else if self.at_keyword() {
            self.consume_token_as_identifier();
        }

        // Consume any whitespace before parameter list
        self.consume_whitespace();

        // Parse parameter list if present
        if self.at_token(Token::LeftParenthesis) {
            self.parse_parameter_list();
        }

        // Consume everything until newline (preserving all tokens)
        self.consume_until_after(Token::Newline);

        // Parse body until "End Function"
        self.parse_statement_list(|parser| {
            (parser.at_token(Token::EndKeyword)
                && parser.peek_next_keyword() == Some(Token::FunctionKeyword))
                || parser.at_component_declaration_start()
        });

        // Consume "End Function" and trailing tokens, or report mismatch as recovery.
        self.consume_expected_end_keyword_terminator(Token::FunctionKeyword, "Function");

        self.builder.finish_node(); // FunctionStatement
    }
}
