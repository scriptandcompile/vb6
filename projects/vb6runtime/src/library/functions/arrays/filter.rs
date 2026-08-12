//! ## `Filter` Function
//!
//! Returns a zero-based array containing a subset of a string array based on specified filter criteria.
//!
//! ## Syntax
//!
//! ```text
//! Filter(sourcearray, match[, include[, compare]])
//! ```
//!
//! ## Parameters
//!
//! - **sourcearray**: Required. One-dimensional array of strings to be searched.
//! - **match**: Required. String to search for.
//! - **include**: Optional. Boolean value indicating whether to return substrings that include
//!   or exclude match. If True (default), Filter returns subset including match. If False,
//!   Filter returns subset excluding match.
//! - **compare**: Optional. Numeric value indicating the kind of string comparison to use.
//!   * 0 = vbBinaryCompare (case-sensitive, default)
//!   * 1 = vbTextCompare (case-insensitive)
//!   * 2 = vbDatabaseCompare (Microsoft Access only)
//!
//! ## Return Value
//!
//! Returns a `Variant` containing a zero-based array of strings. If no matches are found,
//! Filter returns an empty array. If sourcearray is Null or not a one-dimensional array,
//! an error occurs.
//!
//! ## Remarks
//!
//! The `Filter` function searches a string array for elements containing a specified substring
//! and returns a new array with matching (or non-matching) elements. This is useful for
//! filtering lists, implementing search functionality, and processing string collections.
//!
//! ## Important Characteristics
//!
//! - Returns zero-based array regardless of input array bounds
//! - Match is substring search (not whole string match)
//! - Empty string match returns all elements (when include=True)
//! - Returns empty array if no matches found
//! - Case sensitivity controlled by compare parameter
//! - Original array is not modified
//! - Works only with one-dimensional string arrays
//! - Error 13 (Type Mismatch) if sourcearray is not an array
//! - Error 5 (Invalid procedure call) if sourcearray is multi-dimensional
//! - Error 94 (Invalid use of Null) if sourcearray is Null
//! - Returned array starts at index 0
//! - Can be used to implement NOT logic (include=False)
//!
//! ### Common Errors
//!
//! - **Error 13** (Type Mismatch): sourcearray is not an array or not a string array
//! - **Error 5** (Invalid procedure call): sourcearray is multi-dimensional
//! - **Error 94** (Invalid use of Null): sourcearray is Null
//!
//! ## Performance Considerations
//!
//! - Filter is efficient for small to medium arrays (< 10,000 elements)
//! - For very large arrays, consider Dictionary-based approaches
//! - Case-insensitive search is slightly slower than case-sensitive
//! - Filtering already-filtered results is faster than re-filtering original array
//! - Consider caching results for repeated searches
//! - Empty string match returns entire array
//!
//! ## Typical Uses
//!
//! - Filter lists based on user input
//! - Implement search functionality
//! - Remove unwanted items from arrays
//! - Find items matching a pattern
//! - Create subsets of data
//! - Filter file lists
//! - Process search results
//! - Implement autocomplete features
//!
//! ## Limitations
//!
//! - Works only with one-dimensional arrays
//! - Only supports string arrays
//! - Returns zero-based array (even if source is 1-based)
//! - Substring match only (no regex or wildcards)
//! - Cannot filter on multiple criteria in single call
//! - No built-in support for custom comparison functions
//! - Case-insensitive limited to vbTextCompare behavior
//!
//! ## Related Functions
//!
//! - `Array`: Creates a `Variant` array
//! - `Split`: Splits a string into an array
//! - `Join`: Joins array elements into a string
//! - `InStr`: Finds substring position
//! - `LBound`/`UBound`: Gets array bounds
//! - `IsArray`: Checks if variable is an array
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb6
//! Dim fruits() As String
//! Dim filtered() As String
//!
//! fruits = Array("Apple", "Banana", "Cherry", "Date", "Elderberry")
//!
//! ' Find fruits containing "e" (case-sensitive)
//! filtered = Filter(fruits, "e")
//! ' Returns: "Apple", "Cherry", "Date", "Elderberry"
//!
//! ' Find fruits NOT containing "e"
//! filtered = Filter(fruits, "e", False)
//! ' Returns: "Banana"
//!
//! ' Find fruits containing "a" (case-insensitive)
//! filtered = Filter(fruits, "a", True, vbTextCompare)
//! ' Returns: "Apple", "Banana", "Date"
//! ```
//!
//! ### Case-Sensitive vs Case-Insensitive
//!
//! ```vb6
//! Dim names() As String
//! names = Array("John", "jane", "JAMES", "Julia", "jack")
//!
//! ' Case-sensitive search (default)
//! Dim result1() As String
//! result1 = Filter(names, "J")
//! ' Returns: "John", "JAMES", "Julia"
//!
//! ' Case-insensitive search
//! Dim result2() As String
//! result2 = Filter(names, "J", True, vbTextCompare)
//! ' Returns: "John", "jane", "JAMES", "Julia", "jack"
//! ```
//!
//! ### Exclude Matches
//!
//! ```vb6
//! Dim files() As String
//! files = Array("data.txt", "backup.bak", "report.txt", "temp.bak", "notes.txt")
//!
//! ' Get only non-backup files (exclude .bak)
//! Dim textFiles() As String
//! textFiles = Filter(files, ".bak", False)
//! ' Returns: "data.txt", "report.txt", "notes.txt"
//! ```
//!
//! ## Common Patterns
//!
//! ### Filter List Based on User Input
//!
//! ```vb6
//! Function SearchList(items() As String, searchTerm As String) As String()
//!     On Error GoTo ErrorHandler
//!     
//!     If Trim(searchTerm) = "" Then
//!         ' Return all items if search is empty
//!         SearchList = items
//!     Else
//!         ' Return filtered items (case-insensitive)
//!         SearchList = Filter(items, searchTerm, True, vbTextCompare)
//!     End If
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     ' Return empty array on error
//!     Dim emptyArray() As String
//!     ReDim emptyArray(0 To -1)
//!     SearchList = emptyArray
//! End Function
//! ```
//!
//! ### Count Matching Items
//!
//! ```vb6
//! Function CountMatches(items() As String, searchTerm As String) As Long
//!     On Error GoTo ErrorHandler
//!     
//!     Dim matches() As String
//!     matches = Filter(items, searchTerm, True, vbTextCompare)
//!     
//!     ' Check if array is empty
//!     If UBound(matches) >= 0 Then
//!         CountMatches = UBound(matches) + 1
//!     Else
//!         CountMatches = 0
//!     End If
//!     
//!     Exit Function
//!     
//! ErrorHandler:
//!     CountMatches = 0
//! End Function
//! ```
//!
//! ### Filter File List by Extension
//!
//! ```vb6
//! Function GetFilesByExtension(files() As String, extension As String) As String()
//!     ' Ensure extension starts with dot
//!     If Left(extension, 1) <> "." Then
//!         extension = "." & extension
//!     End If
//!     
//!     ' Filter for files with this extension
//!     GetFilesByExtension = Filter(files, extension, True, vbTextCompare)
//! End Function
//!
//! ' Usage
//! Dim allFiles() As String
//! Dim txtFiles() As String
//! allFiles = Array("doc1.txt", "image.jpg", "data.txt", "photo.png")
//! txtFiles = GetFilesByExtension(allFiles, ".txt")
//! ```
//!
//! ### Multiple Filter Criteria
//!
//! ```vb6
//! Function FilterMultiple(items() As String, filters() As String) As String()
//!     Dim result() As String
//!     Dim temp() As String
//!     Dim i As Long
//!     
//!     result = items
//!     
//!     ' Apply each filter sequentially
//!     For i = LBound(filters) To UBound(filters)
//!         temp = Filter(result, filters(i), True, vbTextCompare)
//!         result = temp
//!         
//!         ' Exit early if no matches
//!         If UBound(result) < 0 Then Exit For
//!     Next i
//!     
//!     FilterMultiple = result
//! End Function
//!
//! ' Usage: Find items containing both "test" and "data"
//! Dim criteria() As String
//! criteria = Array("test", "data")
//! filtered = FilterMultiple(sourceArray, criteria)
//! ```
//!
//! ### Populate `ListBox` with Filtered Results
//!
//! ```vb6
//! Sub UpdateFilteredList(lstBox As ListBox, items() As String, searchText As String)
//!     Dim filtered() As String
//!     Dim i As Long
//!     
//!     lstBox.Clear
//!     
//!     On Error GoTo ErrorHandler
//!     
//!     If Trim(searchText) = "" Then
//!         ' Show all items
//!         For i = LBound(items) To UBound(items)
//!             lstBox.AddItem items(i)
//!         Next i
//!     Else
//!         ' Show filtered items
//!         filtered = Filter(items, searchText, True, vbTextCompare)
//!         
//!         If UBound(filtered) >= 0 Then
//!             For i = 0 To UBound(filtered)
//!                 lstBox.AddItem filtered(i)
//!             Next i
//!         End If
//!     End If
//!     
//!     Exit Sub
//!     
//! ErrorHandler:
//!     ' Handle errors silently or show message
//! End Sub
//! ```
//!
//! ### Remove Duplicates with Filter
//!
//! ```vb6
//! Function RemoveDuplicates(items() As String) As String()
//!     Dim result() As String
//!     Dim dict As Object
//!     Dim i As Long
//!     Dim count As Long
//!     
//!     Set dict = CreateObject("Scripting.Dictionary")
//!     dict.CompareMode = vbTextCompare
//!     
//!     ' Add unique items to dictionary
//!     For i = LBound(items) To UBound(items)
//!         If Not dict.Exists(items(i)) Then
//!             dict.Add items(i), Nothing
//!         End If
//!     Next i
//!     
//!     ' Convert to array
//!     ReDim result(0 To dict.Count - 1)
//!     count = 0
//!     For i = 0 To dict.Count - 1
//!         result(count) = dict.Keys()(i)
//!         count = count + 1
//!     Next i
//!     
//!     RemoveDuplicates = result
//! End Function
//! ```
//!
//! ### Filter with Wildcard Simulation
//!
//! ```vb6
//! Function FilterWildcard(items() As String, pattern As String) As Collection
//!     ' Simple wildcard: * at start, end, or both
//!     Dim results As New Collection
//!     Dim filtered() As String
//!     Dim searchTerm As String
//!     Dim i As Long
//!     Dim item As String
//!     
//!     If Left(pattern, 1) = "*" And Right(pattern, 1) = "*" Then
//!         ' Contains search
//!         searchTerm = Mid(pattern, 2, Len(pattern) - 2)
//!         filtered = Filter(items, searchTerm, True, vbTextCompare)
//!         
//!         For i = 0 To UBound(filtered)
//!             results.Add filtered(i)
//!         Next i
//!         
//!     ElseIf Left(pattern, 1) = "*" Then
//!         ' Ends with search
//!         searchTerm = Mid(pattern, 2)
//!         For i = LBound(items) To UBound(items)
//!             If Right(LCase(items(i)), Len(searchTerm)) = LCase(searchTerm) Then
//!                 results.Add items(i)
//!             End If
//!         Next i
//!         
//!     ElseIf Right(pattern, 1) = "*" Then
//!         ' Starts with search
//!         searchTerm = Left(pattern, Len(pattern) - 1)
//!         For i = LBound(items) To UBound(items)
//!             If Left(LCase(items(i)), Len(searchTerm)) = LCase(searchTerm) Then
//!                 results.Add items(i)
//!             End If
//!         Next i
//!         
//!     Else
//!         ' Exact match
//!         For i = LBound(items) To UBound(items)
//!             If LCase(items(i)) = LCase(pattern) Then
//!                 results.Add items(i)
//!             End If
//!         Next i
//!     End If
//!     
//!     Set FilterWildcard = results
//! End Function
//! ```
//!
//! ### Autocomplete Implementation
//!
//! ```vb6
//! Sub TextBox_Change()
//!     Dim allItems() As String
//!     Dim matches() As String
//!     Dim i As Long
//!     
//!     ' Get all possible values (from database, array, etc.)
//!     allItems = GetAllItemNames()
//!     
//!     If Len(Me.txtSearch.Text) > 0 Then
//!         ' Filter items that start with typed text
//!         matches = Filter(allItems, Me.txtSearch.Text, True, vbTextCompare)
//!         
//!         ' Display suggestions
//!         Me.lstSuggestions.Clear
//!         
//!         If UBound(matches) >= 0 Then
//!             For i = 0 To UBound(matches)
//!                 Me.lstSuggestions.AddItem matches(i)
//!             Next i
//!             Me.lstSuggestions.Visible = True
//!         Else
//!             Me.lstSuggestions.Visible = False
//!         End If
//!     Else
//!         Me.lstSuggestions.Visible = False
//!     End If
//! End Sub
//! ```
//!
//! ### Filter Log Entries
//!
//! ```vb6
//! Function FilterLogsByLevel(logEntries() As String, level As String) As String()
//!     ' Assume log format: "[LEVEL] Message"
//!     Dim levelTag As String
//!     levelTag = "[" & UCase(level) & "]"
//!     
//!     FilterLogsByLevel = Filter(logEntries, levelTag, True, vbTextCompare)
//! End Function
//!
//! ' Usage
//! Dim logs() As String
//! Dim errors() As String
//! logs = Array("[INFO] Started", "[ERROR] Failed", "[INFO] Complete", "[ERROR] Timeout")
//! errors = FilterLogsByLevel(logs, "ERROR")
//! ' Returns: "[ERROR] Failed", "[ERROR] Timeout"
//! ```
//!
//! ### Check If Array Contains Value
//!
//! ```vb6
//! Function ArrayContains(items() As String, value As String, _
//!                        Optional caseSensitive As Boolean = False) As Boolean
//!     On Error GoTo ErrorHandler
//!     
//!     Dim matches() As String
//!     Dim compareMode As VbCompareMethod
//!     
//!     If caseSensitive Then
//!         compareMode = vbBinaryCompare
//!     Else
//!         compareMode = vbTextCompare
//!     End If
//!     
//!     matches = Filter(items, value, True, compareMode)
//!     
//!     ' Check if any exact matches
//!     Dim i As Long
//!     For i = 0 To UBound(matches)
//!         If StrComp(matches(i), value, compareMode) = 0 Then
//!             ArrayContains = True
//!             Exit Function
//!         End If
//!     Next i
//!     
//!     ArrayContains = False
//!     Exit Function
//!     
//! ErrorHandler:
//!     ArrayContains = False
//! End Function
//! ```
//!
//! ### Combine Include and Exclude Filters
//!
//! ```vb6
//! Function FilterIncludeExclude(items() As String, includeText As String, _
//!                               excludeText As String) As String()
//!     Dim temp() As String
//!     
//!     ' First include items containing includeText
//!     If includeText <> "" Then
//!         temp = Filter(items, includeText, True, vbTextCompare)
//!     Else
//!         temp = items
//!     End If
//!     
//!     ' Then exclude items containing excludeText
//!     If excludeText <> "" And UBound(temp) >= 0 Then
//!         temp = Filter(temp, excludeText, False, vbTextCompare)
//!     End If
//!     
//!     FilterIncludeExclude = temp
//! End Function
//!
//! ' Usage: Get .txt files but not backup files
//! filtered = FilterIncludeExclude(files, ".txt", "backup")
//! ```
//!
//! ## Advanced Usage
//!
//! ### Dynamic Search with Multiple Columns
//!
//! ```vb6
//! Type RecordData
//!     ID As String
//!     Name As String
//!     Email As String
//!     Department As String
//! End Type
//!
//! Function SearchRecords(records() As RecordData, searchTerm As String) As Long()
//!     ' Search across multiple fields and return matching indices
//!     Dim names() As String
//!     Dim emails() As String
//!     Dim departments() As String
//!     Dim matchedNames() As String
//!     Dim matchedEmails() As String
//!     Dim matchedDepts() As String
//!     Dim results() As Long
//!     Dim i As Long
//!     Dim count As Long
//!     Dim dict As Object
//!     
//!     Set dict = CreateObject("Scripting.Dictionary")
//!     
//!     ' Build arrays for each searchable field
//!     ReDim names(LBound(records) To UBound(records))
//!     ReDim emails(LBound(records) To UBound(records))
//!     ReDim departments(LBound(records) To UBound(records))
//!     
//!     For i = LBound(records) To UBound(records)
//!         names(i) = records(i).Name
//!         emails(i) = records(i).Email
//!         departments(i) = records(i).Department
//!     Next i
//!     
//!     ' Filter each field
//!     On Error Resume Next
//!     matchedNames = Filter(names, searchTerm, True, vbTextCompare)
//!     matchedEmails = Filter(emails, searchTerm, True, vbTextCompare)
//!     matchedDepts = Filter(departments, searchTerm, True, vbTextCompare)
//!     On Error GoTo 0
//!     
//!     ' Collect unique matching indices
//!     For i = LBound(records) To UBound(records)
//!         If InStr(1, records(i).Name, searchTerm, vbTextCompare) > 0 Or _
//!            InStr(1, records(i).Email, searchTerm, vbTextCompare) > 0 Or _
//!            InStr(1, records(i).Department, searchTerm, vbTextCompare) > 0 Then
//!             
//!             If Not dict.Exists(i) Then
//!                 dict.Add i, Nothing
//!             End If
//!         End If
//!     Next i
//!     
//!     ' Convert to array
//!     If dict.Count > 0 Then
//!         ReDim results(0 To dict.Count - 1)
//!         For i = 0 To dict.Count - 1
//!             results(i) = dict.Keys()(i)
//!         Next i
//!     Else
//!         ReDim results(0 To -1)
//!     End If
//!     
//!     SearchRecords = results
//! End Function
//! ```
//!
//! ### Incremental Filter (Type-Ahead)
//!
//! ```vb6
//! Private lastSearch As String
//! Private cachedResults() As String
//!
//! Sub IncrementalSearch(items() As String, currentSearch As String)
//!     Dim filtered() As String
//!     
//!     ' If new search starts with last search, filter cached results
//!     If Len(currentSearch) > Len(lastSearch) And _
//!        Left(currentSearch, Len(lastSearch)) = lastSearch And _
//!        UBound(cachedResults) >= 0 Then
//!         
//!         ' Filter from cached results (faster)
//!         filtered = Filter(cachedResults, currentSearch, True, vbTextCompare)
//!     Else
//!         ' Filter from full list
//!         filtered = Filter(items, currentSearch, True, vbTextCompare)
//!     End If
//!     
//!     ' Update cache
//!     cachedResults = filtered
//!     lastSearch = currentSearch
//!     
//!     ' Display results
//!     DisplayResults filtered
//! End Sub
//! ```
//!
//! ### Category-Based Filtering
//!
//! ```vb6
//! Type Product
//!     Name As String
//!     Category As String
//!     Price As Double
//!     Description As String
//! End Type
//!
//! Function FilterProductsByCategory(products() As Product, _
//!                                   category As String) As Product()
//!     Dim categories() As String
//!     Dim filtered() As String
//!     Dim results() As Product
//!     Dim i As Long
//!     Dim count As Long
//!     
//!     ' Build category array
//!     ReDim categories(LBound(products) To UBound(products))
//!     For i = LBound(products) To UBound(products)
//!         categories(i) = products(i).Category
//!     Next i
//!     
//!     ' Get matching categories
//!     filtered = Filter(categories, category, True, vbTextCompare)
//!     
//!     ' Build result array
//!     ReDim results(0 To UBound(filtered))
//!     count = 0
//!     
//!     For i = LBound(products) To UBound(products)
//!         If InStr(1, products(i).Category, category, vbTextCompare) > 0 Then
//!             results(count) = products(i)
//!             count = count + 1
//!         End If
//!     Next i
//!     
//!     If count > 0 Then
//!         ReDim Preserve results(0 To count - 1)
//!     Else
//!         ReDim results(0 To -1)
//!     End If
//!     
//!     FilterProductsByCategory = results
//! End Function
//! ```
//!
//! ### Filter with Performance Tracking
//!
//! ```vb6
//! Function FilterWithStats(items() As String, searchTerm As String, _
//!                          ByRef matchCount As Long, _
//!                          ByRef elapsedMs As Double) As String()
//!     Dim startTime As Double
//!     Dim results() As String
//!     
//!     startTime = Timer
//!     
//!     On Error GoTo ErrorHandler
//!     results = Filter(items, searchTerm, True, vbTextCompare)
//!     
//!     If UBound(results) >= 0 Then
//!         matchCount = UBound(results) + 1
//!     Else
//!         matchCount = 0
//!     End If
//!     
//!     elapsedMs = (Timer - startTime) * 1000
//!     
//!     FilterWithStats = results
//!     Exit Function
//!     
//! ErrorHandler:
//!     matchCount = 0
//!     elapsedMs = 0
//!     ReDim results(0 To -1)
//!     FilterWithStats = results
//! End Function
//! ```
//!
//! ### Smart Case-Sensitive Filter
//!
//! ```vb6
//! Function SmartFilter(items() As String, searchTerm As String) As String()
//!     Dim compareMode As VbCompareMethod
//!     
//!     ' If search term has uppercase letters, use case-sensitive
//!     ' Otherwise use case-insensitive
//!     If searchTerm <> LCase(searchTerm) Then
//!         compareMode = vbBinaryCompare
//!     Else
//!         compareMode = vbTextCompare
//!     End If
//!     
//!     SmartFilter = Filter(items, searchTerm, True, compareMode)
//! End Function
//! ```
//!
//! ## Error Handling
//!
//! ```vb6
//! Function SafeFilter(items As Variant, searchTerm As String) As Variant
//!     On Error GoTo ErrorHandler
//!     
//!     Dim emptyArray() As String
//!     
//!     ' Check if items is an array
//!     If Not IsArray(items) Then
//!         ReDim emptyArray(0 To -1)
//!         SafeFilter = emptyArray
//!         Exit Function
//!     End If
//!     
//!     ' Check if items is Null
//!     If IsNull(items) Then
//!         ReDim emptyArray(0 To -1)
//!         SafeFilter = emptyArray
//!         Exit Function
//!     End If
//!     
//!     ' Perform filter
//!     SafeFilter = Filter(items, searchTerm, True, vbTextCompare)
//!     Exit Function
//!     
//! ErrorHandler:
//!     Select Case Err.Number
//!         Case 13  ' Type mismatch
//!             Debug.Print "Filter error: sourcearray is not a string array"
//!         Case 5   ' Invalid procedure call
//!             Debug.Print "Filter error: sourcearray is multi-dimensional"
//!         Case 94  ' Invalid use of Null
//!             Debug.Print "Filter error: sourcearray is Null"
//!         Case Else
//!             Debug.Print "Filter error " & Err.Number & ": " & Err.Description
//!     End Select
//!     
//!     ReDim emptyArray(0 To -1)
//!     SafeFilter = emptyArray
//! End Function
//! ```
//!
//! ## Best Practices
//!
//! ### Always Check Result Array
//!
//! ```vb6
//! Dim results() As String
//! results = Filter(items, searchTerm)
//!
//! If UBound(results) >= 0 Then
//!     ' Process results
//!     For i = 0 To UBound(results)
//!         Debug.Print results(i)
//!     Next i
//! Else
//!     Debug.Print "No matches found"
//! End If
//! ```
//!
//! ### Use Error Handling
//!
//! ```vb6
//! On Error Resume Next
//! filtered = Filter(sourceArray, searchText, True, vbTextCompare)
//! If Err.Number <> 0 Then
//!     ' Handle error
//!     ReDim filtered(0 To -1)
//! End If
//! On Error GoTo 0
//! ```
//!
//! ### Default to Case-Insensitive for User Input
//!
//! ```vb6
//! ' Good - User-friendly search
//! results = Filter(items, userInput, True, vbTextCompare)
//!
//! ' Less friendly - Exact case required
//! results = Filter(items, userInput)
//! ```
//!
//! ## Comparison with Other Approaches
//!
//! ### Filter vs Manual Loop
//!
//! ```vb6
//! ' Using Filter (concise)
//! matches = Filter(items, searchTerm, True, vbTextCompare)
//!
//! ' Manual loop (more control)
//! ReDim matches(0 To UBound(items))
//! count = 0
//! For i = LBound(items) To UBound(items)
//!     If InStr(1, items(i), searchTerm, vbTextCompare) > 0 Then
//!         matches(count) = items(i)
//!         count = count + 1
//!     End If
//! Next i
//! If count > 0 Then
//!     ReDim Preserve matches(0 To count - 1)
//! End If
//! ```
//!
//! ### Filter vs Collection/Dictionary
//!
//! ```vb6
//! ' Filter - Returns array
//! Dim arr() As String
//! arr = Filter(items, searchTerm)
//!
//! ' Collection - More flexible but slower
//! Dim coll As New Collection
//! For i = LBound(items) To UBound(items)
//!     If InStr(1, items(i), searchTerm, vbTextCompare) > 0 Then
//!         coll.Add items(i)
//!     End If
//! Next i
//! ```

use crate::array::ArrayValue;
use crate::error::{err_number, VBError, VBResult};
use crate::types::VBType;
use crate::value::VBVariant;

/// Implementation of the `Filter` function.
///
/// Searches a one-dimensional string array for elements containing (or not
/// containing) a specified substring and returns a zero-based array of the
/// matching elements.
///
/// VB6 behavior:
/// - Returns a zero-based array regardless of the input array's bounds
/// - Performs substring matching (not whole-string match)
/// - `include` defaults to `True` (return matches); `False` excludes matches
/// - `compare` accepts `vbUseCompareOption` (-1), `vbBinaryCompare` (0),
///   `vbTextCompare` (1), and `vbDatabaseCompare` (2); without a module-level
///   `Option Compare` or database setting, -1 and 2 behave as binary compare,
///   and any other value raises error 5 (invalid procedure call)
/// - Empty `match` string with `include=True` returns all elements
/// - Returns an empty array when no matches are found, or when `sourcearray`
///   is a zero-length or still undimensioned array
/// - Matching is case-sensitive unless `vbTextCompare` (1) is given, and the
///   original casing of matched elements is preserved
/// - Raises error 94 if `sourcearray` is Null, or if it contains a `Null`
///   element
/// - Raises error 13 if `sourcearray` is not an array, or if it contains a
///   non-string element
/// - Raises error 5 if `sourcearray` is multi-dimensional
pub fn filter(
    sourcearray: &VBVariant,
    match_string: &str,
    include: Option<bool>,
    compare: Option<i32>,
) -> VBResult<VBVariant> {
    // Validate sourcearray is not Null
    if sourcearray.is_null() {
        return Err(VBError::with_description(
            err_number::INVALID_USE_OF_NULL,
            "Invalid use of Null",
        ));
    }

    // Validate sourcearray is an array
    let VBVariant::Array(arr) = sourcearray else {
        return Err(VBError::type_mismatch());
    };

    // A zero-length (or still undimensioned) array returns an empty array
    if !arr.is_initialized() || arr.is_empty() {
        return Ok(VBVariant::Array(ArrayValue::from_vec_with_bounds(
            VBType::String,
            Vec::new(),
            0,
        )));
    }

    // Check for multi-dimensional arrays
    if arr.rank() != 1 {
        return Err(VBError::with_description(
            err_number::INVALID_PROCEDURE_CALL,
            "Multi-dimensional array",
        ));
    }

    let include = include.unwrap_or(true);
    let text_compare = match compare {
        None => false,
        Some(mode) if (-1..=2).contains(&mode) => mode == 1,
        Some(_) => return Err(VBError::invalid_procedure_call()),
    };

    // Collect matching elements in source order
    let lower = arr.lower_bound(0).unwrap();
    let upper = arr.upper_bound(0).unwrap();
    let mut matches: Vec<VBVariant> = Vec::new();

    for i in lower..=upper {
        let element = arr.get(&[i]).unwrap();
        let element_str = match element {
            VBVariant::Null => return Err(VBError::invalid_use_of_null()),
            VBVariant::String(s) => s.as_str(),
            _ => return Err(VBError::type_mismatch()),
        };

        let contains = if text_compare {
            element_str
                .to_lowercase()
                .contains(&match_string.to_lowercase())
        } else {
            element_str.contains(match_string)
        };

        if (include && contains) || (!include && !contains) {
            matches.push(VBVariant::from_string(element_str));
        }
    }

    // Return zero-based array
    Ok(VBVariant::Array(ArrayValue::from_vec_with_bounds(
        VBType::String,
        matches,
        0,
    )))
}

#[cfg(test)]
mod tests {
    use super::filter;
    use crate::array::{ArrayDimension, ArrayValue};
    use crate::error::err_number;
    use crate::types::VBType;
    use crate::value::VBVariant;

    fn make_string_array(strings: &[&str]) -> VBVariant {
        let data: Vec<VBVariant> = strings.iter().map(|s| VBVariant::from_string(*s)).collect();
        VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::String, data, 0))
    }

    fn extract_strings(variant: &VBVariant) -> Vec<String> {
        let arr = variant.as_array().unwrap();
        (0..arr.len())
            .map(|i| {
                arr.get(&[i as i32])
                    .unwrap()
                    .as_str()
                    .unwrap_or("")
                    .to_string()
            })
            .collect()
    }

    #[test]
    fn basic_filter_includes_matches() {
        let source = make_string_array(&["Apple", "Banana", "Cherry", "Date", "Elderberry"]);
        let result = filter(&source, "e", None, None).unwrap();
        let matches = extract_strings(&result);

        assert_eq!(matches, vec!["Apple", "Cherry", "Date", "Elderberry"]);
    }

    #[test]
    fn filter_excludes_matches() {
        let source = make_string_array(&["Apple", "Banana", "Cherry", "Date", "Elderberry"]);
        let result = filter(&source, "e", Some(false), None).unwrap();
        let matches = extract_strings(&result);

        assert_eq!(matches, vec!["Banana"]);
    }

    #[test]
    fn text_compare_is_case_insensitive() {
        let source = make_string_array(&["Apple", "Banana", "Cherry"]);
        let result = filter(&source, "a", None, Some(1)).unwrap();
        let matches = extract_strings(&result);

        assert_eq!(matches, vec!["Apple", "Banana"]);
    }

    #[test]
    fn binary_compare_is_case_sensitive() {
        let source = make_string_array(&["Apple", "apple", "BANANA"]);
        let result = filter(&source, "a", None, Some(0)).unwrap();
        let matches = extract_strings(&result);

        assert_eq!(matches, vec!["apple"]);
    }

    #[test]
    fn empty_match_includes_all() {
        let source = make_string_array(&["a", "b", "c"]);
        let result = filter(&source, "", None, None).unwrap();
        let matches = extract_strings(&result);

        assert_eq!(matches, vec!["a", "b", "c"]);
    }

    #[test]
    fn empty_match_excludes_none() {
        let source = make_string_array(&["a", "b", "c"]);
        let result = filter(&source, "", Some(false), None).unwrap();
        let matches = extract_strings(&result);

        // Empty string is contained in every string, so excluding gives empty result
        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn no_matches_returns_empty_array() {
        let source = make_string_array(&["Apple", "Banana", "Cherry"]);
        let result = filter(&source, "xyz", None, None).unwrap();
        let matches = extract_strings(&result);

        assert_eq!(matches, Vec::<String>::new());
    }

    #[test]
    fn returns_zero_based_array() {
        let source = make_string_array(&["a", "b", "c"]);
        let result = filter(&source, "a", None, None).unwrap();
        let arr = result.as_array().unwrap();

        assert_eq!(arr.lower_bound(0).unwrap(), 0);
    }

    #[test]
    fn null_source_is_error_94() {
        let err = filter(&VBVariant::Null, "test", None, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn non_array_is_error_13() {
        let err = filter(&VBVariant::from_string("not an array"), "test", None, None).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn preserves_original_strings() {
        let source = make_string_array(&["Hello World", "HELLO WORLD", "hello world"]);
        let result = filter(&source, "world", None, Some(1)).unwrap();
        let matches = extract_strings(&result);

        // Text compare finds all three, original casing preserved
        assert_eq!(matches, vec!["Hello World", "HELLO WORLD", "hello world"]);
    }

    #[test]
    fn handles_one_based_arrays() {
        let data: Vec<VBVariant> = vec![
            VBVariant::from_string("Apple"),
            VBVariant::from_string("Banana"),
            VBVariant::from_string("Cherry"),
        ];
        let source = VBVariant::Array(ArrayValue::from_vec(VBType::String, data));
        let result = filter(&source, "e", None, None).unwrap();
        assert_eq!(extract_strings(&result), vec!["Apple", "Cherry"]);
    }

    #[test]
    fn handles_arbitrary_lower_bound() {
        let data: Vec<VBVariant> = vec![
            VBVariant::from_string("a"),
            VBVariant::from_string("bb"),
            VBVariant::from_string("c"),
        ];
        let source =
            VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::String, data, -2));
        let result = filter(&source, "b", None, None).unwrap();
        assert_eq!(extract_strings(&result), vec!["bb"]);
        // Result is zero-based even though the source was not
        assert_eq!(result.as_array().unwrap().lower_bound(0).unwrap(), 0);
    }

    #[test]
    fn non_string_element_is_error_13() {
        let data: Vec<VBVariant> = vec![
            VBVariant::from_string("Apple"),
            VBVariant::from_integer(42),
            VBVariant::from_string("Cherry"),
        ];
        let source = VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::Variant, data, 0));
        let err = filter(&source, "e", None, None).unwrap_err();
        assert_eq!(err.number, err_number::TYPE_MISMATCH);
    }

    #[test]
    fn null_element_is_error_94() {
        let data: Vec<VBVariant> = vec![
            VBVariant::from_string("Apple"),
            VBVariant::Null,
            VBVariant::from_string("Cherry"),
        ];
        let source = VBVariant::Array(ArrayValue::from_vec_with_bounds(VBType::Variant, data, 0));
        let err = filter(&source, "e", None, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_USE_OF_NULL);
    }

    #[test]
    fn invalid_compare_is_error_5() {
        let source = make_string_array(&["Apple", "Banana"]);
        for compare in [3, 5, -2, 100] {
            let err = filter(&source, "a", None, Some(compare)).unwrap_err();
            assert_eq!(
                err.number,
                err_number::INVALID_PROCEDURE_CALL,
                "compare = {compare}"
            );
        }
    }

    #[test]
    fn use_option_compare_defaults_to_binary() {
        let source = make_string_array(&["Apple", "apple"]);
        let result = filter(&source, "a", None, Some(-1)).unwrap();
        assert_eq!(extract_strings(&result), vec!["apple"]);
    }

    #[test]
    fn database_compare_defaults_to_binary() {
        let source = make_string_array(&["Apple", "apple"]);
        let result = filter(&source, "a", None, Some(2)).unwrap();
        assert_eq!(extract_strings(&result), vec!["apple"]);
    }

    #[test]
    fn explicit_binary_compare_is_case_sensitive() {
        let source = make_string_array(&["Apple", "apple"]);
        let result = filter(&source, "a", None, Some(0)).unwrap();
        assert_eq!(extract_strings(&result), vec!["apple"]);
    }

    #[test]
    fn undimensioned_array_returns_empty() {
        let source = VBVariant::Array(ArrayValue::new_dynamic(VBType::String));
        let result = filter(&source, "x", None, None).unwrap();
        assert!(extract_strings(&result).is_empty());
    }

    #[test]
    fn zero_length_array_returns_empty() {
        let source = VBVariant::Array(ArrayValue::from_vec_with_bounds(
            VBType::String,
            Vec::new(),
            0,
        ));
        let result = filter(&source, "x", None, None).unwrap();
        assert!(extract_strings(&result).is_empty());
    }

    #[test]
    fn multi_dimensional_array_is_error_5() {
        let source = VBVariant::Array(
            ArrayValue::new_fixed(
                VBType::String,
                &[ArrayDimension::new(1, 2), ArrayDimension::new(1, 2)],
            )
            .unwrap(),
        );
        let err = filter(&source, "x", None, None).unwrap_err();
        assert_eq!(err.number, err_number::INVALID_PROCEDURE_CALL);
    }

    #[test]
    fn result_bounds_reflect_match_count() {
        let source = make_string_array(&["Apple", "Banana", "Cherry"]);
        let result = filter(&source, "e", None, None).unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.lower_bound(0).unwrap(), 0);
        assert_eq!(arr.upper_bound(0).unwrap(), 1);
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn substring_match_is_not_whole_word() {
        let source = make_string_array(&["banana", "band", "apple"]);
        let result = filter(&source, "ban", None, None).unwrap();
        assert_eq!(extract_strings(&result), vec!["banana", "band"]);
    }

    #[test]
    fn exclude_preserves_order() {
        let source = make_string_array(&["a", "b", "c", "d"]);
        let result = filter(&source, "b", Some(false), None).unwrap();
        assert_eq!(extract_strings(&result), vec!["a", "c", "d"]);
    }
}
