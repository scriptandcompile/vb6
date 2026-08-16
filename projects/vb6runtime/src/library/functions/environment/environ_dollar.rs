//! # `Environ$` Function
//!
//! Returns the string value associated with an environment variable.
//!
//! ## Syntax
//!
//! ```vb6
//! Environ$(envstring)
//! Environ$(number)
//! ```
//!
//! ## Parameters
//!
//! - `envstring`: A string expression containing the name of an environment variable.
//! - `number`: A numeric expression corresponding to the numeric order of an environment string in the environment-string table. The number argument can be any numeric expression, but is rounded to a whole number before it is evaluated.
//!
//! ## Return Value
//!
//! Returns a `String` containing the text assigned to the specified environment variable. If the environment variable doesn't exist, returns an empty string.
//!
//! ## Remarks
//!
//! The `Environ$` function returns the string assigned to the specified environment variable from the operating system's environment-string table. This function cannot be used on the left side of an assignment statement.
//!
//! When using a numeric argument, `Environ$` returns the string that occupies that numeric position in the environment table. In this case, `Environ$` returns all the text including the equal sign (=). If there's no environment string at the specified position, `Environ$` returns a zero-length string.
//!
//! When using a string argument, if the environment variable doesn't exist, a zero-length string is returned.
//!
//! ## Typical Uses
//!
//! ### Example 1: Getting System Path
//! ```vb6
//! Dim systemPath As String
//! systemPath = Environ$("PATH")
//! ```
//!
//! ### Example 2: Getting Temp Directory
//! ```vb6
//! Dim tempDir As String
//! tempDir = Environ$("TEMP")
//! ```
//!
//! ### Example 3: Getting User Name
//! ```vb6
//! Dim userName As String
//! userName = Environ$("USERNAME")
//! ```
//!
//! ### Example 4: Iterating Environment Variables
//! ```vb6
//! Dim i As Integer
//! Dim envVar As String
//! i = 1
//! Do
//!     envVar = Environ$(i)
//!     If envVar <> "" Then Debug.Print envVar
//!     i = i + 1
//! Loop While envVar <> ""
//! ```
//!
//! ## Common Usage Patterns
//!
//! ### Getting Application Data Path
//! ```vb6
//! Dim appDataPath As String
//! appDataPath = Environ$("APPDATA")
//! If appDataPath <> "" Then
//!     appDataPath = appDataPath & "\MyApp\"
//! End If
//! ```
//!
//! ### Getting User Profile Directory
//! ```vb6
//! Dim userProfile As String
//! userProfile = Environ$("USERPROFILE")
//! configFile = userProfile & "\config.ini"
//! ```
//!
//! ### Checking for Development Environment
//! ```vb6
//! Dim devMode As Boolean
//! devMode = (Environ$("DEV_MODE") = "1")
//! If devMode Then
//!     Debug.Print "Running in development mode"
//! End If
//! ```
//!
//! ### Building Full Path with Temp Directory
//! ```vb6
//! Dim tempFile As String
//! tempFile = Environ$("TEMP") & "\tempdata.tmp"
//! Open tempFile For Output As #1
//! ```
//!
//! ### Getting System Drive
//! ```vb6
//! Dim systemDrive As String
//! systemDrive = Environ$("SystemDrive")
//! logPath = systemDrive & "\Logs\app.log"
//! ```
//!
//! ### Listing All Environment Variables
//! ```vb6
//! Dim idx As Integer
//! Dim envEntry As String
//! For idx = 1 To 255
//!     envEntry = Environ$(idx)
//!     If envEntry = "" Then Exit For
//!     List1.AddItem envEntry
//! Next idx
//! ```
//!
//! ### Cross-Platform Path Separator
//! ```vb6
//! Dim pathSep As String
//! If Environ$("OS") Like "Windows*" Then
//!     pathSep = "\"
//! Else
//!     pathSep = "/"
//! End If
//! ```
//!
//! ### Getting Computer Name
//! ```vb6
//! Dim computerName As String
//! computerName = Environ$("COMPUTERNAME")
//! If computerName = "" Then computerName = Environ$("HOSTNAME")
//! ```
//!
//! ### Building Log File Path with User Name
//! ```vb6
//! Dim logFile As String
//! logFile = "C:\Logs\" & Environ$("USERNAME") & ".log"
//! Open logFile For Append As #1
//! Print #1, Now & " - User logged in"
//! Close #1
//! ```
//!
//! ### Checking if Variable Exists
//! ```vb6
//! Dim dbServer As String
//! dbServer = Environ$("DB_SERVER")
//! If dbServer = "" Then
//!     dbServer = "localhost"  ' Default value
//! End If
//! ```
//!
//! ## Related Functions
//!
//! - `Environ`: Non-string variant (returns Variant)
//! - `Command$`: Gets command-line arguments
//! - `CurDir$`: Gets current directory
//! - `GetSetting`: Reads application settings from registry
//! - `Dir$`: Lists files in directory
//!
//! ## Best Practices
//!
//! 1. Always check if the returned value is empty before using it
//! 2. Use string argument form for better code readability
//! 3. Cache frequently accessed environment variables
//! 4. Be aware of case sensitivity on different platforms
//! 5. Avoid modifying environment variables from VB6 (use shell APIs instead)
//! 6. Use proper path combining (avoid double backslashes)
//! 7. Consider using `GetEnvironmentVariable` API for more control
//! 8. Remember that environment variables persist only for the process lifetime
//! 9. Use constants for commonly used environment variable names
//! 10. Validate paths returned from environment variables before using them
//!
//! ## Performance Considerations
//!
//! - Environment variable lookup is relatively fast
//! - Iterating all variables with numeric index is slower than direct lookup
//! - Consider caching values if used frequently in loops
//! - No significant performance difference between `Environ$` and `Environ`
//!
//! ## Platform Differences
//!
//! | Platform | Notes |
//! |----------|-------|
//! | Windows 95/98 | Limited environment space (may fail with many variables) |
//! | Windows NT/2000/XP | Larger environment space, more reliable |
//! | Windows Vista+ | User and system environment variables separated |
//! | Wine/Linux | May return different variables, case sensitivity differs |
//!
//! ## Common Environment Variables
//!
//! | Variable | Description |
//! |----------|-------------|
//! | `PATH` | System search path for executables |
//! | `TEMP` or `TMP` | Temporary files directory |
//! | `APPDATA` | Application data folder (Windows) |
//! | `USERPROFILE` | User's home directory (Windows) |
//! | `USERNAME` | Current user's login name |
//! | `COMPUTERNAME` | Computer's network name |
//! | `SystemDrive` | Drive letter of system installation |
//! | `SystemRoot` | Windows installation directory |
//! | `HOMEDRIVE` | User's home drive letter |
//! | `HOMEPATH` | User's home directory path |
//!
//! ## Common Pitfalls
//!
//! - Not checking for empty string return values
//! - Assuming environment variable names are case-insensitive on all platforms
//! - Using numeric index without checking for empty string to detect end
//! - Creating paths with double backslashes when concatenating
//! - Assuming all common variables exist on all systems
//! - Not handling missing required environment variables gracefully
//!
//! ## Limitations
//!
//! - Cannot be used to set environment variables (use Windows API)
//! - Environment changes don't persist beyond process lifetime
//! - Limited to current process's environment space
//! - Some variables may be protected or unavailable depending on permissions
//! - Variable availability differs between operating systems

use crate::error::{VBError, VBResult};
use crate::state::environment;
use crate::value::VBVariant;

/// Returns the value of an environment variable.
///
/// - **String argument**: the value assigned to the named variable, or `""`
///   when the variable does not exist. The name is matched case-insensitively.
/// - **Numeric argument**: the environment string at that 1-based position in
///   the environment table, including the `=` separator (`NAME=value`).
///   Positions beyond the table return `""`. The number is rounded with VB6's
///   half-to-even `Long` semantics before it is used.
///
/// Returns error 5 (invalid procedure call) for a `Null`/`Empty` argument or a
/// numeric index below 1.
pub fn environ_dollar(arg: &VBVariant) -> VBResult<VBVariant> {
    let result = match arg {
        VBVariant::String(_) => environment::get_env(&arg.as_string()?).unwrap_or_default(),
        VBVariant::Empty | VBVariant::Null => return Err(VBError::invalid_procedure_call()),
        VBVariant::Nothing | VBVariant::Object(_) | VBVariant::Array(_) => {
            return Err(VBError::type_mismatch());
        }
        _ => {
            let index = arg.as_i32()?;
            if index < 1 {
                return Err(VBError::invalid_procedure_call());
            }
            environment::env_at(index as usize).unwrap_or_default()
        }
    };
    Ok(VBVariant::from_string(result))
}

#[cfg(test)]
mod tests {
    use super::*;
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
            environ_dollar(&VBVariant::from_string("VB6_ENVIRON_PATH")).unwrap(),
            VBVariant::from_string("C:\\bin")
        );
    }

    #[test]
    fn lookup_is_case_insensitive() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        assert_eq!(
            environ_dollar(&VBVariant::from_string("vb6_environ_user")).unwrap(),
            VBVariant::from_string("arthur")
        );
    }

    #[test]
    fn returns_empty_string_for_unknown_variable() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        assert_eq!(
            environ_dollar(&VBVariant::from_string("VB6_ENVIRON_MISSING")).unwrap(),
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
            environ_dollar(&VBVariant::from_integer(path)).unwrap(),
            VBVariant::from_string("VB6_ENVIRON_PATH=C:\\bin")
        );
        assert_eq!(
            environ_dollar(&VBVariant::from_integer(user)).unwrap(),
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
            environ_dollar(&VBVariant::from_double(user + 0.4)).unwrap(),
            VBVariant::from_string("VB6_ENVIRON_USER=arthur")
        );
    }

    #[test]
    fn out_of_range_position_returns_empty_string() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        let last = position_of("VB6_ENVIRON_USER") as i16;
        assert_eq!(
            environ_dollar(&VBVariant::from_integer(last + 1)).unwrap(),
            VBVariant::from_string("")
        );
    }

    #[test]
    fn index_below_one_is_error_5() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        let err = environ_dollar(&VBVariant::from_integer(0)).unwrap_err();
        assert_eq!(err.number, crate::error::err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn null_and_empty_are_error_5() {
        let _guard = TEST_LOCK.lock().unwrap();
        reset_with_sample_env();
        assert_eq!(
            environ_dollar(&VBVariant::Null).unwrap_err().number,
            crate::error::err_number::INVALID_PROCEDURE_CALL
        );
        assert_eq!(
            environ_dollar(&VBVariant::Empty).unwrap_err().number,
            crate::error::err_number::INVALID_PROCEDURE_CALL
        );
    }
}
