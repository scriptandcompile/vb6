//! # Shell Function
//!
//! Runs an executable program and returns a Variant (Double) representing the program's task ID if successful, or zero if unsuccessful.
//!
//! ## Syntax
//!
//! ```vb
//! Shell(pathname, [windowstyle])
//! ```
//!
//! ## Parameters
//!
//! - `pathname` - Required. String expression specifying the name of the program to execute, along with any required arguments or command-line switches. May include directory or folder path.
//! - `windowstyle` - Optional. Variant (Integer) corresponding to the style of the window in which the program is to be run. If omitted, the program is started minimized with focus.
//!
//! ## Window Style Values
//!
//! | Constant | Value | Description |
//! |----------|-------|-------------|
//! | vbHide | 0 | Window is hidden and focus is passed to the hidden window |
//! | vbNormalFocus | 1 | Window has focus and is restored to its original size and position |
//! | vbMinimizedFocus | 2 | Window is displayed as an icon with focus |
//! | vbMaximizedFocus | 3 | Window is maximized with focus |
//! | vbNormalNoFocus | 4 | Window is restored to most recent size and position; currently active window remains active |
//! | vbMinimizedNoFocus | 6 | Window is displayed as an icon; currently active window remains active |
//!
//! ## Return Value
//!
//! Returns a Variant (Double) containing the task ID of the started program:
//! - If successful: Returns the task ID (a unique identifier for the process)
//! - If unsuccessful: Returns 0
//! - Task ID can be used with `AppActivate` statement to give focus to the window
//!
//! ## Remarks
//!
//! The Shell function runs an executable program asynchronously. This means that a program started with Shell might not finish executing before the statements following the Shell function are executed.
//!
//! Key characteristics:
//! - Executes programs asynchronously (doesn't wait for completion)
//! - Returns immediately after starting the program
//! - Can launch any executable file (.exe, .com, .bat, .cmd, etc.)
//! - Can include command-line arguments in pathname
//! - Task ID can be used with `AppActivate` to switch focus
//! - If program cannot be started, returns 0
//! - On error, generates Error 5 (Invalid procedure call) or Error 53 (File not found)
//!
//! The pathname can include:
//! - Full path to executable: "C:\Windows\notepad.exe"
//! - Relative path: "..\..\tools\mytool.exe"
//! - Program in system PATH: "notepad.exe"
//! - Command with arguments: "notepad.exe C:\readme.txt"
//!
//! Important considerations:
//! - Shell executes asynchronously - use `AppActivate` or API calls to synchronize
//! - No direct way to know when shelled program terminates from VB6
//! - Can't capture standard output/error directly (use API or temp files)
//! - Security: Be cautious with user-supplied paths to avoid injection
//! - Long filenames with spaces should be enclosed in quotes
//!
//! ## Typical Uses
//!
//! 1. **Launch Applications**: Open external programs from your VB6 app
//! 2. **Open Documents**: Launch files with associated applications
//! 3. **Run Batch Files**: Execute .bat or .cmd scripts
//! 4. **Execute Commands**: Run command-line tools
//! 5. **System Tools**: Open Windows utilities (calc, notepad, etc.)
//! 6. **Background Tasks**: Start processes that run independently
//! 7. **Integration**: Interact with other applications
//! 8. **File Operations**: Use command-line tools for file manipulation
//!
//! ## Basic Examples
//!
//! ```vb
//! ' Example 1: Open Notepad
//! Dim taskId As Double
//! taskId = Shell("notepad.exe", vbNormalFocus)
//! If taskId = 0 Then
//!     MsgBox "Failed to start Notepad"
//! End If
//! ```
//!
//! ```vb
//! ' Example 2: Open file with Notepad
//! Dim taskId As Double
//! taskId = Shell("notepad.exe C:\readme.txt", vbNormalFocus)
//! ```
//!
//! ```vb
//! ' Example 3: Run Calculator maximized
//! Dim taskId As Double
//! taskId = Shell("calc.exe", vbMaximizedFocus)
//! ```
//!
//! ```vb
//! ' Example 4: Execute batch file hidden
//! Dim taskId As Double
//! taskId = Shell("C:\Scripts\backup.bat", vbHide)
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `SafeShell`
//! Execute program with error handling
//! ```vb
//! Function SafeShell(programPath As String, Optional windowStyle As Integer = vbNormalFocus) As Double
//!     On Error Resume Next
//!     SafeShell = Shell(programPath, windowStyle)
//!     
//!     If Err.Number <> 0 Then
//!         MsgBox "Error starting program: " & Err.Description, vbExclamation
//!         SafeShell = 0
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 2: `ShellAndWait`
//! Execute program and wait for completion (using `AppActivate`)
//! ```vb
//! Function ShellAndWait(programPath As String, windowStyle As Integer) As Boolean
//!     Dim taskId As Double
//!     Dim startTime As Double
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     taskId = Shell(programPath, windowStyle)
//!     If taskId = 0 Then
//!         ShellAndWait = False
//!         Exit Function
//!     End If
//!     
//!     ' Give the program time to start
//!     DoEvents
//!     
//!     ' Wait for program window to exist
//!     startTime = Timer
//!     Do While Timer - startTime < 30  ' 30 second timeout
//!         On Error Resume Next
//!         AppActivate taskId
//!         If Err.Number = 0 Then Exit Do
//!         Err.Clear
//!         DoEvents
//!     Loop
//!     
//!     ShellAndWait = True
//!     Exit Function
//!     
//! ErrorHandler:
//!     ShellAndWait = False
//! End Function
//! ```
//!
//! ### Pattern 3: `QuotePath`
//! Ensure path is properly quoted for spaces
//! ```vb
//! Function QuotePath(path As String) As String
//!     If InStr(path, " ") > 0 And Left(path, 1) <> """" Then
//!         QuotePath = """" & path & """"
//!     Else
//!         QuotePath = path
//!     End If
//! End Function
//!
//! ' Usage:
//! taskId = Shell(QuotePath("C:\Program Files\MyApp\app.exe"), vbNormalFocus)
//! ```
//!
//! ### Pattern 4: `OpenFileWithApp`
//! Open file with specific application
//! ```vb
//! Function OpenFileWithApp(appPath As String, filePath As String, _
//!                          Optional windowStyle As Integer = vbNormalFocus) As Boolean
//!     Dim commandLine As String
//!     Dim taskId As Double
//!     
//!     ' Quote paths if they contain spaces
//!     If InStr(appPath, " ") > 0 Then appPath = """" & appPath & """"
//!     If InStr(filePath, " ") > 0 Then filePath = """" & filePath & """"
//!     
//!     commandLine = appPath & " " & filePath
//!     
//!     On Error Resume Next
//!     taskId = Shell(commandLine, windowStyle)
//!     OpenFileWithApp = (taskId <> 0 And Err.Number = 0)
//! End Function
//! ```
//!
//! ### Pattern 5: `ExecuteCommand`
//! Execute command-line command
//! ```vb
//! Function ExecuteCommand(command As String, Optional waitSeconds As Integer = 0) As Double
//!     Dim taskId As Double
//!     Dim endTime As Double
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     ' Run command via cmd.exe
//!     taskId = Shell("cmd.exe /c " & command, vbHide)
//!     
//!     If waitSeconds > 0 Then
//!         endTime = Timer + waitSeconds
//!         Do While Timer < endTime
//!             DoEvents
//!         Loop
//!     End If
//!     
//!     ExecuteCommand = taskId
//!     Exit Function
//!     
//! ErrorHandler:
//!     ExecuteCommand = 0
//! End Function
//! ```
//!
//! ### Pattern 6: `LaunchAndActivate`
//! Launch program and bring to front
//! ```vb
//! Function LaunchAndActivate(programPath As String) As Boolean
//!     Dim taskId As Double
//!     Dim attempts As Integer
//!     
//!     On Error Resume Next
//!     taskId = Shell(programPath, vbNormalFocus)
//!     
//!     If taskId = 0 Then
//!         LaunchAndActivate = False
//!         Exit Function
//!     End If
//!     
//!     ' Try to activate window
//!     For attempts = 1 To 10
//!         DoEvents
//!         AppActivate taskId
//!         If Err.Number = 0 Then
//!             LaunchAndActivate = True
//!             Exit Function
//!         End If
//!         Err.Clear
//!     Next attempts
//!     
//!     LaunchAndActivate = False
//! End Function
//! ```
//!
//! ### Pattern 7: `CheckProgramExists`
//! Verify program exists before shelling
//! ```vb
//! Function CheckProgramExists(programPath As String) As Boolean
//!     On Error Resume Next
//!     CheckProgramExists = (Dir(programPath) <> "")
//! End Function
//!
//! ' Usage:
//! If CheckProgramExists("C:\Tools\mytool.exe") Then
//!     taskId = Shell("C:\Tools\mytool.exe", vbNormalFocus)
//! Else
//!     MsgBox "Program not found"
//! End If
//! ```
//!
//! ### Pattern 8: `ShellWithTimeout`
//! Execute with timeout detection
//! ```vb
//! Function ShellWithTimeout(programPath As String, timeoutSeconds As Integer) As Boolean
//!     Dim taskId As Double
//!     Dim startTime As Double
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     taskId = Shell(programPath, vbNormalFocus)
//!     If taskId = 0 Then
//!         ShellWithTimeout = False
//!         Exit Function
//!     End If
//!     
//!     startTime = Timer
//!     Do While Timer - startTime < timeoutSeconds
//!         DoEvents
//!     Loop
//!     
//!     ShellWithTimeout = True
//!     Exit Function
//!     
//! ErrorHandler:
//!     ShellWithTimeout = False
//! End Function
//! ```
//!
//! ### Pattern 9: `OpenDocument`
//! Open document with default application
//! ```vb
//! Function OpenDocument(filePath As String) As Boolean
//!     Dim taskId As Double
//!     
//!     On Error Resume Next
//!     
//!     ' Use "start" command to open with default app
//!     taskId = Shell("cmd.exe /c start """" """ & filePath & """", vbHide)
//!     
//!     OpenDocument = (taskId <> 0 And Err.Number = 0)
//! End Function
//! ```
//!
//! ### Pattern 10: `RunBatchFile`
//! Execute batch file with parameters
//! ```vb
//! Function RunBatchFile(batchPath As String, parameters As String, _
//!                       Optional hideWindow As Boolean = True) As Double
//!     Dim commandLine As String
//!     Dim windowStyle As Integer
//!     
//!     If InStr(batchPath, " ") > 0 Then
//!         commandLine = """" & batchPath & """"
//!     Else
//!         commandLine = batchPath
//!     End If
//!     
//!     If Len(parameters) > 0 Then
//!         commandLine = commandLine & " " & parameters
//!     End If
//!     
//!     windowStyle = IIf(hideWindow, vbHide, vbNormalFocus)
//!     
//!     On Error Resume Next
//!     RunBatchFile = Shell(commandLine, windowStyle)
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: `ProcessLauncher` Class
//! Manage launching and tracking external processes
//! ```vb
//! ' Class: ProcessLauncher
//! Private m_processes As Collection
//!
//! Private Type ProcessInfo
//!     TaskId As Double
//!     ProgramPath As String
//!     LaunchTime As Date
//!     Description As String
//! End Type
//!
//! Private Sub Class_Initialize()
//!     Set m_processes = New Collection
//! End Sub
//!
//! Public Function LaunchProcess(programPath As String, _
//!                               Optional description As String = "", _
//!                               Optional windowStyle As Integer = vbNormalFocus) As Double
//!     Dim taskId As Double
//!     Dim procInfo As ProcessInfo
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     taskId = Shell(programPath, windowStyle)
//!     
//!     If taskId <> 0 Then
//!         procInfo.TaskId = taskId
//!         procInfo.ProgramPath = programPath
//!         procInfo.LaunchTime = Now
//!         procInfo.Description = description
//!         
//!         m_processes.Add procInfo, CStr(taskId)
//!     End If
//!     
//!     LaunchProcess = taskId
//!     Exit Function
//!     
//! ErrorHandler:
//!     LaunchProcess = 0
//! End Function
//!
//! Public Function ActivateProcess(taskId As Double) As Boolean
//!     On Error Resume Next
//!     AppActivate taskId
//!     ActivateProcess = (Err.Number = 0)
//! End Function
//!
//! Public Function GetProcessCount() As Long
//!     GetProcessCount = m_processes.Count
//! End Function
//!
//! Public Function GetProcessInfo(taskId As Double) As String
//!     Dim procInfo As ProcessInfo
//!     
//!     On Error Resume Next
//!     procInfo = m_processes(CStr(taskId))
//!     
//!     If Err.Number = 0 Then
//!         GetProcessInfo = "Program: " & procInfo.ProgramPath & vbCrLf & _
//!                         "Description: " & procInfo.Description & vbCrLf & _
//!                         "Launched: " & procInfo.LaunchTime & vbCrLf & _
//!                         "Task ID: " & procInfo.TaskId
//!     Else
//!         GetProcessInfo = "Process not found"
//!     End If
//! End Function
//!
//! Public Sub ClearProcesses()
//!     Set m_processes = New Collection
//! End Sub
//! ```
//!
//! ### Example 2: `CommandExecutor` Module
//! Execute command-line commands with output capture
//! ```vb
//! ' Module: CommandExecutor
//!
//! Public Function ExecuteCommandWithOutput(command As String, _
//!                                          ByRef output As String) As Boolean
//!     Dim tempFile As String
//!     Dim fileNum As Integer
//!     Dim commandLine As String
//!     Dim taskId As Double
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     ' Create temp file for output
//!     tempFile = Environ("TEMP") & "\cmdout_" & Format(Now, "yyyymmddhhnnss") & ".txt"
//!     
//!     ' Redirect output to temp file
//!     commandLine = "cmd.exe /c " & command & " > """ & tempFile & """ 2>&1"
//!     
//!     taskId = Shell(commandLine, vbHide)
//!     If taskId = 0 Then
//!         ExecuteCommandWithOutput = False
//!         Exit Function
//!     End If
//!     
//!     ' Wait for command to complete (primitive wait)
//!     Sleep 1000  ' Would need to declare Sleep API
//!     
//!     ' Read output file
//!     fileNum = FreeFile
//!     Open tempFile For Input As #fileNum
//!     output = Input(LOF(fileNum), #fileNum)
//!     Close #fileNum
//!     
//!     ' Clean up
//!     Kill tempFile
//!     
//!     ExecuteCommandWithOutput = True
//!     Exit Function
//!     
//! ErrorHandler:
//!     If fileNum > 0 Then Close #fileNum
//!     On Error Resume Next
//!     If Dir(tempFile) <> "" Then Kill tempFile
//!     ExecuteCommandWithOutput = False
//! End Function
//!
//! Public Function RunCommandHidden(command As String) As Boolean
//!     Dim taskId As Double
//!     
//!     On Error Resume Next
//!     taskId = Shell("cmd.exe /c " & command, vbHide)
//!     RunCommandHidden = (taskId <> 0 And Err.Number = 0)
//! End Function
//!
//! Public Function RunCommandVisible(command As String) As Double
//!     On Error Resume Next
//!     RunCommandVisible = Shell("cmd.exe /k " & command, vbNormalFocus)
//! End Function
//! ```
//!
//! ### Example 3: `ApplicationLauncher` Class
//! Launch applications with comprehensive error handling
//! ```vb
//! ' Class: ApplicationLauncher
//! Private m_lastError As String
//! Private m_lastTaskId As Double
//!
//! Public Function LaunchApplication(programPath As String, _
//!                                   Optional arguments As String = "", _
//!                                   Optional windowStyle As Integer = vbNormalFocus, _
//!                                   Optional verifyExists As Boolean = True) As Boolean
//!     Dim fullCommand As String
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     m_lastError = ""
//!     m_lastTaskId = 0
//!     
//!     ' Verify program exists
//!     If verifyExists Then
//!         If Dir(programPath) = "" Then
//!             m_lastError = "Program not found: " & programPath
//!             LaunchApplication = False
//!             Exit Function
//!         End If
//!     End If
//!     
//!     ' Build command line
//!     fullCommand = programPath
//!     If InStr(fullCommand, " ") > 0 And Left(fullCommand, 1) <> """" Then
//!         fullCommand = """" & fullCommand & """"
//!     End If
//!     
//!     If Len(arguments) > 0 Then
//!         fullCommand = fullCommand & " " & arguments
//!     End If
//!     
//!     ' Launch
//!     m_lastTaskId = Shell(fullCommand, windowStyle)
//!     
//!     If m_lastTaskId = 0 Then
//!         m_lastError = "Shell function returned 0"
//!         LaunchApplication = False
//!     Else
//!         LaunchApplication = True
//!     End If
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     m_lastError = "Error " & Err.Number & ": " & Err.Description
//!     LaunchApplication = False
//! End Function
//!
//! Public Function LaunchAndActivate(programPath As String, _
//!                                   Optional arguments As String = "") As Boolean
//!     Dim success As Boolean
//!     Dim attempts As Integer
//!     
//!     success = LaunchApplication(programPath, arguments, vbNormalFocus)
//!     
//!     If Not success Then
//!         LaunchAndActivate = False
//!         Exit Function
//!     End If
//!     
//!     ' Try to activate
//!     For attempts = 1 To 20
//!         DoEvents
//!         On Error Resume Next
//!         AppActivate m_lastTaskId
//!         If Err.Number = 0 Then
//!             LaunchAndActivate = True
//!             Exit Function
//!         End If
//!         Err.Clear
//!     Next attempts
//!     
//!     LaunchAndActivate = False
//! End Function
//!
//! Public Property Get LastError() As String
//!     LastError = m_lastError
//! End Property
//!
//! Public Property Get LastTaskId() As Double
//!     LastTaskId = m_lastTaskId
//! End Property
//!
//! Public Function OpenFileWithDefaultApp(filePath As String) As Boolean
//!     Dim commandLine As String
//!     
//!     ' Use Windows "start" command
//!     commandLine = "cmd.exe /c start """" """ & filePath & """"
//!     
//!     On Error Resume Next
//!     m_lastTaskId = Shell(commandLine, vbHide)
//!     
//!     OpenFileWithDefaultApp = (m_lastTaskId <> 0 And Err.Number = 0)
//!     
//!     If Not OpenFileWithDefaultApp Then
//!         m_lastError = "Failed to open file: " & Err.Description
//!     End If
//! End Function
//! ```
//!
//! ### Example 4: `BatchFileRunner` Module
//! Execute batch files with enhanced functionality
//! ```vb
//! ' Module: BatchFileRunner
//!
//! Public Function ExecuteBatchFile(batchPath As String, _
//!                                  Optional parameters As String = "", _
//!                                  Optional visible As Boolean = False, _
//!                                  Optional workingDir As String = "") As Double
//!     Dim commandLine As String
//!     Dim windowStyle As Integer
//!     Dim originalDir As String
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     ' Verify batch file exists
//!     If Dir(batchPath) = "" Then
//!         MsgBox "Batch file not found: " & batchPath, vbExclamation
//!         ExecuteBatchFile = 0
//!         Exit Function
//!     End If
//!     
//!     ' Quote path if needed
//!     If InStr(batchPath, " ") > 0 Then
//!         commandLine = """" & batchPath & """"
//!     Else
//!         commandLine = batchPath
//!     End If
//!     
//!     ' Add parameters
//!     If Len(parameters) > 0 Then
//!         commandLine = commandLine & " " & parameters
//!     End If
//!     
//!     ' Change working directory if specified
//!     If Len(workingDir) > 0 Then
//!         originalDir = CurDir
//!         ChDir workingDir
//!     End If
//!     
//!     ' Execute
//!     windowStyle = IIf(visible, vbNormalFocus, vbHide)
//!     ExecuteBatchFile = Shell(commandLine, windowStyle)
//!     
//!     ' Restore directory
//!     If Len(workingDir) > 0 Then
//!         ChDir originalDir
//!     End If
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     If Len(workingDir) > 0 Then
//!         On Error Resume Next
//!         ChDir originalDir
//!     End If
//!     ExecuteBatchFile = 0
//! End Function
//!
//! Public Function RunBatchWithLog(batchPath As String, logPath As String) As Boolean
//!     Dim commandLine As String
//!     Dim taskId As Double
//!     
//!     ' Quote paths
//!     If InStr(batchPath, " ") > 0 Then batchPath = """" & batchPath & """"
//!     If InStr(logPath, " ") > 0 Then logPath = """" & logPath & """"
//!     
//!     ' Redirect output to log
//!     commandLine = "cmd.exe /c " & batchPath & " > " & logPath & " 2>&1"
//!     
//!     On Error Resume Next
//!     taskId = Shell(commandLine, vbHide)
//!     
//!     RunBatchWithLog = (taskId <> 0 And Err.Number = 0)
//! End Function
//!
//! Public Function CreateAndRunBatch(commands() As String, _
//!                                   Optional visible As Boolean = False) As Boolean
//!     Dim batchPath As String
//!     Dim fileNum As Integer
//!     Dim i As Long
//!     Dim taskId As Double
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     ' Create temp batch file
//!     batchPath = Environ("TEMP") & "\temp_" & Format(Now, "yyyymmddhhnnss") & ".bat"
//!     
//!     ' Write commands
//!     fileNum = FreeFile
//!     Open batchPath For Output As #fileNum
//!     Print #fileNum, "@echo off"
//!     For i = LBound(commands) To UBound(commands)
//!         Print #fileNum, commands(i)
//!     Next i
//!     Close #fileNum
//!     
//!     ' Execute
//!     taskId = Shell(batchPath, IIf(visible, vbNormalFocus, vbHide))
//!     
//!     CreateAndRunBatch = (taskId <> 0)
//!     Exit Function
//!     
//! ErrorHandler:
//!     If fileNum > 0 Then Close #fileNum
//!     CreateAndRunBatch = False
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The Shell function can generate the following errors:
//!
//! - **Error 5** (Invalid procedure call or argument): Invalid windowstyle parameter
//! - **Error 53** (File not found): Program or path not found
//! - **Error 76** (Path not found): Directory in path doesn't exist
//!
//! Always use error handling when executing external programs:
//! ```vb
//! On Error Resume Next
//! taskId = Shell(programPath, vbNormalFocus)
//! If Err.Number <> 0 Or taskId = 0 Then
//!     MsgBox "Error: " & Err.Description
//! End If
//! ```
//!
//! ## Performance Considerations
//!
//! - Shell executes asynchronously and returns immediately
//! - No performance impact on VB6 app after launch
//! - Multiple programs can be launched simultaneously
//! - Task ID allows tracking and activation
//! - Consider resource usage when launching many programs
//! - Use `DoEvents` to allow UI updates after Shell
//!
//! ## Best Practices
//!
//! 1. **Error Handling**: Always wrap Shell in error handling
//! 2. **Path Quoting**: Quote paths with spaces using double quotes
//! 3. **Verify Existence**: Check file exists before shelling (Dir function)
//! 4. **Security**: Validate user input to prevent command injection
//! 5. **Resource Management**: Track launched processes
//! 6. **Window Style**: Choose appropriate window style for user experience
//! 7. **Wait Strategy**: Use `AppActivate` or API for synchronization if needed
//! 8. **Return Value**: Check return value (0 = failure)
//! 9. **Documentation**: Document external dependencies
//! 10. **Testing**: Test with various paths and edge cases
//!
//! ## Comparison with Related Functions
//!
//! | Method | Purpose | Wait for Completion | Capture Output | Platform |
//! |--------|---------|---------------------|----------------|----------|
//! | Shell | Execute program | No (async) | No | VB6/VBA |
//! | `CreateProcess` API | Execute program | Optional | Optional | Windows API |
//! | WScript.Shell.Run | Execute program | Optional | No | WSH |
//! | Exec method | Execute program | No | Yes | WSH |
//! | `ShellExecute` API | Execute/open files | No | No | Windows API |
//!
//! ## Platform Considerations
//!
//! - Available in VB6, VBA (Windows only)
//! - Windows-specific function
//! - Task ID is Windows-specific process identifier
//! - Path separators are backslashes (\)
//! - Case-insensitive on Windows
//! - Long filename support (use quotes)
//! - Windows versions may affect behavior
//!
//! ## Limitations
//!
//! - No built-in way to wait for completion
//! - Cannot capture standard output/error directly
//! - Limited to Windows operating system
//! - Task ID becomes invalid when process terminates
//! - No parent-child process relationship tracking
//! - Cannot pass structured data to launched program
//! - Security risks with user-supplied paths
//! - No control over process priority or environment
//!
//! ## Related Functions
//!
//! - `AppActivate`: Activates a running application window by task ID
//! - `CreateObject`: Creates automation objects for inter-app communication
//! - `SendKeys`: Sends keystrokes to active window
//! - `Dir`: Verifies file existence before shelling
//! - `Environ`: Gets environment variables for path construction

use crate::error::VBResult;
use crate::state;
use crate::value::{VBLong, VBString, VBVariant};

/// Implement VB6's `Shell` function.
///
/// Validates the optional `windowstyle` argument against the six documented
/// `VbAppWinStyle` constants (error 5 otherwise; a Null argument raises
/// error 94), then hands the request to the active
/// [`interaction backend`](crate::state::interaction) and returns the task
/// ID as a Variant (Double). The backend decides what "start a program"
/// means here: the native backends launch a real detached process (the task
/// ID is its process id), while the memory backend answers from a scripted
/// response list without touching the OS.
pub fn shell(pathname: &VBString, window_style: Option<&VBVariant>) -> VBResult<VBVariant> {
    let raw_style = match window_style {
        // VB6's documented default for an omitted argument:
        // "the program is started minimized with focus".
        None => state::interaction::WindowStyle::default().id(),
        Some(style) => i64::from(VBLong::try_from(style)?.as_i32()),
    };
    let request = state::interaction::ShellRequest::parse(pathname.as_str(), raw_style)?;
    let task_id = state::interaction::shell(&request)?;
    Ok(VBVariant::from_double(task_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;
    use crate::state::interaction::{memory::MemoryBackend, WindowStyle};
    use crate::state::test_support::lock_test;

    fn set_backend(backend: MemoryBackend) {
        state::interaction::set_backend(Box::new(backend));
    }

    #[test]
    fn returns_the_task_id_as_a_double() {
        let _guard = lock_test();
        set_backend(MemoryBackend::with_shell_responses([4242.0]));
        let result = shell(&VBString::from("notepad.exe"), None).unwrap();
        assert_eq!(result.as_f64().unwrap(), 4242.0);
        state::interaction::reset_backend();
    }

    #[test]
    fn omitted_style_defaults_to_minimized_with_focus() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        shell(&VBString::from("notepad.exe"), None).unwrap();

        let requests = state::interaction::with_memory_backend(|m| m.shell_requests()).unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].pathname, "notepad.exe");
        assert_eq!(requests[0].window_style, WindowStyle::MinimizedFocus);
        state::interaction::reset_backend();
    }

    #[test]
    fn explicit_styles_flow_through_to_the_backend() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        shell(
            &VBString::from("calc.exe"),
            Some(&VBVariant::from_long(3)), // vbMaximizedFocus
        )
        .unwrap();
        shell(
            &VBString::from("backup.bat"),
            Some(&VBVariant::from_integer(0)), // vbHide
        )
        .unwrap();

        let requests = state::interaction::with_memory_backend(|m| m.shell_requests()).unwrap();
        assert_eq!(requests[0].window_style, WindowStyle::MaximizedFocus);
        assert_eq!(requests[1].window_style, WindowStyle::Hide);
        state::interaction::reset_backend();
    }

    #[test]
    fn undefined_window_style_is_error_5() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        // 5 sits between vbNormalNoFocus (4) and vbMinimizedNoFocus (6).
        let err = shell(&VBString::from("x"), Some(&VBVariant::from_long(5))).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(
            err.description.contains("vbMinimizedNoFocus"),
            "{}",
            err.description
        );
        state::interaction::reset_backend();
    }

    #[test]
    fn null_window_style_is_invalid_use_of_null() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let err = shell(&VBString::from("x"), Some(&VBVariant::Null)).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
        state::interaction::reset_backend();
    }

    #[test]
    fn non_numeric_window_style_is_type_mismatch() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let err = shell(
            &VBString::from("x"),
            Some(&VBVariant::from_string("normal")),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
        state::interaction::reset_backend();
    }

    #[test]
    fn scripted_responses_are_returned_in_request_order() {
        let _guard = lock_test();
        set_backend(MemoryBackend::with_shell_responses([111.0, 222.0]));
        assert_eq!(
            shell(&VBString::from("a.exe"), None)
                .unwrap()
                .as_f64()
                .unwrap(),
            111.0
        );
        assert_eq!(
            shell(&VBString::from("b.exe"), None)
                .unwrap()
                .as_f64()
                .unwrap(),
            222.0
        );
        state::interaction::reset_backend();
    }
}
