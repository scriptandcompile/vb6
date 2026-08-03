//! # Seek Statement
//!
//! Sets the position for the next read or write operation in a file opened using the Open statement.
//!
//! ## Syntax
//!
//! ```vb
//! Seek [#]filenumber, position
//! ```
//!
//! ## Parts
//!
//! - **filenumber**: Required. Any valid file number. The number sign (#) is optional but commonly included for clarity.
//! - **position**: Required. Number in the range 1 to 2,147,483,647 (equivalent to 2^31 - 1), indicating where the next read or write should occur.
//!
//! ## Remarks
//!
//! - **File Position**: The Seek statement sets the byte position in a file where the next Input, Output, Get, or Put operation will occur.
//! - **Position Numbering**: File positions are numbered beginning with 1 (the first byte in the file is at position 1, not 0).
//! - **Random Access Files**: For Random mode files, the position parameter specifies a record number rather than a byte position.
//! - **Sequential Files**: For files opened in Input, Output, or Append mode, position specifies the byte position.
//! - **Binary Files**: For Binary mode files, position specifies the byte position.
//! - **Seek Function**: Use the Seek function (without arguments except file number) to return the current file position.
//! - **EOF Behavior**: Setting the position beyond the end of the file doesn't immediately extend the file, but writing to that position will.
//! - **Position Range**: The position must be a positive Long value (1 to 2,147,483,647).
//! - **File Number**: The file must be opened before using Seek.
//!
//! ## Position Interpretation by File Mode
//!
//! | File Mode | Position Represents |
//! |-----------|-------------------|
//! | Random    | Record number (1-based) |
//! | Binary    | Byte position (1-based) |
//! | Input     | Byte position (1-based) |
//! | Output    | Byte position (1-based) |
//! | Append    | Byte position (1-based) |
//!
//! ## Examples
//!
//! ### Seek to Beginning of File
//!
//! ```vb
//! Open "DATA.TXT" For Binary As #1
//! Seek #1, 1   ' Position at first byte
//! ' Read or write operations
//! Close #1
//! ```
//!
//! ### Seek to Specific Byte Position
//!
//! ```vb
//! Open "BINARY.DAT" For Binary As #1
//! Seek #1, 100   ' Position at byte 100
//! Get #1, , myData
//! Close #1
//! ```
//!
//! ### Seek to Specific Record in Random File
//!
//! ```vb
//! Type Employee
//!     ID As Integer
//!     Name As String * 30
//! End Type
//!
//! Dim emp As Employee
//! Open "EMPLOYEE.DAT" For Random As #1 Len = Len(emp)
//! Seek #1, 5   ' Position at record 5
//! Get #1, , emp
//! Close #1
//! ```
//!
//! ### Seek Based on Calculation
//!
//! ```vb
//! Dim recordNumber As Long
//! recordNumber = 10
//! Seek #1, recordNumber
//! ```
//!
//! ### Using Seek with Loop
//!
//! ```vb
//! Open "DATA.BIN" For Binary As #1
//! For i = 1 To 100 Step 10
//!     Seek #1, i
//!     Put #1, , dataArray(i)
//! Next i
//! Close #1
//! ```
//!
//! ### Seek to End of File
//!
//! ```vb
//! Open "APPEND.TXT" For Binary As #1
//! Seek #1, LOF(1) + 1   ' Position after last byte
//! Put #1, , newData
//! Close #1
//! ```
//!
//! ### Combined with Seek Function
//!
//! ```vb
//! Open "DATA.TXT" For Binary As #1
//! currentPos = Seek(1)      ' Get current position
//! Seek #1, currentPos + 50  ' Move 50 bytes forward
//! Close #1
//! ```
//!
//! ### Rewind File
//!
//! ```vb
//! Sub RewindFile(fileNum As Integer)
//!     Seek fileNum, 1  ' Return to beginning
//! End Sub
//! ```
//!
//! ### Seek in Random Access Processing
//!
//! ```vb
//! Type Product
//!     Code As String * 10
//!     Price As Double
//! End Type
//!
//! Dim prod As Product
//! Dim recordNum As Long
//!
//! Open "PRODUCTS.DAT" For Random As #1 Len = Len(prod)
//! recordNum = 25
//! Seek #1, recordNum
//! Get #1, , prod
//! prod.Price = prod.Price * 1.1  ' Increase price by 10%
//! Seek #1, recordNum
//! Put #1, , prod
//! Close #1
//! ```
//!
//! ## Common Errors
//!
//! - **Error 52**: Bad file name or number - file not open or invalid file number
//! - **Error 63**: Bad record number - position is less than 1 or exceeds valid range
//! - **Error 5**: Invalid procedure call - negative or zero position value
//!
//! ## Performance Tips
//!
//! - For sequential reading/writing, you generally don't need Seek as the file pointer advances automatically.
//! - Use Seek when you need random access to specific parts of a file.
//! - Combining Seek with the Seek function allows you to save and restore file positions.
//! - For large files, seeking to specific positions is much faster than reading sequentially.
//!
//! ## See Also
//!
//! - `Seek` function (returns current file position)
//! - `Get` statement (read data from file)
//! - `Put` statement (write data to file)
//! - `Open` statement (open files)
//! - `LOF` function (length of file)
//! - `Loc` function (current position in file)
//!
//! ## References
//!
//! - [Seek Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/seek-statement)
