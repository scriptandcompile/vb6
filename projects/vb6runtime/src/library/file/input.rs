//! # `Input` Function
//!
//! Returns a `String` containing characters from a file opened in `Input` or `Binary` mode.
//!
//! ## Syntax
//!
//! ```vb
//! Input(number, [#]filenumber)
//! ```
//!
//! ## Parameters
//!
//! - `number` (Required): `Long` expression specifying the number of characters to return
//! - `filenumber` (Required): `Integer` file number used in the `Open` statement (the # is optional)
//!
//! ## Return Value
//!
//! Returns a `String` containing `number` characters read from the file. If fewer than `number`
//! characters remain in the file, returns all remaining characters.
//!
//! ## Remarks
//!
//! The `Input` function reads data from files:
//!
//! - Used with files opened in `Input` or `Binary` mode
//! - Returns exactly the number of characters requested (or fewer if end of file reached)
//! - Does not skip or ignore any characters (unlike `Input #` statement)
//! - Reads all characters including commas, quotes, line feeds, carriage returns, etc.
//! - The file pointer advances by the number of characters read
//! - Use `EOF` function to check for end of file before reading
//! - For `Binary` mode files, reads raw bytes
//! - For `Input` mode files, reads text characters
//! - Cannot be used with files opened in `Output` or `Append` mode
//! - The # symbol before filenumber is optional but commonly used
//!
//! ## Typical Uses
//!
//! 1. **Binary File Reading**: Read fixed-size chunks from binary files
//! 2. **Text File Reading**: Read specific number of characters from text files
//! 3. **Fixed-Width Records**: Read fixed-width record data
//! 4. **File Parsing**: Read file content for custom parsing
//! 5. **Header Reading**: Read file headers of known size
//! 6. **Buffer Reading**: Read file content into memory buffers
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Read 10 characters from a file
//! Dim fileNum As Integer
//! Dim content As String
//! fileNum = FreeFile
//! Open "data.txt" For Input As #fileNum
//! content = Input(10, #fileNum)
//! Close #fileNum
//!
//! ' Example 2: Read entire file
//! Dim fileNum As Integer
//! Dim fileContent As String
//! Dim fileSize As Long
//! fileNum = FreeFile
//! Open "document.txt" For Input As #fileNum
//! fileSize = LOF(fileNum)
//! fileContent = Input(fileSize, #fileNum)
//! Close #fileNum
//!
//! ' Example 3: Read file in chunks
//! Dim fileNum As Integer
//! Dim chunk As String
//! fileNum = FreeFile
//! Open "data.bin" For Binary As #fileNum
//! Do While Not EOF(fileNum)
//!     chunk = Input(1024, #fileNum)
//!     ProcessChunk chunk
//! Loop
//! Close #fileNum
//!
//! ' Example 4: Read fixed-width record
//! Dim fileNum As Integer
//! Dim record As String
//! fileNum = FreeFile
//! Open "records.dat" For Binary As #fileNum
//! record = Input(80, #fileNum)  ' Read 80-character record
//! Close #fileNum
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Read entire file into string
//! Function ReadFileToString(fileName As String) As String
//!     Dim fileNum As Integer
//!     Dim fileSize As Long
//!     
//!     fileNum = FreeFile
//!     Open fileName For Input As #fileNum
//!     fileSize = LOF(fileNum)
//!     
//!     If fileSize > 0 Then
//!         ReadFileToString = Input(fileSize, #fileNum)
//!     Else
//!         ReadFileToString = ""
//!     End If
//!     
//!     Close #fileNum
//! End Function
//!
//! ' Pattern 2: Read file in fixed-size chunks
//! Sub ReadFileInChunks(fileName As String, chunkSize As Long)
//!     Dim fileNum As Integer
//!     Dim chunk As String
//!     
//!     fileNum = FreeFile
//!     Open fileName For Binary As #fileNum
//!     
//!     Do While Not EOF(fileNum)
//!         chunk = Input(chunkSize, #fileNum)
//!         Debug.Print "Read " & Len(chunk) & " bytes"
//!     Loop
//!     
//!     Close #fileNum
//! End Sub
//!
//! ' Pattern 3: Read file header
//! Function ReadFileHeader(fileName As String, headerSize As Long) As String
//!     Dim fileNum As Integer
//!     
//!     fileNum = FreeFile
//!     Open fileName For Binary As #fileNum
//!     
//!     If LOF(fileNum) >= headerSize Then
//!         ReadFileHeader = Input(headerSize, #fileNum)
//!     Else
//!         ReadFileHeader = ""
//!     End If
//!     
//!     Close #fileNum
//! End Function
//!
//! ' Pattern 4: Read until delimiter found
//! Function ReadUntilDelimiter(fileNum As Integer, delimiter As String) As String
//!     Dim result As String
//!     Dim char As String
//!     
//!     result = ""
//!     Do While Not EOF(fileNum)
//!         char = Input(1, #fileNum)
//!         If char = delimiter Then
//!             Exit Do
//!         End If
//!         result = result & char
//!     Loop
//!     
//!     ReadUntilDelimiter = result
//! End Function
//!
//! ' Pattern 5: Peek at file content without closing
//! Function PeekFileContent(fileNum As Integer, bytes As Long) As String
//!     Dim currentPos As Long
//!     Dim content As String
//!     
//!     currentPos = Seek(fileNum)
//!     content = Input(bytes, #fileNum)
//!     Seek fileNum, currentPos  ' Restore position
//!     
//!     PeekFileContent = content
//! End Function
//!
//! ' Pattern 6: Read line character by character
//! Function ReadCustomLine(fileNum As Integer) As String
//!     Dim result As String
//!     Dim char As String
//!     
//!     result = ""
//!     Do While Not EOF(fileNum)
//!         char = Input(1, #fileNum)
//!         If char = vbCr Or char = vbLf Then
//!             ' Skip additional line feed if CRLF
//!             If char = vbCr And Not EOF(fileNum) Then
//!                 If Input(1, #fileNum) <> vbLf Then
//!                     Seek fileNum, Seek(fileNum) - 1
//!                 End If
//!             End If
//!             Exit Do
//!         End If
//!         result = result & char
//!     Loop
//!     
//!     ReadCustomLine = result
//! End Function
//!
//! ' Pattern 7: Read binary structure
//! Function ReadBinaryStruct(fileNum As Integer, structSize As Long) As Byte()
//!     Dim data As String
//!     Dim bytes() As Byte
//!     Dim i As Long
//!     
//!     data = Input(structSize, #fileNum)
//!     ReDim bytes(0 To Len(data) - 1)
//!     
//!     For i = 0 To Len(data) - 1
//!         bytes(i) = Asc(Mid$(data, i + 1, 1))
//!     Next i
//!     
//!     ReadBinaryStruct = bytes
//! End Function
//!
//! ' Pattern 8: Safe read with EOF check
//! Function SafeRead(fileNum As Integer, numChars As Long) As String
//!     Dim available As Long
//!     Dim toRead As Long
//!     
//!     available = LOF(fileNum) - Seek(fileNum) + 1
//!     toRead = IIf(numChars < available, numChars, available)
//!     
//!     If toRead > 0 Then
//!         SafeRead = Input(toRead, #fileNum)
//!     Else
//!         SafeRead = ""
//!     End If
//! End Function
//!
//! ' Pattern 9: Read with progress tracking
//! Function ReadFileWithProgress(fileName As String) As String
//!     Dim fileNum As Integer
//!     Dim fileSize As Long
//!     Dim bytesRead As Long
//!     Dim chunk As String
//!     Dim result As String
//!     Const CHUNK_SIZE As Long = 4096
//!     
//!     fileNum = FreeFile
//!     Open fileName For Binary As #fileNum
//!     fileSize = LOF(fileNum)
//!     result = ""
//!     bytesRead = 0
//!     
//!     Do While Not EOF(fileNum)
//!         chunk = Input(CHUNK_SIZE, #fileNum)
//!         result = result & chunk
//!         bytesRead = bytesRead + Len(chunk)
//!         
//!         ' Update progress (0 to 100)
//!         DoEvents
//!         Debug.Print "Progress: " & (bytesRead * 100 / fileSize) & "%"
//!     Loop
//!     
//!     Close #fileNum
//!     ReadFileWithProgress = result
//! End Function
//!
//! ' Pattern 10: Read specific byte range
//! Function ReadByteRange(fileName As String, startPos As Long, numBytes As Long) As String
//!     Dim fileNum As Integer
//!     
//!     fileNum = FreeFile
//!     Open fileName For Binary As #fileNum
//!     
//!     Seek fileNum, startPos
//!     ReadByteRange = Input(numBytes, #fileNum)
//!     
//!     Close #fileNum
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Binary file reader class
//! Public Class BinaryFileReader
//!     Private m_fileNum As Integer
//!     Private m_fileName As String
//!     Private m_isOpen As Boolean
//!     
//!     Public Sub OpenFile(fileName As String)
//!         If m_isOpen Then CloseFile
//!         
//!         m_fileName = fileName
//!         m_fileNum = FreeFile
//!         Open m_fileName For Binary As #m_fileNum
//!         m_isOpen = True
//!     End Sub
//!     
//!     Public Function ReadBytes(numBytes As Long) As String
//!         If Not m_isOpen Then
//!             Err.Raise 5, , "File not open"
//!         End If
//!         
//!         If EOF(m_fileNum) Then
//!             ReadBytes = ""
//!         Else
//!             ReadBytes = Input(numBytes, #m_fileNum)
//!         End If
//!     End Function
//!     
//!     Public Function ReadAll() As String
//!         If Not m_isOpen Then
//!             Err.Raise 5, , "File not open"
//!         End If
//!         
//!         Dim fileSize As Long
//!         fileSize = LOF(m_fileNum) - Seek(m_fileNum) + 1
//!         
//!         If fileSize > 0 Then
//!             ReadAll = Input(fileSize, #m_fileNum)
//!         Else
//!             ReadAll = ""
//!         End If
//!     End Function
//!     
//!     Public Sub CloseFile()
//!         If m_isOpen Then
//!             Close #m_fileNum
//!             m_isOpen = False
//!         End If
//!     End Sub
//!     
//!     Private Sub Class_Terminate()
//!         CloseFile
//!     End Sub
//! End Class
//!
//! ' Example 2: Custom file parser
//! Function ParseFixedWidthFile(fileName As String) As Collection
//!     Dim fileNum As Integer
//!     Dim records As New Collection
//!     Dim recordData As String
//!     Const RECORD_SIZE As Long = 100
//!     
//!     fileNum = FreeFile
//!     Open fileName For Binary As #fileNum
//!     
//!     Do While Not EOF(fileNum)
//!         recordData = Input(RECORD_SIZE, #fileNum)
//!         If Len(recordData) = RECORD_SIZE Then
//!             records.Add ParseRecord(recordData)
//!         End If
//!     Loop
//!     
//!     Close #fileNum
//!     Set ParseFixedWidthFile = records
//! End Function
//!
//! ' Example 3: File comparison utility
//! Function CompareFiles(file1 As String, file2 As String) As Boolean
//!     Dim fileNum1 As Integer, fileNum2 As Integer
//!     Dim chunk1 As String, chunk2 As String
//!     Const CHUNK_SIZE As Long = 8192
//!     
//!     fileNum1 = FreeFile
//!     Open file1 For Binary As #fileNum1
//!     
//!     fileNum2 = FreeFile
//!     Open file2 For Binary As #fileNum2
//!     
//!     ' Check file sizes
//!     If LOF(fileNum1) <> LOF(fileNum2) Then
//!         Close #fileNum1
//!         Close #fileNum2
//!         CompareFiles = False
//!         Exit Function
//!     End If
//!     
//!     ' Compare content
//!     Do While Not EOF(fileNum1)
//!         chunk1 = Input(CHUNK_SIZE, #fileNum1)
//!         chunk2 = Input(CHUNK_SIZE, #fileNum2)
//!         
//!         If chunk1 <> chunk2 Then
//!             Close #fileNum1
//!             Close #fileNum2
//!             CompareFiles = False
//!             Exit Function
//!         End If
//!     Loop
//!     
//!     Close #fileNum1
//!     Close #fileNum2
//!     CompareFiles = True
//! End Function
//!
//! ' Example 4: Large file reader with buffering
//! Function ReadLargeFile(fileName As String, Optional bufferSize As Long = 32768) As String
//!     Dim fileNum As Integer
//!     Dim result As String
//!     Dim chunk As String
//!     Dim chunks As Collection
//!     Dim i As Long
//!     
//!     Set chunks = New Collection
//!     fileNum = FreeFile
//!     Open fileName For Binary As #fileNum
//!     
//!     ' Read in chunks
//!     Do While Not EOF(fileNum)
//!         chunk = Input(bufferSize, #fileNum)
//!         chunks.Add chunk
//!     Loop
//!     
//!     Close #fileNum
//!     
//!     ' Concatenate chunks
//!     result = ""
//!     For i = 1 To chunks.Count
//!         result = result & chunks(i)
//!     Next i
//!     
//!     ReadLargeFile = result
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The `Input` function can raise several errors:
//!
//! - **Bad file mode (Error 54)**: If file is not opened in `Input` or `Binary` mode
//! - **Bad file number (Error 52)**: If filenumber is invalid or file is not open
//! - **Input past end of file (Error 62)**: Only if reading past `EOF` (rare, usually returns partial data)
//! - **Type Mismatch (Error 13)**: If number parameter is not numeric
//!
//! ```vb
//! On Error GoTo ErrorHandler
//! Dim fileNum As Integer
//! Dim content As String
//!
//! fileNum = FreeFile
//! Open "data.txt" For Input As #fileNum
//! content = Input(LOF(fileNum), #fileNum)
//! Close #fileNum
//! Exit Sub
//!
//! ErrorHandler:
//!     If fileNum > 0 Then Close #fileNum
//!     MsgBox "Error reading file: " & Err.Description, vbCritical
//! ```
//!
//! ## Performance Considerations
//!
//! - **Buffer Size**: Reading in larger chunks (4KB-32KB) is more efficient than single bytes
//! - **`String` Concatenation**: For large files, use collection of chunks then join
//! - **Memory Usage**: Reading entire large files into memory can cause issues
//! - **File Mode**: Binary mode is faster than Input mode for raw data
//! - **`LOF` Function**: Call `LOF` once and store result rather than calling repeatedly
//!
//! ## Best Practices
//!
//! 1. **Check `EOF`**: Always check `EOF` before reading to avoid errors
//! 2. **Close Files**: Always close files in error handlers to prevent leaks
//! 3. **Use `LOF`**: Use `LOF` to determine file size before reading entire file
//! 4. **Chunk Reading**: Read large files in chunks to manage memory
//! 5. **Binary Mode**: Use `Binary` mode for most file reading operations
//! 6. **Error Handling**: Wrap file operations in proper error handling
//! 7. **Free Resources**: `Close` files as soon as done reading
//!
//! ## Comparison with Other Functions
//!
//! | Function/Statement | Purpose | Usage |
//! |--------------------|---------|-------|
//! | `Input` | Read exact number of characters | `s = Input(100, #1)` |
//! | `Input #` | Read comma-delimited data | `Input #1, var1, var2` |
//! | `Line Input #` | Read entire line | `Line Input #1, s` |
//! | `Get` | Read binary data into variables | `Get #1, , myVar` |
//! | `LOF` | Get file length | `size = LOF(1)` |
//! | `EOF` | Check end of file | `If EOF(1) Then...` |
//!
//! ## Platform and Version Notes
//!
//! - Available in all VB6 versions
//! - Consistent behavior across Windows platforms
//! - Maximum string length limitations apply (approximately 2GB in VB6)
//! - File must be opened before using Input function
//! - The # symbol before filenumber is optional
//!
//! ## Limitations
//!
//! - Cannot be used with files opened in `Output` or `Append` mode
//! - Reading very large files into single string may cause memory issues
//! - No built-in Unicode support (use `ADODB.Stream` for Unicode)
//! - `String` concatenation for large files can be slow
//! - Limited to approximately 2GB string size on 32-bit systems
//! - No built-in compression or encoding support
//!
//! ## Related Functions
//!
//! - `Input #`: Statement for reading delimited data from files
//! - `Line Input #`: Statement for reading complete lines
//! - `Get`: Statement for reading binary data into variables
//! - `LOF`: Returns the size of an open file in bytes
//! - `EOF`: Returns `True` if at end of file
//! - `Seek`: Function/statement for getting/setting file position
//! - `Open`: Statement for opening files
//! - `Close`: Statement for closing files
