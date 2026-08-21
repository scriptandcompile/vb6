//! # `MsgBox` Function
//!
//! Displays a message in a dialog box, waits for the user to click a button, and returns an Integer indicating which button the user clicked.
//!
//! ## Syntax
//!
//! ```vb
//! MsgBox(prompt, [buttons], [title], [helpfile], [context])
//! ```
//!
//! ## Parameters
//!
//! - **prompt** (Required) - String expression displayed as the message in the dialog box. Maximum length is approximately 1024 characters, depending on the width of the characters used. If prompt consists of more than one line, you can separate the lines using a carriage return character (Chr(13)), a linefeed character (Chr(10)), or a carriage return-linefeed character combination (vbCrLf) between each line.
//! - **buttons** (Optional) - Numeric expression that is the sum of values specifying the number and type of buttons to display, the icon style to use, the identity of the default button, and the modality of the message box. If omitted, the default value for buttons is 0.
//! - **title** (Optional) - String expression displayed in the title bar of the dialog box. If you omit title, the application name is placed in the title bar.
//! - **helpfile** (Optional) - String expression that identifies the Help file to use to provide context-sensitive Help for the dialog box. If helpfile is provided, context must also be provided.
//! - **context** (Optional) - Numeric expression that is the Help context number assigned to the appropriate Help topic by the Help author. If context is provided, helpfile must also be provided.
//!
//! ## Return Value
//!
//! Returns an **Integer** representing which button was clicked:
//! - **vbOK (1)** - OK button was clicked
//! - **vbCancel (2)** - Cancel button was clicked
//! - **vbAbort (3)** - Abort button was clicked
//! - **vbRetry (4)** - Retry button was clicked
//! - **vbIgnore (5)** - Ignore button was clicked
//! - **vbYes (6)** - Yes button was clicked
//! - **vbNo (7)** - No button was clicked
//!
//! ## Remarks
//!
//! The `MsgBox` function is one of the most commonly used VB6 functions for user interaction and debugging.
//! It provides a simple way to display messages, warnings, errors, and questions to the user.
//!
//! ### Button Constants (First Group - Buttons):
//! - **vbOKOnly (0)** - Display OK button only (default)
//! - **vbOKCancel (1)** - Display OK and Cancel buttons
//! - **vbAbortRetryIgnore (2)** - Display Abort, Retry, and Ignore buttons
//! - **vbYesNoCancel (3)** - Display Yes, No, and Cancel buttons
//! - **vbYesNo (4)** - Display Yes and No buttons
//! - **vbRetryCancel (5)** - Display Retry and Cancel buttons
//!
//! ### Icon Constants (Second Group - Icons):
//! - **vbCritical (16)** - Display Critical Message icon (red X)
//! - **vbQuestion (32)** - Display Warning Query icon (question mark)
//! - **vbExclamation (48)** - Display Warning Message icon (exclamation point)
//! - **vbInformation (64)** - Display Information Message icon (lowercase i)
//!
//! ### Default Button Constants (Third Group - Default Button):
//! - **vbDefaultButton1 (0)** - First button is default (default)
//! - **vbDefaultButton2 (256)** - Second button is default
//! - **vbDefaultButton3 (512)** - Third button is default
//! - **vbDefaultButton4 (768)** - Fourth button is default
//!
//! ### Modality Constants (Fourth Group - Modality):
//! - **vbApplicationModal (0)** - Application modal; the user must respond to the message box before continuing work in the current application (default)
//! - **vbSystemModal (4096)** - System modal; all applications are suspended until the user responds to the message box
//!
//! ### Other Constants (Fifth Group - Other):
//! - **vbMsgBoxHelpButton (16384)** - Adds Help button to the message box
//! - **vbMsgBoxSetForeground (65536)** - Specifies the message box window as the foreground window
//! - **vbMsgBoxRight (524288)** - Text is right-aligned
//! - **vbMsgBoxRtlReading (1048576)** - Specifies text should appear as right-to-left reading on Hebrew and Arabic systems
//!
//! ### Key Characteristics:
//! - Blocks execution until user responds (modal dialog)
//! - Return value can be ignored if not needed
//! - Can combine button constants using addition or Or operator
//! - Maximum prompt length is approximately 1024 characters
//! - Can display multi-line messages using vbCrLf
//! - If user presses Escape key, acts as clicking Cancel button
//! - Can invoke help file if helpfile and context parameters provided
//! - Common for debugging with Debug.Print alternative
//!
//! ### Common Use Cases:
//! - Display informational messages to users
//! - Show error messages and warnings
//! - Ask yes/no questions for user confirmation
//! - Debugging by displaying variable values
//! - Alert users of important events
//! - Confirm destructive operations
//! - Display results of calculations
//! - Provide feedback on operation completion
//!
//! ## Typical Uses
//!
//! 1. **Simple Messages** - Display information to the user
//! 2. **Error Handling** - Show error messages with appropriate icons
//! 3. **User Confirmation** - Ask yes/no questions before proceeding
//! 4. **Debugging** - Display variable values during development
//! 5. **Operation Feedback** - Inform user of completion or status
//! 6. **Validation Warnings** - Alert user to invalid input
//! 7. **Save Prompts** - Ask user to save changes before closing
//! 8. **Delete Confirmations** - Verify user wants to delete items
//!
//! ## Basic Examples
//!
//! ```vb
//! ' Example 1: Simple message
//! MsgBox "Operation completed successfully!"
//! ```
//!
//! ```vb
//! ' Example 2: Message with title and icon
//! MsgBox "File not found!", vbExclamation, "Error"
//! ```
//!
//! ```vb
//! ' Example 3: Yes/No question
//! Dim result As Integer
//! result = MsgBox("Do you want to save changes?", vbYesNo + vbQuestion, "Confirm")
//! If result = vbYes Then
//!     ' Save changes
//! End If
//! ```
//!
//! ```vb
//! ' Example 4: Multi-line message
//! MsgBox "Line 1" & vbCrLf & "Line 2" & vbCrLf & "Line 3", vbInformation, "Multi-line"
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Simple error message
//! Sub ShowError(message As String)
//!     MsgBox message, vbCritical, "Error"
//! End Sub
//! ```
//!
//! ```vb
//! ' Pattern 2: Confirmation with default No
//! Function ConfirmAction(message As String) As Boolean
//!     Dim result As Integer
//!     result = MsgBox(message, vbYesNo + vbQuestion + vbDefaultButton2, "Confirm")
//!     ConfirmAction = (result = vbYes)
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 3: Save prompt
//! Function PromptToSave(fileName As String) As Integer
//!     Dim msg As String
//!     msg = "Save changes to " & fileName & "?"
//!     PromptToSave = MsgBox(msg, vbYesNoCancel + vbQuestion, "Save Changes")
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 4: Display detailed error
//! Sub ShowDetailedError(errorMsg As String, errorNumber As Long)
//!     Dim msg As String
//!     msg = "Error #" & errorNumber & vbCrLf & vbCrLf & errorMsg
//!     MsgBox msg, vbCritical + vbOKOnly, "Application Error"
//! End Sub
//! ```
//!
//! ```vb
//! ' Pattern 5: Information with sound
//! Sub ShowInfo(message As String)
//!     MsgBox message, vbInformation + vbOKOnly, "Information"
//!     ' vbInformation plays the system information sound
//! End Sub
//! ```
//!
//! ```vb
//! ' Pattern 6: Retry/Cancel operation
//! Function RetryOperation(operation As String) As Boolean
//!     Dim result As Integer
//!     Dim msg As String
//!     
//!     msg = "Failed to " & operation & "." & vbCrLf & "Would you like to retry?"
//!     result = MsgBox(msg, vbRetryCancel + vbExclamation, "Operation Failed")
//!     RetryOperation = (result = vbRetry)
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 7: Debug variable display
//! Sub DebugShow(variableName As String, value As Variant)
//!     #If DEBUG_MODE Then
//!         MsgBox variableName & " = " & CStr(value), vbInformation, "Debug"
//!     #End If
//! End Sub
//! ```
//!
//! ```vb
//! ' Pattern 8: Abort/Retry/Ignore pattern
//! Function HandleError(errorMsg As String) As Integer
//!     Dim msg As String
//!     msg = "An error occurred:" & vbCrLf & vbCrLf & errorMsg & vbCrLf & vbCrLf & _
//!           "Abort: Stop operation" & vbCrLf & _
//!           "Retry: Try again" & vbCrLf & _
//!           "Ignore: Continue anyway"
//!     HandleError = MsgBox(msg, vbAbortRetryIgnore + vbCritical, "Error")
//! End Function
//! ```
//!
//! ```vb
//! ' Pattern 9: Formatted message with details
//! Sub ShowOperationResult(operation As String, recordCount As Long, elapsedTime As Double)
//!     Dim msg As String
//!     msg = "Operation: " & operation & vbCrLf & _
//!           "Records processed: " & recordCount & vbCrLf & _
//!           "Time elapsed: " & Format(elapsedTime, "0.00") & " seconds"
//!     MsgBox msg, vbInformation, "Operation Complete"
//! End Sub
//! ```
//!
//! ```vb
//! ' Pattern 10: Custom button selection handler
//! Sub ProcessUserChoice(prompt As String)
//!     Dim result As Integer
//!     result = MsgBox(prompt, vbYesNoCancel + vbQuestion, "Choose Option")
//!     
//!     Select Case result
//!         Case vbYes
//!             ' Handle Yes
//!         Case vbNo
//!             ' Handle No
//!         Case vbCancel
//!             ' Handle Cancel
//!     End Select
//! End Sub
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: Smart Message Box Wrapper
//!
//! ```vb
//! ' Module: SmartMessageBox
//! ' Provides enhanced message box functionality with logging and customization
//!
//! Option Explicit
//!
//! Private m_logEnabled As Boolean
//! Private m_defaultTitle As String
//!
//! Public Sub Initialize(appName As String, enableLogging As Boolean)
//!     m_defaultTitle = appName
//!     m_logEnabled = enableLogging
//! End Sub
//!
//! Public Function ShowMessage(message As String, _
//!                            Optional msgType As VbMsgBoxStyle = vbInformation, _
//!                            Optional title As String = "") As Integer
//!     Dim actualTitle As String
//!     
//!     If Len(title) = 0 Then
//!         actualTitle = m_defaultTitle
//!     Else
//!         actualTitle = title
//!     End If
//!     
//!     If m_logEnabled Then
//!         LogMessage message, msgType
//!     End If
//!     
//!     ShowMessage = MsgBox(message, msgType, actualTitle)
//! End Function
//!
//! Public Function Confirm(message As String, _
//!                        Optional defaultToNo As Boolean = False) As Boolean
//!     Dim buttons As VbMsgBoxStyle
//!     Dim result As Integer
//!     
//!     buttons = vbYesNo + vbQuestion
//!     
//!     If defaultToNo Then
//!         buttons = buttons + vbDefaultButton2
//!     End If
//!     
//!     result = MsgBox(message, buttons, m_defaultTitle)
//!     Confirm = (result = vbYes)
//! End Function
//!
//! Public Sub ShowError(message As String, Optional errorNumber As Long = 0)
//!     Dim msg As String
//!     
//!     If errorNumber <> 0 Then
//!         msg = "Error #" & errorNumber & vbCrLf & vbCrLf & message
//!     Else
//!         msg = message
//!     End If
//!     
//!     If m_logEnabled Then
//!         LogMessage "ERROR: " & msg, vbCritical
//!     End If
//!     
//!     MsgBox msg, vbCritical, m_defaultTitle
//! End Sub
//!
//! Public Sub ShowWarning(message As String)
//!     If m_logEnabled Then
//!         LogMessage "WARNING: " & message, vbExclamation
//!     End If
//!     
//!     MsgBox message, vbExclamation, m_defaultTitle
//! End Sub
//!
//! Public Sub ShowInfo(message As String)
//!     If m_logEnabled Then
//!         LogMessage "INFO: " & message, vbInformation
//!     End If
//!     
//!     MsgBox message, vbInformation, m_defaultTitle
//! End Sub
//!
//! Private Sub LogMessage(message As String, msgType As VbMsgBoxStyle)
//!     ' Log to file or debug window
//!     Debug.Print Now & " - " & GetMessageTypeString(msgType) & ": " & message
//! End Sub
//!
//! Private Function GetMessageTypeString(msgType As VbMsgBoxStyle) As String
//!     If (msgType And vbCritical) = vbCritical Then
//!         GetMessageTypeString = "ERROR"
//!     ElseIf (msgType And vbExclamation) = vbExclamation Then
//!         GetMessageTypeString = "WARNING"
//!     ElseIf (msgType And vbQuestion) = vbQuestion Then
//!         GetMessageTypeString = "QUESTION"
//!     Else
//!         GetMessageTypeString = "INFO"
//!     End If
//! End Function
//! ```
//!
//! ### Example 2: Message Box Builder Class
//!
//! ```vb
//! ' Class: MessageBoxBuilder
//! ' Fluent interface for building complex message boxes
//!
//! Option Explicit
//!
//! Private m_prompt As String
//! Private m_title As String
//! Private m_buttons As VbMsgBoxStyle
//! Private m_icon As VbMsgBoxStyle
//! Private m_defaultButton As VbMsgBoxStyle
//!
//! Private Sub Class_Initialize()
//!     m_prompt = ""
//!     m_title = ""
//!     m_buttons = vbOKOnly
//!     m_icon = 0
//!     m_defaultButton = vbDefaultButton1
//! End Sub
//!
//! Public Function WithPrompt(prompt As String) As MessageBoxBuilder
//!     m_prompt = prompt
//!     Set WithPrompt = Me
//! End Function
//!
//! Public Function WithTitle(title As String) As MessageBoxBuilder
//!     m_title = title
//!     Set WithTitle = Me
//! End Function
//!
//! Public Function WithButtons(buttons As VbMsgBoxStyle) As MessageBoxBuilder
//!     m_buttons = buttons
//!     Set WithButtons = Me
//! End Function
//!
//! Public Function WithIcon(icon As VbMsgBoxStyle) As MessageBoxBuilder
//!     m_icon = icon
//!     Set WithIcon = Me
//! End Function
//!
//! Public Function WithDefaultButton(defaultButton As VbMsgBoxStyle) As MessageBoxBuilder
//!     m_defaultButton = defaultButton
//!     Set WithDefaultButton = Me
//! End Function
//!
//! Public Function AddLine(text As String) As MessageBoxBuilder
//!     If Len(m_prompt) > 0 Then
//!         m_prompt = m_prompt & vbCrLf
//!     End If
//!     m_prompt = m_prompt & text
//!     Set AddLine = Me
//! End Function
//!
//! Public Function Show() As Integer
//!     Dim style As VbMsgBoxStyle
//!     style = m_buttons Or m_icon Or m_defaultButton
//!     Show = MsgBox(m_prompt, style, m_title)
//! End Function
//!
//! ' Convenience methods
//! Public Function ShowError(errorMsg As String) As Integer
//!     Set WithPrompt(errorMsg)
//!     Set WithIcon(vbCritical)
//!     ShowError = Show()
//! End Function
//!
//! Public Function AskYesNo(question As String) As Boolean
//!     Set WithPrompt(question)
//!     Set WithButtons(vbYesNo)
//!     Set WithIcon(vbQuestion)
//!     AskYesNo = (Show() = vbYes)
//! End Function
//! ```
//!
//! ### Example 3: Message Queue Manager
//!
//! ```vb
//! ' Class: MessageQueueManager
//! ' Manages queued messages to avoid overwhelming the user
//!
//! Option Explicit
//!
//! Private Type QueuedMessage
//!     prompt As String
//!     buttons As VbMsgBoxStyle
//!     title As String
//!     timestamp As Date
//! End Type
//!
//! Private m_queue As Collection
//! Private m_maxQueueSize As Long
//! Private m_autoShowDelay As Long ' milliseconds
//!
//! Private Sub Class_Initialize()
//!     Set m_queue = New Collection
//!     m_maxQueueSize = 10
//!     m_autoShowDelay = 1000
//! End Sub
//!
//! Public Sub QueueMessage(prompt As String, _
//!                        Optional buttons As VbMsgBoxStyle = vbOKOnly, _
//!                        Optional title As String = "Message")
//!     Dim msg As QueuedMessage
//!     
//!     msg.prompt = prompt
//!     msg.buttons = buttons
//!     msg.title = title
//!     msg.timestamp = Now
//!     
//!     If m_queue.Count >= m_maxQueueSize Then
//!         ' Remove oldest message
//!         m_queue.Remove 1
//!     End If
//!     
//!     m_queue.Add msg
//! End Sub
//!
//! Public Sub ShowNextMessage() As Integer
//!     Dim msg As QueuedMessage
//!     
//!     If m_queue.Count > 0 Then
//!         msg = m_queue(1)
//!         m_queue.Remove 1
//!         ShowNextMessage = MsgBox(msg.prompt, msg.buttons, msg.title)
//!     End If
//! End Sub
//!
//! Public Sub ShowAllMessages()
//!     Dim msg As QueuedMessage
//!     Dim i As Long
//!     
//!     For i = 1 To m_queue.Count
//!         msg = m_queue(i)
//!         MsgBox msg.prompt, msg.buttons, msg.title
//!     Next i
//!     
//!     Set m_queue = New Collection
//! End Sub
//!
//! Public Function GetQueuedCount() As Long
//!     GetQueuedCount = m_queue.Count
//! End Function
//!
//! Public Sub ShowSummary()
//!     Dim msg As String
//!     Dim qMsg As QueuedMessage
//!     Dim i As Long
//!     
//!     If m_queue.Count = 0 Then
//!         MsgBox "No queued messages", vbInformation
//!         Exit Sub
//!     End If
//!     
//!     msg = "Queued Messages (" & m_queue.Count & "):" & vbCrLf & vbCrLf
//!     
//!     For i = 1 To m_queue.Count
//!         qMsg = m_queue(i)
//!         msg = msg & i & ". " & Left(qMsg.prompt, 50)
//!         If Len(qMsg.prompt) > 50 Then msg = msg & "..."
//!         msg = msg & vbCrLf
//!     Next i
//!     
//!     MsgBox msg, vbInformation, "Message Queue"
//! End Sub
//! ```
//!
//! ### Example 4: Auto-Dismissing Message Box (Timer-based)
//!
//! ```vb
//! ' Module: TimedMessageBox
//! ' Shows message boxes that auto-dismiss after timeout
//!
//! Option Explicit
//!
//! #If Win32 Then
//!     Private Declare Function MessageBoxTimeout Lib "user32" Alias "MessageBoxTimeoutA" ( _
//!         ByVal hwnd As Long, _
//!         ByVal lpText As String, _
//!         ByVal lpCaption As String, _
//!         ByVal uType As Long, _
//!         ByVal wLanguageId As Long, _
//!         ByVal dwMilliseconds As Long) As Long
//! #End If
//!
//! Public Function MsgBoxTimed(prompt As String, _
//!                            Optional buttons As VbMsgBoxStyle = vbOKOnly, _
//!                            Optional title As String = "", _
//!                            Optional timeout As Long = 5000) As Integer
//!     #If Win32 Then
//!         ' Use Windows API for timed message box
//!         MsgBoxTimed = MessageBoxTimeout(0, prompt, title, buttons, 0, timeout)
//!     #Else
//!         ' Fallback to regular MsgBox
//!         MsgBoxTimed = MsgBox(prompt, buttons, title)
//!     #End If
//! End Function
//!
//! Public Sub ShowTimedInfo(message As String, Optional seconds As Long = 3)
//!     MsgBoxTimed message, vbInformation, "Information", seconds * 1000
//! End Sub
//!
//! Public Sub ShowTimedWarning(message As String, Optional seconds As Long = 5)
//!     MsgBoxTimed message, vbExclamation, "Warning", seconds * 1000
//! End Sub
//!
//! Public Function ConfirmTimed(message As String, Optional seconds As Long = 10) As Boolean
//!     Dim result As Integer
//!     result = MsgBoxTimed(message, vbYesNo + vbQuestion, "Confirm", seconds * 1000)
//!     ConfirmTimed = (result = vbYes)
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! ' MsgBox rarely fails, but handle potential issues:
//! On Error Resume Next
//! result = MsgBox(userInput, vbOKOnly, "Message")
//! If Err.Number <> 0 Then
//!     Debug.Print "MsgBox error: " & Err.Description
//!     ' Possibly userInput was too long or contained invalid characters
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Performance Considerations
//!
//! - `MsgBox` blocks execution - not suitable for background operations
//! - For debugging, consider Debug.Print as faster alternative
//! - Avoid `MsgBox` in loops - very slow and annoying to users
//! - For status updates, use status bar or progress form instead
//! - System modal (vbSystemModal) blocks ALL applications - use sparingly
//! - Long prompts may be truncated - keep messages concise
//! - Each `MsgBox` call creates and destroys a window - has overhead
//!
//! ## Best Practices
//!
//! 1. **Keep messages concise** - Users don't read long messages
//! 2. **Use appropriate icons** - Help users understand message severity
//! 3. **Provide clear actions** - Button choices should be obvious
//! 4. **Use meaningful titles** - Don't just say "Error" or "Message"
//! 5. **Avoid `MsgBox` in loops** - Queue messages or use alternative feedback
//! 6. **Handle all return values** - Check what button user clicked
//! 7. **Use vbCrLf for readability** - Multi-line messages are easier to read
//! 8. **Consider default button** - Make safe choice the default
//! 9. **Test message length** - Very long messages may not display well
//! 10. **Use for errors, not debugging** - Prefer Debug.Print for development
//!
//! ## Comparison with Alternatives
//!
//! | Approach | Pros | Cons |
//! |----------|------|------|
//! | **`MsgBox`** | Simple, built-in, modal | Blocks execution, limited customization |
//! | **Custom Form** | Full control, rich UI | More code, more complex |
//! | **Debug.Print** | Non-blocking, fast | Not visible to end users |
//! | **Status Bar** | Non-blocking, professional | Limited message length, less visible |
//! | **`InputBox`** | Gets user input | Only single-line text input |
//! | **Notification API** | Modern, non-blocking | Requires Windows 10+, more complex |
//!
//! ## Statement vs Function
//!
//! `MsgBox` can be used as both a statement (no parentheses, no return value) and a function (with parentheses, returns value):
//!
//! ```vb
//! ' As a statement (no return value needed)
//! MsgBox "Hello, World!"
//!
//! ' As a function (capture return value)
//! result = MsgBox("Continue?", vbYesNo)
//! ```
//!
//! ## Platform Notes
//!
//! - Available in VBA (Excel, Access, Word, etc.)
//! - Available in VB6
//! - Available in `VBScript` (limited - no named constants in `VBScript`)
//! - Uses Windows common dialog
//! - Appearance varies by Windows version
//! - Icons and sounds defined by Windows theme
//! - Right-to-left reading supported on appropriate systems
//!
//! ## Limitations
//!
//! - Blocks execution (modal)
//! - Limited to approximately 1024 characters in prompt
//! - Cannot customize button text (e.g., can't change "Yes" to "Accept")
//! - Cannot add custom icons
//! - Cannot resize or reposition the dialog
//! - No timeout capability (without API calls)
//! - No checkboxes or radio buttons
//! - Cannot validate input (use `InputBox` or custom form for that)
//!
//! ## Related Functions
//!
//! - **`InputBox`** - Gets text input from user
//! - **Debug.Print** - Prints to Immediate window (development only)
//! - **Shell** - Runs external programs
//! - **Beep** - Plays system beep sound
//! - **`SendKeys`** - Sends keystrokes to active window
//!
//! ## VB6 Parser Notes
//!
//! `MsgBox` can be used as both a statement and a function. When used as a function (with parentheses
//! and capturing return value), it is parsed as a `CallExpression`. When used as a statement (without
//! parentheses), it may be parsed differently. This module exists primarily for documentation purposes
//! to provide comprehensive reference material for VB6 developers working with user interaction and
//! message display operations.

use crate::error::{err_number, VBError, VBResult};
use crate::state;
use crate::value::{VBLong, VBString, VBVariant};

/// Implement VB6's `MsgBox` function.
///
/// Decodes and validates the `buttons` argument (raising error 5 for
/// invalid values, including a default-button index past the last displayed
/// button), enforces the helpfile/context pairing rule, then hands the
/// request to the active [`interaction backend`](crate::state::interaction)
/// and returns the clicked button's `VbMsgBoxResult` value as an Integer.
pub fn msg_box(
    prompt: &VBString,
    buttons: Option<&VBLong>,
    title: Option<&VBString>,
    helpfile: Option<&VBString>,
    context: Option<&VBLong>,
) -> VBResult<VBVariant> {
    // VB6 requires helpfile and context to appear together.
    if helpfile.is_some() != context.is_some() {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid procedure call or argument: MsgBox requires helpfile and context \
             to be provided together",
        ));
    }

    let raw_buttons = buttons.map_or(0, |b| b.as_i32() as i64);
    let mut request = state::interaction::MsgBoxRequest::parse(prompt.as_str(), raw_buttons)?;

    // An omitted or empty title means "show the application name"; hosts
    // resolve that from `None`.
    let title = title.and_then(|t| {
        let t = t.as_str();
        (!t.is_empty()).then(|| t.to_string())
    });
    request.title = title;

    let button = state::interaction::msg_box(&request)?;
    Ok(VBVariant::from_integer(button.id()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::interaction::{memory::MemoryBackend, MsgBoxButton};
    use crate::state::test_support::lock_test;

    fn set_backend(backend: MemoryBackend) {
        state::interaction::set_backend(Box::new(backend));
    }

    #[test]
    fn plain_ok_dialog_returns_vbok() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let result = msg_box(&VBString::from("hello"), None, None, None, None).unwrap();
        assert_eq!(result.as_vbinteger().unwrap(), 1.into()); // vbOK
        state::interaction::reset_backend();
    }

    #[test]
    fn scripted_yes_no_answers_in_order() {
        let _guard = lock_test();
        let backend = MemoryBackend::with_msgbox_responses([MsgBoxButton::Yes, MsgBoxButton::No]);
        set_backend(backend);
        let buttons = VBLong::from(4 + 32); // vbYesNo + vbQuestion

        assert_eq!(
            msg_box(&VBString::from("save?"), Some(&buttons), None, None, None).unwrap(),
            VBVariant::from_integer(6) // vbYes
        );
        assert_eq!(
            msg_box(&VBString::from("sure?"), Some(&buttons), None, None, None).unwrap(),
            VBVariant::from_integer(7) // vbNo
        );
        state::interaction::reset_backend();
    }

    #[test]
    fn incompatible_scripted_response_is_error_5() {
        let _guard = lock_test();
        // Dialog offers Yes/No but the scripted answer is Cancel.
        set_backend(MemoryBackend::with_msgbox_responses([MsgBoxButton::Cancel]));
        let buttons = VBLong::from(4); // vbYesNo
        let err = msg_box(&VBString::from("go?"), Some(&buttons), None, None, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        assert!(err.description.contains("Cancel"));
        state::interaction::reset_backend();
    }

    #[test]
    fn invalid_buttons_value_is_error_5() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let buttons = VBLong::from(7); // button-set nibble out of range
        let err = msg_box(&VBString::from("x"), Some(&buttons), None, None, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        state::interaction::reset_backend();
    }

    #[test]
    fn default_button_past_last_displayed_is_error_5() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        // vbOKOnly + vbDefaultButton2: no second button exists.
        let buttons = VBLong::from(256);
        let err = msg_box(&VBString::from("x"), Some(&buttons), None, None, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        state::interaction::reset_backend();
    }

    #[test]
    fn helpfile_without_context_is_error_5() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let helpfile = VBString::from("help.hlp");
        let err = msg_box(&VBString::from("x"), None, None, Some(&helpfile), None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);

        let context = VBLong::from(10);
        let err = msg_box(&VBString::from("x"), None, None, None, Some(&context)).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        state::interaction::reset_backend();
    }

    #[test]
    fn paired_helpfile_and_context_are_accepted() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let helpfile = VBString::from("help.hlp");
        let context = VBLong::from(10);
        let result = msg_box(
            &VBString::from("x"),
            None,
            None,
            Some(&helpfile),
            Some(&context),
        )
        .unwrap();
        assert_eq!(result.as_vbinteger().unwrap(), 1.into());
        state::interaction::reset_backend();
    }

    #[test]
    fn empty_title_falls_back_to_application_default() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let title = VBString::from("");
        msg_box(&VBString::from("x"), None, Some(&title), None, None).unwrap();

        let requests = state::interaction::with_memory_backend(|m| m.msgbox_requests()).unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].title, None);
        state::interaction::reset_backend();
    }

    #[test]
    fn title_and_prompt_reach_the_backend_record() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let buttons = VBLong::from(3); // vbYesNoCancel
        let title = VBString::from("Confirm");
        msg_box(
            &VBString::from("Proceed?"),
            Some(&buttons),
            Some(&title),
            None,
            None,
        )
        .unwrap();

        let requests = state::interaction::with_memory_backend(|m| m.msgbox_requests()).unwrap();
        assert_eq!(requests[0].prompt, "Proceed?");
        assert_eq!(requests[0].title.as_deref(), Some("Confirm"));
        assert_eq!(
            requests[0].offered_buttons,
            vec![MsgBoxButton::Yes, MsgBoxButton::No, MsgBoxButton::Cancel]
        );
        state::interaction::reset_backend();
    }
}
