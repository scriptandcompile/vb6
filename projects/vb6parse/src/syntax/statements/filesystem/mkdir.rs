//! # `MkDir` Statement
//!
//! Creates a new directory or folder.
//!
//! ## Syntax
//!
//! ```vb
//! MkDir path
//! ```
//!
//! - `path`: Required. String expression that identifies the directory or folder to be created. May include drive.
//!   If no drive is specified, `MkDir` creates the new directory or folder on the current drive.
//!
//! ## Remarks
//!
//! - An error occurs if you try to create a directory or folder that already exists
//! - The `path` argument can include absolute or relative paths
//! - You can use `MkDir` to create nested directories by creating parent directories first
//! - On Windows systems, both forward slashes (/) and backslashes (\) can be used as path separators
//! - The directory name can include the drive letter
//! - UNC paths are supported on network drives
//!
//! ## Examples
//!
//! ```vb
//! ' Create a directory in the current directory
//! MkDir "MyNewFolder"
//!
//! ' Create a directory with full path
//! MkDir "C:\Program Files\MyApp"
//!
//! ' Create a directory on another drive
//! MkDir "D:\Data\Reports"
//!
//! ' Create nested directories (parent must exist first)
//! MkDir "C:\Temp"
//! MkDir "C:\Temp\Logs"
//! MkDir "C:\Temp\Logs\Archive"
//!
//! ' Create directory on network drive
//! MkDir "\\Server\Share\NewFolder"
//! ```
//!
//! ## Reference
//!
//! [MkDir Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/mkdir-statement)

use crate::parsers::SyntaxKind;

use crate::Token;
use crate::parsers::cst::Parser;

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
