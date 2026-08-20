//! `GetAutoServerSettings` Function
//!
//! Returns information about the security settings for a `DCOM` (`Distributed Component Object Model`) server.
//!
//! # Syntax
//!
//! ```vb
//! GetAutoServerSettings(progid, clsid, machine)
//! ```
//!
//! # Parameters
//!
//! - `progid` - Required. String expression that specifies the programmatic identifier (`ProgID`) of the server.
//! - `clsid` - Required. String expression that specifies the class identifier (`CLSID`) of the server.
//! - `machine` - Required. String expression that specifies the name of the machine where the server is located.
//!
//! # Return Value
//!
//! Returns a `Long` value containing security settings information for the specified `DCOM` server.
//!
//! # Remarks
//!
//! - This function is specific to `DCOM` (`Distributed Component Object Model`) automation servers.
//! - Used primarily in distributed computing scenarios.
//! - Returns security configuration information from the Windows registry.
//! - The function is part of VB6's `DCOM` support infrastructure.
//! - Typically used in enterprise applications with distributed components.
//! - Requires appropriate `DCOM` permissions on the target machine.
//! - The progid and clsid must correspond to a registered `COM`/`DCOM` server.
//! - Machine name can be a `NetBIOS` name, `DNS` name, or `IP` address.
//! - Returns 0 if the server settings cannot be retrieved.
//!
//! # Typical Uses
//!
//! - Querying DCOM server security configurations
//! - Validating remote server accessibility
//! - Auditing distributed component settings
//! - Troubleshooting DCOM connection issues
//! - Enterprise application deployment verification
//! - Remote component diagnostics
//!
//! # Basic Usage Examples
//!
//! ```vb
//! ' Check DCOM server settings
//! Dim settings As Long
//! settings = GetAutoServerSettings("MyServer.Application", _
//!                                   "{12345678-1234-1234-1234-123456789012}", _
//!                                   "SERVER01")
//!
//! If settings <> 0 Then
//!     Debug.Print "Server settings retrieved: " & settings
//! Else
//!     Debug.Print "Unable to retrieve server settings"
//! End If
//!
//! ' Verify remote component availability
//! Dim result As Long
//! result = GetAutoServerSettings("Excel.Application", _
//!                                "{00024500-0000-0000-C000-000000000046}", _
//!                                "REMOTE-PC")
//!
//! If result <> 0 Then
//!     MsgBox "Remote Excel server is configured"
//! End If
//!
//! ' Query local server settings
//! Dim localSettings As Long
//! localSettings = GetAutoServerSettings("MyApp.Server", _
//!                                       "{ABCDEF01-2345-6789-ABCD-EF0123456789}", _
//!                                       ".")
//! ```
//!
//! # Common Patterns
//!
//! ## 1. DCOM Server Validation
//!
//! ```vb
//! Function ValidateDCOMServer(progID As String, _
//!                             clsID As String, _
//!                             serverName As String) As Boolean
//!     Dim settings As Long
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     settings = GetAutoServerSettings(progID, clsID, serverName)
//!     
//!     If settings <> 0 Then
//!         Debug.Print "DCOM server validated on " & serverName
//!         ValidateDCOMServer = True
//!     Else
//!         Debug.Print "DCOM server not accessible on " & serverName
//!         ValidateDCOMServer = False
//!     End If
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     Debug.Print "Error validating DCOM server: " & Err.Description
//!     ValidateDCOMServer = False
//! End Function
//! ```
//!
//! ## 2. Multi-Server Configuration Check
//!
//! ```vb
//! Sub CheckServersConfiguration()
//!     Dim servers() As String
//!     Dim i As Long
//!     Dim settings As Long
//!     
//!     servers = Array("SERVER01", "SERVER02", "SERVER03")
//!     
//!     For i = LBound(servers) To UBound(servers)
//!         settings = GetAutoServerSettings("MyApp.DataServer", _
//!                                          "{11111111-2222-3333-4444-555555555555}", _
//!                                          servers(i))
//!         
//!         If settings <> 0 Then
//!             Debug.Print servers(i) & " - Configured: " & settings
//!         Else
//!             Debug.Print servers(i) & " - Not configured"
//!         End If
//!     Next i
//! End Sub
//! ```
//!
//! # Error Handling
//!
//! ```vb
//! Function SafeGetAutoServerSettings(progID As String, _
//!                                    clsID As String, _
//!                                    serverName As String) As Long
//!     On Error GoTo ErrorHandler
//!     
//!     SafeGetAutoServerSettings = GetAutoServerSettings(progID, clsID, serverName)
//!     Exit Function
//!     
//! ErrorHandler:
//!     Select Case Err.Number
//!         Case 429  ' ActiveX component can't create object
//!             Debug.Print "Server not available: " & serverName
//!         Case 462  ' Remote server machine does not exist
//!             Debug.Print "Machine not found: " & serverName
//!         Case 70   ' Permission denied
//!             Debug.Print "Access denied to server: " & serverName
//!         Case Else
//!             Debug.Print "Error " & Err.Number & ": " & Err.Description
//!     End Select
//!     
//!     SafeGetAutoServerSettings = 0
//! End Function
//! ```
//!
//! Common errors:
//! - **Error 429**: `ActiveX` component can't create object - server not registered or accessible.
//! - **Error 462**: Remote server machine does not exist or is unavailable.
//! - **Error 70**: Permission denied - insufficient `DCOM` permissions.
//! - **Error 5**: Invalid procedure call - invalid `ProgID` or `CLSID` format.
//!
//! # Limitations
//!
//! - Windows-specific functionality (DCOM is Windows-only)
//! - Requires DCOM to be enabled and properly configured
//! - Network connectivity required for remote servers
//! - Security settings may block access
//! - Return value interpretation is not well documented
//! - Limited to COM/DCOM servers
//! - May not work with modern .NET components
//! - Deprecated in favor of newer technologies (WCF, REST APIs)
//!
//! # Related Functions
//!
//! - `CreateObject` - Creates an instance of a `COM` object
//! - `GetObject` - Returns a reference to an `ActiveX` object

use crate::error::VBResult;
use crate::value::VBVariant;

/// Returns security settings for a `DCOM` automation server.
///
/// This is a dummy implementation — `DCOM` is Windows-specific and not
/// supported by this runtime.  The function always returns `0`, which is
/// the value VB6 returns when no server settings can be retrieved (i.e.
/// when the machine does not have a `DCOM` server set up or the current
/// user lacks the necessary `DCOM` security configuration).
pub fn get_auto_server_settings(
    progid: &VBVariant,
    clsid: &VBVariant,
    machine: &VBVariant,
) -> VBResult<VBVariant> {
    // Validate argument types — all three must be convertible to String.
    // VB6 raises a Type Mismatch error (13) if non-string values are passed.
    let _progid = progid.as_string()?;
    let _clsid = clsid.as_string()?;
    let _machine = machine.as_string()?;

    // DCOM is not supported on this platform.  Return 0 to indicate that no
    // server settings could be retrieved — the same value VB6 returns when the
    // server is not registered or the user lacks the correct DCOM permissions.
    Ok(VBVariant::from_long(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;

    #[test]
    fn returns_zero() {
        let result = get_auto_server_settings(
            &VBVariant::from_string("MyServer.Application"),
            &VBVariant::from_string("{12345678-1234-1234-1234-123456789012}"),
            &VBVariant::from_string("SERVER01"),
        )
        .unwrap();
        assert_eq!(result, VBVariant::from_long(0));
    }

    #[test]
    fn accepts_any_string_arguments() {
        let result = get_auto_server_settings(
            &VBVariant::from_string(""),
            &VBVariant::from_string(""),
            &VBVariant::from_string(""),
        )
        .unwrap();
        assert_eq!(result, VBVariant::from_long(0));
    }

    #[test]
    fn null_prog_id_is_error_94() {
        let err = get_auto_server_settings(
            &VBVariant::Null,
            &VBVariant::from_string("clsid"),
            &VBVariant::from_string("machine"),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn null_cls_id_is_error_94() {
        let err = get_auto_server_settings(
            &VBVariant::from_string("progid"),
            &VBVariant::Null,
            &VBVariant::from_string("machine"),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn null_machine_is_error_94() {
        let err = get_auto_server_settings(
            &VBVariant::from_string("progid"),
            &VBVariant::from_string("clsid"),
            &VBVariant::Null,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }
}
