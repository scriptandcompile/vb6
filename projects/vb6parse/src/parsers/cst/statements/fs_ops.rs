use super::Parser;
use crate::language::Token;
use crate::parsers::SyntaxKind;

// Extracted from: filesystem/chdir.rs
impl Parser<'_> {
    /// Parses a `ChDir` statement.
    ///
    /// `ChDir` statement syntax:
    /// ```vb
    /// ChDir path
    /// ```
    ///
    /// - **path**: Required. String expression that specifies the directory path.
    pub(crate) fn parse_ch_dir_statement(&mut self) {
        self.builder.start_node(SyntaxKind::ChDirStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
            self.consume_whitespace();
        }

        self.builder.finish_node();
    }
}

// Extracted from: filesystem/mkdir.rs
impl Parser<'_> {
    /// Parses a `MkDir` statement.
    ///
    /// `MkDir` statement syntax:
    /// ```vb
    /// MkDir path
    /// ```
    ///
    /// - **path**: Required. String expression that identifies the directory or folder to be created. May include drive.
    pub(crate) fn parse_mkdir_statement(&mut self) {
        self.builder.start_node(SyntaxKind::MkDirStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }

        self.builder.finish_node();
    }
}

// Extracted from: filesystem/setattr.rs
impl Parser<'_> {
    /// Parses a `SetAttr` statement.
    ///
    /// `SetAttr` statement syntax:
    /// ```vb
    /// SetAttr pathname, attributes
    /// ```
    ///
    /// - **pathname**: Required. String expression that specifies a file name.
    /// - **attributes**: Required. Numeric expression specifying the file attributes.
    pub(crate) fn parse_setattr_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::SetAttrStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse pathname expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse comma
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();
        }

        // Parse attributes expression
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }

        self.builder.finish_node();
    }
}

// Extracted from: filesystem/chdrive.rs
impl Parser<'_> {
    /// Parses a `ChDrive` statement.
    ///
    /// `ChDrive` statement syntax:
    /// ```vb
    /// ChDrive drive
    /// ```
    ///
    /// - **drive**: Required. String expression that specifies the drive.
    pub(crate) fn parse_ch_drive_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::ChDriveStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
            self.consume_whitespace();
        }

        self.builder.finish_node();
    }
}

// Extracted from: filesystem/rmdir.rs
impl Parser<'_> {
    /// Parses an `RmDir` statement.
    ///
    /// `RmDir` statement syntax:
    /// ```vb
    /// RmDir path
    /// ```
    ///
    /// - **path**: Required. String expression that identifies the directory or folder to be removed. May include drive.
    pub(crate) fn parse_rmdir_statement(&mut self) {
        self.builder.start_node(SyntaxKind::RmDirStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }

        self.builder.finish_node();
    }
}
