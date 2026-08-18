//! # `LoadResString` Function
//!
//! Returns a string from a resource (.res) file.
//!
//! ## Syntax
//!
//! ```vb
//! LoadResString(index)
//! ```
//!
//! ## Parameters
//!
//! - `index` (Required): Integer identifying the string resource
//!   - Must be a numeric ID (string names not supported for string resources)
//!   - Must match the ID used when the resource was compiled
//!   - Typically ranges from 1 to 65535
//!
//! ## Return Value
//!
//! Returns a String:
//! - String containing the text from the resource file
//! - Empty string ("") if resource not found (in some VB versions)
//! - Raises error 326 if resource not found
//! - Preserves all formatting including line breaks
//! - Unicode strings supported in VB6
//!
//! ## Remarks
//!
//! The `LoadResString` function loads text from embedded resources:
//!
//! - Loads strings from compiled resource (.res) files
//! - Resource file must be linked to project at compile time
//! - Primary method for internationalization (i18n) in VB6
//! - Allows localizing applications without code changes
//! - Strings can be translated by replacing resource file
//! - No external text files needed at runtime
//! - Embedded in compiled EXE/DLL
//! - Only one resource file per project
//! - Resource file added via Project > Add File
//! - Resource files created with Resource Editor or RC.EXE
//! - Index must be numeric (string names not supported)
//! - Common for error messages, prompts, labels
//! - Supports Unicode in VB6
//! - Error 326: "Resource with identifier not found" if ID doesn't exist
//! - Error 48: "Error loading from file" if resource file corrupt
//! - More maintainable than hardcoded strings
//! - Easier to update text without recompiling code
//! - Standard practice for multi-language applications
//! - Can store long text passages
//! - Supports special characters and formatting
//!
//! ## Typical Uses
//!
//! 1. **Load Error Message**
//!    ```vb
//!    MsgBox LoadResString(1001), vbCritical
//!    ```
//!
//! 2. **Load Form Caption**
//!    ```vb
//!    Me.Caption = LoadResString(2001)
//!    ```
//!
//! 3. **Load Label Text**
//!    ```vb
//!    lblWelcome.Caption = LoadResString(3001)
//!    ```
//!
//! 4. **Load Menu Caption**
//!    ```vb
//!    mnuFile.Caption = LoadResString(4001)
//!    ```
//!
//! 5. **Load Button Caption**
//!    ```vb
//!    cmdOK.Caption = LoadResString(5001)
//!    ```
//!
//! 6. **Load `MessageBox` Text**
//!    ```vb
//!    MsgBox LoadResString(6001), vbInformation
//!    ```
//!
//! 7. **Load `StatusBar` Text**
//!    ```vb
//!    StatusBar1.SimpleText = LoadResString(7001)
//!    ```
//!
//! 8. **Load `ToolTip` Text**
//!    ```vb
//!    cmdSave.ToolTipText = LoadResString(8001)
//!    ```
//!
//! ## Basic Examples
//!
//! ### Example 1: Loading Messages
//! ```vb
//! ' Load various UI strings from resources
//! Me.Caption = LoadResString(1001)          ' "My Application"
//! lblTitle.Caption = LoadResString(1002)    ' "Welcome!"
//! cmdOK.Caption = LoadResString(1003)       ' "OK"
//! cmdCancel.Caption = LoadResString(1004)   ' "Cancel"
//! ```
//!
//! ### Example 2: Error Messages
//! ```vb
//! ' Use resource strings for error messages
//! If Not fileExists Then
//!     MsgBox LoadResString(2001), vbCritical  ' "File not found"
//! End If
//!
//! If accessDenied Then
//!     MsgBox LoadResString(2002), vbCritical  ' "Access denied"
//! End If
//! ```
//!
//! ### Example 3: Error Handling
//! ```vb
//! On Error Resume Next
//! Dim msg As String
//! msg = LoadResString(9999)
//! If Err.Number = 326 Then
//!     msg = "String resource not found!"
//!     Err.Clear
//! End If
//! MsgBox msg
//! ```
//!
//! ### Example 4: Form Initialization
//! ```vb
//! Private Sub Form_Load()
//!     ' Load all UI strings from resources
//!     Me.Caption = LoadResString(1001)
//!     lblName.Caption = LoadResString(1002)
//!     lblAddress.Caption = LoadResString(1003)
//!     cmdSave.Caption = LoadResString(1004)
//!     cmdCancel.Caption = LoadResString(1005)
//! End Sub
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `SafeLoadResString`
//! ```vb
//! Function SafeLoadResString(ByVal resID As Integer, _
//!                            Optional ByVal defaultText As String = "") As String
//!     On Error Resume Next
//!     SafeLoadResString = LoadResString(resID)
//!     If Err.Number <> 0 Then
//!         SafeLoadResString = defaultText
//!         Err.Clear
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 2: `LoadFormStrings`
//! ```vb
//! Sub LoadFormStrings(frm As Form, ByVal baseID As Integer)
//!     Dim ctrl As Control
//!     Dim id As Integer
//!     
//!     On Error Resume Next
//!     frm.Caption = LoadResString(baseID)
//!     
//!     id = baseID + 1
//!     For Each ctrl In frm.Controls
//!         If TypeOf ctrl Is Label Or TypeOf ctrl Is CommandButton Then
//!             ctrl.Caption = LoadResString(id)
//!             id = id + 1
//!         End If
//!     Next ctrl
//! End Sub
//! ```
//!
//! ### Pattern 3: `FormatResString`
//! ```vb
//! Function FormatResString(ByVal resID As Integer, _
//!                          ParamArray args()) As String
//!     Dim template As String
//!     Dim i As Long
//!     
//!     template = LoadResString(resID)
//!     
//!     For i = LBound(args) To UBound(args)
//!         template = Replace(template, "{" & i & "}", CStr(args(i)))
//!     Next i
//!     
//!     FormatResString = template
//! End Function
//! ```
//!
//! ### Pattern 4: `GetErrorMessage`
//! ```vb
//! Function GetErrorMessage(ByVal errorCode As Long) As String
//!     Const BASE_ERROR_ID = 10000
//!     On Error Resume Next
//!     
//!     GetErrorMessage = LoadResString(BASE_ERROR_ID + errorCode)
//!     If Err.Number <> 0 Then
//!         GetErrorMessage = "Unknown error: " & errorCode
//!         Err.Clear
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 5: `LoadMenuStrings`
//! ```vb
//! Sub LoadMenuStrings()
//!     Const MENU_BASE = 4000
//!     
//!     mnuFile.Caption = LoadResString(MENU_BASE + 1)      ' "&File"
//!     mnuFileNew.Caption = LoadResString(MENU_BASE + 2)   ' "&New"
//!     mnuFileOpen.Caption = LoadResString(MENU_BASE + 3)  ' "&Open"
//!     mnuFileSave.Caption = LoadResString(MENU_BASE + 4)  ' "&Save"
//!     mnuFileExit.Caption = LoadResString(MENU_BASE + 5)  ' "E&xit"
//! End Sub
//! ```
//!
//! ### Pattern 6: `CachedResString`
//! ```vb
//! Dim resStringCache As New Collection
//!
//! Function CachedLoadResString(ByVal resID As Integer) As String
//!     Dim key As String
//!     On Error Resume Next
//!     
//!     key = "RES_" & resID
//!     CachedLoadResString = resStringCache(key)
//!     
//!     If Err.Number <> 0 Then
//!         Err.Clear
//!         CachedLoadResString = LoadResString(resID)
//!         resStringCache.Add CachedLoadResString, key
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 7: `ResStringExists`
//! ```vb
//! Function ResStringExists(ByVal resID As Integer) As Boolean
//!     On Error Resume Next
//!     Dim s As String
//!     s = LoadResString(resID)
//!     ResStringExists = (Err.Number = 0)
//!     Err.Clear
//! End Function
//! ```
//!
//! ### Pattern 8: `LoadResStringArray`
//! ```vb
//! Function LoadResStringArray(ByVal startID As Integer, _
//!                             ByVal count As Integer) As String()
//!     Dim result() As String
//!     Dim i As Integer
//!     
//!     ReDim result(0 To count - 1)
//!     
//!     On Error Resume Next
//!     For i = 0 To count - 1
//!         result(i) = LoadResString(startID + i)
//!         If Err.Number <> 0 Then
//!             result(i) = ""
//!             Err.Clear
//!         End If
//!     Next i
//!     
//!     LoadResStringArray = result
//! End Function
//! ```
//!
//! ### Pattern 9: `ShowResMessage`
//! ```vb
//! Sub ShowResMessage(ByVal resID As Integer, _
//!                    Optional ByVal icon As VbMsgBoxStyle = vbInformation)
//!     On Error Resume Next
//!     Dim msg As String
//!     msg = LoadResString(resID)
//!     
//!     If Err.Number = 0 Then
//!         MsgBox msg, icon
//!     Else
//!         MsgBox "Message resource " & resID & " not found", vbCritical
//!         Err.Clear
//!     End If
//! End Sub
//! ```
//!
//! ### Pattern 10: `MultiLineResString`
//! ```vb
//! Function MultiLineResString(ByVal resID As Integer) As String
//!     Dim text As String
//!     text = LoadResString(resID)
//!     ' Resource strings preserve line breaks
//!     MultiLineResString = text
//! End Function
//! ```
//!
//! ## Advanced Examples
//!
//! ### Example 1: Localization Manager
//! ```vb
//! ' Class: LocalizationManager
//! Private m_cache As Collection
//! Private m_languageID As Integer
//!
//! Private Sub Class_Initialize()
//!     Set m_cache = New Collection
//!     m_languageID = 1033 ' Default to English (US)
//! End Sub
//!
//! Public Property Let LanguageID(ByVal newLanguage As Integer)
//!     m_languageID = newLanguage
//!     ClearCache
//! End Property
//!
//! Public Function GetString(ByVal baseID As Integer) As String
//!     Dim resID As Integer
//!     Dim key As String
//!     
//!     On Error Resume Next
//!     resID = baseID + m_languageID
//!     key = "STR_" & resID
//!     
//!     GetString = m_cache(key)
//!     If Err.Number <> 0 Then
//!         Err.Clear
//!         GetString = LoadResString(resID)
//!         If Err.Number = 0 Then
//!             m_cache.Add GetString, key
//!         Else
//!             ' Fallback to default language
//!             GetString = LoadResString(baseID + 1033)
//!             Err.Clear
//!         End If
//!     End If
//! End Function
//!
//! Public Sub LocalizeForm(frm As Form)
//!     Dim ctrl As Control
//!     On Error Resume Next
//!     
//!     ' Load form caption
//!     frm.Caption = GetString(GetFormBaseID(frm))
//!     
//!     ' Load control captions
//!     For Each ctrl In frm.Controls
//!         If HasCaption(ctrl) Then
//!             ctrl.Caption = GetString(GetControlID(ctrl))
//!         End If
//!     Next ctrl
//! End Sub
//!
//! Private Sub ClearCache()
//!     Set m_cache = New Collection
//! End Sub
//!
//! Private Function HasCaption(ctrl As Control) As Boolean
//!     HasCaption = TypeOf ctrl Is Label Or _
//!                  TypeOf ctrl Is CommandButton Or _
//!                  TypeOf ctrl Is CheckBox Or _
//!                  TypeOf ctrl Is OptionButton
//! End Function
//!
//! Private Sub Class_Terminate()
//!     Set m_cache = Nothing
//! End Sub
//! ```
//!
//! ### Example 2: Error Message System
//! ```vb
//! ' Module: ErrorMessages
//! Private Const ERR_BASE = 20000
//!
//! Public Enum AppError
//!     errFileNotFound = 1
//!     errAccessDenied = 2
//!     errInvalidFormat = 3
//!     errNetworkError = 4
//!     errDatabaseError = 5
//! End Enum
//!
//! Public Sub ShowError(ByVal errorType As AppError, _
//!                      Optional ByVal additionalInfo As String = "")
//!     Dim msg As String
//!     On Error Resume Next
//!     
//!     msg = LoadResString(ERR_BASE + errorType)
//!     If Err.Number <> 0 Then
//!         msg = "Unknown error occurred"
//!         Err.Clear
//!     End If
//!     
//!     If Len(additionalInfo) > 0 Then
//!         msg = msg & vbCrLf & vbCrLf & additionalInfo
//!     End If
//!     
//!     MsgBox msg, vbCritical, LoadResString(ERR_BASE)
//! End Sub
//!
//! Public Function GetErrorText(ByVal errorType As AppError) As String
//!     On Error Resume Next
//!     GetErrorText = LoadResString(ERR_BASE + errorType)
//!     If Err.Number <> 0 Then
//!         GetErrorText = "Unknown error"
//!         Err.Clear
//!     End If
//! End Function
//! ```
//!
//! ### Example 3: Multi-Language Application
//! ```vb
//! ' Form with language selection
//! Public Enum Language
//!     langEnglish = 0
//!     langSpanish = 1000
//!     langFrench = 2000
//!     langGerman = 3000
//! End Enum
//!
//! Private currentLanguage As Language
//!
//! Private Sub Form_Load()
//!     ' Default to English
//!     currentLanguage = langEnglish
//!     LoadLanguage
//! End Sub
//!
//! Private Sub cboLanguage_Click()
//!     Select Case cboLanguage.ListIndex
//!         Case 0: currentLanguage = langEnglish
//!         Case 1: currentLanguage = langSpanish
//!         Case 2: currentLanguage = langFrench
//!         Case 3: currentLanguage = langGerman
//!     End Select
//!     LoadLanguage
//! End Sub
//!
//! Private Sub LoadLanguage()
//!     Dim baseID As Integer
//!     baseID = 10000 + currentLanguage
//!     
//!     On Error Resume Next
//!     Me.Caption = LoadResString(baseID + 1)
//!     lblWelcome.Caption = LoadResString(baseID + 2)
//!     lblInstructions.Caption = LoadResString(baseID + 3)
//!     cmdStart.Caption = LoadResString(baseID + 4)
//!     cmdExit.Caption = LoadResString(baseID + 5)
//!     
//!     ' Update menu
//!     mnuFile.Caption = LoadResString(baseID + 10)
//!     mnuHelp.Caption = LoadResString(baseID + 11)
//! End Sub
//! ```
//!
//! ### Example 4: String Template System
//! ```vb
//! ' Module: StringTemplates
//! Private Const TEMPLATE_BASE = 30000
//!
//! Public Function GetFormattedString(ByVal templateID As Integer, _
//!                                    ParamArray values()) As String
//!     Dim template As String
//!     Dim result As String
//!     Dim i As Long
//!     
//!     On Error Resume Next
//!     template = LoadResString(TEMPLATE_BASE + templateID)
//!     If Err.Number <> 0 Then
//!         GetFormattedString = ""
//!         Err.Clear
//!         Exit Function
//!     End If
//!     
//!     result = template
//!     For i = LBound(values) To UBound(values)
//!         result = Replace(result, "{" & i & "}", CStr(values(i)))
//!     Next i
//!     
//!     GetFormattedString = result
//! End Function
//!
//! Public Function GetWelcomeMessage(ByVal userName As String) As String
//!     ' Template: "Welcome, {0}! You have {1} new messages."
//!     GetWelcomeMessage = GetFormattedString(1, userName, GetMessageCount())
//! End Function
//!
//! Public Function GetSaveConfirmation(ByVal filename As String) As String
//!     ' Template: "Do you want to save changes to {0}?"
//!     GetSaveConfirmation = GetFormattedString(2, filename)
//! End Function
//!
//! Private Function GetMessageCount() As Long
//!     ' Implementation would return actual message count
//!     GetMessageCount = 5
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb
//! ' Error 326: Resource with identifier not found
//! On Error Resume Next
//! Dim msg As String
//! msg = LoadResString(9999)
//! If Err.Number = 326 Then
//!     MsgBox "String resource not found!"
//! End If
//!
//! ' Error 48: Error loading from file
//! msg = LoadResString(1001)
//! If Err.Number = 48 Then
//!     MsgBox "Resource file is corrupt or missing!"
//! End If
//!
//! ' Safe loading pattern
//! Function TryLoadResString(ByVal resID As Integer, _
//!                           ByRef outString As String) As Boolean
//!     On Error Resume Next
//!     outString = LoadResString(resID)
//!     TryLoadResString = (Err.Number = 0)
//!     If Err.Number <> 0 Then
//!         outString = ""
//!     End If
//!     Err.Clear
//! End Function
//! ```
//!
//! ## Performance Considerations
//!
//! - **Fast Access**: Strings embedded in EXE (very fast loading)
//! - **No File I/O**: No disk access required
//! - **No Caching**: Each call loads fresh copy (implement caching if needed)
//! - **Memory Efficient**: Strings only loaded when accessed
//! - **Cache Strategy**: For frequently used strings, cache in Collection or array
//! - **Startup Time**: Loading many strings at startup may slow `Form_Load`
//!
//! ## Best Practices
//!
//! 1. **Use constants** for string IDs for maintainability
//! 2. **Group by category** using ID ranges (1000s for errors, 2000s for menus, etc.)
//! 3. **Cache frequently used strings** to improve performance
//! 4. **Always handle errors** - resource might not exist
//! 5. **Document string IDs** in code or separate file
//! 6. **Use templates** with placeholders for dynamic content
//! 7. **Organize by language** using ID offsets (English: 0, Spanish: +1000, etc.)
//! 8. **Test all languages** before deployment
//! 9. **Provide fallbacks** for missing strings
//! 10. **Keep strings updated** in sync with code changes
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Return Type | Data Type |
//! |----------|---------|-------------|-----------|
//! | **`LoadResString`** | Load string from resources | String | Text strings |
//! | **`LoadResPicture`** | Load image from resources | `StdPicture` | Images |
//! | **`LoadResData`** | Load binary data from resources | Byte array | Binary data |
//! | **`LoadString`** (API) | Windows API alternative | String | Text strings |
//!
//! ## `LoadResString` vs Hardcoded Strings
//!
//! ```vb
//! ' Hardcoded - difficult to localize
//! MsgBox "File not found", vbCritical
//!
//! ' Resource string - easy to localize
//! MsgBox LoadResString(1001), vbCritical
//! ```
//!
//! **Advantages of `LoadResString`:**
//! - Easy localization (just replace .res file)
//! - Centralized string management
//! - No code changes needed for translations
//! - Consistent messaging across application
//!
//! ## String ID Organization
//!
//! ```vb
//! ' Recommended ID ranges
//! Const STR_APP_BASE = 1000         ' Application strings
//! Const STR_ERROR_BASE = 2000       ' Error messages
//! Const STR_MENU_BASE = 3000        ' Menu items
//! Const STR_DIALOG_BASE = 4000      ' Dialog messages
//! Const STR_STATUS_BASE = 5000      ' Status messages
//! Const STR_HELP_BASE = 6000        ' Help text
//!
//! ' Language offsets
//! Const LANG_ENGLISH = 0
//! Const LANG_SPANISH = 10000
//! Const LANG_FRENCH = 20000
//! ```
//!
//! ## Platform Notes
//!
//! - Available in VB6 (not in early VB versions)
//! - Requires resource file (.res) linked to project
//! - Resource file created with Resource Editor or RC.EXE
//! - Only one resource file per project
//! - Resources embedded in compiled EXE/DLL
//! - Supports Unicode strings in VB6
//! - Index must be Integer (1-65535)
//! - String names not supported (numeric IDs only)
//! - Standard method for internationalization
//! - Preserves formatting including line breaks
//!
//! ## Limitations
//!
//! - **One Resource File**: Only one .res file per project
//! - **Numeric IDs Only**: Cannot use string names for string resources
//! - **Compile Time**: Must recompile to update strings
//! - **No Modification**: Cannot modify resources at runtime
//! - **Limited Editor**: VB6 Resource Editor is basic
//! - **ID Range**: Limited to 1-65535
//! - **No Encryption**: Strings easily extractable from EXE
//! - **No Formatting**: No printf-style formatting (must implement manually)
//! - **No Pluralization**: No built-in plural form handling
//! - **No Context**: All strings in flat namespace
//!
//! ## Related Functions
//!
//! - `LoadResPicture`: Load picture from resource file
//! - `LoadResData`: Load binary data from resource file
//! - `LoadPicture`: Load picture from external file
//! - `Format`: Format strings with values
//! - `Replace`: Replace placeholders in strings

use super::resfile::{rt, ResFile, STRINGS_PER_BUNDLE};
use super::resource_not_found;
use crate::error::VBResult;
use crate::state::resources;
use crate::value::VBVariant;

/// Implementation of the `LoadResString` function.
///
/// VB6 behavior:
/// - Reads a string from the project's linked `.res` file
/// - `index` must be numeric; string names are not supported for strings
/// - Returns the string, preserving embedded line breaks and Unicode
/// - Raises error 326 if the string, or the resource file, is not found
///
/// # String bundles
///
/// Win32 does not store one resource per string. Strings are grouped into
/// bundles of [`STRINGS_PER_BUNDLE`], so the string with ID `index` lives in
/// the `RT_STRING` resource whose ordinal is `index / 16 + 1`, at position
/// `index % 16` within it. Each bundle is a run of 16 length-prefixed UTF-16LE
/// strings:
///
/// ```text
/// u16  length in UTF-16 code units (0 = that slot is unused)
/// ...  `length` UTF-16LE code units
/// ```
///
/// A zero-length slot means no string was defined for that ID, which is
/// reported as error 326 exactly as a missing bundle is.
pub fn loadresstring(index: &VBVariant) -> VBResult<VBVariant> {
    // Strings are addressed only by number: a name cannot identify a slot
    // within a bundle.
    if !index.is_numeric() {
        return Err(resource_not_found());
    }
    let id = index.as_i32().map_err(|_| resource_not_found())?;
    let id = u16::try_from(id).map_err(|_| resource_not_found())?;

    let text = resources::with_file(|res| string_from_bundle(res, id))?;
    Ok(VBVariant::from_string(text))
}

/// Extracts the string with ID `id` from its `RT_STRING` bundle.
fn string_from_bundle(res: &ResFile, id: u16) -> VBResult<String> {
    let bundle_ordinal = id / STRINGS_PER_BUNDLE + 1;
    let slot = usize::from(id % STRINGS_PER_BUNDLE);

    let entry = res
        .find_by_ordinal(rt::STRING, bundle_ordinal)
        .ok_or_else(resource_not_found)?;
    let data = res.data(entry);

    // Walk the length-prefixed runs to reach the requested slot. Slots before
    // it must still be traversed, since each one's length sets the next offset.
    let mut offset = 0usize;
    for current in 0..=slot {
        let length = read_u16(data, offset).ok_or_else(resource_not_found)?;
        offset += 2;

        let byte_length = usize::from(length) * 2;
        let units = data
            .get(offset..offset + byte_length)
            .ok_or_else(resource_not_found)?;

        if current == slot {
            if length == 0 {
                // Slot defined by the bundle's layout but holding no string.
                return Err(resource_not_found());
            }
            let code_units: Vec<u16> = units
                .as_chunks::<2>()
                .0
                .iter()
                .map(|pair| u16::from_le_bytes(*pair))
                .collect();
            return Ok(String::from_utf16_lossy(&code_units));
        }

        offset += byte_length;
    }

    Err(resource_not_found())
}

/// Reads a little-endian `u16` at `offset`, or `None` past the end.
fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    data.get(offset..offset + 2)
        .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::err_number;
    use crate::library::resources::test_support::{null_record, record, with_linked_res};

    /// Builds an `RT_STRING` bundle body from 16 optional slots.
    fn bundle(slots: &[&str]) -> Vec<u8> {
        assert!(slots.len() <= STRINGS_PER_BUNDLE as usize);
        let mut bytes = Vec::new();
        for slot in 0..STRINGS_PER_BUNDLE as usize {
            let text = slots.get(slot).copied().unwrap_or("");
            let units: Vec<u16> = text.encode_utf16().collect();
            bytes.extend_from_slice(&(units.len() as u16).to_le_bytes());
            for unit in units {
                bytes.extend_from_slice(&unit.to_le_bytes());
            }
        }
        bytes
    }

    /// A `.res` image whose first bundle (IDs 0-15) holds `slots`.
    fn res_with_first_bundle(slots: &[&str]) -> Vec<u8> {
        let mut bytes = null_record();
        bytes.extend(record(rt::STRING, 1, &bundle(slots)));
        bytes
    }

    #[test]
    fn loads_the_first_string_in_a_bundle() {
        // Slot 0 of bundle 1 is ID 0.
        let image = res_with_first_bundle(&["first"]);
        with_linked_res(&image, || {
            let value = loadresstring(&VBVariant::from_integer(0)).unwrap();
            assert_eq!(value.as_str(), Some("first"));
        });
    }

    #[test]
    fn loads_a_later_slot_in_the_same_bundle() {
        let image = res_with_first_bundle(&["zero", "one", "two", "three"]);
        with_linked_res(&image, || {
            assert_eq!(
                loadresstring(&VBVariant::from_integer(2)).unwrap().as_str(),
                Some("two")
            );
            assert_eq!(
                loadresstring(&VBVariant::from_integer(3)).unwrap().as_str(),
                Some("three")
            );
        });
    }

    #[test]
    fn maps_ids_to_the_correct_bundle() {
        // ID 1001 -> bundle ordinal 1001/16+1 = 63, slot 1001%16 = 9.
        let mut slots = vec![""; 16];
        slots[9] = "the-message";
        let mut image = null_record();
        image.extend(record(rt::STRING, 63, &bundle(&slots)));

        with_linked_res(&image, || {
            let value = loadresstring(&VBVariant::from_long(1001)).unwrap();
            assert_eq!(value.as_str(), Some("the-message"));
        });
    }

    #[test]
    fn preserves_line_breaks_and_unicode() {
        let image = res_with_first_bundle(&["line1\r\nline2 \u{00e9}\u{4e2d}"]);
        with_linked_res(&image, || {
            let value = loadresstring(&VBVariant::from_integer(0)).unwrap();
            assert_eq!(value.as_str(), Some("line1\r\nline2 \u{00e9}\u{4e2d}"));
        });
    }

    #[test]
    fn empty_slot_raises_326() {
        // Slot 5 is present in the layout but zero-length.
        let image = res_with_first_bundle(&["zero"]);
        with_linked_res(&image, || {
            let error = loadresstring(&VBVariant::from_integer(5)).unwrap_err();
            assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
        });
    }

    #[test]
    fn missing_bundle_raises_326() {
        let image = res_with_first_bundle(&["zero"]);
        with_linked_res(&image, || {
            // ID 5000 lands in bundle 313, which the file does not contain.
            let error = loadresstring(&VBVariant::from_long(5000)).unwrap_err();
            assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
        });
    }

    #[test]
    fn string_index_is_rejected() {
        let image = res_with_first_bundle(&["zero"]);
        with_linked_res(&image, || {
            let error = loadresstring(&VBVariant::from_string("MESSAGE")).unwrap_err();
            assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
        });
    }

    #[test]
    fn out_of_range_index_raises_326() {
        let image = res_with_first_bundle(&["zero"]);
        with_linked_res(&image, || {
            for index in [
                VBVariant::from_long(-1),
                VBVariant::from_long(70000),
                VBVariant::Null,
            ] {
                let error = loadresstring(&index).unwrap_err();
                assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
            }
        });
    }

    #[test]
    fn truncated_bundle_raises_326_rather_than_panicking() {
        // A bundle claiming a 40-unit string but holding only a few bytes.
        let mut body = Vec::new();
        body.extend_from_slice(&40u16.to_le_bytes());
        body.extend_from_slice(&[0x41, 0x00, 0x42, 0x00]);
        let mut image = null_record();
        image.extend(record(rt::STRING, 1, &body));

        with_linked_res(&image, || {
            let error = loadresstring(&VBVariant::from_integer(0)).unwrap_err();
            assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
        });
    }

    #[test]
    fn no_linked_resource_file_raises_326() {
        let _guard = crate::state::test_support::lock_test();
        resources::clear();
        let error = loadresstring(&VBVariant::from_integer(1)).unwrap_err();
        assert_eq!(error.number, err_number::RESOURCE_NOT_FOUND);
    }
}
