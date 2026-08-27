use super::Parser;
use crate::language::Token;
use crate::parsers::SyntaxKind;

// Extracted from: file_operations/open.rs
impl Parser<'_> {
    // VB6 Open statement syntax:
    // - Open pathname For mode [Access access] [lock] As [#]filenumber [Len=reclength]
    //
    // Enables input/output (I/O) to a file.
    //
    // The Open statement syntax has these parts:
    //
    // | Part       | Description |
    // |------------|-------------|
    // | pathname   | Required. String expression that specifies a file name — may include directory or folder, and drive. |
    // | mode       | Required. Keyword specifying the file mode: Append, Binary, Input, Output, or Random. If unspecified, the file is opened for Random access. |
    // | access     | Optional. Keyword specifying the operations permitted on the open file: Read, Write, or Read Write. |
    // | lock       | Optional. Keyword specifying the operations restricted on the open file by other processes: Shared, Lock Read, Lock Write, and Lock Read Write. |
    // | filenumber | Required. A valid file number in the range 1 to 511, inclusive. Use the FreeFile function to obtain the next available file number. |
    // | reclength  | Optional. Number less than or equal to 32,767 (bytes). For files opened for random access, this value is the record length. For sequential files, this value is the number of characters buffered. |
    //
    // Remarks:
    // - You must open a file before any I/O operation can be performed on it.
    // - If pathname specifies a file that doesn't exist, it is created when a file is opened for Append, Binary, Output, or Random modes.
    // - If the file is already opened by another process and the specified type of access is not allowed, the Open operation fails and an error occurs.
    // - The Len clause is ignored if mode is Binary.
    // - In Binary, Input, and Random modes, you can open a file using a different file number without first closing the file. In Append and Output modes, you must close a file before opening it with a different file number.
    //
    // Examples:
    // ```vb
    // ' Open for input
    // Open "TESTFILE" For Input As #1
    //
    // ' Open for output
    // Open "TESTFILE" For Output As #1
    //
    // ' Open for append
    // Open "TESTFILE" For Append As #1
    //
    // ' Open for binary
    // Open "TESTFILE" For Binary As #1
    //
    // ' Open for random with record length
    // Open "TESTFILE" For Random As #1 Len = 512
    //
    // ' Open with access control
    // Open "TESTFILE" For Input Access Read As #1
    //
    // ' Open with locking
    // Open "TESTFILE" For Binary Lock Read Write As #1
    //
    // ' Open with variable
    // Dim fileNum As Integer
    // fileNum = FreeFile
    // Open fileName For Input As fileNum
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/open-statement)
    pub(crate) fn parse_open_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::OpenStatement.to_raw());

        // Consume the Open keyword
        self.consume_whitespace();
        self.consume_token();

        // Parse pathname expression
        self.consume_whitespace();
        self.parse_expression();

        // Parse "For mode" - consume For keyword, then mode in KeywordClause
        self.consume_whitespace();
        self.consume_token(); // For

        self.consume_whitespace();
        self.try_parse_mode_keyword();

        // Optional: Access [Read|Write|ReadWrite]
        self.consume_whitespace();
        if self.at_token(Token::AccessKeyword) {
            self.parse_access_clause();
        }

        // Optional: Lock [Read|Write|ReadWrite] or Shared
        self.consume_whitespace();
        if self.at_token(Token::LockKeyword) || self.is_shared_keyword() {
            self.parse_lock_clause();
        }

        // Optional: "As [#]filenumber"
        self.consume_whitespace();
        if self.at_token(Token::AsKeyword) {
            self.parse_filenumber_clause();
        }

        // Optional: "Len = expression"
        self.consume_whitespace();
        self.parse_len_clause();

        self.builder.finish_node();
    }

    /// Try to parse a mode keyword (Input, Output, Append, Binary, Random) as `KeywordClause`.
    fn try_parse_mode_keyword(&mut self) {
        let modes = [
            SyntaxKind::InputKeyword,
            SyntaxKind::OutputKeyword,
            SyntaxKind::AppendKeyword,
            SyntaxKind::BinaryKeyword,
            SyntaxKind::RandomKeyword,
        ];
        for &mode in &modes {
            let Some((_text, token)) = self.tokens.get(self.pos) else {
                return;
            };
            if SyntaxKind::from(*token) == mode {
                self.parse_keyword_clause(mode);
                return;
            }
        }
    }

    /// Parse Access clause: Access [Read|Write|ReadWrite]
    fn parse_access_clause(&mut self) {
        self.builder.start_node(SyntaxKind::KeywordClause.to_raw());
        self.consume_token(); // Access
        loop {
            self.consume_whitespace();
            if self.at_token(Token::ReadKeyword) || self.at_token(Token::WriteKeyword) {
                self.consume_token();
            } else {
                break;
            }
        }
        self.builder.finish_node();
    }

    /// Parse Lock clause: Lock [Read|Write|ReadWrite] or Shared
    fn parse_lock_clause(&mut self) {
        self.builder.start_node(SyntaxKind::KeywordClause.to_raw());
        if self.at_token(Token::LockKeyword) {
            self.consume_token(); // Lock
            loop {
                self.consume_whitespace();
                if self.at_token(Token::ReadKeyword) || self.at_token(Token::WriteKeyword) {
                    self.consume_token();
                } else {
                    break;
                }
            }
        } else {
            // Shared
            if let Some((text, _)) = self.tokens.get(self.pos) {
                self.builder.token(SyntaxKind::Identifier.to_raw(), text);
                self.pos += 1;
            }
        }
        self.builder.finish_node();
    }

    /// Parse filenumber clause: As [#]filenumber
    fn parse_filenumber_clause(&mut self) {
        self.builder
            .start_node(SyntaxKind::ExpressionClause.to_raw());
        let (text, token) = (
            self.tokens[self.pos].0,
            SyntaxKind::from(self.tokens[self.pos].1),
        );
        self.builder.token(token.to_raw(), text);
        self.pos += 1;
        self.parse_expression();
        self.builder.finish_node();
    }

    /// Parse Len clause: Len = expression
    fn parse_len_clause(&mut self) {
        if let Some((text, token)) = self.tokens.get(self.pos) {
            let kind = SyntaxKind::from(*token);
            if kind != SyntaxKind::LenKeyword {
                return;
            }
            self.builder
                .start_node(SyntaxKind::ExpressionClause.to_raw());
            self.builder.token(kind.to_raw(), text);
            self.pos += 1;
            self.consume_whitespace();
            if let Some((eq_text, eq_token)) = self.tokens.get(self.pos) {
                let eq_kind = SyntaxKind::from(*eq_token);
                if eq_kind == SyntaxKind::EqualityOperator {
                    self.builder.token(eq_kind.to_raw(), eq_text);
                    self.pos += 1;
                }
            }
            self.consume_whitespace();
            self.parse_expression();
            self.builder.finish_node();
        }
    }

    /// Check if the current token is "Shared" (case-insensitive identifier in Open context)
    fn is_shared_keyword(&self) -> bool {
        self.tokens
            .get(self.pos)
            .is_some_and(|(text, _)| text.to_uppercase() == "SHARED")
    }
}

// Extracted from: file_operations/reset.rs
impl Parser<'_> {
    // VB6 Reset statement syntax:
    // - Reset
    //
    // Closes all disk files opened using the Open statement.
    //
    // The Reset statement closes all active files opened by the Open statement
    // and writes the contents of all file buffers to disk.
    //
    // Use Reset to ensure all file data is written to disk before ending your program.
    // This is particularly important in programs that may terminate abnormally.
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/reset-statement)
    pub(crate) fn parse_reset_statement(&mut self) {
        self.builder.start_node(SyntaxKind::ResetStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/put.rs
impl Parser<'_> {
    /// Parse a Put statement.
    ///
    /// VB6 Put statement syntax:
    /// - Put [#]filenumber, [recnumber], varname
    ///
    /// Writes data from a variable to a disk file.
    ///
    /// The Put statement syntax has these parts:
    ///
    /// | Part          | Description |
    /// |---------------|-------------|
    /// | filenumber    | Required. Any valid file number. |
    /// | recnumber     | Optional. Variant (Long). Record number (Random mode files) or byte number (Binary mode files) at which writing begins. |
    /// | varname       | Required. Valid variable name containing data to be written to disk. |
    ///
    /// Remarks:
    /// - Put is used with files opened in Binary or Random mode.
    /// - For files opened in Random mode, the record length specified in the Open statement determines the number of bytes written.
    /// - For files opened in Binary mode, Put writes any number of bytes.
    /// - The first record or byte in a file is at position 1, the second at position 2, and so on.
    /// - If you omit recnumber, the next record or byte following the last Put or Get statement (or pointed to by the last Seek function) is written.
    /// - You must include delimiting commas, for example: Put #1, , myVariable
    /// - For files opened in Random mode, the following rules apply:
    ///   * If the length of the data being written is less than the length specified in the Len clause, subsequent records on disk are aligned on record-length boundaries.
    ///   * The space between the end of one record and the beginning of the next is padded with the existing file contents.
    ///   * If the variable being written is a variable-length string, Put writes a 2-byte descriptor containing the string length and then writes the string data.
    /// - For files opened in Binary mode, all the Random rules apply, except:
    ///   * The Len clause in the Open statement has no effect.
    ///   * Put writes the data contiguously, with no padding between records.
    /// - Put statements usually mirror Get statements. That is, data written with Put is typically read with Get.
    ///
    /// Examples:
    /// ```vb
    /// Put #1, , myRecord
    /// Put #1, recordNumber, customerData
    /// Put fileNum, , buffer
    /// Put #1, filePosition, userData
    /// ```
    ///
    /// [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/put-statement)
    pub(crate) fn parse_put_statement(&mut self) {
        self.builder.start_node(SyntaxKind::PutStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        self.parse_expression();
        self.consume_whitespace();

        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();

            if !self.at_token(Token::Comma) && !self.is_at_end() {
                self.parse_expression();
                self.consume_whitespace();
            }

            if self.at_token(Token::Comma) {
                self.consume_token();
                self.consume_whitespace();

                self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
            }
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/print.rs
impl Parser<'_> {
    // VB6 Print # statement syntax:
    // - Print #filenumber, [outputlist]
    //
    // Writes display-formatted data to a sequential file.
    //
    // The Print # statement syntax has these parts:
    //
    // | Part        | Description |
    // |-------------|-------------|
    // | filenumber  | Required. Any valid file number. |
    // | outputlist  | Optional. Expression or list of expressions to print. |
    //
    // Remarks:
    // - Data written with Print # is usually read from a file with Line Input # or Input.
    // - If you omit outputlist and include only a list separator after filenumber, a blank line is printed to the file.
    // - Multiple expressions can be separated with either a space or a semicolon.
    // - A space has the same effect as a semicolon.
    // - For Boolean data, either True or False is printed.
    // - The True and False keywords are not translated, regardless of locale.
    // - Date data is written to the file using the standard short date format recognized by your system.
    // - When either the date or the time component is missing or zero, only the part provided gets written to the file.
    // - Nothing is written to the file if outputlist data is Empty. However, if outputlist data is Null, Null is output to the file.
    // - For error data, the output appears as Error errorcode. The Error keyword is not translated, regardless of locale.
    // - All data written to the file using Print # is internationally aware; that is, the data is properly formatted using the appropriate decimal separator and thousands separator.
    // - When data is written to a file, several universal assumptions are followed:
    //   * Numeric data is always written using the period as the decimal separator.
    //   * For numeric data, a leading space is always reserved for the sign of the number.
    //   * A trailing space is included after each number.
    // - Unlike the Print method, the Print # statement doesn't insert commas or spaces between items as they are written to the file.
    // - When you use the Print # statement, you insert explicit delimiters in your output list when you want to add commas or spaces.
    // - The Print # statement usually writes Variant data to a file the same way it writes other data types.
    // - However, there are some exceptions:
    //   * If the data being written is a Variant of VarType vbError, an error message string is not written to the file.
    //   * Only the word Error and the error code are written.
    //   * If the data being written is a Variant of VarType vbEmpty, nothing is written to the file.
    //
    // Examples:
    // ```vb
    // ' Basic usage
    // Print #1, "Hello World"
    //
    // ' Multiple items
    // Print #1, x, y, z
    //
    // ' With semicolon separator
    // Print #1, "Name: "; userName; " Age: "; userAge
    //
    // ' Blank line
    // Print #1,
    //
    // ' Variable file number
    // Dim fileNum As Integer
    // fileNum = FreeFile
    // Print #fileNum, data
    //
    // ' Complex expressions
    // Print #1, Format$(Now, "yyyy-mm-dd"), totalAmount
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/print-statement)
    pub(crate) fn parse_print_statement(&mut self) {
        self.parsing_header = false;

        self.builder.start_node(SyntaxKind::PrintStatement.to_raw());

        // Consume any leading whitespace and the Print keyword.
        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if self.at_token(Token::Octothorpe) {
            // `Print #filenumber, outputlist`: file output. Parse the file
            // number and the output list structurally so the interpreter can
            // evaluate it (it treats every `Print` as console output).
            self.consume_token();
            self.consume_whitespace();
            if !self.at_token(Token::Newline) && !self.at_token(Token::Comma) && !self.is_at_end() {
                self.parse_expression();
                self.consume_whitespace();
            }
            if self.at_token(Token::Comma) {
                self.consume_token();
                self.consume_whitespace();
                self.parse_print_output_list();
            }
            self.consume_whitespace();
            self.consume_until_after(Token::Newline);
        } else if self.at_token(Token::Newline) || self.is_at_end() {
            // Bare `Print` with no output list.
            self.consume_until_after(Token::Newline);
        } else {
            // Bare `Print [outputlist]`: parse the output list as expressions
            // so the interpreter can evaluate them. (Real VB6 requires an
            // object qualifier for `Print` in a standard module; this is a
            // console-output extension shared by the interpreter and compiler.)
            self.parse_print_output_list();
            self.consume_whitespace();
            self.consume_until_after(Token::Newline);
        }

        self.builder.finish_node();
    }

    /// Parse the output list of a bare `Print` statement, mirroring
    /// `parse_unparenthesized_arguments` with semicolon separators enabled.
    fn parse_print_output_list(&mut self) {
        self.builder.start_node(SyntaxKind::ArgumentList.to_raw());

        loop {
            if self.at_token(Token::Newline) || self.is_at_end() {
                break;
            }

            self.builder.start_node(SyntaxKind::Argument.to_raw());

            // Empty arguments (an immediate separator) print nothing.
            if !self.at_token(Token::Comma) && !self.at_token(Token::Semicolon) {
                self.parse_expression();
            }

            self.builder.finish_node();

            self.consume_whitespace();

            if self.at_token(Token::Comma) || self.at_token(Token::Semicolon) {
                self.consume_token();
                self.consume_whitespace();
            } else {
                break;
            }
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/lock.rs
impl Parser<'_> {
    // VB6 Lock statement syntax:
    // - Lock [#]filenumber[, recordrange]
    //
    // Controls access to all or part of an open file.
    //
    // The Lock statement syntax has these parts:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | filenumber    | Required. Any valid file number. |
    // | recordrange   | Optional. Range of records to lock. Can be: record, start To end, or omitted for entire file. |
    //
    // Remarks:
    // - Lock and Unlock are used in environments where multiple processes might need access to the same file.
    // - Lock and Unlock statements are always used in pairs.
    // - The Lock statement locks all or part of a file opened using the Open statement.
    // - The first record or byte in a file is at position 1, the second at position 2, and so on.
    // - If you specify just one record number, only that record is locked.
    // - If you specify a range, all records in that range are locked.
    // - For files opened in Binary, Input, or Output mode, Lock always locks the entire file,
    //   regardless of the recordrange argument.
    // - For files opened in Random mode, Lock locks the specified record or range of records.
    // - Locked portions of a file can't be accessed by other processes until unlocked with Unlock.
    // - Use Unlock to remove the lock from a portion of a file.
    //
    // Examples:
    // ```vb
    // Lock #1
    // Lock #1, 5
    // Lock #1, 10 To 20
    // Lock fileNum, recordNum
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/lock-statement)
    pub(crate) fn parse_lock_statement(&mut self) {
        self.builder.start_node(SyntaxKind::LockStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse filenumber expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse optional comma and recordrange
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();

            if !self.is_at_end() && !self.at_token(Token::Newline) {
                // Parse start expression
                self.parse_expression();
                self.consume_whitespace();

                // Optional To ... end range
                if self.at_token(Token::ToKeyword) {
                    self.consume_token();
                    self.consume_whitespace();

                    // Parse end expression
                    self.parse_expression();
                    self.consume_whitespace();
                }
            }
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/unlock.rs
impl Parser<'_> {
    // VB6 Unlock statement syntax:
    // - Unlock [#]filenumber[, recordrange]
    //
    // Removes access restrictions on all or part of an open file.
    //
    // The Unlock statement syntax has these parts:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | filenumber    | Required. Any valid file number. |
    // | recordrange   | Optional. Range of records to unlock. Can be: record, start To end, or omitted for entire file. |
    //
    // Remarks:
    // - Unlock is used to remove locks placed on a file with the Lock statement.
    // - The Unlock statement allows other processes to access the unlocked portions of the file.
    // - The arguments to Unlock must exactly match those used with the corresponding Lock statement.
    // - The first record or byte in a file is at position 1, the second at position 2, and so on.
    // - If you specify just one record number, only that record is unlocked.
    // - If you specify a range, all records in that range are unlocked.
    // - For files opened in Binary, Input, or Output mode, Unlock always unlocks the entire file,
    //   regardless of the recordrange argument.
    // - For files opened in Random mode, Unlock unlocks the specified record or range of records.
    // - Each Lock statement must have a corresponding Unlock statement with the same file number
    //   and record range.
    //
    // Examples:
    // ```vb
    // Unlock #1
    // Unlock #1, 5
    // Unlock #1, 10 To 20
    // Unlock fileNum, recordNum
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/unlock-statement)
    pub(crate) fn parse_unlock_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::UnlockStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse filenumber expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse optional comma and recordrange
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();

            if !self.is_at_end() && !self.at_token(Token::Newline) {
                // Parse start expression
                self.parse_expression();
                self.consume_whitespace();

                // Optional To ... end range
                if self.at_token(Token::ToKeyword) {
                    self.consume_token();
                    self.consume_whitespace();

                    // Parse end expression
                    self.parse_expression();
                    self.consume_whitespace();
                }
            }
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/name.rs
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

// Extracted from: file_operations/close.rs
impl Parser<'_> {
    // VB6 Close statement syntax:
    // - Close [filenumberlist]
    //
    // Closes input or output files opened using the Open statement.
    //
    // filenumberlist: Optional. One or more file numbers using the syntax:
    // [[#]filenumber] [, [#]filenumber] ...
    //
    // If filenumberlist is omitted, all active files opened by the Open statement are closed.
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/close-statement)
    pub(crate) fn parse_close_statement(&mut self) {
        self.builder.start_node(SyntaxKind::CloseStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        if !self.at_token(Token::Newline) && !self.is_at_end() {
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/line_input.rs
impl Parser<'_> {
    // VB6 Line Input statement syntax:
    // - Line Input #filenumber, varname
    //
    // Reads a single line from an open sequential file and assigns it to a String variable.
    //
    // The Line Input # statement syntax has these parts:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | filenumber    | Required. Any valid file number. |
    // | varname       | Required. Valid String or Variant variable name. |
    //
    // Remarks:
    // - Data read with Line Input # is usually written to a file with Print #.
    // - The Line Input # statement reads from a file one character at a time until it encounters
    //   a carriage return (Chr(13)) or carriage return–linefeed (Chr(13) + Chr(10)) sequence.
    // - Carriage return–linefeed sequences are skipped rather than appended to the character string.
    // - Line Input # is useful for reading text files that have been created in a text editor or
    //   with the Print # statement.
    // - Unlike Input #, Line Input # doesn't parse the data as it's read – you get the entire line as-is.
    // - If end of file is reached before reading a complete line, an error occurs.
    //
    // Examples:
    // ```vb
    // Line Input #1, textLine
    // Line Input #fileNum, dataBuffer
    // Line Input #1, myString
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/line-input-statement)
    pub(crate) fn parse_line_input_statement(&mut self) {
        // if we are now parsing a Line Input statement, we are no longer in the header.
        self.parsing_header = false;

        self.builder
            .start_node(SyntaxKind::LineInputStatement.to_raw());

        // Consume "Line" keyword
        self.consume_token();

        // Consume "Input" keyword (should be next)
        if self.at_token(Token::InputKeyword) {
            self.consume_token();
        }

        // Consume everything until newline
        // This includes: "#", filenumber, ",", varname
        while !self.is_at_end() && !self.at_token(Token::Newline) {
            self.consume_token();
        }

        // Consume the newline
        if self.at_token(Token::Newline) {
            self.consume_token();
        }

        self.builder.finish_node(); // LineInputStatement
    }
}

// Extracted from: file_operations/width.rs
impl Parser<'_> {
    /// Parse a Width # statement.
    ///
    /// The Width # statement assigns an output line width to a file opened using the Open statement.
    ///
    /// Syntax:
    /// ```vb
    /// Width #filenumber, width
    /// ```
    ///
    /// Example:
    /// ```vb
    /// Width #1, 80
    /// ```
    pub(crate) fn parse_width_statement(&mut self) {
        self.builder.start_node(SyntaxKind::WidthStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse filenumber expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse comma
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();
        }

        // Parse width expression
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/file_copy.rs
impl Parser<'_> {
    // VB6 FileCopy statement syntax:
    // - FileCopy source, destination
    //
    // Copies a file.
    //
    // The FileCopy statement syntax has these named arguments:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | source        | Required. String expression that specifies a file name. May include directory or folder, and drive. |
    // | destination   | Required. String expression that specifies a file name. May include directory or folder, and drive. |
    //
    // Remarks:
    // - If you try to use the FileCopy statement on a currently open file, an error occurs.
    // - FileCopy can copy files between directories/folders and between drives.
    // - Both source and destination can include path information (drive and directory/folder).
    // - If destination specifies a directory/folder that doesn't exist, FileCopy creates it.
    //
    // Examples:
    // ```vb
    // FileCopy "C:\SOURCE.TXT", "C:\DEST.TXT"
    // FileCopy oldFile, newFile
    // FileCopy App.Path & "\data.dat", "C:\Backup\data.dat"
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/filecopy-statement)
    pub(crate) fn parse_file_copy_statement(&mut self) {
        self.builder
            .start_node(SyntaxKind::FileCopyStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse source expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse comma
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();
        }

        // Parse destination expression
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
            self.consume_whitespace();
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/seek.rs
impl Parser<'_> {
    /// Parses a Seek statement.
    ///
    /// Seek statement syntax:
    /// ```vb
    /// Seek [#]filenumber, position
    /// ```
    ///
    /// - **filenumber**: Required. Any valid file number. The number sign (#) is optional.
    /// - **position**: Required. Number indicating where the next read or write should occur.
    pub(crate) fn parse_seek_statement(&mut self) {
        self.builder.start_node(SyntaxKind::SeekStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        // Parse filenumber expression
        self.parse_expression();
        self.consume_whitespace();

        // Parse comma
        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();
        }

        // Parse position expression
        if !self.is_at_end() && !self.at_token(Token::Newline) {
            self.parse_expression();
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/kill.rs
impl Parser<'_> {
    // VB6 Kill statement syntax:
    // - Kill pathname
    //
    // Deletes files from a disk.
    //
    // The Kill statement syntax has this part:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | pathname      | Required. String expression that specifies one or more file names to be deleted. May include directory or folder, and drive. |
    //
    // Remarks:
    // - Kill supports the use of multiple-character (*) and single-character (?) wildcards to specify multiple files.
    // - An error occurs if you try to use Kill to delete an open file.
    // - To remove a directory or folder, use the RmDir statement.
    //
    // Examples:
    // ```vb
    // Kill "C:\DATA.TXT"
    // Kill "C:\*.TXT"           ' Delete all .txt files
    // Kill "C:\TEST?.TXT"       ' Delete TEST1.TXT, TESTA.TXT, etc.
    // Kill App.Path & "\temp.dat"
    // Kill myFileName
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/kill-statement)
    pub(crate) fn parse_kill_statement(&mut self) {
        self.builder.start_node(SyntaxKind::KillStatement.to_raw());

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

// Extracted from: file_operations/write.rs
impl Parser<'_> {
    /// Parse a Write # statement.
    ///
    /// The Write # statement writes data to a sequential file with automatic
    /// formatting: commas between items and quotation marks around strings.
    ///
    /// Syntax:
    /// ```vb
    /// Write #filenumber, [outputlist]
    /// ```
    ///
    /// Example:
    /// ```vb
    /// Write #1, "Hello", 42, True
    /// ```
    pub(crate) fn parse_write_statement(&mut self) {
        self.builder.start_node(SyntaxKind::WriteStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        self.parse_expression();
        self.consume_whitespace();

        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/input.rs
impl Parser<'_> {
    // VB6 Input statement syntax:
    // - Input #filenumber, varlist
    //
    // Reads data from an open sequential file and assigns the data to variables.
    //
    // The Input # statement syntax has these parts:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | filenumber    | Required. Any valid file number. |
    // | varlist       | Required. Comma-delimited list of variables that are assigned values read from the file. Variables can't be arrays or object variables. However, variables that describe an element of an array or user-defined type may be used. |
    //
    // Remarks:
    // - Data read with Input # is usually written to a file with Write #.
    // - Use this statement only with files opened in Input or Binary mode.
    // - The Input # statement reads data items from a sequential file and assigns them to variables.
    // - Data items in the file must appear in the same order as the variables in varlist and be separated by commas.
    // - If the data item to be read is a quoted string, Input # strips the quotation marks.
    // - Input # is typically used to read data that was written to a file using the Write # statement.
    // - For files opened for Binary access, Input # reads all the bytes it needs to complete the varlist.
    // - If end of file is reached before all variables are filled, an error occurs.
    //
    // Examples:
    // ```vb
    // Input #1, name, age
    // Input #fileNum, x, y, z
    // Input #1, firstName, lastName, address
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/input-statement)
    pub(crate) fn parse_input_statement(&mut self) {
        self.builder.start_node(SyntaxKind::InputStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        self.parse_expression();
        self.consume_whitespace();

        if !self.is_at_end() && self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();
            self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
        }

        self.builder.finish_node();
    }
}

// Extracted from: file_operations/get.rs
impl Parser<'_> {
    // VB6 Get statement syntax:
    // - Get [#]filenumber, [recnumber], varname
    //
    // Reads data from an open disk file into a variable.
    //
    // The Get statement syntax has these parts:
    //
    // | Part          | Description |
    // |---------------|-------------|
    // | filenumber    | Required. Any valid file number. |
    // | recnumber     | Optional. Variant (Long). Record number (Random mode files) or byte number (Binary mode files) at which reading begins. |
    // | varname       | Required. Valid variable name into which data is read. |
    //
    // Remarks:
    // - Get is used with files opened in Binary or Random mode.
    // - For files opened in Random mode, the record length specified in the Open statement determines the number of bytes read.
    // - For files opened in Binary mode, Get reads any number of bytes.
    // - The first record or byte in a file is at position 1, the second at position 2, and so on.
    // - If you omit recnumber, the next record or byte following the last Get or Put statement (or pointed to by the last Seek function) is read.
    // - You must include delimiting commas, for example: Get #1, , myVariable
    // - For files opened in Random mode, the following rules apply:
    //   * If the length of the data being read is less than the length specified in the Len clause, subsequent records on disk are aligned on record-length boundaries.
    //   * The space between the end of one record and the beginning of the next is padded with existing file contents.
    //   * If the variable being read is a variable-length string, Get reads a 2-byte descriptor containing the string length and then reads the string data.
    // - For files opened in Binary mode, all the Random rules apply, except:
    //   * The Len clause in the Open statement has no effect.
    //   * Get reads the data contiguously, with no padding between records.
    //
    // Examples:
    // ```vb
    // Get #1, , myRecord
    // Get #1, recordNumber, customerData
    // Get fileNum, , buffer
    // ```
    //
    // [Reference](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/get-statement)
    pub(crate) fn parse_get_statement(&mut self) {
        self.builder.start_node(SyntaxKind::GetStatement.to_raw());

        self.consume_whitespace();
        self.consume_token();
        self.consume_whitespace();

        self.parse_expression();
        self.consume_whitespace();

        if self.at_token(Token::Comma) {
            self.consume_token();
            self.consume_whitespace();

            if !self.at_token(Token::Comma) && !self.is_at_end() {
                self.parse_expression();
                self.consume_whitespace();
            }

            if self.at_token(Token::Comma) {
                self.consume_token();
                self.consume_whitespace();

                self.parse_separated_expressions(Token::Comma, SyntaxKind::ArgumentList);
            }
        }

        self.builder.finish_node();
    }
}
