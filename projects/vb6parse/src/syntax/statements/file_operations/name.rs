//! # Name Statement
//!
//! Renames a disk file, directory, or folder.
//!
//! ## Syntax
//!
//! ```vb
//! Name oldpathname As newpathname
//! ```
//!
//! - `oldpathname`: Required. String expression that specifies the existing file name and location. May include directory or folder, and drive.
//! - `newpathname`: Required. String expression that specifies the new file name and location. May include directory or folder, and drive.
//!   Cannot specify a different drive from the one specified in `oldpathname`.
//!
//! ## Remarks
//!
//! - The `Name` statement renames a file and moves it to a different directory or folder, if necessary
//! - `Name` can move a file across directories or folders, but both `oldpathname` and `newpathname` must be on the same drive
//! - Using `Name` on an open file produces an error. You must close an open file before renaming it
//! - `Name` arguments can include relative or absolute paths
//! - The `Name` statement can also rename directories or folders
//! - If `newpathname` already exists, an error occurs
//! - Wildcard characters (* and ?) are not allowed in either `oldpathname` or `newpathname`
//!
//! ## Examples
//!
//! ```vb
//! ' Rename a file
//! Name "OLDFILE.TXT" As "NEWFILE.TXT"
//!
//! ' Move and rename a file
//! Name "C:\Data\Report.doc" As "C:\Archive\OldReport.doc"
//!
//! ' Rename a directory
//! Name "C:\OldFolder" As "C:\NewFolder"
//!
//! ' Move file to different directory (same drive)
//! Name "C:\Temp\Test.dat" As "C:\Data\Test.dat"
//!
//! ' Using variables
//! Dim oldName As String, newName As String
//! oldName = "File1.txt"
//! newName = "File2.txt"
//! Name oldName As newName
//! ```
//!
//! ## Reference
//!
//! [Name Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/name-statement)

use crate::Token;
use crate::parsers::cst::Parser;
use crate::parsers::syntaxkind::SyntaxKind;

impl Parser<'_> {
    // VB6 Name statement syntax:
    // - Name oldpathname As newpathname
    //
    // Renames a disk file, directory, or folder.
    //
    // The Name statement syntax has these named arguments:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | oldpathname   | Required. String expression that specifies the existing file name and location. May include directory or folder, and drive. |
    // | newpathname   | Required. String expression that specifies the new file name and location. May include directory or folder, and drive. Cannot specify a different drive from the one specified in oldpathname. |
    //
    // Remarks:
    // - The Name statement renames a file and moves it to a different directory or folder, if necessary
    // - Name can move a file across directories or folders, but both oldpathname and newpathname must be on the same drive
    // - Using Name on an open file produces an error. You must close an open file before renaming it
    // - Name arguments can include relative or absolute paths
    // - The Name statement can also rename directories or folders
    // - If newpathname already exists, an error occurs
    // - Wildcard characters (* and ?) are not allowed in either oldpathname or newpathname
    //
    // Examples:
    // ```vb
    // Name "OLDFILE.TXT" As "NEWFILE.TXT"
    // Name "C:\Data\Report.doc" As "C:\Archive\OldReport.doc"
    // Name oldName As newName
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/name-statement)
    pub(crate) fn parse_name_statement(&mut self) {
        self.builder.start_node(SyntaxKind::NameStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse oldpath expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse 'As' keyword
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.consume_token();
            self.consume_whitespace();

            // Parse newpath expression
            if !self.is_at_end() && !self.at_token(Token::Newline) {
                self.parse_expression();
                self.consume_whitespace();
            }
        }

        self.builder.finish_node();
    }
}
