use crate::parsers::cst::Parser;
use crate::parsers::syntaxkind::SyntaxKind;

/// # `RSet` Statement
///
/// Right-aligns a string within a string variable or copies one user-defined variable to another.
///
/// ## Syntax
///
/// ```vb
/// RSet stringvar = string
/// RSet varname1 = varname2  ' For user-defined types
/// ```
///
/// ## Parts
///
/// - **stringvar**: Required. String variable or property name to be right-aligned.
/// - **string**: Required. String expression to be right-aligned within stringvar.
/// - **varname1**: Required. Variable of a user-defined type.
/// - **varname2**: Required. Variable of a different user-defined type.
///
/// ## Remarks
///
/// - **String Alignment**: When used with string variables, `RSet` right-aligns the string within
///   the variable. If the string is shorter than the variable, spaces are added on the left to
///   achieve right alignment.
/// - **Fixed-Length Strings**: `RSet` is particularly useful with fixed-length strings where you
///   need to right-justify text within a specific width.
/// - **User-Defined Types**: When used with user-defined types (UDTs), `RSet` copies data from one
///   variable to another on a byte-by-byte basis. This can be useful for converting between
///   different UDT structures that have the same size.
/// - **Shorter Strings**: If the source string is shorter than the target variable, spaces are
///   added on the left side to right-align the text.
/// - **Longer Strings**: If the source string is longer than the target variable, the string is
///   truncated on the left side, keeping only the rightmost characters that fit.
/// - **Comparison to `LSet`**: `RSet` is the opposite of `LSet`. While `LSet` left-aligns strings,
///   `RSet` right-aligns them.
///
/// ## Example
///
/// ```vb
/// Dim MyString As String * 10
/// MyString = String(10, "X")  ' Fill with X's
/// RSet MyString = "VB6"       ' Result: "       VB6"
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
/// RSet VarB = VarA  ' Copy VarA to VarB byte-by-byte
/// ```
///
/// ## See Also
///
/// - `LSet` statement (left-align strings)
/// - `Mid` statement (replace characters in a string)
/// - Fixed-length string variables
///
/// ## References
///
/// - [RSet Statement (Visual Basic 6.0)](https://docs.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa266258(v=vs.60))
impl Parser<'_> {
    pub(crate) fn parse_rset_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::RSetStatement.to_raw());

        self.consume_whitespace();

        if let Some((text, _)) = self.tokens.get(self.pos) {
            self.builder.token(SyntaxKind::RSetKeyword.to_raw(), text);
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
