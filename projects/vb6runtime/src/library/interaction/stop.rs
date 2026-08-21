//! # Stop Statement
//!
//! Suspends execution.
//!
//! ## Syntax
//!
//! ```vb
//! Stop
//! ```
//!
//! ## Remarks
//!
//! - **Execution Suspension**: The Stop statement suspends execution but doesn't close any files or clear variables unless it is in a compiled executable (.exe) file.
//! - **Debug Mode**: In the development environment, Stop causes the program to enter break mode, allowing you to examine variables, step through code, and use debugging tools.
//! - **Compiled Executable**: In a compiled .exe file, Stop acts like the End statement, terminating the program and closing all files.
//! - **Break Mode**: When Stop is encountered in the IDE, VB6 pauses execution and highlights the Stop statement, allowing you to inspect the current state.
//! - **Multiple Stop Statements**: You can place Stop statements anywhere in your code to create breakpoints for debugging.
//! - **Not for Production**: Stop statements should generally be removed before distributing your application, as they can cause unexpected behavior.
//! - **Alternative to Breakpoints**: Stop provides a code-based alternative to setting breakpoints in the IDE.
//! - **No Arguments**: The Stop statement takes no arguments or parameters.
//!
//! ## Common Uses
//!
//! - **Debugging**: Pause execution to examine variable values and program state
//! - **Conditional Breakpoints**: Combined with If statements for conditional debugging
//! - **Error Investigation**: Stop execution when an error condition is detected
//! - **Loop Debugging**: Pause execution during specific loop iterations
//! - **Testing**: Verify code paths during development
//!
//! ## Examples
//!
//! ### Simple Stop
//!
//! ```vb
//! Sub Test()
//!     Dim x As Integer
//!     x = 10
//!     Stop  ' Execution pauses here in IDE
//!     x = x + 5
//! End Sub
//! ```
//!
//! ### Conditional Stop for Debugging
//!
//! ```vb
//! Sub ProcessData(value As Integer)
//!     If value < 0 Then
//!         Stop  ' Pause when invalid data is encountered
//!     End If
//!     ' Process value
//! End Sub
//! ```
//!
//! ### Stop in Loop for Specific Iteration
//!
//! ```vb
//! For i = 1 To 100
//!     If i = 50 Then
//!         Stop  ' Pause at iteration 50
//!     End If
//!     ProcessItem i
//! Next i
//! ```
//!
//! ### Stop on Error Condition
//!
//! ```vb
//! Sub CalculateTotal()
//!     Dim total As Double
//!     total = GetSubtotal()
//!     
//!     If total < 0 Then
//!         Stop  ' Investigate negative total
//!     End If
//!     
//!     SaveTotal total
//! End Sub
//! ```
//!
//! ### Multiple Stop Statements for Debugging Path
//!
//! ```vb
//! Function ValidateData(data As String) As Boolean
//!     Stop  ' Entry point
//!     
//!     If Len(data) = 0 Then
//!         Stop  ' Empty string case
//!         ValidateData = False
//!         Exit Function
//!     End If
//!     
//!     Stop  ' Normal processing
//!     ValidateData = True
//! End Function
//! ```
//!
//! ### Stop in Select Case
//!
//! ```vb
//! Select Case userType
//!     Case 1
//!         ProcessAdmin
//!     Case 2
//!         ProcessUser
//!     Case Else
//!         Stop  ' Unknown user type - investigate
//! End Select
//! ```
//!
//! ### Stop with Error Handler
//!
//! ```vb
//! On Error GoTo ErrorHandler
//!
//! ProcessData
//! Exit Sub
//!
//! ErrorHandler:
//!     Stop  ' Pause to examine error
//!     MsgBox Err.Description
//! End Sub
//! ```
//!
//! ### Stop in Class Module
//!
//! ```vb
//! Private Sub Class_Initialize()
//!     Stop  ' Verify initialization sequence
//!     InitializeProperties
//! End Sub
//! ```
//!
//! ### Stop Before Critical Operation
//!
//! ```vb
//! Sub DeleteAllRecords()
//!     Stop  ' Verify this operation should proceed
//!     
//!     Dim rs As Recordset
//!     Set rs = db.OpenRecordset("Data")
//!     
//!     Do While Not rs.EOF
//!         rs.Delete
//!         rs.MoveNext
//!     Loop
//! End Sub
//! ```
//!
//! ### Stop in Property Procedure
//!
//! ```vb
//! Public Property Let Value(ByVal newValue As Integer)
//!     If newValue < 0 Then
//!         Stop  ' Negative value assigned
//!     End If
//!     m_Value = newValue
//! End Property
//! ```
//!
//! ### Stop with `DoEvents`
//!
//! ```vb
//! For i = 1 To 1000
//!     DoEvents
//!     ProcessItem i
//!     
//!     If ShouldDebug Then
//!         Stop  ' Conditional pause
//!     End If
//! Next i
//! ```
//!
//! ### Stop in Event Handler
//!
//! ```vb
//! Private Sub Form_Load()
//!     Stop  ' Debug form initialization
//!     InitializeControls
//!     LoadData
//! End Sub
//! ```
//!
//! ### Stop with Assert-like Check
//!
//! ```vb
//! Sub ProcessArray(arr() As Integer)
//!     If UBound(arr) < LBound(arr) Then
//!         Stop  ' Invalid array bounds
//!     End If
//!     
//!     For i = LBound(arr) To UBound(arr)
//!         ProcessItem arr(i)
//!     Next i
//! End Sub
//! ```
//!
//! ## Important Notes
//!
//! - **IDE vs. Compiled**: Behavior differs between development environment and compiled executables
//! - **Production Code**: Remove Stop statements before distributing your application
//! - **Alternative Debugging**: Modern IDEs prefer breakpoints over Stop statements
//! - **No File Closure**: In IDE, Stop doesn't close files or clear variables
//! - **Break Mode**: Allows interactive debugging in the IDE
//! - **End Alternative**: In compiled .exe, behaves like the End statement
//! - **Code-Based Breakpoint**: Useful when you need a breakpoint that travels with the code
//! - **No Performance Impact**: When compiled, can be configured to be removed by compiler
//!
//! ## Best Practices
//!
//! - Use Stop for temporary debugging during development
//! - Remove Stop statements before final release
//! - Use meaningful comments explaining why Stop is placed at a location
//! - Consider using conditional compilation to automatically remove Stop in release builds
//! - Prefer IDE breakpoints for most debugging scenarios
//! - Use Stop when you need to share debugging points with team members
//! - Document any Stop statements that remain for legitimate reasons
//!
//! ## Differences from End
//!
//! - **Stop**: In IDE, enters break mode; in .exe, terminates program
//! - **End**: Always terminates program and closes all files
//! - **Exit**: Exits specific procedure/function/loop without terminating program
//!
//! ## See Also
//!
//! - `End` statement (terminate program)
//! - `Exit` statement (exit procedure, function, or loop)
//! - `DoEvents` function (yield execution to the operating system)
//! - `Debug.Assert` method (conditional debugging)
//!
//! ## References
//!
//! - [Stop Statement - Microsoft Docs](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/stop-statement)

use crate::state;

/// Implement VB6's `Stop` statement.
///
/// Signals a break request to the active interaction backend. In the
/// development environment (a debugger is attached, e.g. the WASM
/// playground) the host enters break mode with all state preserved; in a
/// plain run — like a compiled `.exe` — `Stop` acts like the `End`
/// statement. The interpreter decides which behavior applies; this call
/// only delivers the runtime-side signal.
pub fn stop() {
    state::interaction::stop();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::test_support::lock_test;

    #[test]
    fn stop_does_not_panic() {
        let _guard = lock_test();
        stop();
    }
}
