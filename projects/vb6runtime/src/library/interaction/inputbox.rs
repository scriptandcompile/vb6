//! # `InputBox` Function
//!
//! Displays a prompt in a dialog box, waits for the user to input text or click a button,
//! and returns a `String` containing the contents of the text box.
//!
//! ## Syntax
//!
//! ```vb
//! InputBox(prompt[, title][, default][, xpos][, ypos][, helpfile, context])
//! ```
//!
//! ## Parameters
//!
//! - `prompt` (Required): `String` expression displayed as the message in the dialog box. Maximum length is approximately 1024 characters, depending on width of characters used. Can include line breaks using `vbCrLf`, `vbNewLine`, or `Chr(13) & Chr(10)`
//! - `title` (Optional): `String` expression displayed in the title bar of the dialog box. If omitted, the application name is displayed
//! - `default` (Optional): `String` expression displayed in the text box as the default response if no other input is provided. If omitted, the text box is displayed empty
//! - `xpos` (Optional): Numeric expression that specifies, in twips, the horizontal distance from the left edge of the screen. If omitted, the dialog box is horizontally centered
//! - `ypos` (Optional): Numeric expression that specifies, in twips, the vertical distance from the top of the screen. If omitted, the dialog box is positioned vertically approximately one-third down the screen
//! - `helpfile` (Optional): `String` expression that identifies the Help file to use. If provided, context must also be provided
//! - `context` (Optional): Numeric expression that identifies the Help context number assigned to the Help topic. If provided, helpfile must also be provided
//!
//! ## Return Value
//!
//! Returns a `String`:
//! - If OK is clicked or Enter is pressed: Returns the text in the text box
//! - If Cancel is clicked: Returns an empty string ("")
//! - If Esc is pressed: Returns an empty string ("")
//! - If the default parameter is provided and user clicks OK without entering text: Returns the default value
//!
//! ## Remarks
//!
//! The `InputBox` function provides a simple way to get user input:
//!
//! - Displays a modal dialog box that blocks execution until user responds
//! - The dialog always includes OK and Cancel buttons
//! - Pressing Enter is equivalent to clicking OK
//! - Pressing Esc is equivalent to clicking Cancel
//! - Cannot distinguish between Cancel and an empty string entered by user
//! - For multi-line input, use a custom form instead
//! - Maximum prompt length is approximately 1024 characters
//! - Position parameters (xpos, ypos) are in twips (1440 twips = 1 inch)
//! - Help integration requires both helpfile and context parameters
//! - The text box accepts a single line of text (no multi-line support)
//! - Input is returned as typed (no automatic validation or conversion)
//! - Always returns a `String`, even if numeric input is expected
//!
//! ## Typical Uses
//!
//! 1. **Simple User Input**: Get basic text input from users
//! 2. **Configuration Values**: Prompt for settings or preferences
//! 3. **Data Entry**: Quick single-value data entry
//! 4. **File Names**: Prompt for file or folder names
//! 5. **Search Terms**: Get search queries from users
//! 6. **Passwords**: Simple password entry (though text is visible)
//! 7. **Numeric Input**: Get numeric values (requires validation)
//! 8. **Confirmation Input**: Request verification text from users
//!
//! ## Basic Usage Examples
//!
//! ```vb
//! ' Example 1: Simple input
//! Dim userName As String
//! userName = InputBox("Enter your name:")
//! If userName <> "" Then
//!     MsgBox "Hello, " & userName
//! End If
//!
//! ' Example 2: With title and default
//! Dim age As String
//! age = InputBox("Enter your age:", "Age Entry", "18")
//! If IsNumeric(age) Then
//!     MsgBox "You are " & age & " years old"
//! End If
//!
//! ' Example 3: With position
//! Dim response As String
//! response = InputBox("Enter response:", "Input", "", 1000, 1000)
//!
//! ' Example 4: Multi-line prompt
//! Dim email As String
//! email = InputBox("Please enter your email address:" & vbCrLf & _
//!                  "This will be used for notifications.", _
//!                  "Email Address")
//! ```
//!
//! ## Common Patterns
//!
//! ```vb
//! ' Pattern 1: Validate numeric input
//! Function GetNumericInput(prompt As String, Optional default As String = "0") As Double
//!     Dim input As String
//!     Dim result As Double
//!     
//!     Do
//!         input = InputBox(prompt, "Numeric Input", default)
//!         
//!         If input = "" Then
//!             GetNumericInput = 0
//!             Exit Function
//!         End If
//!         
//!         If IsNumeric(input) Then
//!             GetNumericInput = CDbl(input)
//!             Exit Function
//!         End If
//!         
//!         MsgBox "Please enter a valid number", vbExclamation
//!     Loop
//! End Function
//!
//! ' Pattern 2: Required input (loop until provided)
//! Function GetRequiredInput(prompt As String, title As String) As String
//!     Dim input As String
//!     
//!     Do
//!         input = InputBox(prompt, title)
//!         
//!         If input <> "" Then
//!             GetRequiredInput = input
//!             Exit Function
//!         End If
//!         
//!         If MsgBox("Input is required. Try again?", vbYesNo) = vbNo Then
//!             GetRequiredInput = ""
//!             Exit Function
//!         End If
//!     Loop
//! End Function
//!
//! ' Pattern 3: Input with validation
//! Function GetEmailAddress() As String
//!     Dim email As String
//!     
//!     Do
//!         email = InputBox("Enter your email address:", "Email")
//!         
//!         If email = "" Then
//!             GetEmailAddress = ""
//!             Exit Function
//!         End If
//!         
//!         If InStr(email, "@") > 0 And InStr(email, ".") > 0 Then
//!             GetEmailAddress = email
//!             Exit Function
//!         End If
//!         
//!         MsgBox "Please enter a valid email address", vbExclamation
//!     Loop
//! End Function
//!
//! ' Pattern 4: Input with range validation
//! Function GetIntegerInRange(prompt As String, minVal As Long, maxVal As Long) As Long
//!     Dim input As String
//!     Dim value As Long
//!     
//!     Do
//!         input = InputBox(prompt & vbCrLf & _
//!                         "Range: " & minVal & " to " & maxVal, _
//!                         "Input", CStr(minVal))
//!         
//!         If input = "" Then
//!             GetIntegerInRange = minVal
//!             Exit Function
//!         End If
//!         
//!         If IsNumeric(input) Then
//!             value = CLng(input)
//!             If value >= minVal And value <= maxVal Then
//!                 GetIntegerInRange = value
//!                 Exit Function
//!             End If
//!         End If
//!         
//!         MsgBox "Please enter a value between " & minVal & " and " & maxVal, vbExclamation
//!     Loop
//! End Function
//!
//! ' Pattern 5: File name input with validation
//! Function GetFileName(prompt As String, Optional extension As String = "") As String
//!     Dim fileName As String
//!     
//!     Do
//!         fileName = InputBox(prompt, "File Name")
//!         
//!         If fileName = "" Then
//!             GetFileName = ""
//!             Exit Function
//!         End If
//!         
//!         ' Check for invalid characters
//!         If InStr(fileName, "\") > 0 Or InStr(fileName, "/") > 0 Or _
//!            InStr(fileName, ":") > 0 Or InStr(fileName, "*") > 0 Or _
//!            InStr(fileName, "?") > 0 Or InStr(fileName, """") > 0 Or _
//!            InStr(fileName, "<") > 0 Or InStr(fileName, ">") > 0 Or _
//!            InStr(fileName, "|") > 0 Then
//!             MsgBox "File name contains invalid characters", vbExclamation
//!         Else
//!             If extension <> "" And Right$(fileName, Len(extension)) <> extension Then
//!                 fileName = fileName & extension
//!             End If
//!             GetFileName = fileName
//!             Exit Function
//!         End If
//!     Loop
//! End Function
//!
//! ' Pattern 6: Password input (simple - visible text)
//! Function GetPassword(prompt As String) As String
//!     Dim password As String
//!     Dim confirm As String
//!     
//!     password = InputBox(prompt, "Password")
//!     
//!     If password = "" Then
//!         GetPassword = ""
//!         Exit Function
//!     End If
//!     
//!     confirm = InputBox("Confirm password:", "Confirm Password")
//!     
//!     If password = confirm Then
//!         GetPassword = password
//!     Else
//!         MsgBox "Passwords do not match", vbExclamation
//!         GetPassword = ""
//!     End If
//! End Function
//!
//! ' Pattern 7: Multiple inputs in sequence
//! Sub GetUserInfo()
//!     Dim firstName As String
//!     Dim lastName As String
//!     Dim email As String
//!     
//!     firstName = InputBox("Enter first name:", "User Information")
//!     If firstName = "" Then Exit Sub
//!     
//!     lastName = InputBox("Enter last name:", "User Information")
//!     If lastName = "" Then Exit Sub
//!     
//!     email = InputBox("Enter email:", "User Information")
//!     If email = "" Then Exit Sub
//!     
//!     MsgBox "User: " & firstName & " " & lastName & vbCrLf & _
//!            "Email: " & email
//! End Sub
//!
//! ' Pattern 8: Input with list of options in prompt
//! Function GetOption() As String
//!     Dim choice As String
//!     Dim prompt As String
//!     
//!     prompt = "Select an option:" & vbCrLf & _
//!              "1 - Create new file" & vbCrLf & _
//!              "2 - Open existing file" & vbCrLf & _
//!              "3 - Exit" & vbCrLf & vbCrLf & _
//!              "Enter choice (1-3):"
//!     
//!     choice = InputBox(prompt, "Main Menu", "1")
//!     
//!     Select Case choice
//!         Case "1", "2", "3"
//!             GetOption = choice
//!         Case Else
//!             GetOption = ""
//!     End Select
//! End Function
//!
//! ' Pattern 9: Trim and clean input
//! Function GetCleanInput(prompt As String, title As String) As String
//!     Dim input As String
//!     
//!     input = InputBox(prompt, title)
//!     
//!     ' Trim whitespace
//!     input = Trim$(input)
//!     
//!     ' Remove multiple spaces
//!     Do While InStr(input, "  ") > 0
//!         input = Replace(input, "  ", " ")
//!     Loop
//!     
//!     GetCleanInput = input
//! End Function
//!
//! ' Pattern 10: Cancel detection with default
//! Function GetInputWithCancelDetection(prompt As String, defaultValue As String) As Variant
//!     Dim input As String
//!     
//!     input = InputBox(prompt, "Input", defaultValue)
//!     
//!     If input = "" And defaultValue <> "" Then
//!         ' User likely clicked Cancel
//!         GetInputWithCancelDetection = Null
//!     Else
//!         GetInputWithCancelDetection = input
//!     End If
//! End Function
//! ```
//!
//! ## Advanced Usage Examples
//!
//! ```vb
//! ' Example 1: Configuration manager with InputBox
//! Public Class ConfigManager
//!     Private m_settings As Collection
//!     
//!     Private Sub Class_Initialize()
//!         Set m_settings = New Collection
//!     End Sub
//!     
//!     Public Function GetSetting(key As String, prompt As String, _
//!                                Optional defaultValue As String = "") As String
//!         On Error Resume Next
//!         GetSetting = m_settings(key)
//!         
//!         If Err.Number <> 0 Or GetSetting = "" Then
//!             Err.Clear
//!             GetSetting = InputBox(prompt, "Configuration: " & key, defaultValue)
//!             
//!             If GetSetting <> "" Then
//!                 m_settings.Add GetSetting, key
//!             End If
//!         End If
//!         On Error GoTo 0
//!     End Function
//!     
//!     Public Sub ClearSettings()
//!         Set m_settings = New Collection
//!     End Sub
//! End Class
//!
//! ' Example 2: Data validation wrapper
//! Public Class InputValidator
//!     Public Enum ValidationType
//!         vtText = 0
//!         vtInteger = 1
//!         vtDecimal = 2
//!         vtEmail = 3
//!         vtDate = 4
//!     End Enum
//!     
//!     Public Function GetValidatedInput(prompt As String, _
//!                                       validationType As ValidationType, _
//!                                       Optional title As String = "Input", _
//!                                       Optional defaultValue As String = "") As Variant
//!         Dim input As String
//!         Dim isValid As Boolean
//!         
//!         Do
//!             input = InputBox(prompt, title, defaultValue)
//!             
//!             If input = "" Then
//!                 GetValidatedInput = Null
//!                 Exit Function
//!             End If
//!             
//!             isValid = ValidateInput(input, validationType)
//!             
//!             If isValid Then
//!                 GetValidatedInput = ConvertInput(input, validationType)
//!                 Exit Function
//!             Else
//!                 MsgBox "Invalid input. Please try again.", vbExclamation
//!             End If
//!         Loop
//!     End Function
//!     
//!     Private Function ValidateInput(input As String, vType As ValidationType) As Boolean
//!         Select Case vType
//!             Case vtText
//!                 ValidateInput = Len(input) > 0
//!             Case vtInteger
//!                 ValidateInput = IsNumeric(input) And InStr(input, ".") = 0
//!             Case vtDecimal
//!                 ValidateInput = IsNumeric(input)
//!             Case vtEmail
//!                 ValidateInput = InStr(input, "@") > 0 And InStr(input, ".") > 0
//!             Case vtDate
//!                 ValidateInput = IsDate(input)
//!             Case Else
//!                 ValidateInput = False
//!         End Select
//!     End Function
//!     
//!     Private Function ConvertInput(input As String, vType As ValidationType) As Variant
//!         Select Case vType
//!             Case vtInteger
//!                 ConvertInput = CLng(input)
//!             Case vtDecimal
//!                 ConvertInput = CDbl(input)
//!             Case vtDate
//!                 ConvertInput = CDate(input)
//!             Case Else
//!                 ConvertInput = input
//!         End Select
//!     End Function
//! End Class
//!
//! ' Example 3: Wizard-style input sequence
//! Function RunWizard() As Boolean
//!     Dim step1 As String, step2 As String, step3 As String
//!     Dim prompt As String
//!     
//!     ' Step 1
//!     prompt = "Step 1 of 3:" & vbCrLf & _
//!              "Enter project name:"
//!     step1 = InputBox(prompt, "Project Wizard")
//!     If step1 = "" Then
//!         RunWizard = False
//!         Exit Function
//!     End If
//!     
//!     ' Step 2
//!     prompt = "Step 2 of 3:" & vbCrLf & _
//!              "Enter project location:"
//!     step2 = InputBox(prompt, "Project Wizard", "C:\Projects")
//!     If step2 = "" Then
//!         RunWizard = False
//!         Exit Function
//!     End If
//!     
//!     ' Step 3
//!     prompt = "Step 3 of 3:" & vbCrLf & _
//!              "Enter description:"
//!     step3 = InputBox(prompt, "Project Wizard")
//!     If step3 = "" Then step3 = "(No description)"
//!     
//!     ' Create project
//!     MsgBox "Creating project:" & vbCrLf & _
//!            "Name: " & step1 & vbCrLf & _
//!            "Location: " & step2 & vbCrLf & _
//!            "Description: " & step3
//!     
//!     RunWizard = True
//! End Function
//!
//! ' Example 4: Search query builder
//! Function BuildSearchQuery() As String
//!     Dim searchTerm As String
//!     Dim filters As String
//!     Dim query As String
//!     
//!     searchTerm = InputBox("Enter search term:", "Search")
//!     If searchTerm = "" Then
//!         BuildSearchQuery = ""
//!         Exit Function
//!     End If
//!     
//!     filters = InputBox("Enter filters (optional):" & vbCrLf & _
//!                       "Examples: category:books, year:2020", _
//!                       "Search Filters")
//!     
//!     query = "search=" & searchTerm
//!     If filters <> "" Then
//!         query = query & "&filters=" & filters
//!     End If
//!     
//!     BuildSearchQuery = query
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! The `InputBox` function rarely raises errors, but should be used with error handling:
//!
//! ```vb
//! On Error GoTo ErrorHandler
//! Dim userInput As String
//!
//! userInput = InputBox("Enter value:", "Input")
//!
//! If userInput = "" Then
//!     MsgBox "No input provided", vbInformation
//! Else
//!     ProcessInput userInput
//! End If
//! Exit Sub
//!
//! ErrorHandler:
//!     MsgBox "Error getting input: " & Err.Description, vbCritical
//! ```
//!
//! ## Performance Considerations
//!
//! - **Modal Dialog**: Blocks execution until user responds
//! - **User Interaction**: Performance depends entirely on user response time
//! - **String Returns**: Always returns a `String` regardless of expected data type
//! - **No Timeout**: Dialog waits indefinitely for user action
//! - **Lightweight**: Minimal overhead for displaying dialog
//!
//! ## Best Practices
//!
//! 1. **Clear Prompts**: Write clear, concise prompts that explain what input is needed
//! 2. **Validate Input**: Always validate and convert input as needed (`InputBox` returns `String`)
//! 3. **Handle Cancel**: Check for empty string return value (could be Cancel or empty input)
//! 4. **Provide Defaults**: Use default parameter for common or suggested values
//! 5. **Error Handling**: Wrap `InputBox` calls in error handling
//! 6. **Alternative UI**: For complex input, use custom forms instead of `InputBox`
//! 7. **Accessibility**: Consider users who need keyboard navigation
//! 8. **Multi-line Prompts**: Use `vbCrLf` to create multi-line prompts for clarity
//!
//! ## Comparison with Other Functions
//!
//! | Function | Purpose | Return Type |
//! |----------|---------|-------------|
//! | `InputBox` | Get user text input | `String` |
//! | `MsgBox` | Display message, get button click | `VbMsgBoxResult` |
//! | Custom Form | Complex input with multiple fields | Varies |
//! | `FileDialog` | Get file/folder selection | `String` |
//!
//! ## Platform and Version Notes
//!
//! - Available in all VB6 versions
//! - Dialog appearance follows Windows theme
//! - Maximum prompt length is approximately 1024 characters
//! - Position parameters use twips (1440 twips = 1 inch)
//! - No built-in password masking (text is visible)
//! - Help integration requires compiled help files (.hlp or .chm)
//!
//! ## Limitations
//!
//! - Single-line text input only (no multi-line support)
//! - Cannot distinguish between Cancel and empty input
//! - No input masking for passwords
//! - No built-in validation
//! - Modal dialog blocks all application interaction
//! - No timeout option (waits indefinitely)
//! - Limited formatting options for prompt text
//! - Cannot customize button labels (always OK/Cancel)
//! - No progress indication for long operations
//!
//! ## Related Functions
//!
//! - `MsgBox`: Display messages and get button responses
//! - `Input`: Read from files (different from `InputBox`)
//! - Custom Forms: For complex input scenarios
//! - `Shell`: Execute external programs for advanced input

use crate::error::{err_number, VBError, VBResult};
use crate::state;
use crate::value::{VBLong, VBString, VBVariant};

/// Implement VB6's `InputBox` function.
///
/// Enforces the helpfile/context pairing rule (raising error 5 when only
/// one is supplied — neither reaches the backend since no platform has a
/// Help system), assembles an [`InputBoxRequest`](crate::state::interaction::InputBoxRequest)
/// from the remaining arguments, hands it to the active
/// [`interaction backend`](crate::state::interaction), and returns the
/// entered text (or `""` for Cancel) as a String.
pub fn input_box(
    prompt: &VBString,
    title: Option<&VBString>,
    default: Option<&VBString>,
    xpos: Option<&VBLong>,
    ypos: Option<&VBLong>,
    helpfile: Option<&VBString>,
    context: Option<&VBLong>,
) -> VBResult<VBVariant> {
    // VB6 requires helpfile and context to appear together.
    if helpfile.is_some() != context.is_some() {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Invalid procedure call or argument: InputBox requires helpfile and context \
             to be provided together",
        ));
    }

    let mut request = state::interaction::InputBoxRequest::new(prompt.as_str());

    // An omitted or empty title means "show the application name"; hosts
    // resolve that from `None`.
    let title = title.and_then(|t| {
        let t = t.as_str();
        (!t.is_empty()).then(|| t.to_string())
    });
    request.title = title;

    if let Some(default) = default {
        request.default_response = default.as_str().to_string();
    }
    if let (Some(xpos), Some(ypos)) = (xpos.map(|v| v.as_i32()), ypos.map(|v| v.as_i32())) {
        request.xpos = Some(xpos);
        request.ypos = Some(ypos);
    }

    let answer = state::interaction::input_box(&request)?;
    Ok(VBVariant::from_string(answer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::interaction::{memory::MemoryBackend, InputBoxRecord};
    use crate::state::test_support::lock_test;

    fn set_backend(backend: MemoryBackend) {
        state::interaction::set_backend(Box::new(backend));
    }

    #[test]
    fn scripted_answer_is_returned_as_a_string() {
        let _guard = lock_test();
        set_backend(MemoryBackend::with_input_responses(["Arthur"]));
        let result =
            input_box(&VBString::from("name?"), None, None, None, None, None, None).unwrap();
        assert_eq!(result, VBVariant::from_string("Arthur"));
        state::interaction::reset_backend();
    }

    #[test]
    fn scripted_answers_are_consumed_in_order_then_default_kicks_in() {
        let _guard = lock_test();
        set_backend(MemoryBackend::with_input_responses(["first", "second"]));
        let default = VBString::from("fallback");

        assert_eq!(
            input_box(&VBString::from("?"), None, None, None, None, None, None).unwrap(),
            VBVariant::from_string("first")
        );
        assert_eq!(
            input_box(&VBString::from("?"), None, None, None, None, None, None).unwrap(),
            VBVariant::from_string("second")
        );
        // Queue dry: the dialog's default response is returned.
        assert_eq!(
            input_box(
                &VBString::from("?"),
                None,
                Some(&default),
                None,
                None,
                None,
                None
            )
            .unwrap(),
            VBVariant::from_string("fallback")
        );
        state::interaction::reset_backend();
    }

    #[test]
    fn omitted_default_and_exhausted_queue_yield_empty_string() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let result = input_box(&VBString::from("?"), None, None, None, None, None, None).unwrap();
        assert_eq!(result, VBVariant::from_string(""));
        state::interaction::reset_backend();
    }

    #[test]
    fn queued_empty_string_scripts_a_cancel() {
        let _guard = lock_test();
        set_backend(MemoryBackend::with_input_responses([""]));
        let result = input_box(&VBString::from("?"), None, None, None, None, None, None).unwrap();
        assert_eq!(result, VBVariant::from_string(""));
        state::interaction::reset_backend();
    }

    #[test]
    fn helpfile_without_context_is_error_5() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let helpfile = VBString::from("help.hlp");
        let err = input_box(
            &VBString::from("?"),
            None,
            None,
            None,
            None,
            Some(&helpfile),
            None,
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);

        let context = VBLong::from(10);
        let err = input_box(
            &VBString::from("?"),
            None,
            None,
            None,
            None,
            None,
            Some(&context),
        )
        .unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
        state::interaction::reset_backend();
    }

    #[test]
    fn paired_helpfile_and_context_are_accepted_but_ignored() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let helpfile = VBString::from("help.hlp");
        let context = VBLong::from(10);
        let result = input_box(
            &VBString::from("?"),
            None,
            None,
            None,
            None,
            Some(&helpfile),
            Some(&context),
        )
        .unwrap();
        assert_eq!(result, VBVariant::from_string(""));

        // The Help arguments never reach the backend's record.
        let requests: Vec<InputBoxRecord> =
            state::interaction::with_memory_backend(|m| m.inputbox_requests()).unwrap();
        assert_eq!(requests.len(), 1);
        state::interaction::reset_backend();
    }

    #[test]
    fn every_argument_reaches_the_backend_record() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let title = VBString::from("Config");
        let default = VBString::from("8080");
        let xpos = VBLong::from(1440);
        let ypos = VBLong::from(-2880);
        input_box(
            &VBString::from("port?"),
            Some(&title),
            Some(&default),
            Some(&xpos),
            Some(&ypos),
            None,
            None,
        )
        .unwrap();

        let requests = state::interaction::with_memory_backend(|m| m.inputbox_requests()).unwrap();
        assert_eq!(requests[0].prompt, "port?");
        assert_eq!(requests[0].title.as_deref(), Some("Config"));
        assert_eq!(requests[0].default_response, "8080");
        assert_eq!(requests[0].xpos, Some(1440));
        assert_eq!(requests[0].ypos, Some(-2880));
        state::interaction::reset_backend();
    }

    #[test]
    fn empty_title_falls_back_to_application_default() {
        let _guard = lock_test();
        set_backend(MemoryBackend::new());
        let title = VBString::from("");
        input_box(
            &VBString::from("?"),
            Some(&title),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let requests = state::interaction::with_memory_backend(|m| m.inputbox_requests()).unwrap();
        assert_eq!(requests[0].title, None);
        state::interaction::reset_backend();
    }
}
