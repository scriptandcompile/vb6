use super::Parser;
use crate::language::Token;
use crate::parsers::SyntaxKind;

// Extracted from: system_interaction/beep.rs
impl Parser<'_> {
    // VB6 Beep statement syntax:
    // - Beep
    //
    // Emits a standard system beep sound.
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/beep-statement)
    pub(crate) fn parse_beep_statement(&mut self) {
        self.builder.start_node(SyntaxKind::BeepStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        self.builder.finish_node();
    }
}

// Extracted from: system_interaction/savepicture.rs
impl Parser<'_> {
    /// Parses a `SavePicture` statement.
    ///
    /// `SavePicture` statement syntax:
    /// ```vb
    /// SavePicture picture, filename
    /// ```
    ///
    /// - **picture**: Required. A property or graphic object from which to save the image.
    /// - **filename**: Required. String expression specifying the name of the file to which the graphic is saved.
    pub(crate) fn parse_savepicture_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::SavePictureStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse picture expression wrapped in ArgumentList/Argument
        self.builder.start_node(SyntaxKind::ArgumentList.to_raw());
        self.parse_savepicture_argument();

        self.consume_whitespace();

        // Parse comma
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();
        }

        // Parse filename expression wrapped in Argument
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_savepicture_argument();
            self.consume_whitespace();
        }

        self.builder.finish_node(); // ArgumentList

        self.builder.finish_node();
    }

    /// Parse a single `SavePicture` argument, wrapped in an Argument node.
    fn parse_savepicture_argument(&mut self) {
        self.builder.start_node(SyntaxKind::Argument.to_raw());
        self.parse_expression();
        self.builder.finish_node();
    }
}

// Extracted from: system_interaction/stop.rs
impl Parser<'_> {
    /// Parses a Stop statement.
    pub(crate) fn parse_stop_statement(&mut self) {
        self.builder.start_node(SyntaxKind::StopStatement.to_raw());
        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();
        self.builder.finish_node();
    }
}

// Extracted from: system_interaction/delete_setting.rs
impl Parser<'_> {
    /// Parses a `DeleteSetting` statement.
    ///
    /// Deletes a section or key setting from an application's registry entry.
    ///
    /// ## Syntax
    ///
    /// ```vb
    /// DeleteSetting appname [, section [, key]]
    /// ```
    ///
    /// ## Reference
    ///
    /// [DeleteSetting Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/deletesetting-statement)
    pub(crate) fn parse_delete_setting_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::DeleteSettingStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.at_token(Token::Newline) && !self.is_at_end() {
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}

// Extracted from: system_interaction/sendkeys.rs
impl Parser<'_> {
    /// Parses a `SendKeys` statement.
    ///
    /// Sends one or more keystrokes to the active window as if typed at the keyboard.
    ///
    /// ## Syntax
    ///
    /// ```vb
    /// SendKeys string [, wait]
    /// ```
    ///
    /// ## Parts
    ///
    /// - **string**: Required. String expression specifying the keystrokes to send.
    /// - **wait**: Optional. Boolean value specifying the wait mode.
    ///
    /// ## Reference
    ///
    /// [SendKeys Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/sendkeys-statement)
    pub(crate) fn parse_sendkeys_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::SendKeysStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.at_token(Token::Newline) && !self.is_at_end() {
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}

// Extracted from: system_interaction/app_activate.rs
impl Parser<'_> {
    // VB6 AppActivate statement syntax:
    // - AppActivate title[, wait]
    //
    // Activates an application window.
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/appactivate-statement)
    pub(crate) fn parse_app_activate_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::AppActivateStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.at_token(Token::Newline) && !self.is_at_end() {
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}

// Extracted from: system_interaction/unload.rs
impl Parser<'_> {
    /// Parses an Unload statement.
    ///
    /// Unload statement syntax:
    /// ```vb
    /// Unload object
    /// ```
    ///
    /// - **object**: Required. An object expression that evaluates to a Form or control.
    pub(crate) fn parse_unload_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::UnloadStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }

        self.builder.finish_node();
    }
}

// Extracted from: system_interaction/load.rs
impl Parser<'_> {
    /// Parses a Load statement.
    ///
    /// Load statement syntax:
    /// ```vb
    /// Load object
    /// ```
    ///
    /// - **object**: Required. An object expression that evaluates to a Form or control.
    pub(crate) fn parse_load_statement(&mut self) {
        self.builder.start_node(SyntaxKind::LoadStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }

        self.builder.finish_node();
    }
}

// Extracted from: system_interaction/savesetting.rs
impl Parser<'_> {
    /// Parses a `SaveSetting` statement.
    ///
    /// ```vb
    /// SaveSetting appname, section, key, setting
    /// ```
    pub(crate) fn parse_savesetting_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::SaveSettingStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.at_token(Token::Newline) && !self.is_at_end() {
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}
