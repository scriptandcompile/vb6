//! `GetAttr` Function
//!
//! Returns an Integer representing the attributes of a file, directory, or folder.
//!
//! # Syntax
//!
//! ```vb
//! GetAttr(pathname)
//! ```
//!
//! # Parameters
//!
//! - `pathname` - Required. String expression that specifies a file name. May include directory or folder, and drive.
//!
//! # Return Value
//!
//! Returns an Integer representing the sum of the following attribute values:
//!
//! | Constant | Value | Description |
//! |----------|-------|-------------|
//! | vbNormal | 0 | Normal (no attributes set) |
//! | vbReadOnly | 1 | Read-only |
//! | vbHidden | 2 | Hidden |
//! | vbSystem | 4 | System file |
//! | vbVolume | 8 | Volume label |
//! | vbDirectory | 16 | Directory or folder |
//! | vbArchive | 32 | File has changed since last backup |
//! | vbAlias | 64 | Specified file name is an alias (Macintosh only) |
//!
//! # Remarks
//!
//! - Use the And operator to test for a specific attribute.
//! - The return value can be a combination of multiple attributes.
//! - To check if a file has a specific attribute, use bitwise AND comparison.
//! - `GetAttr` generates an error if the file doesn't exist.
//! - Use `Dir` to check if a file exists before calling `GetAttr`.
//! - On systems other than Macintosh, `vbAlias` is never set.
//! - The `vbVolume` constant is used for volume labels only.
//!
//! # Typical Uses
//!
//! - Checking if a file is read-only before attempting to modify it
//! - Detecting hidden or system files
//! - Verifying if a path points to a directory
//! - Determining file attributes for backup operations
//! - Filtering files based on attributes
//! - Security and permission checks
//!
//! # Basic Usage Examples
//!
//! ```vb
//! ' Check if file is read-only
//! Dim attr As Integer
//! attr = GetAttr("C:\data.txt")
//!
//! If attr And vbReadOnly Then
//!     MsgBox "File is read-only"
//! End If
//!
//! ' Check if path is a directory
//! Dim pathAttr As Integer
//! pathAttr = GetAttr("C:\MyFolder")
//!
//! If pathAttr And vbDirectory Then
//!     MsgBox "This is a directory"
//! End If
//!
//! ' Check for hidden file
//! Dim fileAttr As Integer
//! fileAttr = GetAttr("C:\hidden.sys")
//!
//! If fileAttr And vbHidden Then
//!     MsgBox "File is hidden"
//! End If
//!
//! ' Check for system file
//! If GetAttr("C:\Windows\System32\ntoskrnl.exe") And vbSystem Then
//!     MsgBox "This is a system file"
//! End If
//! ```
//!
//! # Common Patterns
//!
//! ## 1. Check Read-Only Before Writing
//!
//! ```vb
//! Sub WriteToFile(filename As String, content As String)
//!     Dim attr As Integer
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     attr = GetAttr(filename)
//!     
//!     If attr And vbReadOnly Then
//!         MsgBox "Cannot write to read-only file: " & filename
//!         Exit Sub
//!     End If
//!     
//!     ' Proceed with writing
//!     Dim fileNum As Integer
//!     fileNum = FreeFile
//!     Open filename For Output As #fileNum
//!     Print #fileNum, content
//!     Close #fileNum
//!     
//!     Exit Sub
//!     
//! ErrorHandler:
//!     MsgBox "Error accessing file: " & Err.Description
//! End Sub
//! ```
//!
//! ## 2. Detect Directory vs File
//!
//! ```vb
//! Function IsDirectory(path As String) As Boolean
//!     On Error GoTo ErrorHandler
//!     
//!     Dim attr As Integer
//!     attr = GetAttr(path)
//!     
//!     IsDirectory = (attr And vbDirectory) <> 0
//!     Exit Function
//!     
//! ErrorHandler:
//!     IsDirectory = False
//! End Function
//!
//! ' Usage
//! If IsDirectory("C:\Windows") Then
//!     Debug.Print "Path is a directory"
//! Else
//!     Debug.Print "Path is a file"
//! End If
//! ```
//!
//! ## 3. Find Hidden Files
//!
//! ```vb
//! Sub ListHiddenFiles(folderPath As String)
//!     Dim filename As String
//!     Dim fullPath As String
//!     Dim attr As Integer
//!     
//!     filename = Dir(folderPath & "\*.*", vbHidden)
//!     
//!     Do While filename <> ""
//!         fullPath = folderPath & "\" & filename
//!         
//!         On Error Resume Next
//!         attr = GetAttr(fullPath)
//!         On Error GoTo 0
//!         
//!         If (attr And vbHidden) And Not (attr And vbDirectory) Then
//!             Debug.Print "Hidden file: " & filename
//!         End If
//!         
//!         filename = Dir
//!     Loop
//! End Sub
//! ```
//!
//! ## 4. Check Multiple Attributes
//!
//! ```vb
//! Function GetFileAttributeDescription(filename As String) As String
//!     Dim attr As Integer
//!     Dim description As String
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     attr = GetAttr(filename)
//!     description = ""
//!     
//!     If attr = vbNormal Then
//!         description = "Normal"
//!     Else
//!         If attr And vbReadOnly Then description = description & "Read-Only "
//!         If attr And vbHidden Then description = description & "Hidden "
//!         If attr And vbSystem Then description = description & "System "
//!         If attr And vbDirectory Then description = description & "Directory "
//!         If attr And vbArchive Then description = description & "Archive "
//!     End If
//!     
//!     GetFileAttributeDescription = Trim(description)
//!     Exit Function
//!     
//! ErrorHandler:
//!     GetFileAttributeDescription = "Error: " & Err.Description
//! End Function
//! ```
//!
//! ## 5. Safe File Delete
//!
//! ```vb
//! Function SafeDeleteFile(filename As String) As Boolean
//!     Dim attr As Integer
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     ' Check if file exists and get attributes
//!     attr = GetAttr(filename)
//!     
//!     ' Don't delete system or read-only files
//!     If attr And vbSystem Then
//!         MsgBox "Cannot delete system file: " & filename
//!         SafeDeleteFile = False
//!         Exit Function
//!     End If
//!     
//!     ' Remove read-only attribute if set
//!     If attr And vbReadOnly Then
//!         SetAttr filename, attr And Not vbReadOnly
//!     End If
//!     
//!     ' Delete the file
//!     Kill filename
//!     SafeDeleteFile = True
//!     Exit Function
//!     
//! ErrorHandler:
//!     MsgBox "Error deleting file: " & Err.Description
//!     SafeDeleteFile = False
//! End Function
//! ```
//!
//! ## 6. List Files by Attribute
//!
//! ```vb
//! Sub ListFilesByAttribute(folderPath As String, attributeFlag As Integer)
//!     Dim filename As String
//!     Dim fullPath As String
//!     Dim attr As Integer
//!     
//!     filename = Dir(folderPath & "\*.*", vbNormal + vbHidden + vbSystem)
//!     
//!     Do While filename <> ""
//!         fullPath = folderPath & "\" & filename
//!         
//!         On Error Resume Next
//!         attr = GetAttr(fullPath)
//!         On Error GoTo 0
//!         
//!         If (attr And attributeFlag) And Not (attr And vbDirectory) Then
//!             Debug.Print filename & " (Attribute: " & attr & ")"
//!         End If
//!         
//!         filename = Dir
//!     Loop
//! End Sub
//!
//! ' Usage
//! ListFilesByAttribute "C:\Temp", vbArchive  ' List files needing backup
//! ```
//!
//! ## 7. Backup File Scanner
//!
//! ```vb
//! Function NeedsBackup(filename As String) As Boolean
//!     Dim attr As Integer
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     attr = GetAttr(filename)
//!     
//!     ' Check if archive bit is set
//!     NeedsBackup = (attr And vbArchive) <> 0
//!     Exit Function
//!     
//! ErrorHandler:
//!     NeedsBackup = False
//! End Function
//!
//! Sub FindFilesNeedingBackup(folderPath As String, lst As ListBox)
//!     Dim filename As String
//!     Dim fullPath As String
//!     
//!     lst.Clear
//!     filename = Dir(folderPath & "\*.*")
//!     
//!     Do While filename <> ""
//!         fullPath = folderPath & "\" & filename
//!         
//!         If NeedsBackup(fullPath) Then
//!             lst.AddItem filename
//!         End If
//!         
//!         filename = Dir
//!     Loop
//! End Sub
//! ```
//!
//! ## 8. File Permission Check
//!
//! ```vb
//! Function CanModifyFile(filename As String) As Boolean
//!     Dim attr As Integer
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     attr = GetAttr(filename)
//!     
//!     ' File can be modified if it's not read-only and not a system file
//!     CanModifyFile = Not ((attr And vbReadOnly) Or (attr And vbSystem))
//!     Exit Function
//!     
//! ErrorHandler:
//!     CanModifyFile = False
//! End Function
//!
//! Sub ModifyFileIfAllowed(filename As String)
//!     If CanModifyFile(filename) Then
//!         ' Proceed with modification
//!         Debug.Print "File can be modified"
//!     Else
//!         Debug.Print "File is protected"
//!     End If
//! End Sub
//! ```
//!
//! ## 9. Directory Tree Walker
//!
//! ```vb
//! Sub WalkDirectory(path As String, Optional level As Integer = 0)
//!     Dim filename As String
//!     Dim fullPath As String
//!     Dim attr As Integer
//!     Dim indent As String
//!     
//!     indent = String(level * 2, " ")
//!     filename = Dir(path & "\*.*", vbDirectory)
//!     
//!     Do While filename <> ""
//!         If filename <> "." And filename <> ".." Then
//!             fullPath = path & "\" & filename
//!             
//!             On Error Resume Next
//!             attr = GetAttr(fullPath)
//!             On Error GoTo 0
//!             
//!             If attr And vbDirectory Then
//!                 Debug.Print indent & "[DIR] " & filename
//!                 WalkDirectory fullPath, level + 1
//!             Else
//!                 Debug.Print indent & filename
//!             End If
//!         End If
//!         
//!         filename = Dir
//!     Loop
//! End Sub
//! ```
//!
//! ## 10. File Attribute Report
//!
//! ```vb
//! Sub GenerateAttributeReport(folderPath As String)
//!     Dim filename As String
//!     Dim fullPath As String
//!     Dim attr As Integer
//!     Dim normalCount As Long
//!     Dim readOnlyCount As Long
//!     Dim hiddenCount As Long
//!     Dim systemCount As Long
//!     Dim archiveCount As Long
//!     
//!     filename = Dir(folderPath & "\*.*", vbNormal + vbHidden + vbSystem)
//!     
//!     Do While filename <> ""
//!         fullPath = folderPath & "\" & filename
//!         
//!         On Error Resume Next
//!         attr = GetAttr(fullPath)
//!         On Error GoTo 0
//!         
//!         If Not (attr And vbDirectory) Then
//!             If attr = vbNormal Then normalCount = normalCount + 1
//!             If attr And vbReadOnly Then readOnlyCount = readOnlyCount + 1
//!             If attr And vbHidden Then hiddenCount = hiddenCount + 1
//!             If attr And vbSystem Then systemCount = systemCount + 1
//!             If attr And vbArchive Then archiveCount = archiveCount + 1
//!         End If
//!         
//!         filename = Dir
//!     Loop
//!     
//!     Debug.Print "File Attribute Report for: " & folderPath
//!     Debug.Print String(50, "=")
//!     Debug.Print "Normal: " & normalCount
//!     Debug.Print "Read-Only: " & readOnlyCount
//!     Debug.Print "Hidden: " & hiddenCount
//!     Debug.Print "System: " & systemCount
//!     Debug.Print "Archive: " & archiveCount
//! End Sub
//! ```
//!
//! # Advanced Usage
//!
//! ## 1. File Attribute Manager Class
//!
//! ```vb
//! ' Class: FileAttributeManager
//! Private m_Filename As String
//! Private m_Attributes As Integer
//!
//! Public Sub LoadFile(filename As String)
//!     m_Filename = filename
//!     RefreshAttributes
//! End Sub
//!
//! Private Sub RefreshAttributes()
//!     On Error Resume Next
//!     m_Attributes = GetAttr(m_Filename)
//!     On Error GoTo 0
//! End Sub
//!
//! Public Property Get IsReadOnly() As Boolean
//!     IsReadOnly = (m_Attributes And vbReadOnly) <> 0
//! End Property
//!
//! Public Property Let IsReadOnly(value As Boolean)
//!     If value Then
//!         SetAttr m_Filename, m_Attributes Or vbReadOnly
//!     Else
//!         SetAttr m_Filename, m_Attributes And Not vbReadOnly
//!     End If
//!     RefreshAttributes
//! End Property
//!
//! Public Property Get IsHidden() As Boolean
//!     IsHidden = (m_Attributes And vbHidden) <> 0
//! End Property
//!
//! Public Property Let IsHidden(value As Boolean)
//!     If value Then
//!         SetAttr m_Filename, m_Attributes Or vbHidden
//!     Else
//!         SetAttr m_Filename, m_Attributes And Not vbHidden
//!     End If
//!     RefreshAttributes
//! End Property
//!
//! Public Property Get IsDirectory() As Boolean
//!     IsDirectory = (m_Attributes And vbDirectory) <> 0
//! End Property
//!
//! Public Property Get AttributeValue() As Integer
//!     AttributeValue = m_Attributes
//! End Property
//! ```
//!
//! ## 2. Smart File Filter
//!
//! ```vb
//! Type FileFilter
//!     IncludeReadOnly As Boolean
//!     IncludeHidden As Boolean
//!     IncludeSystem As Boolean
//!     IncludeArchive As Boolean
//!     ExcludeDirectories As Boolean
//! End Type
//!
//! Function MatchesFilter(filename As String, filter As FileFilter) As Boolean
//!     Dim attr As Integer
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     attr = GetAttr(filename)
//!     
//!     ' Check exclusions first
//!     If filter.ExcludeDirectories And (attr And vbDirectory) Then
//!         MatchesFilter = False
//!         Exit Function
//!     End If
//!     
//!     ' Check inclusions
//!     MatchesFilter = True
//!     
//!     If Not filter.IncludeReadOnly And (attr And vbReadOnly) Then
//!         MatchesFilter = False
//!     ElseIf Not filter.IncludeHidden And (attr And vbHidden) Then
//!         MatchesFilter = False
//!     ElseIf Not filter.IncludeSystem And (attr And vbSystem) Then
//!         MatchesFilter = False
//!     ElseIf Not filter.IncludeArchive And (attr And vbArchive) Then
//!         MatchesFilter = False
//!     End If
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     MatchesFilter = False
//! End Function
//! ```
//!
//! ## 3. Attribute Change Monitor
//!
//! ```vb
//! Type FileSnapshot
//!     Filename As String
//!     Attributes As Integer
//!     Timestamp As Date
//! End Type
//!
//! Private m_Snapshots As Collection
//!
//! Sub InitializeMonitor()
//!     Set m_Snapshots = New Collection
//! End Sub
//!
//! Sub TakeSnapshot(filename As String)
//!     Dim snapshot As FileSnapshot
//!     
//!     snapshot.Filename = filename
//!     snapshot.Attributes = GetAttr(filename)
//!     snapshot.Timestamp = Now
//!     
//!     m_Snapshots.Add snapshot, filename
//! End Sub
//!
//! Function DetectAttributeChanges(filename As String) As Boolean
//!     Dim currentAttr As Integer
//!     Dim snapshot As FileSnapshot
//!     Dim i As Long
//!     
//!     currentAttr = GetAttr(filename)
//!     
//!     ' Find snapshot
//!     For i = 1 To m_Snapshots.Count
//!         snapshot = m_Snapshots(i)
//!         If snapshot.Filename = filename Then
//!             DetectAttributeChanges = (snapshot.Attributes <> currentAttr)
//!             Exit Function
//!         End If
//!     Next i
//!     
//!     DetectAttributeChanges = False
//! End Function
//! ```
//!
//! ## 4. Bulk Attribute Operations
//!
//! ```vb
//! Sub SetAttributeForMultipleFiles(filenames() As String, _
//!                                  attributeToSet As Integer, _
//!                                  enable As Boolean)
//!     Dim i As Long
//!     Dim currentAttr As Integer
//!     Dim newAttr As Integer
//!     
//!     For i = LBound(filenames) To UBound(filenames)
//!         On Error Resume Next
//!         
//!         currentAttr = GetAttr(filenames(i))
//!         
//!         If enable Then
//!             newAttr = currentAttr Or attributeToSet
//!         Else
//!             newAttr = currentAttr And Not attributeToSet
//!         End If
//!         
//!         SetAttr filenames(i), newAttr
//!         
//!         On Error GoTo 0
//!     Next i
//! End Sub
//!
//! ' Usage
//! Sub MakeFilesReadOnly()
//!     Dim files() As String
//!     files = Array("file1.txt", "file2.txt", "file3.txt")
//!     
//!     SetAttributeForMultipleFiles files, vbReadOnly, True
//! End Sub
//! ```
//!
//! ## 5. File Security Analyzer
//!
//! ```vb
//! Type SecurityIssue
//!     Filename As String
//!     IssueType As String
//!     Severity As String
//! End Type
//!
//! Function AnalyzeFileSecurity(folderPath As String) As Collection
//!     Dim issues As New Collection
//!     Dim filename As String
//!     Dim fullPath As String
//!     Dim attr As Integer
//!     Dim issue As SecurityIssue
//!     
//!     filename = Dir(folderPath & "\*.*", vbNormal + vbHidden + vbSystem)
//!     
//!     Do While filename <> ""
//!         fullPath = folderPath & "\" & filename
//!         
//!         On Error Resume Next
//!         attr = GetAttr(fullPath)
//!         On Error GoTo 0
//!         
//!         If Not (attr And vbDirectory) Then
//!             ' Check for security issues
//!             
//!             ' Issue 1: System files that are not hidden
//!             If (attr And vbSystem) And Not (attr And vbHidden) Then
//!                 issue.Filename = filename
//!                 issue.IssueType = "System file not hidden"
//!                 issue.Severity = "Medium"
//!                 issues.Add issue
//!             End If
//!             
//!             ' Issue 2: Important files not marked read-only
//!             If InStr(1, filename, "config", vbTextCompare) > 0 Then
//!                 If Not (attr And vbReadOnly) Then
//!                     issue.Filename = filename
//!                     issue.IssueType = "Config file not read-only"
//!                     issue.Severity = "Low"
//!                     issues.Add issue
//!                 End If
//!             End If
//!         End If
//!         
//!         filename = Dir
//!     Loop
//!     
//!     Set AnalyzeFileSecurity = issues
//! End Function
//! ```
//!
//! ## 6. Attribute Comparison Tool
//!
//! ```vb
//! Function CompareFileAttributes(file1 As String, file2 As String) As String
//!     Dim attr1 As Integer
//!     Dim attr2 As Integer
//!     Dim differences As String
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     attr1 = GetAttr(file1)
//!     attr2 = GetAttr(file2)
//!     
//!     differences = ""
//!     
//!     If (attr1 And vbReadOnly) <> (attr2 And vbReadOnly) Then
//!         differences = differences & "Read-Only differs" & vbCrLf
//!     End If
//!     
//!     If (attr1 And vbHidden) <> (attr2 And vbHidden) Then
//!         differences = differences & "Hidden differs" & vbCrLf
//!     End If
//!     
//!     If (attr1 And vbSystem) <> (attr2 And vbSystem) Then
//!         differences = differences & "System differs" & vbCrLf
//!     End If
//!     
//!     If (attr1 And vbArchive) <> (attr2 And vbArchive) Then
//!         differences = differences & "Archive differs" & vbCrLf
//!     End If
//!     
//!     If differences = "" Then
//!         CompareFileAttributes = "Attributes are identical"
//!     Else
//!         CompareFileAttributes = "Differences found:" & vbCrLf & differences
//!     End If
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     CompareFileAttributes = "Error: " & Err.Description
//! End Function
//! ```
//!
//! # Error Handling
//!
//! ```vb
//! Function SafeGetAttr(pathname As String, _
//!                      Optional defaultValue As Integer = -1) As Integer
//!     On Error GoTo ErrorHandler
//!     
//!     SafeGetAttr = GetAttr(pathname)
//!     Exit Function
//!     
//! ErrorHandler:
//!     Select Case Err.Number
//!         Case 53  ' File not found
//!             Debug.Print "File not found: " & pathname
//!         Case 76  ' Path not found
//!             Debug.Print "Path not found: " & pathname
//!         Case 68  ' Device unavailable
//!             Debug.Print "Device unavailable: " & pathname
//!         Case Else
//!             Debug.Print "Error " & Err.Number & ": " & Err.Description
//!     End Select
//!     
//!     SafeGetAttr = defaultValue
//! End Function
//! ```
//!
//! Common errors:
//! - **Error 53 (File not found)**: The specified file doesn't exist.
//! - **Error 76 (Path not found)**: The specified path doesn't exist.
//! - **Error 68 (Device unavailable)**: Network drive or removable media not available.
//!
//! # Performance Considerations
//!
//! - `GetAttr` is a fast file system call
//! - Use `Dir` to check existence before calling `GetAttr` if unsure
//! - For multiple files, consider caching attribute values
//! - Network paths may be slower than local paths
//! - Accessing removable media can cause delays if not present
//!
//! # Best Practices
//!
//! 1. **Always use error handling** - file may not exist
//! 2. **Use bitwise AND** to test individual attributes
//! 3. **Check Dir first** if file existence is uncertain
//! 4. **Cache results** when checking attributes repeatedly
//! 5. **Use constants** (vbReadOnly, etc.) instead of numeric values
//! 6. **Handle network paths carefully** - may be unavailable
//! 7. **Don't assume attributes** - always check explicitly
//!
//! # Comparison with Other Functions
//!
//! ## `GetAttr` vs `FileLen`
//!
//! ```vb
//! ' GetAttr - Returns file attributes
//! attr = GetAttr("file.txt")  ' Returns attribute flags
//!
//! ' FileLen - Returns file size
//! size = FileLen("file.txt")  ' Returns size in bytes
//! ```
//!
//! ## `GetAttr` vs `Dir`
//!
//! ```vb
//! ' GetAttr - Gets attributes of specific file
//! attr = GetAttr("file.txt")
//!
//! ' Dir - Searches for files matching pattern
//! filename = Dir("*.txt", vbNormal)
//! ```
//!
//! ## `GetAttr` vs `FileDateTime`
//!
//! ```vb
//! ' GetAttr - Returns attributes
//! attr = GetAttr("file.txt")
//!
//! ' FileDateTime - Returns modification date/time
//! dt = FileDateTime("file.txt")
//! ```
//!
//! # Limitations
//!
//! - Returns Integer (limited to values 0-32767 due to VB6 Integer type)
//! - `vbAlias` only works on Macintosh systems
//! - Cannot set attributes (use `SetAttr` for that)
//! - No support for extended attributes or NTFS features
//! - Limited to file system attributes only
//! - Does not provide security descriptor information
//!
//! # Bitwise Operations
//!
//! Testing for specific attributes:
//!
//! ```vb
//! ' Test if read-only
//! If (attr And vbReadOnly) Then ' Has read-only attribute
//!
//! ' Test if NOT read-only
//! If Not (attr And vbReadOnly) Then ' Doesn't have read-only
//!
//! ' Test for multiple attributes
//! If (attr And (vbReadOnly Or vbHidden)) Then ' Has either attribute
//!
//! ' Test for exact attribute combination
//! If attr = (vbReadOnly Or vbHidden) Then ' Has exactly these two
//! ```
//!
//! # Related Functions
//!
//! - `SetAttr` - Sets the attributes of a file
//! - `Dir` - Returns a file or directory name matching a pattern
//! - `FileLen` - Returns the length of a file in bytes
//! - `FileDateTime` - Returns the date and time a file was created or last modified
//! - `FileAttr` - Returns the file mode or file handle for an open file
//! - `FileExists` - Checks if a file exists (custom function using Dir)

use crate::error::{VBError, VBResult};
use crate::state::file;
use crate::value::{VBInteger, VBVariant};

// VB6 file attribute constants.

/// Normal file (no special attributes).
pub const VB_NORMAL: i16 = 0;
/// Read-only file.
pub const VB_READ_ONLY: i16 = 1;
/// Hidden file.
pub const VB_HIDDEN: i16 = 2;
/// System file.
pub const VB_SYSTEM: i16 = 4;
/// Directory.
pub const VB_DIRECTORY: i16 = 16;
/// Archive flag.
pub const VB_ARCHIVE: i16 = 32;

/// Get the attributes of a file or directory.
///
/// # Arguments
///
/// * `pathname` - The file path.
///
/// # Returns
///
/// Returns the file attributes as an integer.
pub fn getattr(pathname: VBVariant) -> VBResult<VBInteger> {
    // Get the file path
    let path_str = match pathname {
        VBVariant::String(s) => s.as_str().to_string(),
        _ => {
            return Err(VBError::with_description(
                13, // Type mismatch
                "Type mismatch in GetAttr",
            ));
        }
    };

    // Get attributes through the backend
    let attrs = file::get_attrs(std::path::Path::new(&path_str)).map_err(|e| {
        VBError::with_description(
            match e.kind() {
                std::io::ErrorKind::NotFound => 53, // File not found
                _ => 57,                            // Device I/O error
            },
            e.to_string(),
        )
    })?;

    Ok(VBInteger::from(attrs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::file::{self};

    #[test]
    fn getattr_returns_normal_for_file() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create a file
        std::fs::write(dir.path().join("test.txt"), "Hello").unwrap();

        let attrs = getattr(VBVariant::from_string("test.txt")).unwrap();
        assert_eq!(attrs.as_i16() & VB_DIRECTORY, 0);
    }

    #[test]
    fn getattr_returns_directory_for_dir() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        // Create directory
        std::fs::create_dir(dir.path().join("testdir")).unwrap();

        let attrs = getattr(VBVariant::from_string("testdir")).unwrap();
        assert_ne!(attrs.as_i16() & VB_DIRECTORY, 0);
    }

    #[test]
    fn getattr_rejects_nonexistent_file() {
        let _guard = crate::state::test_support::lock_test();

        let dir = tempfile::tempdir().unwrap();
        file::set_root(dir.path());

        let result = getattr(VBVariant::from_string("nonexistent.txt"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 53);
    }

    #[test]
    fn getattr_rejects_non_string() {
        let _guard = crate::state::test_support::lock_test();

        let result = getattr(VBVariant::Long(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().number, 13);
    }
}
