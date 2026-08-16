//! # Environ Function
//!
//! Returns the String value associated with an operating system environment variable.
//!
//! ## Syntax
//!
//! ```vb
//! Environ(envstring | number)
//! ```
//!
//! ## Parameters
//!
//! - **envstring**: Optional (if number provided). String expression containing the name of an
//!   environment variable.
//! - **number**: Optional (if envstring provided). Numeric expression corresponding to the
//!   numeric order of an environment string in the environment-string table. The number argument
//!   can be any numeric expression, but is rounded to a whole number before it is evaluated.
//!
//! ## Return Value
//!
//! Returns a String containing the value assigned to envstring or the environment variable at
//! position number. Returns a zero-length string ("") if envstring is not found or if there is
//! no environment string at position number.
//!
//! ## Remarks
//!
//! The `Environ` function retrieves values from the operating system's environment variables.
//! Environment variables are system-level or user-level settings that provide information about
//! the operating system environment.
//!
//! **Important Characteristics:**
//!
//! - Reads environment variables from the operating system
//! - Can access by name (string) or position (number)
//! - Case-insensitive on Windows
//! - Returns empty string if variable not found
//! - Position-based access starts at 1 (not 0)
//! - Number of environment variables varies by system
//! - Environment changes during execution are not reflected
//! - Snapshot taken at application start
//! - Cannot modify environment variables (read-only)
//! - Different users may have different environment variables
//!
//! ## Common Environment Variables
//!
//! **Windows:**
//! - `PATH` - Executable search path
//! - `TEMP` or `TMP` - Temporary files directory
//! - `USERNAME` - Current user name
//! - `USERPROFILE` - User's profile directory
//! - `COMPUTERNAME` - Computer name
//! - `SYSTEMROOT` - Windows system directory
//! - `PROGRAMFILES` - Program Files directory
//! - `HOMEDRIVE` - User's home drive (e.g., C:)
//! - `HOMEPATH` - User's home directory path
//! - `APPDATA` - Application data directory
//! - `WINDIR` - Windows directory
//! - `PROCESSOR_ARCHITECTURE` - CPU architecture
//! - `NUMBER_OF_PROCESSORS` - Number of CPU cores
//! - `OS` - Operating system name
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! ' Get environment variable by name
//! Dim userName As String
//! userName = Environ("USERNAME")
//! MsgBox "Current user: " & userName
//!
//! ' Get temp directory
//! Dim tempDir As String
//! tempDir = Environ("TEMP")
//!
//! ' Get by position
//! Dim firstEnvVar As String
//! firstEnvVar = Environ(1)
//! ```
//!
//! ### Check if Variable Exists
//!
//! ```vb
//! Function EnvironVarExists(varName As String) As Boolean
//!     EnvironVarExists = (Len(Environ(varName)) > 0)
//! End Function
//!
//! ' Usage
//! If EnvironVarExists("JAVA_HOME") Then
//!     MsgBox "Java is configured"
//! Else
//!     MsgBox "Java not found"
//! End If
//! ```
//!
//! ### Build File Paths
//!
//! ```vb
//! Function GetTempFilePath(fileName As String) As String
//!     Dim tempDir As String
//!     tempDir = Environ("TEMP")
//!     
//!     If Right(tempDir, 1) <> "\" Then
//!         tempDir = tempDir & "\"
//!     End If
//!     
//!     GetTempFilePath = tempDir & fileName
//! End Function
//! ```
//!
//! ## Common Patterns
//!
//! ### Get User Directories
//!
//! ```vb
//! Function GetUserProfile() As String
//!     GetUserProfile = Environ("USERPROFILE")
//! End Function
//!
//! Function GetAppDataPath() As String
//!     GetAppDataPath = Environ("APPDATA")
//! End Function
//!
//! Function GetDesktopPath() As String
//!     GetDesktopPath = Environ("USERPROFILE") & "\Desktop"
//! End Function
//!
//! Function GetMyDocuments() As String
//!     GetMyDocuments = Environ("USERPROFILE") & "\Documents"
//! End Function
//! ```
//!
//! ### System Information
//!
//! ```vb
//! Function GetComputerName() As String
//!     GetComputerName = Environ("COMPUTERNAME")
//! End Function
//!
//! Function GetProcessorCount() As Integer
//!     Dim procCount As String
//!     procCount = Environ("NUMBER_OF_PROCESSORS")
//!     
//!     If IsNumeric(procCount) Then
//!         GetProcessorCount = CInt(procCount)
//!     Else
//!         GetProcessorCount = 1
//!     End If
//! End Function
//!
//! Function GetSystemArchitecture() As String
//!     GetSystemArchitecture = Environ("PROCESSOR_ARCHITECTURE")
//! End Function
//! ```
//!
//! ### List All Environment Variables
//!
//! ```vb
//! Sub ListAllEnvironmentVariables()
//!     Dim i As Integer
//!     Dim envVar As String
//!     
//!     i = 1
//!     Do
//!         envVar = Environ(i)
//!         If envVar = "" Then Exit Do
//!         
//!         Debug.Print i & ": " & envVar
//!         i = i + 1
//!     Loop
//! End Sub
//! ```
//!
//! ### Parse Environment Variable
//!
//! ```vb
//! Function GetEnvironVarName(envString As String) As String
//!     Dim equalPos As Integer
//!     equalPos = InStr(envString, "=")
//!     
//!     If equalPos > 0 Then
//!         GetEnvironVarName = Left(envString, equalPos - 1)
//!     Else
//!         GetEnvironVarName = ""
//!     End If
//! End Function
//!
//! Function GetEnvironVarValue(envString As String) As String
//!     Dim equalPos As Integer
//!     equalPos = InStr(envString, "=")
//!     
//!     If equalPos > 0 Then
//!         GetEnvironVarValue = Mid(envString, equalPos + 1)
//!     Else
//!         GetEnvironVarValue = ""
//!     End If
//! End Function
//! ```
//!
//! ### Safe Path Construction
//!
//! ```vb
//! Function BuildSafePath(envVar As String, subPath As String) As String
//!     Dim basePath As String
//!     basePath = Environ(envVar)
//!     
//!     If basePath = "" Then
//!         BuildSafePath = ""
//!         Exit Function
//!     End If
//!     
//!     ' Ensure path ends with backslash
//!     If Right(basePath, 1) <> "\" Then
//!         basePath = basePath & "\"
//!     End If
//!     
//!     ' Remove leading backslash from subPath if present
//!     If Left(subPath, 1) = "\" Then
//!         subPath = Mid(subPath, 2)
//!     End If
//!     
//!     BuildSafePath = basePath & subPath
//! End Function
//! ```
//!
//! ### Configuration File Paths
//!
//! ```vb
//! Function GetConfigFilePath(appName As String, fileName As String) As String
//!     Dim appDataPath As String
//!     Dim configDir As String
//!     
//!     appDataPath = Environ("APPDATA")
//!     configDir = appDataPath & "\" & appName
//!     
//!     ' Create directory if it doesn't exist
//!     If Dir(configDir, vbDirectory) = "" Then
//!         MkDir configDir
//!     End If
//!     
//!     GetConfigFilePath = configDir & "\" & fileName
//! End Function
//! ```
//!
//! ### Search PATH Variable
//!
//! ```vb
//! Function FindInPath(executable As String) As String
//!     Dim pathVar As String
//!     Dim paths() As String
//!     Dim i As Integer
//!     Dim testPath As String
//!     
//!     pathVar = Environ("PATH")
//!     paths = Split(pathVar, ";")
//!     
//!     For i = LBound(paths) To UBound(paths)
//!         testPath = paths(i) & "\" & executable
//!         If Dir(testPath) <> "" Then
//!             FindInPath = testPath
//!             Exit Function
//!         End If
//!     Next i
//!     
//!     FindInPath = ""
//! End Function
//! ```
//!
//! ### Check Operating System
//!
//! ```vb
//! Function IsWindows() As Boolean
//!     Dim osVar As String
//!     osVar = UCase(Environ("OS"))
//!     IsWindows = (InStr(osVar, "WINDOWS") > 0)
//! End Function
//!
//! Function GetWindowsDirectory() As String
//!     GetWindowsDirectory = Environ("WINDIR")
//! End Function
//!
//! Function GetSystemRoot() As String
//!     GetSystemRoot = Environ("SYSTEMROOT")
//! End Function
//! ```
//!
//! ### Program Files Paths
//!
//! ```vb
//! Function GetProgramFilesPath() As String
//!     GetProgramFilesPath = Environ("PROGRAMFILES")
//! End Function
//!
//! Function GetProgramFilesX86Path() As String
//!     GetProgramFilesX86Path = Environ("PROGRAMFILES(X86)")
//! End Function
//!
//! Function FindProgramPath(programName As String) As String
//!     Dim progFiles As String
//!     Dim testPath As String
//!     
//!     ' Check Program Files
//!     progFiles = Environ("PROGRAMFILES")
//!     testPath = progFiles & "\" & programName
//!     If Dir(testPath, vbDirectory) <> "" Then
//!         FindProgramPath = testPath
//!         Exit Function
//!     End If
//!     
//!     ' Check Program Files (x86)
//!     progFiles = Environ("PROGRAMFILES(X86)")
//!     If progFiles <> "" Then
//!         testPath = progFiles & "\" & programName
//!         If Dir(testPath, vbDirectory) <> "" Then
//!             FindProgramPath = testPath
//!             Exit Function
//!         End If
//!     End If
//!     
//!     FindProgramPath = ""
//! End Function
//! ```
//!
//! ## Advanced Usage
//!
//! ### Environment Variable Dictionary
//!
//! ```vb
//! Function GetEnvironmentDictionary() As Collection
//!     Dim dict As New Collection
//!     Dim i As Integer
//!     Dim envVar As String
//!     Dim varName As String
//!     Dim varValue As String
//!     Dim equalPos As Integer
//!     
//!     i = 1
//!     Do
//!         envVar = Environ(i)
//!         If envVar = "" Then Exit Do
//!         
//!         equalPos = InStr(envVar, "=")
//!         If equalPos > 0 Then
//!             varName = Left(envVar, equalPos - 1)
//!             varValue = Mid(envVar, equalPos + 1)
//!             
//!             On Error Resume Next
//!             dict.Add varValue, UCase(varName)
//!             On Error GoTo 0
//!         End If
//!         
//!         i = i + 1
//!     Loop
//!     
//!     Set GetEnvironmentDictionary = dict
//! End Function
//! ```
//!
//! ### Expand Environment Variables in String
//!
//! ```vb
//! Function ExpandEnvironmentString(inputString As String) As String
//!     Dim result As String
//!     Dim startPos As Integer
//!     Dim endPos As Integer
//!     Dim varName As String
//!     Dim varValue As String
//!     
//!     result = inputString
//!     
//!     ' Find %VAR% patterns
//!     Do
//!         startPos = InStr(result, "%")
//!         If startPos = 0 Then Exit Do
//!         
//!         endPos = InStr(startPos + 1, result, "%")
//!         If endPos = 0 Then Exit Do
//!         
//!         varName = Mid(result, startPos + 1, endPos - startPos - 1)
//!         varValue = Environ(varName)
//!         
//!         result = Left(result, startPos - 1) & varValue & Mid(result, endPos + 1)
//!     Loop
//!     
//!     ExpandEnvironmentString = result
//! End Function
//!
//! ' Usage
//! expandedPath = ExpandEnvironmentString("%TEMP%\myfile.txt")
//! ```
//!
//! ### Create Application Log File
//!
//! ```vb
//! Function CreateLogFile(appName As String) As String
//!     Dim logDir As String
//!     Dim logFile As String
//!     Dim dateStamp As String
//!     
//!     logDir = Environ("TEMP") & "\Logs"
//!     
//!     ' Create logs directory
//!     If Dir(logDir, vbDirectory) = "" Then
//!         MkDir logDir
//!     End If
//!     
//!     dateStamp = Format(Date, "yyyy-mm-dd")
//!     logFile = logDir & "\" & appName & "_" & dateStamp & ".log"
//!     
//!     CreateLogFile = logFile
//! End Function
//! ```
//!
//! ### Check Development Environment
//!
//! ```vb
//! Function IsDevelopmentEnvironment() As Boolean
//!     ' Check for common development environment variables
//!     IsDevelopmentEnvironment = (Len(Environ("VSCODE_PID")) > 0) Or _
//!                               (Len(Environ("TERM_PROGRAM")) > 0) Or _
//!                               (Len(Environ("VSAPPIDDIR")) > 0)
//! End Function
//!
//! Function GetJavaHome() As String
//!     GetJavaHome = Environ("JAVA_HOME")
//! End Function
//!
//! Function GetPythonPath() As String
//!     GetPythonPath = Environ("PYTHONPATH")
//! End Function
//! ```
//!
//! ### Build Connection String
//!
//! ```vb
//! Function BuildConnectionString() As String
//!     Dim server As String
//!     Dim database As String
//!     
//!     server = Environ("DB_SERVER")
//!     database = Environ("DB_NAME")
//!     
//!     If server = "" Then server = "localhost"
//!     If database = "" Then database = "default"
//!     
//!     BuildConnectionString = "Server=" & server & ";Database=" & database
//! End Function
//! ```
//!
//! ### Export Environment to File
//!
//! ```vb
//! Sub ExportEnvironmentToFile(filePath As String)
//!     Dim fileNum As Integer
//!     Dim i As Integer
//!     Dim envVar As String
//!     
//!     fileNum = FreeFile
//!     Open filePath For Output As #fileNum
//!     
//!     Print #fileNum, "Environment Variables"
//!     Print #fileNum, "Generated: " & Now
//!     Print #fileNum, String(80, "-")
//!     
//!     i = 1
//!     Do
//!         envVar = Environ(i)
//!         If envVar = "" Then Exit Do
//!         
//!         Print #fileNum, envVar
//!         i = i + 1
//!     Loop
//!     
//!     Close #fileNum
//! End Sub
//! ```
//!
//! ### Portable Path Builder
//!
//! ```vb
//! Function GetPortableAppPath(relativePath As String) As String
//!     Dim basePath As String
//!     
//!     ' Try to get from environment first
//!     basePath = Environ("APP_BASE_PATH")
//!     
//!     ' Fall back to current directory
//!     If basePath = "" Then
//!         basePath = App.Path
//!     End If
//!     
//!     If Right(basePath, 1) <> "\" Then
//!         basePath = basePath & "\"
//!     End If
//!     
//!     GetPortableAppPath = basePath & relativePath
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! Function SafeEnviron(varName As String, Optional defaultValue As String = "") As String
//!     On Error Resume Next
//!     SafeEnviron = Environ(varName)
//!     
//!     If Err.Number <> 0 Or SafeEnviron = "" Then
//!         SafeEnviron = defaultValue
//!     End If
//! End Function
//!
//! Function GetEnvironWithFallback(preferredVar As String, fallbackVar As String) As String
//!     GetEnvironWithFallback = Environ(preferredVar)
//!     
//!     If GetEnvironWithFallback = "" Then
//!         GetEnvironWithFallback = Environ(fallbackVar)
//!     End If
//! End Function
//! ```
//!
//! ### Common Errors
//!
//! - **Error 5** (Invalid procedure call): Invalid number argument (< 1)
//! - No error for missing variables (returns empty string)
//!
//! ## Performance Considerations
//!
//! - `Environ` is relatively fast (direct OS call)
//! - Environment snapshot taken at application start
//! - Cache frequently used values to avoid repeated calls
//! - Position-based enumeration stops at first empty string
//! - String comparison is case-insensitive on Windows
//!
//! ## Best Practices
//!
//! ### Always Check for Empty String
//!
//! ```vb
//! ' Good - Check before using
//! tempDir = Environ("TEMP")
//! If tempDir = "" Then
//!     tempDir = "C:\Temp"  ' Fallback
//! End If
//!
//! ' Avoid - Assuming variable exists
//! tempDir = Environ("TEMP")  ' May be empty!
//! ```
//!
//! ### Use Constants for Variable Names
//!
//! ```vb
//! ' Good - Constants for maintainability
//! Const ENV_TEMP = "TEMP"
//! Const ENV_USERNAME = "USERNAME"
//!
//! tempDir = Environ(ENV_TEMP)
//! userName = Environ(ENV_USERNAME)
//! ```
//!
//! ### Provide Defaults
//!
//! ```vb
//! Function GetTempDir() As String
//!     GetTempDir = Environ("TEMP")
//!     If GetTempDir = "" Then GetTempDir = Environ("TMP")
//!     If GetTempDir = "" Then GetTempDir = "C:\Temp"
//! End Function
//! ```
//!
//! ### Case Insensitive on Windows
//!
//! ```vb
//! ' All equivalent on Windows
//! userName = Environ("USERNAME")
//! userName = Environ("username")
//! userName = Environ("UserName")
//! ```
//!
//! ## Comparison with Other Methods
//!
//! ### Environ vs Registry
//!
//! ```vb
//! ' Environ - Quick, read-only access to environment
//! tempDir = Environ("TEMP")
//!
//! ' Registry - More control, can read/write, more complex
//! ' (Requires Windows API or Registry object)
//! ```
//!
//! ### Environ vs Command Line
//!
//! ```vb
//! ' Environ - Environment variables
//! userName = Environ("USERNAME")
//!
//! ' Command - Command line arguments
//! args = Command()
//! ```
//!
//! ## Limitations
//!
//! - Read-only access (cannot modify environment variables)
//! - Snapshot at application start (changes not reflected)
//! - Position-based enumeration order not guaranteed
//! - Cannot create or delete environment variables
//! - Limited to process environment (not system-wide)
//! - No wildcard or pattern matching
//! - Case-sensitive on Unix/Linux (VB6 primarily Windows)
//!
//! ## Related Functions
//!
//! - `Command`: Returns command-line arguments
//! - `CurDir`: Returns current directory
//! - `ChDir`: Changes current directory
//! - `App.Path`: Returns application path
//! - `Shell`: Executes external programs (can set environment)
//! - `GetSetting`: Reads application settings from registry
//! - `SaveSetting`: Writes application settings to registry

use crate::error::VBResult;
use crate::value::VBVariant;

use super::environ_dollar::environ_dollar;

/// Returns the value of an environment variable, exactly as [`environ_dollar`]
/// does, except that a `Null` argument propagates as `Null` instead of raising
/// an error.
///
/// `Environ` is the Variant-returning counterpart of `Environ$`; its runtime
/// semantics are otherwise identical (per the VBAL specification, the two
/// functions differ only in the declared type of the return value).
pub fn environ(arg: &VBVariant) -> VBResult<VBVariant> {
    if arg.is_null() {
        return Ok(VBVariant::Null);
    }
    environ_dollar(arg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::environment;
    use crate::state::test_support::{position_of, TEST_LOCK};

    fn reset_with_sample_env() {
        environment::reset();
        environment::set_env("VB6_ENVIRON_PATH", "C:\\bin");
        environment::set_env("VB6_ENVIRON_USER", "arthur");
    }

    #[test]
    fn returns_value_for_existing_variable() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        assert_eq!(
            environ(&VBVariant::from_string("VB6_ENVIRON_PATH")).unwrap(),
            VBVariant::from_string("C:\\bin")
        );
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        assert_eq!(
            environ(&VBVariant::from_string("vb6_environ_user")).unwrap(),
            VBVariant::from_string("arthur")
        );
    }

    #[test]
    fn returns_empty_string_for_unknown_variable() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        assert_eq!(
            environ(&VBVariant::from_string("VB6_ENVIRON_MISSING")).unwrap(),
            VBVariant::from_string("")
        );
    }

    #[test]
    fn numeric_argument_returns_position_including_equals() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        let path = position_of("VB6_ENVIRON_PATH") as i16;
        let user = position_of("VB6_ENVIRON_USER") as i16;
        assert_eq!(
            environ(&VBVariant::from_integer(path)).unwrap(),
            VBVariant::from_string("VB6_ENVIRON_PATH=C:\\bin")
        );
        assert_eq!(
            environ(&VBVariant::from_integer(user)).unwrap(),
            VBVariant::from_string("VB6_ENVIRON_USER=arthur")
        );
    }

    #[test]
    fn numeric_argument_is_rounded_to_a_whole_number() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        let user = position_of("VB6_ENVIRON_USER") as f64;
        // 0.4 below the position rounds down to the user entry.
        assert_eq!(
            environ(&VBVariant::from_double(user + 0.4)).unwrap(),
            VBVariant::from_string("VB6_ENVIRON_USER=arthur")
        );
    }

    #[test]
    fn out_of_range_position_returns_empty_string() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        let last = position_of("VB6_ENVIRON_USER") as i16;
        assert_eq!(
            environ(&VBVariant::from_integer(last + 1)).unwrap(),
            VBVariant::from_string("")
        );
    }

    #[test]
    fn index_below_one_is_error_5() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        let err = environ(&VBVariant::from_integer(0)).unwrap_err();
        assert_eq!(err.number, crate::error::err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn null_propagates_as_null() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        assert_eq!(environ(&VBVariant::Null).unwrap(), VBVariant::Null);
    }

    #[test]
    fn empty_is_error_5() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        assert_eq!(
            environ(&VBVariant::Empty).unwrap_err().number,
            crate::error::err_number::INVALID_PROCEDURE_CALL
        );
    }
}
