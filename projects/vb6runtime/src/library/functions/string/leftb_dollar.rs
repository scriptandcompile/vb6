//! # `LeftB$` Function
//!
//! Returns a `String` containing a specified number of bytes from the left side of a string.
//!
//! ## Syntax
//!
//! ```vb6
//! LeftB$(string, length)
//! ```
//!
//! ## Parameters
//!
//! - `string`: Required. String expression from which the leftmost bytes are returned. If `string` contains `Null`, `Null` is returned.
//! - `length`: Required. Numeric expression indicating how many bytes to return. If 0, a zero-length string ("") is returned. If greater than or equal to the number of bytes in `string`, the entire string is returned.
//!
//! ## Return Value
//!
//! Returns a `String` containing the leftmost `length` bytes from `string`. If `length` is 0, returns an empty string. If `length` is greater than or equal to the byte length of `string`, returns the entire string.
//!
//! ## Remarks
//!
//! The `LeftB$` function is used with byte data contained in a string. Instead of specifying the number of characters to return, `length` specifies the number of bytes.
//!
//! This function is particularly useful when working with:
//! - Binary data stored in strings
//! - ANSI strings where you need byte-level control
//! - Double-byte character set (DBCS) strings
//! - Legacy file formats that use byte-oriented data
//! - Network protocols that specify byte lengths
//!
//! `LeftB$` is the byte-oriented version of `Left$`. While `Left$` counts characters, `LeftB$` counts bytes. In single-byte character sets (like standard ASCII), these are equivalent, but in DBCS systems (like Japanese, Chinese, or Korean Windows), characters may occupy multiple bytes.
//!
//! The `LeftB$` function always returns a `String`. The `LeftB` function returns a `Variant`.
//!
//! ## Typical Uses
//!
//! ### Example 1: Extracting Binary Header
//! ```vb6
//! Dim data As String
//! data = binaryData
//! header = LeftB$(data, 4)  ' Get first 4 bytes
//! ```
//!
//! ### Example 2: Reading Fixed-Byte Records
//! ```vb6
//! Dim record As String
//! record = GetRecord()
//! idBytes = LeftB$(record, 8)  ' First 8 bytes = ID
//! ```
//!
//! ### Example 3: Processing DBCS Strings
//! ```vb6
//! Dim jpText As String
//! jpText = "日本語"  ' Japanese text
//! bytes = LeftB$(jpText, 4)  ' Get first 4 bytes (may be 2 DBCS chars)
//! ```
//!
//! ### Example 4: Protocol Header Extraction
//! ```vb6
//! Dim packet As String
//! packet = ReceivePacket()
//! magic = LeftB$(packet, 2)  ' 2-byte magic number
//! ```
//!
//! ## Common Usage Patterns
//!
//! ### Extracting File Signature
//! ```vb6
//! Dim fileData As String
//! Open fileName For Binary As #1
//! fileData = Input$(LOF(1), #1)
//! Close #1
//! signature = LeftB$(fileData, 4)
//! If signature = "MZ" & Chr$(0) & Chr$(0) Then
//!     Debug.Print "Executable file"
//! End If
//! ```
//!
//! ### Reading Binary Structure
//! ```vb6
//! Dim buffer As String
//! buffer = GetBinaryData()
//! version = LeftB$(buffer, 2)  ' 2-byte version field
//! ```
//!
//! ### Processing Network Data
//! ```vb6
//! Dim netData As String
//! netData = Socket.Receive()
//! header = LeftB$(netData, 16)  ' 16-byte protocol header
//! ```
//!
//! ### Validating Byte Prefix
//! ```vb6
//! If LeftB$(data, 3) = Chr$(0xFF) & Chr$(0xFE) & Chr$(0xFD) Then
//!     Debug.Print "Valid magic bytes"
//! End If
//! ```
//!
//! ### Extracting BMP Header
//! ```vb6
//! Dim bmpData As String
//! Open "image.bmp" For Binary As #1
//! bmpData = Input$(54, #1)  ' BMP header is 54 bytes
//! Close #1
//! fileType = LeftB$(bmpData, 2)  ' "BM" for BMP files
//! ```
//!
//! ### Reading Length-Prefixed Data
//! ```vb6
//! Dim message As String
//! message = buffer
//! lenBytes = LeftB$(message, 4)
//! msgLen = CLng(AscB(MidB$(lenBytes, 1, 1))) + _
//!          CLng(AscB(MidB$(lenBytes, 2, 1))) * 256
//! ```
//!
//! ### Processing DBCS Carefully
//! ```vb6
//! Dim text As String
//! text = dbcsString
//! ' Be careful not to split DBCS characters
//! If LenB(text) > 10 Then
//!     truncated = LeftB$(text, 10)
//! End If
//! ```
//!
//! ### Comparing Byte Sequences
//! ```vb6
//! Dim data1 As String, data2 As String
//! If LeftB$(data1, 8) = LeftB$(data2, 8) Then
//!     Debug.Print "Headers match"
//! End If
//! ```
//!
//! ### Extracting GUID Bytes
//! ```vb6
//! Dim guidStr As String
//! guidStr = GetGUIDBytes()
//! data1 = LeftB$(guidStr, 4)   ' First DWORD
//! data2 = MidB$(guidStr, 5, 2) ' First WORD
//! ```
//!
//! ### Processing Binary Chunks
//! ```vb6
//! Dim chunk As String
//! Dim offset As Long
//! offset = 1
//! Do While offset <= LenB(binaryData)
//!     chunk = MidB$(binaryData, offset, 512)
//!     If LenB(chunk) = 0 Then Exit Do
//!     processChunk LeftB$(chunk, 512)
//!     offset = offset + 512
//! Loop
//! ```
//!
//! ## Related Functions
//!
//! - `LeftB`: Variant version that returns a `Variant`
//! - `Left$`: Character-based version that counts characters
//! - `RightB$`: Returns bytes from the right side of a string
//! - `MidB$`: Returns bytes from the middle of a string
//! - `LenB`: Returns the number of bytes in a string
//! - `AscB`: Returns the byte value of the first byte
//! - `ChrB$`: Returns a string containing a single byte
//!
//! ## Best Practices
//!
//! 1. Use `LeftB$` when working with binary data or byte-oriented protocols
//! 2. Be careful with DBCS strings - splitting may corrupt characters
//! 3. Always validate byte length before extraction to avoid errors
//! 4. Use `LenB` to get byte length, not `Len`
//! 5. Prefer `LeftB$` over `Left$` for binary file operations
//! 6. Remember that byte positions are 1-based, not 0-based
//! 7. Use with `InputB$` when reading binary files
//! 8. Test DBCS string operations on appropriate language systems
//! 9. Combine with `AscB` and `ChrB$` for byte-level manipulation
//! 10. Document whether your code expects SBCS or DBCS strings
//!
//! ## Performance Considerations
//!
//! - `LeftB$` is slightly faster than `Left$` for binary data
//! - No performance penalty for requesting more bytes than available
//! - More efficient than character-by-character byte extraction
//! - Direct byte access is faster than converting to byte arrays
//! - Minimal overhead compared to `Left$` in SBCS environments
//!
//! ## Character Set Behavior
//!
//! | Environment | Byte per Char | Notes |
//! |-------------|---------------|-------|
//! | English Windows | 1 byte | `LeftB$` and `Left$` behave identically |
//! | DBCS Windows | 1-2 bytes | `LeftB$` may split multi-byte characters |
//! | Unicode VB6 | 2 bytes | Internal strings are Unicode but converted |
//! | Binary Data | N/A | `LeftB$` treats data as raw bytes |
//!
//! ## Common Pitfalls
//!
//! - Using `LeftB$` on DBCS strings without checking character boundaries
//! - Confusing byte length (`LenB`) with character length (`Len`)
//! - Assuming one byte equals one character in all locales
//! - Not handling `Null` string values (causes runtime error)
//! - Passing negative length values (causes runtime error)
//! - Using `LeftB$` for text processing (use `Left$` instead)
//! - Forgetting that VB6 strings are internally Unicode
//! - Splitting surrogate pairs in Unicode environments
//! - Using with text functions instead of byte functions
//!
//! ## Limitations
//!
//! - Cannot specify starting byte position (use `MidB$` instead)
//! - May corrupt DBCS characters if not used carefully
//! - Returns `Null` if the string argument is `Null`
//! - Length parameter cannot be `Null`
//! - Not suitable for modern Unicode string processing
//! - Limited to VB6's internal string representation
//! - May produce unexpected results with emoji or complex Unicode
