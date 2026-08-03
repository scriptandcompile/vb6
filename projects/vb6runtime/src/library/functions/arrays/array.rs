//! ## `Array` Function
//!
//! Returns a `Variant` containing an array formed from the comma-delimited arg list of values
//! passed into the function.
//!
//! ## Syntax
//!
//! ```text
//! Array(arglist)
//! ```
//!
//! ## Parameters
//!
//! - `arglist`: Required. A comma-delimited list of values that are assigned to the elements
//!   of the array contained within the `Variant`. If no arguments are specified, an array of zero
//!   length is created.
//!
//! ## Return Value
//!
//! Returns a `Variant` whose subtype is `Array` containing the specified elements.
//!
//! ## Remarks
//!
//! - `Variant Array`: The `Array` function returns a `Variant` that contains an array. The array
//!   elements are `Variants` that can hold any data type.
//! - `Zero-Based`: The array created by the `Array` function is zero-based. The first element
//!   has an index of 0.
//! - `Dynamic Size`: The size of the array is determined by the number of arguments provided.
//! - `Mixed Types`: `Array` elements can be of different types since they are stored as `Variants`.
//! - `Assignment`: The result must be assigned to a `Variant` variable, not an array declared
//!   with specific dimensions.
//! - `Empty Array`: Calling `Array()` with no arguments creates a zero-length array.
//! - `LBound and UBound`: You can use `LBound` and `UBound` to determine the array bounds.
//!   `LBound` always returns 0, `UBound` returns (number of elements - 1).
//! - `Option Base`: The `Array` function is not affected by `Option Base` statements; it always
//!   creates zero-based arrays.
//!
//! ## Important Characteristics
//!
//! ### Assignment Requirements
//!
//! ```vb
//! ' Correct - assign to Variant
//! Dim v As Variant
//! v = Array(1, 2, 3)  ' OK
//!
//! ' Incorrect - cannot assign to typed array
//! Dim arr(2) As Integer
//! arr = Array(1, 2, 3)  ' ERROR: Type mismatch
//! ```
//!
//! ### Zero-Based Indexing
//!
//! ```vb
//! Dim arr As Variant
//! arr = Array("A", "B", "C")
//! Debug.Print LBound(arr)  ' Always 0
//! Debug.Print UBound(arr)  ' 2 (not 3!)
//!
//! ' First element is arr(0), last is arr(2)
//! ```
//!
//! ### Performance Considerations
//!
//! - `Array()` creates a `Variant` array, which has more overhead than typed arrays
//! - For large arrays with known types, consider using `ReDim` instead
//! - `Array()` is best for small, temporary arrays or mixed-type collections
//! - Each element is a `Variant`, which uses more memory than native types
//!
//! ## Related Functions
//!
//! - `Split`: Splits a string into an array of substrings
//! - `Join`: Concatenates array elements into a string
//! - `LBound`: Returns the lowest available subscript for an array dimension
//! - `UBound`: Returns the highest available subscript for an array dimension
//! - `IsArray`: Determines whether a variable is an array
//! - `Filter`: Returns a zero-based array containing a subset of a string array
//!
//! ## Examples
//!
//! ### Basic Array Creation
//!
//! ```vb
//! Dim myArray As Variant
//! myArray = Array(1, 2, 3, 4, 5)
//! ' myArray contains: [1, 2, 3, 4, 5]
//! ' LBound(myArray) = 0, UBound(myArray) = 4
//! ```
//!
//! ### Mixed Data Types
//!
//! ```vb
//! Dim mixed As Variant
//! mixed = Array("Hello", 42, True, #1/1/2025#, 3.14)
//! ' Array can hold different types
//! ```
//!
//! ### String Array
//!
//! ```vb
//! Dim names As Variant
//! names = Array("Alice", "Bob", "Charlie")
//! Debug.Print names(0)  ' Prints: Alice
//! ```
//!
//! ### Empty Array
//!
//! ```vb
//! Dim emptyArr As Variant
//! emptyArr = Array()
//! ' Creates a zero-length array
//! ' UBound(emptyArr) = -1
//! ```
//!
//! ### Using For Each
//!
//! ```vb
//! Dim values As Variant
//! values = Array(10, 20, 30, 40)
//!
//! Dim item As Variant
//! For Each item In values
//!     Debug.Print item
//! Next item
//! ```
//!
//! ### Array as Function Return
//!
//! ```vb
//! Function GetColors() As Variant
//!     GetColors = Array("Red", "Green", "Blue")
//! End Function
//! ```
//!
//! ### Accessing Elements
//!
//! ```vb
//! Dim data As Variant
//! data = Array("A", "B", "C")
//! Debug.Print data(0)  ' A
//! Debug.Print data(1)  ' B
//! Debug.Print data(2)  ' C
//! ```
//!
//! ## Common Patterns
//!
//! ### Initialize Lookup Table
//!
//! ```vb
//! Function GetMonthName(monthNum As Integer) As String
//!     Dim months As Variant
//!     months = Array("Jan", "Feb", "Mar", "Apr", "May", "Jun", _
//!                    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec")
//!     
//!     If monthNum >= 1 And monthNum <= 12 Then
//!         GetMonthName = months(monthNum - 1)
//!     Else
//!         GetMonthName = ""
//!     End If
//! End Function
//! ```
//!
//! ### Configuration Data
//!
//! ```vb
//! Sub ProcessFiles()
//!     Dim extensions As Variant
//!     extensions = Array(".txt", ".doc", ".pdf", ".xls")
//!     
//!     Dim ext As Variant
//!     For Each ext In extensions
//!         ProcessFileType CStr(ext)
//!     Next ext
//! End Sub
//! ```
//!
//! ### Quick Test Data
//!
//! ```vb
//! Sub TestFunction()
//!     Dim testCases As Variant
//!     testCases = Array(0, 1, 10, 100, -1, -100)
//!     
//!     Dim testValue As Variant
//!     For Each testValue In testCases
//!         Debug.Print "Testing: " & testValue & " -> " & MyFunction(testValue)
//!     Next testValue
//! End Sub
//! ```
//!
//! ### Passing Multiple Values
//!
//! ```vb
//! Sub UpdateRecord()
//!     SaveData Array("Name", "John"), _
//!             Array("Age", 30), _
//!             Array("City", "NYC")
//! End Sub
//!
//! Sub SaveData(ParamArray fields())
//!     Dim field As Variant
//!     For Each field In fields
//!         Debug.Print field(0) & ": " & field(1)
//!     Next field
//! End Sub
//! ```
//!
//! ### Enumeration Substitute
//!
//! ```vb
//! Function GetStatusText(status As Integer) As String
//!     Dim statuses As Variant
//!     statuses = Array("Pending", "Processing", "Complete", "Failed")
//!     
//!     If status >= 0 And status <= 3 Then
//!         GetStatusText = statuses(status)
//!     Else
//!         GetStatusText = "Unknown"
//!     End If
//! End Function
//! ```
//!
//! ### Split Alternative (VB6 Early Versions)
//!
//! ```vb
//! ' Before Split function was widely available
//! Function GetHeaderFields() As Variant
//!     GetHeaderFields = Array("ID", "Name", "Date", "Status")
//! End Function
//! ```
//!
//! ### Matrix/Grid Data
//!
//! ```vb
//! Sub CreateGrid()
//!     Dim row1 As Variant, row2 As Variant, row3 As Variant
//!     row1 = Array(1, 2, 3)
//!     row2 = Array(4, 5, 6)
//!     row3 = Array(7, 8, 9)
//!     
//!     Dim grid As Variant
//!     grid = Array(row1, row2, row3)
//!     
//!     ' Access: grid(0)(0) = 1, grid(1)(2) = 6, etc.
//! End Sub
//! ```
//!
//! ### Default Values
//!
//! ```vb
//! Function GetDefaults() As Variant
//!     GetDefaults = Array(0, "", False, Null, Empty)
//! End Function
//! ```
//!
