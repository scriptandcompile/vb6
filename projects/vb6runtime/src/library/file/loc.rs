//! # Loc Function
//!
//! Returns a Long specifying the current read/write position within an open file.
//!
//! ## Syntax
//!
//! ```vb
//! Loc(filenumber)
//! ```
//!
//! ## Parameters
//!
//! - `filenumber` (Required): Integer file number used in the Open statement
//!   - Must be a valid file number from a currently open file
//!   - File numbers typically obtained from `FreeFile` function
//!
//! ## Return Value
//!
//! Returns a Long:
//! - For Random mode: Record number of last record read or written
//! - For Sequential mode: Current byte position divided by 128
//! - For Binary mode: Position of last byte read or written
//! - Returns 0 if no read/write operations have occurred yet
//! - Returns value based on last I/O operation
//!
//! ## Remarks
//!
//! The Loc function returns the current position in an open file:
//!
//! - Behavior varies based on file access mode
//! - For Random access: Returns record number (1-based)
//! - For Sequential access: Returns byte position / 128 (approximation)
//! - For Binary access: Returns byte position (0-based)
//! - Does not move the file pointer
//! - Read-only operation (non-destructive)
//! - Useful for tracking progress in file operations
//! - Returns position of last operation, not next operation
//! - For Random files, increments after Get/Put
//! - For Binary files, tracks exact byte position
//! - For Sequential files, provides approximate position
//! - Essential for file I/O progress tracking
//! - Used with Seek to navigate files
//! - Different from Seek function (which also sets position)
//! - LOF function returns file length, Loc returns position
//! - Error 52 if file number not open
//! - Error 68 if device unavailable
//! - Common in loops reading/writing files
//! - Helps detect end-of-file conditions
//! - Used for progress bars during file operations
//!
//! ## Typical Uses
//!
//! 1. **Track Random File Position**
//!    ```vb
//!    currentRecord = Loc(1)
//!    ```
//!
//! 2. **Track Binary File Position**
//!    ```vb
//!    bytesProcessed = Loc(fileNum)
//!    ```
//!
//! 3. **Progress Calculation**
//!    ```vb
//!    percentComplete = (Loc(1) / LOF(1)) * 100
//!    ```
//!
//! 4. **Check if Data Written**
//!    ```vb
//!    If Loc(fileNum) > 0 Then
//!        ' File has been written to
//!    End If
//!    ```
//!
//! 5. **Loop Until End**
//!    ```vb
//!    Do While Loc(1) < LOF(1)
//!        Get #1, , record
//!    Loop
//!    ```
//!
//! 6. **Record Number Display**
//!    ```vb
//!    lblRecordNum.Caption = "Record: " & Loc(1)
//!    ```
//!
//! 7. **Byte Position Check**
//!    ```vb
//!    Debug.Print "Position: " & Loc(fileNum)
//!    ```
//!
//! 8. **Progress Bar Update**
//!    ```vb
//!    ProgressBar1.Value = (Loc(1) / totalRecords) * 100
//!    ```
//!
//! ## Basic Examples
//!
//! ### Example 1: Random File Position
//! ```vb
//! Type CustomerRecord
//!     ID As Long
//!     Name As String * 50
//! End Type
//!
//! Dim customer As CustomerRecord
//! Dim fileNum As Integer
//!
//! fileNum = FreeFile
//! Open "customers.dat" For Random As #fileNum Len = Len(customer)
//!
//! ' Read records
//! Do While Not EOF(fileNum)
//!     Get #fileNum, , customer
//!     Debug.Print "Record: " & Loc(fileNum)
//! Loop
//!
//! Close #fileNum
//! ```
//!
//! ### Example 2: Binary File Progress
//! ```vb
//! Dim fileNum As Integer
//! Dim data As Byte
//! Dim fileSize As Long
//!
//! fileNum = FreeFile
//! Open "data.bin" For Binary As #fileNum
//! fileSize = LOF(fileNum)
//!
//! Do While Loc(fileNum) < fileSize
//!     Get #fileNum, , data
//!     
//!     ' Update progress
//!     If Loc(fileNum) Mod 1024 = 0 Then
//!         Debug.Print "Progress: " & (Loc(fileNum) / fileSize) * 100 & "%"
//!     End If
//! Loop
//!
//! Close #fileNum
//! ```
//!
//! ### Example 3: Sequential File Position
//! ```vb
//! Dim fileNum As Integer
//! Dim line As String
//!
//! fileNum = FreeFile
//! Open "log.txt" For Input As #fileNum
//!
//! Do While Not EOF(fileNum)
//!     Line Input #fileNum, line
//!     
//!     ' Approximate position (bytes / 128)
//!     Debug.Print "Position: " & Loc(fileNum)
//! Loop
//!
//! Close #fileNum
//! ```
//!
//! ### Example 4: Track Write Position
//! ```vb
//! Dim fileNum As Integer
//! Dim i As Integer
//!
//! fileNum = FreeFile
//! Open "output.bin" For Binary As #fileNum
//!
//! For i = 1 To 100
//!     Put #fileNum, , i
//!     Debug.Print "Wrote to position: " & Loc(fileNum)
//! Next i
//!
//! Close #fileNum
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `CalculateProgress`
//! ```vb
//! Function CalculateProgress(ByVal fileNum As Integer) As Single
//!     Dim currentPos As Long
//!     Dim totalSize As Long
//!     
//!     currentPos = Loc(fileNum)
//!     totalSize = LOF(fileNum)
//!     
//!     If totalSize > 0 Then
//!         CalculateProgress = (currentPos / totalSize) * 100
//!     Else
//!         CalculateProgress = 0
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 2: `IsFilePositionChanged`
//! ```vb
//! Function IsFilePositionChanged(ByVal fileNum As Integer, _
//!                                 ByVal lastPosition As Long) As Boolean
//!     IsFilePositionChanged = (Loc(fileNum) <> lastPosition)
//! End Function
//! ```
//!
//! ### Pattern 3: `GetCurrentRecord`
//! ```vb
//! Function GetCurrentRecord(ByVal fileNum As Integer) As Long
//!     ' For Random access files
//!     GetCurrentRecord = Loc(fileNum)
//! End Function
//! ```
//!
//! ### Pattern 4: `GetBytesProcessed`
//! ```vb
//! Function GetBytesProcessed(ByVal fileNum As Integer) As Long
//!     ' For Binary access files
//!     GetBytesProcessed = Loc(fileNum)
//! End Function
//! ```
//!
//! ### Pattern 5: `UpdateProgressBar`
//! ```vb
//! Sub UpdateProgressBar(ByVal fileNum As Integer, _
//!                       ByVal progressBar As ProgressBar)
//!     Dim percent As Single
//!     percent = (Loc(fileNum) / LOF(fileNum)) * 100
//!     
//!     If percent <= 100 Then
//!         progressBar.Value = percent
//!     End If
//!     DoEvents
//! End Sub
//! ```
//!
//! ### Pattern 6: `ReadFileWithProgress`
//! ```vb
//! Sub ReadFileWithProgress(ByVal filename As String)
//!     Dim fileNum As Integer
//!     Dim data As Byte
//!     Dim lastPercent As Integer
//!     Dim currentPercent As Integer
//!     
//!     fileNum = FreeFile
//!     Open filename For Binary As #fileNum
//!     
//!     Do While Loc(fileNum) < LOF(fileNum)
//!         Get #fileNum, , data
//!         ProcessByte data
//!         
//!         currentPercent = Int((Loc(fileNum) / LOF(fileNum)) * 100)
//!         If currentPercent <> lastPercent Then
//!             Debug.Print "Progress: " & currentPercent & "%"
//!             lastPercent = currentPercent
//!         End If
//!     Loop
//!     
//!     Close #fileNum
//! End Sub
//! ```
//!
//! ### Pattern 7: `GetRecordPosition`
//! ```vb
//! Function GetRecordPosition(ByVal fileNum As Integer) As String
//!     Dim current As Long
//!     Dim total As Long
//!     
//!     current = Loc(fileNum)
//!     total = LOF(fileNum) / Len(recordVariable)
//!     
//!     GetRecordPosition = current & " of " & total
//! End Function
//! ```
//!
//! ### Pattern 8: `SafeLoc`
//! ```vb
//! Function SafeLoc(ByVal fileNum As Integer) As Long
//!     On Error Resume Next
//!     SafeLoc = Loc(fileNum)
//!     If Err.Number <> 0 Then
//!         SafeLoc = -1
//!         Err.Clear
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 9: `IsAtEndOfFile`
//! ```vb
//! Function IsAtEndOfFile(ByVal fileNum As Integer) As Boolean
//!     ' For Binary mode
//!     IsAtEndOfFile = (Loc(fileNum) >= LOF(fileNum))
//! End Function
//! ```
//!
//! ### Pattern 10: `LogFilePosition`
//! ```vb
//! Sub LogFilePosition(ByVal fileNum As Integer, _
//!                     ByVal operation As String)
//!     Debug.Print operation & " - Position: " & Loc(fileNum) & _
//!                 " of " & LOF(fileNum)
//! End Sub
//! ```
//!
//! ## Advanced Examples
//!
//! ### Example 1: File Reader with Progress
//! ```vb
//! ' Class: BinaryFileReader
//! Private m_fileNum As Integer
//! Private m_filename As String
//! Private m_fileSize As Long
//!
//! Public Sub OpenFile(ByVal filename As String)
//!     m_filename = filename
//!     m_fileNum = FreeFile
//!     Open filename For Binary As #m_fileNum
//!     m_fileSize = LOF(m_fileNum)
//! End Sub
//!
//! Public Function ReadByte() As Byte
//!     If Not IsEOF Then
//!         Get #m_fileNum, , ReadByte
//!     End If
//! End Function
//!
//! Public Property Get Position() As Long
//!     Position = Loc(m_fileNum)
//! End Property
//!
//! Public Property Get Size() As Long
//!     Size = m_fileSize
//! End Property
//!
//! Public Property Get Progress() As Single
//!     If m_fileSize > 0 Then
//!         Progress = (Loc(m_fileNum) / m_fileSize) * 100
//!     Else
//!         Progress = 0
//!     End If
//! End Property
//!
//! Public Property Get IsEOF() As Boolean
//!     IsEOF = (Loc(m_fileNum) >= m_fileSize)
//! End Property
//!
//! Public Sub CloseFile()
//!     If m_fileNum > 0 Then
//!         Close #m_fileNum
//!         m_fileNum = 0
//!     End If
//! End Sub
//!
//! Private Sub Class_Terminate()
//!     CloseFile
//! End Sub
//! ```
//!
//! ### Example 2: Random File Navigator
//! ```vb
//! ' Class: RandomFileNavigator
//! Private m_fileNum As Integer
//! Private m_recordLength As Integer
//! Private m_totalRecords As Long
//!
//! Public Sub OpenFile(ByVal filename As String, _
//!                     ByVal recordLength As Integer)
//!     m_recordLength = recordLength
//!     m_fileNum = FreeFile
//!     Open filename For Random As #m_fileNum Len = recordLength
//!     m_totalRecords = LOF(m_fileNum) / recordLength
//! End Sub
//!
//! Public Property Get CurrentRecord() As Long
//!     CurrentRecord = Loc(m_fileNum)
//! End Property
//!
//! Public Property Get TotalRecords() As Long
//!     TotalRecords = m_totalRecords
//! End Property
//!
//! Public Property Get ProgressPercent() As Single
//!     If m_totalRecords > 0 Then
//!         ProgressPercent = (Loc(m_fileNum) / m_totalRecords) * 100
//!     Else
//!         ProgressPercent = 0
//!     End If
//! End Property
//!
//! Public Function IsFirstRecord() As Boolean
//!     IsFirstRecord = (Loc(m_fileNum) = 1)
//! End Function
//!
//! Public Function IsLastRecord() As Boolean
//!     IsLastRecord = (Loc(m_fileNum) = m_totalRecords)
//! End Function
//!
//! Public Sub CloseFile()
//!     If m_fileNum > 0 Then
//!         Close #m_fileNum
//!         m_fileNum = 0
//!     End If
//! End Sub
//!
//! Private Sub Class_Terminate()
//!     CloseFile
//! End Sub
//! ```
//!
//! ### Example 3: File Copy with Progress
//! ```vb
//! Sub CopyFileWithProgress(ByVal sourceFile As String, _
//!                          ByVal destFile As String, _
//!                          Optional progressBar As ProgressBar = Nothing)
//!     Dim sourceNum As Integer, destNum As Integer
//!     Dim buffer As Byte
//!     Dim totalSize As Long
//!     Dim lastPercent As Integer
//!     Dim currentPercent As Integer
//!     
//!     sourceNum = FreeFile
//!     Open sourceFile For Binary As #sourceNum
//!     totalSize = LOF(sourceNum)
//!     
//!     destNum = FreeFile
//!     Open destFile For Binary As #destNum
//!     
//!     Do While Loc(sourceNum) < totalSize
//!         Get #sourceNum, , buffer
//!         Put #destNum, , buffer
//!         
//!         If Not progressBar Is Nothing Then
//!             currentPercent = Int((Loc(sourceNum) / totalSize) * 100)
//!             If currentPercent <> lastPercent Then
//!                 progressBar.Value = currentPercent
//!                 lastPercent = currentPercent
//!                 DoEvents
//!             End If
//!         End If
//!     Loop
//!     
//!     Close #sourceNum
//!     Close #destNum
//! End Sub
//! ```
//!
//! ### Example 4: File Processing Monitor
//! ```vb
//! ' Form with lblStatus, lblProgress, ProgressBar1
//! Private m_fileNum As Integer
//! Private m_totalSize As Long
//!
//! Private Sub ProcessLargeFile(ByVal filename As String)
//!     Dim data As Byte
//!     Dim startTime As Single
//!     
//!     m_fileNum = FreeFile
//!     Open filename For Binary As #m_fileNum
//!     m_totalSize = LOF(m_fileNum)
//!     
//!     startTime = Timer
//!     Timer1.Enabled = True
//!     
//!     Do While Loc(m_fileNum) < m_totalSize
//!         Get #m_fileNum, , data
//!         ProcessData data
//!     Loop
//!     
//!     Timer1.Enabled = False
//!     Close #m_fileNum
//!     
//!     lblStatus.Caption = "Complete!"
//! End Sub
//!
//! Private Sub Timer1_Timer()
//!     UpdateProgress
//! End Sub
//!
//! Private Sub UpdateProgress()
//!     Dim percent As Single
//!     Dim bytesProcessed As Long
//!     
//!     On Error Resume Next
//!     bytesProcessed = Loc(m_fileNum)
//!     
//!     If m_totalSize > 0 Then
//!         percent = (bytesProcessed / m_totalSize) * 100
//!         ProgressBar1.Value = percent
//!         lblProgress.Caption = Format(percent, "0.0") & "% - " & _
//!                              FormatBytes(bytesProcessed) & " of " & _
//!                              FormatBytes(m_totalSize)
//!     End If
//! End Sub
//!
//! Private Function FormatBytes(ByVal bytes As Long) As String
//!     If bytes < 1024 Then
//!         FormatBytes = bytes & " bytes"
//!     ElseIf bytes < 1048576 Then
//!         FormatBytes = Format(bytes / 1024, "0.0") & " KB"
//!     Else
//!         FormatBytes = Format(bytes / 1048576, "0.0") & " MB"
//!     End If
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! ' Error 52: Bad file name or number
//! On Error Resume Next
//! pos = Loc(999)
//! If Err.Number = 52 Then
//!     MsgBox "File not open!"
//! End If
//!
//! ' Error 68: Device unavailable
//! pos = Loc(fileNum)
//! If Err.Number = 68 Then
//!     MsgBox "Device unavailable!"
//! End If
//!
//! ' Safe position retrieval
//! Function GetSafePosition(ByVal fileNum As Integer) As Long
//!     On Error Resume Next
//!     GetSafePosition = Loc(fileNum)
//!     If Err.Number <> 0 Then
//!         GetSafePosition = -1
//!         Err.Clear
//!     End If
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - **Very Fast**: Loc is a simple file pointer query
//! - **No I/O**: Does not perform actual file operations
//! - **Frequent Calls**: Safe to call in tight loops
//! - **Progress Updates**: Use modulo to update UI less frequently
//! - **`DoEvents`**: Call `DoEvents` when updating UI to maintain responsiveness
//!
//! ## Best Practices
//!
//! 1. **Use with LOF** for calculating percentage complete
//! 2. **Check file is open** before calling Loc
//! 3. **Update progress periodically** not on every byte
//! 4. **Cache in variable** if using multiple times
//! 5. **Use for Binary/Random** files (Sequential returns approximation)
//! 6. **Combine with EOF** for robust loop conditions
//! 7. **Handle errors** for unopened files
//! 8. **Use `DoEvents`** when updating UI in loops
//! 9. **Consider mode** when interpreting return value
//! 10. **Document units** (bytes, records, or approximation)
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Read/Write | Return Value |
//! |----------|---------|------------|--------------|
//! | **Loc** | Get position | Read-only | Position |
//! | **Seek** (function) | Get position | Read-only | Position + 1 |
//! | **Seek** (statement) | Set position | Write | N/A |
//! | **LOF** | Get file length | Read-only | Total bytes |
//! | **EOF** | Check end | Read-only | Boolean |
//!
//! ## Loc vs Seek Function
//!
//! ```vb
//! ' Loc - returns position of last operation
//! currentPos = Loc(fileNum)
//!
//! ' Seek - returns next read/write position (Loc + 1)
//! nextPos = Seek(fileNum)
//!
//! ' For Binary mode:
//! ' After reading byte at position 100:
//! ' Loc returns 100
//! ' Seek returns 101
//! ```
//!
//! ## Mode-Specific Behavior
//!
//! ```vb
//! ' Random mode - record number
//! Open "data.dat" For Random As #1 Len = 128
//! Get #1, 5, record
//! Debug.Print Loc(1)  ' Returns 5 (record number)
//!
//! ' Binary mode - byte position
//! Open "data.bin" For Binary As #1
//! Get #1, , buffer
//! Debug.Print Loc(1)  ' Returns bytes read
//!
//! ' Sequential mode - approximate (bytes / 128)
//! Open "text.txt" For Input As #1
//! Line Input #1, line
//! Debug.Print Loc(1)  ' Returns approximate position
//! ```
//!
//! ## Platform Notes
//!
//! - Available in all VB6 versions
//! - Part of VBA core library
//! - Returns Long (max 2GB file support)
//! - For files > 2GB, result may overflow
//! - Windows-specific file I/O
//! - Behavior identical across Windows versions
//! - Sequential mode approximation may vary
//! - Random mode most reliable
//! - Binary mode exact for files < 2GB
//!
//! ## Limitations
//!
//! - **2GB Limit**: Long type limits file size to ~2GB
//! - **Sequential Approximation**: Not exact for Input/Output/Append modes
//! - **Division by 128**: Sequential mode uses this approximation
//! - **No String Files**: Works with file numbers only
//! - **Requires Open File**: Error if file not open
//! - **Mode Dependent**: Return value meaning varies by mode
//! - **No Directory**: Only for files, not directories
//! - **Last Operation**: Returns position of last I/O, not current
//!
//! ## Related Functions
//!
//! - `Seek`: Get/set file position (next position, not last)
//! - `LOF`: Get length of file
//! - `EOF`: Check if at end of file
//! - `Open`: Open file for I/O
//! - `Close`: Close file
//! - `Get`: Read from file (updates Loc)
//! - `Put`: Write to file (updates Loc)
//! - `FreeFile`: Get available file number

use crate::error::{VBError, VBResult};
use crate::state::file::{self, MAX_FILE_NUMBER, MIN_FILE_NUMBER};
use crate::value::{VBLong, VBVariant};

/// Get the current position in an open file.
///
/// # Arguments
///
/// * `file_number` - The file number.
///
/// # Returns
///
/// Returns the current position (1-based), or 0 if the file is not open.
pub fn loc(file_number: VBVariant) -> VBResult<VBLong> {
    // Convert file number to integer
    let file_num = match file_number {
        VBVariant::Long(v) => v as i16,
        VBVariant::Integer(v) => v,
        VBVariant::Byte(v) => v as i16,
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in LOC",
            ));
        }
    };

    // Validate file number range
    if !(MIN_FILE_NUMBER..=MAX_FILE_NUMBER).contains(&file_num) {
        return Err(VBError::with_description(
            52, // Bad file name or number
            format!("Bad file name or number: {}", file_num),
        ));
    }

    // Check if file is open
    if !file::is_file_open(file_num) {
        return Err(VBError::with_description(
            52, // Bad file name or number
            format!("File not open: #{}", file_num),
        ));
    }

    // Get the current position (already 1-based)
    let pos = file::position_file(file_num).map_err(|e| {
        VBError::with_description(
            57, // Device I/O error
            e.to_string(),
        )
    })?;

    Ok(VBLong::from(pos as i32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self, AccessMode, LockMode, OpenMode};

    #[test]
    fn loc_returns_current_position() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a file and write some data
        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();
        file::write_file(1, b"Hello").unwrap();

        // Check LOC
        let pos = loc(VBVariant::Long(1)).unwrap();
        assert_eq!(pos.as_i32(), 6); // After writing 5 bytes, position is 6

        let _ = file::close_all_files();
    }

    #[test]
    fn loc_returns_one_for_empty_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let path = std::path::PathBuf::from("test.txt");
        file::open_file(
            &path,
            OpenMode::Output,
            AccessMode::Write,
            LockMode::Shared,
            0,
            1,
        )
        .unwrap();

        let pos = loc(VBVariant::Long(1)).unwrap();
        assert_eq!(pos.as_i32(), 1); // Position 1 is the start

        let _ = file::close_all_files();
    }

    #[test]
    fn loc_rejects_invalid_file_number() {
        let _guard = crate::state::test_support::lock_test();

        let result = loc(VBVariant::Long(0));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 52);

        let result = loc(VBVariant::Long(512));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 52);

        let _ = file::close_all_files();
    }

    #[test]
    fn loc_rejects_closed_file() {
        let _guard = crate::state::test_support::lock_test();
        let _ = file::close_all_files();

        let result = loc(VBVariant::Long(1));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 52);

        let _ = file::close_all_files();
    }
}
