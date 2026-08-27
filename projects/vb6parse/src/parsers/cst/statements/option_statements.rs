//! Option statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 Option statements:
//! - Option Explicit - Require explicit variable declarations
//! - Option Base - Set default lower bound for array subscripts
//! - Option Compare - Set string comparison method
//! - Option Private - Set module visibility

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse an Option statement: Option Explicit On/Off or Option Base 0/1
    pub(crate) fn parse_option_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::OptionStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume "Option" keyword
        self.consume_token();

        // Consume everything until newline (preserving all tokens)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // OptionStatement
    }

    /// Parse an Option Base statement.
    ///
    /// Sets the default lower bound for array subscripts.
    ///
    /// # Syntax
    ///
    /// | Clause | Description |
    /// |--------|-------------|
    /// | `Option Base 0` | Sets the default lower bound to 0 (default) |
    /// | `Option Base 1` | Sets the default lower bound to 1 |
    ///
    /// # Remarks
    ///
    /// The `Option Base` statement is used to set the default lower bound for array subscripts
    /// in a module. By default, VB6 uses 0 as the lower bound. Using `Option Base 1` changes
    /// this to 1 for all arrays that don't explicitly specify bounds.
    ///
    /// - Must be used at module level (before any procedures)
    /// - Only values 0 and 1 are allowed
    /// - Affects only arrays declared without explicit lower bounds
    /// - Does not affect arrays declared with explicit bounds (e.g., `Dim arr(5 To 10)`)
    ///
    /// # Examples
    ///
    /// ```vb6
    /// Option Base 1
    ///
    /// Sub Example()
    ///     Dim arr(10)  ' Lower bound is 1, upper bound is 10
    ///     arr(1) = "First element"
    /// End Sub
    /// ```
    ///
    /// # References
    ///
    /// [Microsoft Documentation](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/option-base-statement)
    pub(crate) fn parse_option_base_statement(&mut self) {
        self.parse_option_statement();
    }

    /// Parse an Option Compare statement.
    ///
    /// Sets the string comparison method for the module.
    ///
    /// # Syntax
    ///
    /// | Clause | Description |
    /// |--------|-------------|
    /// | `Option Compare Binary` | Case-sensitive string comparison based on binary representation |
    /// | `Option Compare Text` | Case-insensitive string comparison |
    /// | `Option Compare Database` | String comparison based on database locale (Access only) |
    ///
    /// # Remarks
    ///
    /// The `Option Compare` statement is used to set the default string comparison method
    /// for a module. This affects how VB6 compares strings in operations like `=`, `<`, `>`,
    /// and in string functions.
    ///
    /// - Must be used at module level (before any procedures)
    /// - If not specified, the default is `Binary`
    /// - **Binary**: Case-sensitive comparison based on internal binary representation of characters
    /// - **Text**: Case-insensitive comparison (A = a, B = b, etc.)
    /// - **Database**: Uses database sort order (Microsoft Access only)
    ///
    /// Binary comparison is faster but case-sensitive. Text comparison is case-insensitive
    /// but may be slower. The comparison method affects:
    /// - String comparisons in If statements
    /// - `InStr` function
    /// - `StrComp` function (unless comparison argument is specified)
    /// - Select Case with string expressions
    ///
    /// # Examples
    ///
    /// ```vb6
    /// Option Compare Text
    ///
    /// Sub Example()
    ///     If "ABC" = "abc" Then  ' True with Text, False with Binary
    ///         Debug.Print "Strings are equal"
    ///     End If
    /// End Sub
    /// ```
    ///
    /// ```vb6
    /// Option Compare Binary
    ///
    /// Sub Example()
    ///     If "ABC" = "abc" Then  ' False - case sensitive
    ///         Debug.Print "This won't print"
    ///     End If
    /// End Sub
    /// ```
    ///
    /// # References
    ///
    /// [Microsoft Documentation](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/option-compare-statement)
    pub(crate) fn parse_option_compare_statement(&mut self) {
        self.parse_option_statement();
    }

    /// Parse an Option Private statement.
    ///
    /// Controls the visibility of module-level entities (classes, functions, etc.).
    ///
    /// # Syntax
    ///
    /// | Clause | Description |
    /// |--------|-------------|
    /// | `Option Private Module` | Makes entities in the module private to the project |
    ///
    /// # Remarks
    ///
    /// The `Option Private Module` statement is used to indicate that the entire module
    /// is private to the project in which it resides. This means that the module and its
    /// public members are not available to other projects or type libraries.
    ///
    /// - Must be used at module level (at the very top of the module)
    /// - Only valid in standard modules (.bas files) and class modules (.cls files)
    /// - Does not affect the visibility of members within the same project
    /// - When used in a class module, the class cannot be created from outside the project
    /// - Has no effect in form modules (.frm files)
    ///
    /// This is particularly useful for creating helper modules or classes that should only
    /// be used internally within a project and not exposed to external projects that might
    /// reference this one.
    ///
    /// # Examples
    ///
    /// ```vb6
    /// Option Private Module
    ///
    /// ' This module's public functions are only accessible within this project
    /// Public Function InternalHelper() As String
    ///     InternalHelper = "This is private to the project"
    /// End Function
    /// ```
    ///
    /// # References
    ///
    /// [Microsoft Documentation](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/option-private-statement)
    pub(crate) fn parse_option_private_statement(&mut self) {
        self.parse_option_statement();
    }
}
