use crate::parsers::cst::Parser;
use crate::parsers::syntaxkind::SyntaxKind;

/// # `LSet` Statement
///
/// Left-aligns a string within a string variable or copies one user-defined variable to another.
///
/// ## Syntax
///
/// ```vb
/// LSet stringvar = string
/// LSet varname1 = varname2  ' For user-defined types
/// ```
///
/// ## Parts
///
/// - **stringvar**: Required. String variable or property name to be left-aligned.
/// - **string**: Required. String expression to be left-aligned within stringvar.
/// - **varname1**: Required. Variable of a user-defined type.
/// - **varname2**: Required. Variable of a different user-defined type.
///
/// ## Remarks
///
/// - **String Alignment**: When used with string variables, `LSet` left-aligns the string within
///   the variable. If the string is shorter than the variable, spaces are added on the right to
///   achieve left alignment.
/// - **Fixed-Length Strings**: `LSet` is particularly useful with fixed-length strings where you
///   need to left-justify text within a specific width.
/// - **User-Defined Types**: When used with user-defined types (UDTs), `LSet` copies data from one
///   variable to another on a byte-by-byte basis. This can be useful for converting between
///   different UDT structures that have the same size.
/// - **Shorter Strings**: If the source string is shorter than the target variable, spaces are
///   added on the right side to left-align the text.
/// - **Longer Strings**: If the source string is longer than the target variable, the string is
///   truncated on the right side, keeping only the leftmost characters that fit.
/// - **Comparison to `RSet`**: `LSet` is the opposite of `RSet`. While `LSet` left-aligns strings,
///   `RSet` right-aligns them.
///
/// ## Example
///
/// ```vb
/// Dim MyString As String * 10
/// MyString = String(10, "X")  ' Fill with X's
/// LSet MyString = "VB6"       ' Result: "VB6       "
/// ```
///
/// ## Example with User-Defined Types
///
/// ```vb
/// Type TypeA
///     Name As String * 20
///     Age As Integer
/// End Type
///
/// Type TypeB
///     Data As String * 22
/// End Type
///
/// Dim VarA As TypeA
/// Dim VarB As TypeB
///
/// VarA.Name = "John"
/// VarA.Age = 30
/// LSet VarB = VarA  ' Copy VarA to VarB byte-by-byte
/// ```
///
/// ## See Also
///
/// - `RSet` statement (right-align strings)
/// - `Mid` statement (replace characters in a string)
/// - Fixed-length string variables
///
/// ## References
///
/// - [LSet Statement (Visual Basic 6.0)](https://docs.microsoft.com/en-us/office/vba/language/reference/user-interface-help/lset-statement)
impl Parser<'_> {
    /// Parses an `LSet` statement: `LSet target = value`
    pub(crate) fn parse_lset_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::LSetStatement.to_raw());

        self.consume_whitespace();

        if let Some((text, _)) = self.tokens.get(self.pos) {
            self.builder.token(SyntaxKind::LSetKeyword.to_raw(), text);
            self.pos += 1;
        }

        self.consume_whitespace();

        self.parse_expression();

        self.consume_whitespace();

        if let Some((text, token)) = self.tokens.get(self.pos) {
            let kind = SyntaxKind::from(*token);
            if kind == SyntaxKind::EqualityOperator {
                self.builder.token(kind.to_raw(), text);
                self.pos += 1;
            }
        }

        self.consume_whitespace();

        self.parse_expression();

        self.builder.finish_node();
    }
}
