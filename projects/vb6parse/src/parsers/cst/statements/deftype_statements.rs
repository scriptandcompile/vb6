//! `DefType` statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 `DefType` statements which set default data types
//! for variables based on their first letter.
//!
//! `DefType` statements include:
//! - `DefBool`: Boolean type
//! - `DefByte`: Byte type
//! - `DefInt`: Integer type
//! - `DefLng`: Long type
//! - `DefCur`: Currency type
//! - `DefSng`: Single type
//! - `DefDbl`: Double type
//! - `DefDec`: Decimal type
//! - `DefDate`: Date type
//! - `DefStr`: String type
//! - `DefObj`: Object type
//! - `DefVar`: Variant type
//!
//! Syntax: `DefType` letterrange [, letterrange] ...
//! where letterrange is a single letter or a range like A-Z
//!
//! [Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa263421(v=vs.60))

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse a Visual Basic 6 `DefType` statement with syntax:
    ///
    /// `DefType` letterrange \[, letterrange\] ...
    ///
    /// The `DefType` statement syntax has these parts:
    ///
    /// | Part          | Description |
    /// |---------------|-------------|
    /// | `DefBool`       | Sets default type to Boolean for variables starting with specified letters. |
    /// | `DefByte`       | Sets default type to Byte for variables starting with specified letters. |
    /// | `DefInt`        | Sets default type to Integer for variables starting with specified letters. |
    /// | `DefLng`        | Sets default type to Long for variables starting with specified letters. |
    /// | `DefCur`        | Sets default type to Currency for variables starting with specified letters. |
    /// | `DefSng`        | Sets default type to Single for variables starting with specified letters. |
    /// | `DefDbl`        | Sets default type to Double for variables starting with specified letters. |
    /// | `DefDec`        | Sets default type to Decimal for variables starting with specified letters. |
    /// | `DefDate`       | Sets default type to Date for variables starting with specified letters. |
    /// | `DefStr`        | Sets default type to String for variables starting with specified letters. |
    /// | `DefObj`        | Sets default type to Object for variables starting with specified letters. |
    /// | `DefVar`        | Sets default type to Variant for variables starting with specified letters. |
    /// | letterrange   | A single letter or range of letters (e.g., A, A-Z, M-P). Multiple ranges separated by commas. |
    ///
    /// Examples:
    /// - `DefInt` A-Z (all variables default to Integer)
    /// - `DefStr` S (variables starting with S default to String)
    /// - `DefLng` L, M-N (variables starting with L, M, or N default to Long)
    ///
    /// [Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa263421(v=vs.60))
    pub(crate) fn parse_deftype_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::DefTypeStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume the DefType keyword (DefBool, DefByte, DefInt, etc.)
        // Note: The tokenizer already validated this is a valid DefType keyword token.
        self.consume_token();

        // Consume any whitespace after DefType keyword
        self.consume_whitespace();

        // Parse letter ranges until newline
        // Letter ranges can be:
        // - Single letter: A
        // - Range: A-Z
        // - Multiple ranges separated by commas: A, M-Z
        // Note: Range validation (e.g., ensuring A <= Z in A-Z) is deferred to semantic analysis.
        // The CST preserves the exact source structure without semantic validation.
        loop {
            // Check if we've reached the end of the line
            if self.at_token(Token::Newline) || self.is_at_end() {
                break;
            }

            // Consume the letter or range
            // This includes identifiers (for letters) and minus signs (for ranges)
            self.consume_token();
        }

        // Consume the newline
        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.builder.finish_node(); // DefTypeStatement
    }
}
