//! Type statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 Type (user-defined type) statements.
//!
//! Type statement syntax:
//!
//! \[Public | Private\] Type typename
//! elementname \[(subscripts)\] As type
//! \[elementname \[(subscripts)\] As type\]
//! ...
//! End Type
//!
//! User-defined types (UDTs) provide a way to create custom data structures
//! that group related variables of different data types under one name.
//!
//! [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/type-statement)

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse a Visual Basic 6 Type statement with syntax:
    ///
    /// \[Public | Private\] Type typename
    /// elementname \[(subscripts)\] As type
    /// \[elementname \[(subscripts)\] As type\]
    /// ...
    /// End Type
    ///
    /// The Type statement syntax has these parts:
    ///
    /// | Part        | Optional / Required | Description |
    /// |-------------|---------------------|-------------|
    /// | Public      | Optional | Indicates that the Type is accessible to all other procedures in all modules. If used in a module that contains an Option Private statement, the Type is not available outside the project. |
    /// | Private     | Optional | Indicates that the Type is accessible only to other procedures in the module where it is declared. |
    /// | typename    | Required | Name of the user-defined type; follows standard variable naming conventions. |
    /// | elementname | Required | Name of an element (field) of the user-defined type. Element names follow standard variable naming conventions, except that keywords can be used. |
    /// | subscripts  | Optional | Dimensions of an array element. Use only parentheses when declaring an array whose size can change. The subscript syntax has these parts: \[lower To\] upper \[, \[lower To\] upper\] ... |
    /// | type        | Required | Data type of the element; may be Byte, Boolean, Integer, Long, Currency, Single, Double, Decimal (not currently supported), Date, String (variable-length or fixed-length), Object, Variant, another user-defined type, or an object type. |
    ///
    /// Remarks:
    /// - User-defined types are typically used to create records similar to those in databases.
    /// - User-defined types can contain elements of different data types.
    /// - Array elements within user-defined types can be dynamic arrays (using empty parentheses).
    /// - Fixed-length strings can be used in user-defined types.
    /// - Type statements can only be used at the module level. Once you declare a user-defined type using the Type statement, you can declare a variable of that type anywhere within the scope of the declaration.
    /// - User-defined types are useful for passing multiple related values as a single unit to procedures.
    /// - Cannot be used in class modules unless they are Private.
    ///
    /// ## Examples
    ///
    /// ### Basic User-Defined Type
    ///
    /// ```vb
    /// Type Employee
    ///     EmployeeID As Long
    ///     FirstName As String
    ///     LastName As String
    ///     HireDate As Date
    ///     Salary As Currency
    /// End Type
    /// ```
    ///
    /// ### Type with Fixed-Length String
    ///
    /// ```vb
    /// Type CustomerRecord
    ///     CustomerID As Long
    ///     CustomerName As String * 50
    ///     Address As String * 100
    ///     City As String * 30
    ///     ZipCode As String * 10
    /// End Type
    /// ```
    ///
    /// ### Type with Array Element
    ///
    /// ```vb
    /// Type SalesData
    ///     SalesPersonID As Long
    ///     MonthlySales(1 To 12) As Currency
    ///     QuarterlySales(1 To 4) As Currency
    /// End Type
    /// ```
    ///
    /// ### Nested User-Defined Types
    ///
    /// ```vb
    /// Type Address
    ///     Street As String
    ///     City As String
    ///     State As String
    ///     ZipCode As String
    /// End Type
    ///
    /// Type Person
    ///     Name As String
    ///     HomeAddress As Address
    ///     WorkAddress As Address
    /// End Type
    /// ```
    ///
    /// ### Public Type Declaration
    ///
    /// ```vb
    /// Public Type Point
    ///     x As Single
    ///     y As Single
    /// End Type
    /// ```
    ///
    /// ### Private Type Declaration
    ///
    /// ```vb
    /// Private Type InternalData
    ///     Buffer(0 To 255) As Byte
    ///     Length As Integer
    /// End Type
    /// ```
    ///
    /// ### Type with Variant Element
    ///
    /// ```vb
    /// Type FlexibleRecord
    ///     RecordType As Integer
    ///     Data As Variant
    /// End Type
    /// ```
    ///
    /// ### Type for API Structures
    ///
    /// ```vb
    /// Type RECT
    ///     Left As Long
    ///     Top As Long
    ///     Right As Long
    ///     Bottom As Long
    /// End Type
    /// ```
    ///
    /// ## Common Patterns
    ///
    /// ### Using Type in Declarations
    ///
    /// ```vb
    /// Dim emp As Employee
    /// emp.EmployeeID = 1001
    /// emp.FirstName = "John"
    /// emp.LastName = "Doe"
    /// ```
    ///
    /// ### Passing Type to Procedures
    ///
    /// ```vb
    /// Sub UpdateEmployee(empData As Employee)
    ///     ' Update database with employee data
    ///     Debug.Print empData.FirstName & " " & empData.LastName
    /// End Sub
    /// ```
    ///
    /// ### Arrays of User-Defined Types
    ///
    /// ```vb
    /// Dim employees(1 To 100) As Employee
    /// employees(1).EmployeeID = 1001
    /// employees(1).FirstName = "John"
    /// ```
    ///
    /// ## Best Practices
    ///
    /// 1. Use meaningful names for Type and element names
    /// 2. Use fixed-length strings when the length is known and constant
    /// 3. Group related data into a single Type
    /// 4. Use Public for Types that need to be shared across modules
    /// 5. Use Private for Types that are module-specific
    /// 6. Document complex Types with comments
    /// 7. Consider performance implications of large Types
    ///
    /// ## Important Notes
    ///
    /// - Type statements cannot be nested within procedures
    /// - Type statements must appear at module level
    /// - In class modules, Type must be Private
    /// - Elements of a Type can be other user-defined types
    /// - User-defined types are passed by value unless explicitly passed `ByRef`
    /// - Type members are accessed using the dot (.) operator
    ///
    /// ## See Also
    ///
    /// - `Dim` statement (declaring variables of user-defined types)
    /// - `Public` statement (module-level public declarations)
    /// - `Private` statement (module-level private declarations)
    /// - Fixed-length strings (`String * length`)
    ///
    /// ## References
    ///
    /// - [Microsoft Docs: Type Statement](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/type-statement)
    /// - [User-Defined Types](https://learn.microsoft.com/en-us/office/vba/language/concepts/getting-started/creating-your-own-data-types)
    pub(crate) fn parse_type_statement(&mut self) {
        // if we are now parsing a type statement, we are no longer in the header.
        self.parsing_header = false;
        self.builder.start_node(SyntaxKind::TypeStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume optional Public/Private keyword
        if self.at_token(Token::PublicKeyword) || self.at_token(Token::PrivateKeyword) {
            self.consume_token();

            // Consume any whitespace after visibility modifier
            self.consume_whitespace();
        }

        // Consume "Type" keyword
        self.consume_token();

        // Consume any whitespace after "Type"
        self.consume_whitespace();

        // Consume type name (keywords can be used as type names in VB6)
        if self.at_token(Token::Identifier) {
            self.consume_token();
        } else if self.at_keyword() {
            self.consume_token_as_identifier();
        }

        // Consume everything until newline (preserving all tokens)
        self.consume_until_after(Token::Newline);

        // Wrap body in StatementList for formatter indent tracking
        self.builder.start_node(SyntaxKind::StatementList.to_raw());

        // Parse type members until "End Type"
        while !self.is_at_end() {
            // Check if we've reached "End Type"
            if self.at_token(Token::EndKeyword)
                && self.peek_next_keyword() == Some(Token::TypeKeyword)
            {
                break;
            }

            // Recover from missing End Type when another top-level component starts.
            if self.at_component_declaration_start() {
                break;
            }

            if self.at_compiler_directive_keyword(Token::IfKeyword) {
                self.parse_type_compiler_directive();
                continue;
            }

            // Orphan #ElseIf/#Else/#End If (not inside a #If) — consume as raw tokens
            if self.at_compiler_directive_keyword(Token::ElseIfKeyword)
                || self.at_compiler_directive_keyword(Token::ElseKeyword)
                || self.at_compiler_end_if_directive()
            {
                self.consume_compiler_directive_prefix();
                self.consume_until_after(Token::Newline);
                continue;
            }

            // Consume type member lines (elementname [(subscripts)] As type)
            // This includes whitespace, comments, identifiers, operators, and newlines
            match self.current_token() {
                Some(Token::Whitespace
                | Token::Newline
                | Token::EndOfLineComment
                | Token::RemComment
                | Token::Identifier
                | Token::AsKeyword
                | Token::LeftParenthesis
                | Token::RightParenthesis
                | Token::ToKeyword
                | Token::IntegerLiteral
                | Token::LongLiteral
                | Token::Comma
                | Token::MultiplicationOperator // For String * length
                | Token::SubtractionOperator   // For negative array bounds
                // Data type keywords that can appear in Type members
                | Token::ByteKeyword
                | Token::BooleanKeyword
                | Token::IntegerKeyword
                | Token::LongKeyword
                | Token::CurrencyKeyword
                | Token::SingleKeyword
                | Token::DoubleKeyword
                | Token::DateKeyword
                | Token::StringKeyword
                | Token::ObjectKeyword
                | Token::VariantKeyword) => {
                    self.consume_token();
                }
                _ => {
                    // Check if this is a keyword being used as an identifier (VB6 allows this)
                    if self.at_keyword() {
                        self.consume_token();
                    } else {
                        self.consume_error(vec!["type member name".to_string()]);
                    }
                }
            }
        }

        self.builder.finish_node(); // StatementList

        // Consume "End Type" and trailing tokens, or report mismatch as recovery.
        self.consume_expected_end_keyword_terminator(Token::TypeKeyword, "Type");

        self.builder.finish_node(); // TypeStatement
    }

    /// Parse a compiler directive block (`#If` … `#End If`) inside a Type body.
    ///
    /// Creates a `CompilerDirective` node with `CompilerIfClause`, optional
    /// `CompilerElseIfClause` / `CompilerElseClause`, and `CompilerEndIfClause`.
    /// The body between clauses is parsed using the same Type-member logic as
    /// the main `parse_type_statement` loop.
    fn parse_type_compiler_directive(&mut self) {
        self.builder
            .start_node(SyntaxKind::CompilerDirective.to_raw());

        // CompilerIfClause
        self.builder
            .start_node(SyntaxKind::CompilerIfClause.to_raw());
        self.consume_compiler_directive_prefix();
        self.consume_until_after(Token::Newline);
        self.builder.finish_node();

        loop {
            // Wrap body in StatementList (matching regular parse_compiler_directive)
            self.builder.start_node(SyntaxKind::StatementList.to_raw());

            // Parse body (type members until directive or End Type)
            while !self.is_at_end() {
                if self.at_token(Token::EndKeyword)
                    && self.peek_next_keyword() == Some(Token::TypeKeyword)
                {
                    break;
                }
                if self.at_component_declaration_start() {
                    break;
                }
                if self.at_compiler_directive_keyword(Token::ElseIfKeyword)
                    || self.at_compiler_directive_keyword(Token::ElseKeyword)
                    || self.at_compiler_end_if_directive()
                {
                    break;
                }
                match self.current_token() {
                    Some(
                        Token::Whitespace
                        | Token::Newline
                        | Token::EndOfLineComment
                        | Token::RemComment
                        | Token::Identifier
                        | Token::AsKeyword
                        | Token::LeftParenthesis
                        | Token::RightParenthesis
                        | Token::ToKeyword
                        | Token::IntegerLiteral
                        | Token::LongLiteral
                        | Token::Comma
                        | Token::MultiplicationOperator
                        | Token::SubtractionOperator
                        | Token::ByteKeyword
                        | Token::BooleanKeyword
                        | Token::IntegerKeyword
                        | Token::LongKeyword
                        | Token::CurrencyKeyword
                        | Token::SingleKeyword
                        | Token::DoubleKeyword
                        | Token::DateKeyword
                        | Token::StringKeyword
                        | Token::ObjectKeyword
                        | Token::VariantKeyword,
                    ) => {
                        self.consume_token();
                    }
                    _ => {
                        if self.at_keyword() {
                            self.consume_token();
                        } else {
                            self.consume_error(vec!["type member name".to_string()]);
                        }
                    }
                }
            }

            self.builder.finish_node(); // StatementList

            if self.at_compiler_directive_keyword(Token::ElseIfKeyword) {
                self.builder
                    .start_node(SyntaxKind::CompilerElseIfClause.to_raw());
                self.consume_compiler_directive_prefix();
                self.consume_until_after(Token::Newline);
                self.builder.finish_node();
                continue;
            }

            if self.at_compiler_directive_keyword(Token::ElseKeyword) {
                self.builder
                    .start_node(SyntaxKind::CompilerElseClause.to_raw());
                self.consume_compiler_directive_prefix();
                self.consume_until_after(Token::Newline);
                self.builder.finish_node();
                continue;
            }

            if self.at_compiler_end_if_directive() {
                self.builder
                    .start_node(SyntaxKind::CompilerEndIfClause.to_raw());
                self.consume_compiler_directive_prefix();
                self.consume_until_after(Token::Newline);
                self.builder.finish_node();
            }

            break;
        }

        self.builder.finish_node(); // CompilerDirective
    }
}
