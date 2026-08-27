//! `Declare` statement parsing for VB6 CST.
//!
//! This module handles parsing of VB6 declaration statements:
//! - `Declare`: External function/sub declarations
//! - `Event`: Custom event declarations in classes
//! - `Implements`: Interface implementation declarations
//!
//! `Declare` statement syntax:
//! `\[ Public | Private \] Declare { Sub | Function } name Lib "libname" \[ Alias "aliasname" \] \[ ( arglist ) \] \[ As type \]`
//!
//! `Event` statement syntax:
//! `\[ Public \] Event eventname \[ ( arglist ) \]`
//!
//! `Implements` statement syntax:
//! `Implements interfacename`
//!
//! `Sub` statements are handled in the `sub_statements` module.
//! `Function` statements are handled in the `function_statements` module.
//! `Dim`/`ReDim` and general `Variable` declarations are handled in the `array_statements` module.
//! `Property` statements are handled in the `property_statements` module.
//! `Parameter` lists are handled in the `parameters` module.
//!
//! [Declare Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa243324(v=vs.60))
//! [Event Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/event-statement)
//! [Implements Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/implements-statement)

use crate::language::Token;
use crate::parsers::SyntaxKind;

use super::Parser;

impl Parser<'_> {
    /// Parse a Visual Basic 6 `Declare` statement with syntax:
    ///
    /// `\[ Public | Private \] Declare { Sub | Function } name Lib "libname" \[ Alias "aliasname" \] \[ ( arglist ) \] \[ As type \]`
    ///
    /// The `Declare` statement syntax has these parts:
    ///
    /// | Part        | Optional / Required | Description |
    /// |-------------|---------------------|-------------|
    /// | Public      | Optional | Indicates that the `Declare` statement is accessible to all other procedures in all modules. |
    /// | Private     | Optional | Indicates that the `Declare` statement is accessible only to other procedures in the module where it is declared. |
    /// | Sub         | Required | Indicates that the procedure doesn't return a value. |
    /// | Function    | Required | Indicates that the procedure returns a value that can be used in an expression. |
    /// | name        | Required | Name of the external procedure; follows standard variable naming conventions. |
    /// | Lib         | Required | Indicates that a DLL or code resource contains the procedure being declared. The `Lib` clause is required for all declarations. |
    /// | libname     | Required | Name of the DLL or code resource that contains the declared procedure. |
    /// | Alias       | Optional | Indicates that the procedure being called has another name in the DLL. This is useful when the external procedure name is the same as a keyword. |
    /// | aliasname   | Optional | Name of the procedure in the DLL or code resource. If the first character is not a number sign (#), aliasname is the name of the procedure's entry point in the DLL. |
    /// | arglist     | Optional | List of variables representing arguments that are passed to the procedure when it is called. |
    /// | type        | Optional | Data type of the value returned by a Function procedure; may be `Byte`, `Boolean`, `Integer`, `Long`, `Currency`, `Single`, `Double`, `Decimal`, `Date`, `String`, `Object`, `Variant`, or any user-defined type. |
    ///
    /// The arglist argument has the following syntax and parts:
    ///
    /// `\[ Optional \] \[ ByVal | ByRef \] \[ ParamArray \] varname \[ ( ) \] \[ As type \]`
    ///
    /// [Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa243324(v=vs.60))
    pub(crate) fn parse_declare_statement(&mut self) {
        // Declare statements are only valid in the header section
        self.builder
            .start_node(SyntaxKind::DeclareStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume optional Public/Private keyword
        if self.at_token(Token::PublicKeyword) || self.at_token(Token::PrivateKeyword) {
            self.consume_token();

            // Consume any whitespace after visibility modifier
            self.consume_whitespace();
        }

        // Consume "Declare" keyword
        self.consume_token();

        // Consume any whitespace after "Declare"
        self.consume_whitespace();

        // Consume optional PtrSafe modifier used in 64-bit VBA declarations.
        if let Some((text, token)) = self.tokens.get(self.pos)
            && *token == Token::Identifier
            && text.eq_ignore_ascii_case("ptrsafe")
        {
            self.consume_token();
            self.consume_whitespace();
        }

        // Consume "Sub" or "Function" keyword
        if self.at_token(Token::SubKeyword) || self.at_token(Token::FunctionKeyword) {
            self.consume_token();
        }

        // Consume any whitespace after Sub/Function
        self.consume_whitespace();

        // Consume procedure name (keywords can be used as procedure names in VB6)
        if self.at_token(Token::Identifier) {
            self.consume_token();
        } else if self.at_keyword() {
            self.consume_token_as_identifier();
        }

        // Consume any whitespace before Lib
        self.consume_whitespace();

        // Consume "Lib" keyword
        if self.at_token(Token::LibKeyword) {
            self.consume_token();
        }

        // Consume any whitespace after Lib
        self.consume_whitespace();

        // Consume library name string
        if self.at_token(Token::StringLiteral) {
            self.consume_token();
        }

        // Consume any whitespace after library name
        self.consume_whitespace();

        // Consume optional Alias clause
        if self.at_token(Token::AliasKeyword) {
            self.consume_token();

            // Consume any whitespace after Alias
            self.consume_whitespace();

            // Consume alias name string
            if self.at_token(Token::StringLiteral) {
                self.consume_token();
            }

            // Consume any whitespace after alias name
            self.consume_whitespace();
        }

        // Parse parameter list if present
        if self.at_token(Token::LeftParenthesis) {
            self.parse_parameter_list();
        }

        // Consume everything until newline (includes "As Type" if present for Function)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // DeclareStatement
    }

    /// Parse a Visual Basic 6 `Event` statement with syntax:
    ///
    /// `\[ Public \] Event eventname \[ ( arglist ) \]`
    ///
    /// The `Event` statement syntax has these parts:
    ///
    /// | Part        | Optional / Required | Description |
    /// |-------------|---------------------|-------------|
    /// | Public      | Optional | Indicates that the `Event` is accessible to all other procedures in all modules. Events are Public by default. Note that events can't be Private. |
    /// | eventname   | Required | Name of the event; follows standard variable naming conventions. |
    /// | arglist     | Optional | List of variables representing arguments that are passed to the event handler when the event occurs. |
    ///
    /// The `arglist` argument has the following syntax and parts:
    ///
    /// `\[ ByVal | ByRef \] varname \[ ( ) \] \[ As type \]`
    ///
    /// Remarks:
    /// - `Event` statements can appear only in class modules, form modules, and user controls.
    /// - `Event`s are raised using the `RaiseEvent` statement.
    /// - `Event`s declared with `Public` are available to all procedures in the same project.
    /// - `Event`s cannot be declared as `Private`, `Static`, or `Friend`.
    /// - `Event`s cannot have named arguments, `Optional` arguments, or `ParamArray` arguments.
    /// - `Event`s do not have return values.
    ///
    /// Examples:
    /// ```vb
    /// Public Event StatusChanged(ByVal NewStatus As String)
    /// Event DataReceived(ByVal Data() As Byte)
    /// Event Click()
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/event-statement)
    pub(crate) fn parse_event_statement(&mut self) {
        // Event statements are only valid in class modules
        self.builder.start_node(SyntaxKind::EventStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume optional Public keyword
        if self.at_token(Token::PublicKeyword) {
            self.consume_token();

            // Consume any whitespace after visibility modifier
            self.consume_whitespace();
        }

        // Consume "Event" keyword
        self.consume_token();

        // Consume any whitespace after "Event"
        self.consume_whitespace();

        // Consume event name (keywords can be used as event names in VB6)
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

        // Consume everything until newline
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // EventStatement
    }

    /// Parse an Implements statement.
    ///
    /// VB6 Implements statement syntax:
    /// - Implements interfacename
    ///
    /// Specifies an interface or class that will be implemented in the class module in which it appears.
    ///
    /// The Implements statement syntax has this part:
    ///
    /// | Part          | Description |
    /// |---------------|-------------|
    /// | interfacename | Required. Name of an interface or class whose methods and properties will be implemented in the class containing the Implements statement. |
    ///
    /// Remarks:
    /// - The Implements statement is used only in class modules.
    /// - Once you have specified that a class implements an interface, you must provide a procedure in the class for each public procedure defined in the interface.
    /// - The procedure in the implementing class must have the same name and signature as the procedure in the interface.
    /// - A class module can implement more than one interface by including a separate Implements statement for each interface.
    /// - The interface must be defined in a separate class module.
    /// - You can't implement an interface within a single class module.
    ///
    /// Examples:
    /// ```vb
    /// ' In class module clsInterface:
    /// Public Sub DoSomething(x As Integer)
    /// End Sub
    ///
    /// ' In implementing class:
    /// Implements clsInterface
    ///
    /// Private Sub clsInterface_DoSomething(x As Integer)
    ///     ' Implementation code
    /// End Sub
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/implements-statement)
    pub(crate) fn parse_implements_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::ImplementsStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume "Implements" keyword
        self.consume_token();

        // Consume everything until newline (the interface name)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // ImplementsStatement
    }

    /// Parse an Object statement.
    ///
    /// VB6 Object statement syntax:
    /// - Object = "{UUID}#version#flags"; "filename"
    /// - Object = *\G{UUID}#version#flags; "filename"
    ///
    /// Declares external `ActiveX` controls and libraries that a form or user control depends on.
    ///
    /// The Object statement syntax has these parts:
    ///
    /// | Part     | Description |
    /// |----------|-------------|
    /// | UUID     | Required. The class ID (CLSID) of the `ActiveX` control, enclosed in braces. |
    /// | version  | Required. The version number of the control (e.g., "2.0"). |
    /// | flags    | Required. Additional flags (typically "0"). |
    /// | filename | Required. The filename of the OCX or DLL containing the control. |
    ///
    /// Remarks:
    /// - Object statements appear in form (.frm) and user control (.ctl) files.
    /// - They are placed at the module level, after VERSION and before BEGIN.
    /// - Multiple Object statements can appear to declare dependencies on multiple controls.
    /// - The format can use either "{UUID}" or "*\G{UUID}" prefix.
    /// - These declarations are automatically maintained by the VB6 IDE when you add controls to a form.
    ///
    /// Examples:
    /// ```vb
    /// Object = "{831FDD16-0C5C-11D2-A9FC-0000F8754DA1}#2.0#0"; "mscomctl.ocx"
    /// Object = "{F9043C88-F6F2-101A-A3C9-08002B2F49FB}#1.2#0"; "COMDLG32.OCX"
    /// Object = "*\G{00025600-0000-0000-C000-000000000046}#5.2#0"; "stdole2.tlb"
    /// ```
    pub(crate) fn parse_object_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::ObjectStatement.to_raw());

        // Consume any leading whitespace
        self.consume_whitespace();

        // Consume "Object" keyword
        self.consume_token();

        // Consume everything until newline (=, UUID string, semicolon, filename)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // ObjectStatement
    }
}

// Extracted from: declarations/variables.rs
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

// Extracted from: declarations/erase.rs
impl Parser<'_> {
    /// Parse an Erase statement: Erase array1 [, array2] ...
    ///
    /// VB6 Erase statement syntax:
    /// - Erase arraylist
    ///
    /// The Erase statement is used to reinitialize the elements of fixed-size arrays
    /// and to release storage space used by dynamic arrays.
    ///
    /// The arraylist argument is a list of one or more comma-delimited array variable names.
    ///
    /// Behavior:
    /// - For fixed-size arrays: Reinitializes the elements to their default values
    ///   (0 for numeric types, "" for strings, Nothing for objects)
    /// - For dynamic arrays: Deallocates the memory used by the array
    ///
    /// Examples:
    /// ```vb
    /// Erase myArray
    /// Erase array1, array2, array3
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/erase-statement)
    pub(crate) fn parse_erase_statement(&mut self) {
        // if we are now parsing an erase statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::EraseStatement.to_raw());

        // Consume "Erase" keyword
        self.consume_token();

        // Consume everything until newline (array names, commas, etc.)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // EraseStatement
    }
}

// Extracted from: declarations/arrays.rs
impl Parser<'_> {
    /// Parse a `ReDim` statement.
    ///
    /// VB6 `ReDim` statement syntax:
    /// - `ReDim` [Preserve] varname(subscripts) [As type] [, varname(subscripts) [As type]] ...
    ///
    /// Used at procedure level to reallocate storage space for dynamic array variables.
    ///
    /// [Reference](https://learn.microsoft.com/en-us/previous-versions/visualstudio/visual-basic-6/aa266231(v=vs.60))
    pub(crate) fn parse_redim_statement(&mut self) {
        // if we are now parsing a ReDim statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::ReDimStatement.to_raw());

        // This parser may be entered while positioned on leading whitespace
        // (e.g. an indented statement), so consume it before the keyword to
        // ensure "ReDim" is never parsed as a variable name below.
        self.consume_whitespace();

        // Consume "ReDim" keyword
        self.consume_token();
        self.consume_whitespace();

        // Optional Preserve
        if self.at_token(Token::PreserveKeyword) {
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

            // Variable name (keywords can be used as variable names in VB6, e.g. `Name`)
            if self.at_token(Token::Identifier) {
                self.consume_token();
            } else if self.at_keyword() {
                self.consume_token_as_identifier();
            } else {
                // Error recovery
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
                // Type name
                self.consume_token();
                while self.at_token(Token::PeriodOperator) {
                    self.consume_token();
                    self.consume_token();
                }
            }

            self.consume_whitespace();

            if self.at_token(Token::Comma) {
                self.consume_token();
            } else {
                break;
            }
        }

        // Consume everything until newline (Preserve, variable declarations, etc.)
        self.consume_until_after(Token::Newline);

        self.builder.finish_node(); // ReDimStatement
    }
}
