//! # Oct Function
//!
//! Returns a String representing the octal (base-8) value of a number.
//!
//! ## Syntax
//!
//! ```vb
//! Oct(number)
//! ```
//!
//! ## Parameters
//!
//! - `number` - Required. Any valid numeric expression or string expression. If not a whole number,
//!   it is rounded to the nearest whole number before being evaluated.
//!
//! ## Return Value
//!
//! Returns a `String` representing the octal value of the number.
//!
//! ## Remarks
//!
//! The `Oct` function converts a decimal number to its octal (base-8) representation. Octal numbers
//! use only the digits 0-7. This function is primarily used for low-level programming, bit manipulation,
//! file permissions (especially on Unix systems), and legacy system integration.
//!
//! If `number` is negative, the function returns the octal representation of the two's complement
//! representation of the number. For Long values, this produces an 11-digit octal string.
//!
//! The returned string contains only the octal digits without any prefix (like "&O" or "0o").
//! To use the result in VB6 code as an octal literal, you need to prefix it with "&O".
//!
//! If `number` is `Null`, `Oct` returns `Null`. If `number` is not a whole number, it is rounded
//! to the nearest whole number before conversion.
//!
//! ## Typical Uses
//!
//! 1. **File Permission Representation**: Converting Unix file permissions to octal notation (e.g., 755, 644)
//! 2. **Bit Mask Display**: Showing bit patterns in octal for easier reading than binary
//! 3. **Legacy System Integration**: Interfacing with systems that use octal notation
//! 4. **Low-Level Programming**: Working with hardware registers or memory addresses
//! 5. **Data Conversion**: Converting between number bases for educational or debugging purposes
//! 6. **System Programming**: Unix/Linux system calls often use octal for permissions
//! 7. **Debugging Display**: Showing numeric values in alternative base for analysis
//! 8. **Configuration Files**: Some configuration formats use octal notation
//!
//! ## Basic Examples
//!
//! ### Example 1: Simple Conversion
//! ```vb
//! Dim result As String
//! result = Oct(64)        ' Returns "100"
//! result = Oct(8)         ' Returns "10"
//! result = Oct(511)       ' Returns "777"
//! ```
//!
//! ### Example 2: File Permissions
//! ```vb
//! ' Convert Unix file permission to octal
//! Dim permissions As Integer
//! permissions = 493       ' Decimal for rwxr-xr-x
//! Debug.Print Oct(permissions)  ' Displays "755"
//! ```
//!
//! ### Example 3: Negative Numbers
//! ```vb
//! ' Negative numbers show two's complement
//! Dim negValue As String
//! negValue = Oct(-1)      ' Returns "177777" for Integer
//! negValue = Oct(-10)     ' Returns "177766"
//! ```
//!
//! ### Example 4: Display Value in Multiple Bases
//! ```vb
//! Sub ShowBases(num As Long)
//!     Debug.Print "Decimal: " & num
//!     Debug.Print "Hex: " & Hex(num)
//!     Debug.Print "Octal: " & Oct(num)
//!     Debug.Print "Binary: " & ConvertToBinary(num)  ' Custom function
//! End Sub
//! ```
//!
//! ## Common Patterns
//!
//! ### Pattern 1: `ConvertFilePermission`
//! ```vb
//! Function ConvertFilePermission(owner As Integer, group As Integer, other As Integer) As String
//!     ' Each parameter is 0-7 (rwx bits)
//!     Dim permValue As Integer
//!     permValue = owner * 64 + group * 8 + other
//!     ConvertFilePermission = Oct(permValue)
//! End Function
//! ' Usage: perms = ConvertFilePermission(7, 5, 5)  ' Returns "755"
//! ```
//!
//! ### Pattern 2: `DisplayInOctal`
//! ```vb
//! Function DisplayInOctal(value As Long) As String
//!     ' Display with octal prefix for clarity
//!     DisplayInOctal = "&O" & Oct(value)
//! End Function
//! ' Usage: Debug.Print DisplayInOctal(64)  ' Displays "&O100"
//! ```
//!
//! ### Pattern 3: `ParseOctalString`
//! ```vb
//! Function ParseOctalString(octStr As String) As Long
//!     ' Convert octal string back to decimal
//!     If Left(octStr, 2) = "&O" Then
//!         octStr = Mid(octStr, 3)
//!     End If
//!     ParseOctalString = CLng("&O" & octStr)
//! End Function
//! ```
//!
//! ### Pattern 4: `ValidateOctalDigits`
//! ```vb
//! Function ValidateOctalDigits(octStr As String) As Boolean
//!     Dim i As Integer
//!     ValidateOctalDigits = True
//!     For i = 1 To Len(octStr)
//!         If Mid(octStr, i, 1) < "0" Or Mid(octStr, i, 1) > "7" Then
//!             ValidateOctalDigits = False
//!             Exit Function
//!         End If
//!     Next i
//! End Function
//! ```
//!
//! ### Pattern 5: `ConvertBetweenBases`
//! ```vb
//! Sub ShowAllBases(decimalValue As Long)
//!     Debug.Print "Dec: " & decimalValue & _
//!                 " Hex: " & Hex(decimalValue) & _
//!                 " Oct: " & Oct(decimalValue)
//! End Sub
//! ```
//!
//! ### Pattern 6: `CheckOctalRange`
//! ```vb
//! Function IsValidOctalPermission(value As Integer) As Boolean
//!     ' Check if value represents valid Unix permissions (0-777)
//!     IsValidOctalPermission = (value >= 0 And value <= 511)  ' 511 = &O777
//! End Function
//! ```
//!
//! ### Pattern 7: `FormatOctalWithPadding`
//! ```vb
//! Function FormatOctal(value As Long, digits As Integer) As String
//!     Dim octStr As String
//!     octStr = Oct(value)
//!     FormatOctal = String(digits - Len(octStr), "0") & octStr
//! End Function
//! ' Usage: perms = FormatOctal(64, 4)  ' Returns "0100"
//! ```
//!
//! ### Pattern 8: `ExtractOctalDigits`
//! ```vb
//! Function GetOctalDigit(value As Long, position As Integer) As Integer
//!     ' Get specific octal digit (0-based from right)
//!     Dim octStr As String
//!     octStr = Oct(value)
//!     If position >= 0 And position < Len(octStr) Then
//!         GetOctalDigit = CInt(Mid(octStr, Len(octStr) - position, 1))
//!     Else
//!         GetOctalDigit = 0
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 9: `CompareOctalValues`
//! ```vb
//! Function CompareAsOctal(val1 As Long, val2 As Long) As String
//!     If val1 = val2 Then
//!         CompareAsOctal = Oct(val1) & " equals " & Oct(val2)
//!     ElseIf val1 > val2 Then
//!         CompareAsOctal = Oct(val1) & " > " & Oct(val2)
//!     Else
//!         CompareAsOctal = Oct(val1) & " < " & Oct(val2)
//!     End If
//! End Function
//! ```
//!
//! ### Pattern 10: `BuildOctalTable`
//! ```vb
//! Sub CreateOctalTable(maxValue As Integer)
//!     Dim i As Integer
//!     Debug.Print "Dec", "Oct", "Hex"
//!     Debug.Print "---", "---", "---"
//!     For i = 0 To maxValue
//!         Debug.Print i, Oct(i), Hex(i)
//!     Next i
//! End Sub
//! ```
//!
//! ## Advanced Usage
//!
//! ### Example 1: File Permission Manager
//! ```vb
//! ' Complete class for managing Unix file permissions
//! Class FilePermissionManager
//!     Private m_owner As Integer
//!     Private m_group As Integer
//!     Private m_other As Integer
//!     
//!     Public Property Let OwnerPermissions(value As Integer)
//!         If value >= 0 And value <= 7 Then
//!             m_owner = value
//!         Else
//!             Err.Raise 5, , "Owner permissions must be 0-7"
//!         End If
//!     End Property
//!     
//!     Public Property Let GroupPermissions(value As Integer)
//!         If value >= 0 And value <= 7 Then
//!             m_group = value
//!         Else
//!             Err.Raise 5, , "Group permissions must be 0-7"
//!         End If
//!     End Property
//!     
//!     Public Property Let OtherPermissions(value As Integer)
//!         If value >= 0 And value <= 7 Then
//!             m_other = value
//!         Else
//!             Err.Raise 5, , "Other permissions must be 0-7"
//!         End If
//!     End Property
//!     
//!     Public Function GetDecimalValue() As Integer
//!         GetDecimalValue = m_owner * 64 + m_group * 8 + m_other
//!     End Function
//!     
//!     Public Function GetOctalString() As String
//!         GetOctalString = Oct(GetDecimalValue())
//!     End Function
//!     
//!     Public Function GetFormattedOctal() As String
//!         Dim octStr As String
//!         octStr = Oct(GetDecimalValue())
//!         ' Pad to 3 digits
//!         GetFormattedOctal = String(3 - Len(octStr), "0") & octStr
//!     End Function
//!     
//!     Public Function GetSymbolicNotation() As String
//!         GetSymbolicNotation = ConvertToSymbolic(m_owner) & _
//!                              ConvertToSymbolic(m_group) & _
//!                              ConvertToSymbolic(m_other)
//!     End Function
//!     
//!     Private Function ConvertToSymbolic(perm As Integer) As String
//!         Dim result As String
//!         result = IIf(perm And 4, "r", "-")
//!         result = result & IIf(perm And 2, "w", "-")
//!         result = result & IIf(perm And 1, "x", "-")
//!         ConvertToSymbolic = result
//!     End Function
//!     
//!     Public Sub SetFromOctal(octalStr As String)
//!         Dim decValue As Long
//!         ' Remove any prefix
//!         If Left(octalStr, 2) = "&O" Then
//!             octalStr = Mid(octalStr, 3)
//!         End If
//!         
//!         ' Convert to decimal
//!         decValue = CLng("&O" & octalStr)
//!         
//!         ' Extract individual permissions
//!         m_other = decValue Mod 8
//!         decValue = decValue \ 8
//!         m_group = decValue Mod 8
//!         m_owner = decValue \ 8
//!     End Sub
//!     
//!     Public Function GenerateReport() As String
//!         Dim report As String
//!         report = "File Permissions:" & vbCrLf
//!         report = report & "  Decimal: " & GetDecimalValue() & vbCrLf
//!         report = report & "  Octal: " & GetFormattedOctal() & vbCrLf
//!         report = report & "  Symbolic: " & GetSymbolicNotation() & vbCrLf
//!         report = report & "  Owner: " & m_owner & " (" & ConvertToSymbolic(m_owner) & ")" & vbCrLf
//!         report = report & "  Group: " & m_group & " (" & ConvertToSymbolic(m_group) & ")" & vbCrLf
//!         report = report & "  Other: " & m_other & " (" & ConvertToSymbolic(m_other) & ")"
//!         GenerateReport = report
//!     End Function
//! End Class
//! ```
//!
//! ### Example 2: Number Base Converter Utility
//! ```vb
//! ' Utility module for converting between different number bases
//! Module NumberBaseConverter
//!     Public Function ConvertToBase(value As Long, fromBase As Integer, toBase As Integer) As String
//!         Dim decValue As Long
//!         
//!         ' First convert to decimal
//!         If fromBase = 10 Then
//!             decValue = value
//!         ElseIf fromBase = 8 Then
//!             decValue = CLng("&O" & CStr(value))
//!         ElseIf fromBase = 16 Then
//!             decValue = CLng("&H" & CStr(value))
//!         Else
//!             Err.Raise 5, , "Unsupported base: " & fromBase
//!         End If
//!         
//!         ' Then convert from decimal to target base
//!         Select Case toBase
//!             Case 2
//!                 ConvertToBase = ConvertToBinary(decValue)
//!             Case 8
//!                 ConvertToBase = Oct(decValue)
//!             Case 10
//!                 ConvertToBase = CStr(decValue)
//!             Case 16
//!                 ConvertToBase = Hex(decValue)
//!             Case Else
//!                 Err.Raise 5, , "Unsupported base: " & toBase
//!         End Select
//!     End Function
//!     
//!     Private Function ConvertToBinary(value As Long) As String
//!         Dim result As String
//!         Dim tempValue As Long
//!         
//!         If value = 0 Then
//!             ConvertToBinary = "0"
//!             Exit Function
//!         End If
//!         
//!         tempValue = value
//!         Do While tempValue > 0
//!             result = (tempValue Mod 2) & result
//!             tempValue = tempValue \ 2
//!         Loop
//!         ConvertToBinary = result
//!     End Function
//!     
//!     Public Function FormatWithPrefix(value As String, base As Integer) As String
//!         Select Case base
//!             Case 2
//!                 FormatWithPrefix = "0b" & value
//!             Case 8
//!                 FormatWithPrefix = "&O" & value
//!             Case 16
//!                 FormatWithPrefix = "&H" & value
//!             Case Else
//!                 FormatWithPrefix = value
//!         End Select
//!     End Function
//!     
//!     Public Sub DisplayConversionTable(startValue As Integer, endValue As Integer)
//!         Dim i As Integer
//!         Debug.Print "Dec", "Bin", "Oct", "Hex"
//!         Debug.Print String(40, "-")
//!         
//!         For i = startValue To endValue
//!             Debug.Print i, _
//!                        ConvertToBinary(i), _
//!                        Oct(i), _
//!                        Hex(i)
//!         Next i
//!     End Sub
//!     
//!     Public Function ValidateBaseString(value As String, base As Integer) As Boolean
//!         Dim i As Integer
//!         Dim ch As String
//!         Dim validChars As String
//!         
//!         Select Case base
//!             Case 2
//!                 validChars = "01"
//!             Case 8
//!                 validChars = "01234567"
//!             Case 16
//!                 validChars = "0123456789ABCDEFabcdef"
//!             Case Else
//!                 ValidateBaseString = False
//!                 Exit Function
//!         End Select
//!         
//!         For i = 1 To Len(value)
//!             ch = Mid(value, i, 1)
//!             If InStr(validChars, ch) = 0 Then
//!                 ValidateBaseString = False
//!                 Exit Function
//!             End If
//!         Next i
//!         
//!         ValidateBaseString = True
//!     End Function
//! End Module
//! ```
//!
//! ### Example 3: Bit Mask Analyzer
//! ```vb
//! ' Tool for analyzing and displaying bit masks in various formats
//! Class BitMaskAnalyzer
//!     Private m_value As Long
//!     
//!     Public Sub SetValue(value As Long)
//!         m_value = value
//!     End Sub
//!     
//!     Public Function GetValue() As Long
//!         GetValue = m_value
//!     End Function
//!     
//!     Public Function GetOctal() As String
//!         GetOctal = Oct(m_value)
//!     End Function
//!     
//!     Public Function GetHex() As String
//!         GetHex = Hex(m_value)
//!     End Function
//!     
//!     Public Function GetBinary() As String
//!         Dim result As String
//!         Dim tempValue As Long
//!         Dim bitCount As Integer
//!         
//!         tempValue = m_value
//!         For bitCount = 31 To 0 Step -1
//!             If tempValue And (2 ^ bitCount) Then
//!                 result = result & "1"
//!             Else
//!                 result = result & "0"
//!             End If
//!             If bitCount Mod 4 = 0 And bitCount > 0 Then
//!                 result = result & " "
//!             End If
//!         Next bitCount
//!         GetBinary = result
//!     End Function
//!     
//!     Public Function CountSetBits() As Integer
//!         Dim count As Integer
//!         Dim tempValue As Long
//!         
//!         count = 0
//!         tempValue = m_value
//!         Do While tempValue > 0
//!             If tempValue And 1 Then count = count + 1
//!             tempValue = tempValue \ 2
//!         Loop
//!         CountSetBits = count
//!     End Function
//!     
//!     Public Function IsBitSet(position As Integer) As Boolean
//!         If position >= 0 And position < 32 Then
//!             IsBitSet = (m_value And (2 ^ position)) <> 0
//!         End If
//!     End Function
//!     
//!     Public Sub SetBit(position As Integer)
//!         If position >= 0 And position < 32 Then
//!             m_value = m_value Or (2 ^ position)
//!         End If
//!     End Sub
//!     
//!     Public Sub ClearBit(position As Integer)
//!         If position >= 0 And position < 32 Then
//!             m_value = m_value And Not (2 ^ position)
//!         End If
//!     End Sub
//!     
//!     Public Sub ToggleBit(position As Integer)
//!         If position >= 0 And position < 32 Then
//!             m_value = m_value Xor (2 ^ position)
//!         End If
//!     End Sub
//!     
//!     Public Function GenerateReport() As String
//!         Dim report As String
//!         report = "Bit Mask Analysis:" & vbCrLf
//!         report = report & "  Decimal: " & m_value & vbCrLf
//!         report = report & "  Hexadecimal: " & GetHex() & vbCrLf
//!         report = report & "  Octal: " & GetOctal() & vbCrLf
//!         report = report & "  Binary: " & GetBinary() & vbCrLf
//!         report = report & "  Set Bits: " & CountSetBits()
//!         GenerateReport = report
//!     End Function
//! End Class
//! ```
//!
//! ### Example 4: Legacy System Interface
//! ```vb
//! ' Module for interfacing with legacy systems using octal notation
//! Module LegacySystemInterface
//!     ' Configuration record structure
//!     Private Type ConfigRecord
//!         DeviceID As Integer
//!         StatusFlags As Integer
//!         ControlWord As Integer
//!         ErrorMask As Integer
//!     End Type
//!     
//!     Public Function FormatDeviceConfig(config As ConfigRecord) As String
//!         Dim output As String
//!         output = "Device Configuration:" & vbCrLf
//!         output = output & "  Device ID: " & config.DeviceID & " (&O" & Oct(config.DeviceID) & ")" & vbCrLf
//!         output = output & "  Status Flags: &O" & FormatOctalPadded(config.StatusFlags, 6) & vbCrLf
//!         output = output & "  Control Word: &O" & FormatOctalPadded(config.ControlWord, 6) & vbCrLf
//!         output = output & "  Error Mask: &O" & FormatOctalPadded(config.ErrorMask, 6)
//!         FormatDeviceConfig = output
//!     End Function
//!     
//!     Private Function FormatOctalPadded(value As Long, width As Integer) As String
//!         Dim octStr As String
//!         octStr = Oct(value)
//!         If Len(octStr) < width Then
//!             FormatOctalPadded = String(width - Len(octStr), "0") & octStr
//!         Else
//!             FormatOctalPadded = octStr
//!         End If
//!     End Function
//!     
//!     Public Function ParseOctalConfig(configStr As String) As ConfigRecord
//!         Dim lines() As String
//!         Dim config As ConfigRecord
//!         Dim i As Integer
//!         Dim parts() As String
//!         
//!         lines = Split(configStr, vbCrLf)
//!         For i = LBound(lines) To UBound(lines)
//!             If InStr(lines(i), ":") > 0 Then
//!                 parts = Split(lines(i), ":")
//!                 If UBound(parts) >= 1 Then
//!                     If InStr(lines(i), "Device ID") > 0 Then
//!                         config.DeviceID = ExtractOctalValue(parts(1))
//!                     ElseIf InStr(lines(i), "Status Flags") > 0 Then
//!                         config.StatusFlags = ExtractOctalValue(parts(1))
//!                     ElseIf InStr(lines(i), "Control Word") > 0 Then
//!                         config.ControlWord = ExtractOctalValue(parts(1))
//!                     ElseIf InStr(lines(i), "Error Mask") > 0 Then
//!                         config.ErrorMask = ExtractOctalValue(parts(1))
//!                     End If
//!                 End If
//!             End If
//!         Next i
//!         
//!         ParseOctalConfig = config
//!     End Function
//!     
//!     Private Function ExtractOctalValue(valueStr As String) As Integer
//!         Dim cleanStr As String
//!         cleanStr = Trim(valueStr)
//!         
//!         ' Look for &O prefix
//!         If InStr(cleanStr, "&O") > 0 Then
//!             cleanStr = Mid(cleanStr, InStr(cleanStr, "&O") + 2)
//!             ' Remove any non-octal characters
//!             Dim i As Integer
//!             Dim octStr As String
//!             For i = 1 To Len(cleanStr)
//!                 If Mid(cleanStr, i, 1) >= "0" And Mid(cleanStr, i, 1) <= "7" Then
//!                     octStr = octStr & Mid(cleanStr, i, 1)
//!                 Else
//!                     Exit For
//!                 End If
//!             Next i
//!             If Len(octStr) > 0 Then
//!                 ExtractOctalValue = CInt("&O" & octStr)
//!             End If
//!         End If
//!     End Function
//!     
//!     Public Sub LogConfigChange(oldConfig As ConfigRecord, newConfig As ConfigRecord)
//!         Debug.Print "Configuration Change:"
//!         If oldConfig.DeviceID <> newConfig.DeviceID Then
//!             Debug.Print "  Device ID: " & Oct(oldConfig.DeviceID) & " -> " & Oct(newConfig.DeviceID)
//!         End If
//!         If oldConfig.StatusFlags <> newConfig.StatusFlags Then
//!             Debug.Print "  Status: " & Oct(oldConfig.StatusFlags) & " -> " & Oct(newConfig.StatusFlags)
//!         End If
//!         If oldConfig.ControlWord <> newConfig.ControlWord Then
//!             Debug.Print "  Control: " & Oct(oldConfig.ControlWord) & " -> " & Oct(newConfig.ControlWord)
//!         End If
//!         If oldConfig.ErrorMask <> newConfig.ErrorMask Then
//!             Debug.Print "  Errors: " & Oct(oldConfig.ErrorMask) & " -> " & Oct(newConfig.ErrorMask)
//!         End If
//!     End Sub
//! End Module
//! ```
//!
//! ## Error Handling
//!
//! The `Oct` function can raise errors in the following situations:
//!
//! - **Type Mismatch (Error 13)**: When the argument cannot be converted to a numeric value
//! - **Overflow (Error 6)**: When the number is too large for the data type
//! - **Invalid Procedure Call (Error 5)**: In rare cases with invalid arguments
//!
//! Always validate input before calling `Oct`, especially when working with user input:
//!
//! ```vb
//! On Error Resume Next
//! result = Oct(userInput)
//! If Err.Number <> 0 Then
//!     MsgBox "Invalid numeric value for octal conversion"
//!     Err.Clear
//! End If
//! On Error GoTo 0
//! ```
//!
//! ## Performance Considerations
//!
//! - The `Oct` function is very fast for simple conversions
//! - String concatenation with results can be slow in loops; use string builders or arrays
//! - For repeated conversions, cache results when possible
//! - Converting negative numbers involves two's complement calculation (slightly slower)
//!
//! ## Best Practices
//!
//! 1. **Use for Specific Purposes**: Octal is less common than hex; use it when specifically needed
//! 2. **Document Usage**: Always comment why octal notation is being used
//! 3. **Validate Input**: Check that input values are appropriate for octal conversion
//! 4. **Add Prefixes**: When displaying, consider adding "&O" prefix for clarity
//! 5. **Pad When Needed**: Use padding for fixed-width displays (file permissions, etc.)
//! 6. **Handle Negatives**: Be aware of two's complement representation for negative values
//! 7. **Combine with Hex**: Often useful to show both hex and octal for bit patterns
//! 8. **Test Edge Cases**: Verify behavior with 0, negative numbers, and maximum values
//! 9. **Consider Alternatives**: Hex is often more readable; use octal only when appropriate
//! 10. **Round Awareness**: Remember that non-integers are rounded before conversion
//!
//! ## Comparison with Related Functions
//!
//! | Function | Purpose | Returns | Use Case |
//! |----------|---------|---------|----------|
//! | **Oct** | Decimal to octal | String (base-8) | Unix permissions, legacy systems |
//! | **Hex** | Decimal to hexadecimal | String (base-16) | Memory addresses, colors, general bit patterns |
//! | **Str** | Number to string | String (base-10) | Standard numeric display |
//! | **`CStr`** | Convert to string | String (base-10) | Type conversion with formatting control |
//! | **Format** | Formatted string | String (custom format) | Complex number formatting with patterns |
//!
//! ## Platform and Version Notes
//!
//! - Available in all versions of VBA and VB6
//! - Behavior is consistent across Windows platforms
//! - The maximum length of the returned string depends on the data type of the input
//! - For Long values: up to 11 octal digits (for negative numbers)
//! - For Integer values: up to 6 octal digits (for negative numbers)
//!
//! ## Limitations
//!
//! - Only converts to octal (base-8); no support for arbitrary bases
//! - Cannot convert octal strings back to decimal (use `CLng("&O" & octStr)` instead)
//! - Negative numbers use two's complement, which may be confusing
//! - No built-in padding or formatting options
//! - Does not validate that the result is a "valid" octal in a specific context
//!
//! ## Related Functions
//!
//! - `Hex`: Convert number to hexadecimal string
//! - `Str`: Convert number to decimal string
//! - `CStr`: Convert value to string
//! - `Format`: Format number with custom patterns
//! - `CLng`: Convert string (including "&O" prefix) to Long
//! - `Val`: Parse numeric string to number

use crate::error::{VBError, VBResult};
use crate::value::VBVariant;

/// Implementation of the `Oct` function.
///
/// Converts a number to its octal (base-8) string representation. The result
/// contains only digits 0-7 and does not include any prefix ("&O" or "0o").
///
/// Unlike `Oct$`, this variant propagates `Null` — if the argument is `Null`,
/// it returns `Null` rather than raising an error.
///
/// VB6 behavior:
/// - Fractional values are rounded to the nearest integer before conversion
/// - Negative numbers use two's complement representation
/// - `Integer` values produce up to 6 octal digits
/// - `Long` values produce up to 11 octal digits
/// - Returns `Null` if `number` is `Null`
/// - Raises error 13 if `number` cannot be converted to a number
pub fn oct(number: &VBVariant) -> VBResult<VBVariant> {
    if number.is_null() {
        return Ok(VBVariant::Null);
    }

    let result = match number {
        VBVariant::Integer(v) => format!("{:o}", *v as u16),
        VBVariant::Long(v) => format!("{:o}", *v as u32),
        VBVariant::Byte(v) => format!("{:o}", *v as u32),
        _ => {
            let v = number.as_i64()?;
            if let Ok(long_val) = i32::try_from(v) {
                format!("{:o}", long_val as u32)
            } else {
                return Err(VBError::overflow());
            }
        }
    };

    Ok(VBVariant::from_string(&result))
}

#[cfg(test)]
mod tests {
    use super::oct;
    use crate::value::VBVariant;

    #[test]
    fn oct_zero() {
        let result = oct(&VBVariant::from_long(0)).unwrap();
        assert_eq!(result, VBVariant::from_string("0"));
    }

    #[test]
    fn oct_positive_long() {
        assert_eq!(
            oct(&VBVariant::from_long(8)).unwrap(),
            VBVariant::from_string("10")
        );
        assert_eq!(
            oct(&VBVariant::from_long(64)).unwrap(),
            VBVariant::from_string("100")
        );
        assert_eq!(
            oct(&VBVariant::from_long(511)).unwrap(),
            VBVariant::from_string("777")
        );
    }

    #[test]
    fn oct_positive_integer() {
        assert_eq!(
            oct(&VBVariant::from_integer(8)).unwrap(),
            VBVariant::from_string("10")
        );
        assert_eq!(
            oct(&VBVariant::from_integer(-1)).unwrap(),
            VBVariant::from_string("177777")
        );
    }

    #[test]
    fn oct_negative_long() {
        assert_eq!(
            oct(&VBVariant::from_long(-1)).unwrap(),
            VBVariant::from_string("37777777777")
        );
        assert_eq!(
            oct(&VBVariant::from_long(-256)).unwrap(),
            VBVariant::from_string("37777777400")
        );
    }

    #[test]
    fn oct_byte() {
        assert_eq!(
            oct(&VBVariant::from_byte(8)).unwrap(),
            VBVariant::from_string("10")
        );
        assert_eq!(
            oct(&VBVariant::from_byte(64)).unwrap(),
            VBVariant::from_string("100")
        );
    }

    #[test]
    fn oct_double_rounds() {
        // 8.5 rounds to 8 → "10"
        assert_eq!(
            oct(&VBVariant::Double(8.5)).unwrap(),
            VBVariant::from_string("10")
        );
        // 8.6 rounds to 9 → "11"
        assert_eq!(
            oct(&VBVariant::Double(8.6)).unwrap(),
            VBVariant::from_string("11")
        );
    }

    #[test]
    fn oct_from_string() {
        assert_eq!(
            oct(&VBVariant::from_string("255")).unwrap(),
            VBVariant::from_string("377")
        );
    }

    #[test]
    fn oct_null_returns_null() {
        let result = oct(&VBVariant::Null).unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn oct_max_long() {
        assert_eq!(
            oct(&VBVariant::from_long(i32::MAX)).unwrap(),
            VBVariant::from_string("17777777777")
        );
    }

    #[test]
    fn oct_min_long() {
        assert_eq!(
            oct(&VBVariant::from_long(i32::MIN)).unwrap(),
            VBVariant::from_string("20000000000")
        );
    }

    #[test]
    fn oct_empty_returns_zero() {
        let result = oct(&VBVariant::Empty).unwrap();
        assert_eq!(result, VBVariant::from_string("0"));
    }

    #[test]
    fn oct_file_permissions() {
        // 493 decimal = 755 octal (rwxr-xr-x)
        assert_eq!(
            oct(&VBVariant::from_long(493)).unwrap(),
            VBVariant::from_string("755")
        );
        // 420 decimal = 644 octal (rw-r--r--)
        assert_eq!(
            oct(&VBVariant::from_long(420)).unwrap(),
            VBVariant::from_string("644")
        );
    }
}
