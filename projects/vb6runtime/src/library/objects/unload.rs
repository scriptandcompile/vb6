//! # Unload Statement
//!
//! Removes a form or control from memory.
//!
//! ## Syntax
//!
//! ```vb
//! Unload object
//! ```
//!
//! ## Parts
//!
//! - **object**: Required. An object expression that evaluates to a Form or control. If object is
//!   a form, unloading the form causes all controls on the form to be unloaded as well.
//!
//! ## Remarks
//!
//! - **Form Unloading**: When a form is unloaded, all of its controls are removed from memory and
//!   all values of the form's properties are lost. You can use the `Hide` method to make a form
//!   invisible without unloading it, allowing you to continue to access properties of the form
//!   and its controls.
//! - **Control Arrays**: When you unload a control created at run time with the `Load` statement,
//!   the control is removed from the control array, and the array's upper bound is decremented by one.
//! - **`QueryUnload` Event**: Before a form is unloaded, the `QueryUnload` event procedure is called.
//!   Setting the `Cancel` argument to `True` in the `QueryUnload` event prevents the form from
//!   being unloaded.
//! - **Unload Event**: After the `QueryUnload` event, the `Unload` event procedure is called. You
//!   can include code in this event procedure to save data or clean up resources.
//! - **Me Keyword**: Within a form's code, you can use `Unload Me` to unload the form itself.
//! - **Subsequent References**: Any subsequent references to properties or controls on an unloaded
//!   form will cause the form to be reloaded and its `Load` event to fire.
//!
//! ## Examples
//!
//! ### Simple Form Unload
//!
//! ```vb
//! Unload Form1
//! ```
//!
//! ### Unload Current Form
//!
//! ```vb
//! Private Sub cmdClose_Click()
//!     Unload Me
//! End Sub
//! ```
//!
//! ### Unload Control Array Element
//!
//! ```vb
//! Unload txtDynamic(5)
//! ```
//!
//! ### Unload With Cleanup
//!
//! ```vb
//! Private Sub Form_Unload(Cancel As Integer)
//!     ' Save data before closing
//!     SaveSettings
//!     CloseDatabase
//! End Sub
//! ```
//!
//! ### Conditional Unload
//!
//! ```vb
//! If UserConfirmed Then
//!     Unload frmDialog
//! End If
//! ```
//!
//! ## Common Patterns
//!
//! ### Save Data Before Unload
//!
//! ```vb
//! Private Sub Form_Unload(Cancel As Integer)
//!     If DataModified Then
//!         Dim response As VbMsgBoxResult
//!         response = MsgBox("Save changes?", vbYesNoCancel)
//!         If response = vbYes Then
//!             SaveData
//!         ElseIf response = vbCancel Then
//!             Cancel = True ' Prevent unload
//!         End If
//!     End If
//! End Sub
//! ```
//!
//! ### Unload Multiple Forms
//!
//! ```vb
//! Sub CloseAllForms()
//!     Dim frm As Form
//!     For Each frm In Forms
//!         If frm.Name <> "frmMain" Then
//!             Unload frm
//!         End If
//!     Next frm
//! End Sub
//! ```
//!
//! ### Unload Dynamically Created Controls
//!
//! ```vb
//! Dim i As Integer
//! For i = 1 To 10
//!     Unload lblDynamic(i)
//! Next i
//! ```
//!
//! ## Best Practices
//!
//! 1. **Use Unload vs Hide**: Use `Unload` when you're done with a form and want to free memory.
//!    Use `Hide` when you want to make a form invisible but may need to show it again soon.
//! 2. **Clean Up Resources**: Use the `Unload` event to close database connections, release objects,
//!    and perform other cleanup tasks.
//! 3. **Prevent Accidental Closes**: Use the `QueryUnload` event with `Cancel = True` to prevent
//!    forms from being unloaded when necessary.
//! 4. **Main Form Considerations**: Unloading the startup form (main form) terminates the application
//!    unless you've specified a Sub Main procedure.
//! 5. **Memory Management**: Unloading forms and controls frees memory, which is important in
//!    applications that create many forms or controls dynamically.
//!
//! ## Important Notes
//!
//! - Unloading a form removes it from memory completely
//! - Any data stored in form-level variables is lost
//! - Controls on an unloaded form cannot be accessed
//! - The `Unload` event fires before the form is actually removed
//! - MDI child forms are unloaded when the MDI parent is unloaded
//! - You cannot unload a control that wasn't created with the `Load` statement
//!
//! ## See Also
//!
//! - `Load` statement (loads a form or control into memory)
//! - `Show` method (displays a form)
//! - `Hide` method (hides a form without unloading)
//! - `QueryUnload` event (fires before a form is unloaded)
//! - `Unload` event (fires when a form is being unloaded)
//!
//! ## References
//!
//! - [Microsoft Docs: Unload Statement](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/unload-statement)
