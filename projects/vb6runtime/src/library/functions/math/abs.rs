//! # `Abs` Function
//!
//! Returns the absolute value of a number.
//!
//! ## Syntax
//!
//! ```vb
//! Abs(number)
//! ```
//!
//! ## Parts
//!
//! - **number**: Required. Any valid numeric expression. If number contains Null, Null is returned;
//!   if it is an uninitialized variable, zero is returned.
//!
//! ## Return Value
//!
//! The return type is the same as the input type, except:
//! - If number is a Variant containing Null, returns Null
//! - If number is an uninitialized Variant, returns 0
//! - The absolute value is always non-negative (>= 0)
//!
//! ## Remarks
//!
//! - **Absolute Value**: The absolute value of a number is its unsigned magnitude. For example,
//!   Abs(-1) and Abs(1) both return 1.
//! - **Type Preservation**: The return type matches the input type. If you pass an Integer, you
//!   get an Integer back. If you pass a Double, you get a Double back.
//! - **Null Handling**: If the argument is Null, the function returns Null.
//! - **Overflow**: For the most negative value of Integer (-32768) or Long (-2147483648), Abs
//!   will cause an overflow error because the positive equivalent is outside the valid range.
//! - **Performance**: Abs is highly optimized and very fast for numeric operations.
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```vb
//! Dim result As Integer
//! result = Abs(-50)
//! ' result = 50
//! ```
//!
//! ### With Positive Numbers
//!
//! ```vb
//! Dim value As Integer
//! value = Abs(25)
//! ' value = 25 (unchanged)
//! ```
//!
//! ### With Floating Point
//!
//! ```vb
//! Dim distance As Double
//! distance = Abs(-12.75)
//! ' distance = 12.75
//! ```
//!
//! ### With Zero
//!
//! ```vb
//! Dim zero As Integer
//! zero = Abs(0)
//! ' zero = 0
//! ```
//!
//! ### With Expressions
//!
//! ```vb
//! Dim x As Integer, y As Integer
//! x = 10
//! y = 20
//! Dim difference As Integer
//! difference = Abs(x - y)
//! ' difference = 10
//! ```
//!
//! ### Calculating Distance
//!
//! ```vb
//! Function Distance(x1 As Double, y1 As Double, x2 As Double, y2 As Double) As Double
//!     Distance = Sqr((x2 - x1) ^ 2 + (y2 - y1) ^ 2)
//! End Function
//!
//! ' Often used with Abs for 1D distance:
//! Dim dist As Double
//! dist = Abs(x2 - x1)
//! ```
//!
//! ### With Currency
//!
//! ```vb
//! Dim amount As Currency
//! amount = Abs(-1234.56@)
//! ' amount = 1234.56
//! ```
//!
//! ### With Variants
//!
//! ```vb
//! Dim v As Variant
//! v = -42
//! Dim result As Variant
//! result = Abs(v)
//! ' result = 42
//! ```
//!
//! ## Common Patterns
//!
//! ### Ensuring Positive Values
//!
//! ```vb
//! Sub ProcessValue(ByVal input As Integer)
//!     Dim positiveInput As Integer
//!     positiveInput = Abs(input)
//!     ' Always work with positive values
//!     DoSomething positiveInput
//! End Sub
//! ```
//!
//! ### Calculating Difference
//!
//! ```vb
//! Function GetDifference(a As Long, b As Long) As Long
//!     GetDifference = Abs(a - b)
//! End Function
//! ```
//!
//! ### Data Validation
//!
//! ```vb
//! Function IsWithinTolerance(actual As Double, expected As Double, tolerance As Double) As Boolean
//!     IsWithinTolerance = (Abs(actual - expected) <= tolerance)
//! End Function
//! ```
//!
//! ### Financial Calculations
//!
//! ```vb
//! Function CalculateVariance(actual As Currency, budget As Currency) As Currency
//!     CalculateVariance = Abs(actual - budget)
//! End Function
//! ```
//!
//! ### Array Processing
//!
//! ```vb
//! Sub MakeArrayPositive(arr() As Integer)
//!     Dim i As Integer
//!     For i = LBound(arr) To UBound(arr)
//!         arr(i) = Abs(arr(i))
//!     Next i
//! End Sub
//! ```
//!
//! ### Comparison Logic
//!
//! ```vb
//! Function MaxAbsValue(a As Double, b As Double) As Double
//!     If Abs(a) > Abs(b) Then
//!         MaxAbsValue = Abs(a)
//!     Else
//!         MaxAbsValue = Abs(b)
//!     End If
//! End Function
//! ```
//!
//! ### Coordinate Systems
//!
//! ```vb
//! Function ManhattanDistance(x1 As Integer, y1 As Integer, x2 As Integer, y2 As Integer) As Integer
//!     ManhattanDistance = Abs(x2 - x1) + Abs(y2 - y1)
//! End Function
//! ```
//!
//! ## Related Functions
//!
//! - `Sgn`: Returns the sign of a number (-1, 0, or 1)
//! - `Fix`: Returns the integer portion of a number (truncates toward zero)
//! - `Int`: Returns the integer portion of a number (rounds down)
//! - `Round`: Rounds a number to a specified number of decimal places
//!
//! ## Type Compatibility
//!
//! | Input Type | Return Type | Notes |
//! |------------|-------------|-------|
//! | Byte | Byte | Always positive already |
//! | Integer | Integer | Can overflow at -32768 |
//! | Long | Long | Can overflow at -2147483648 |
//! | Single | Single | Preserves precision |
//! | Double | Double | Preserves precision |
//! | Currency | Currency | Preserves 4 decimal places |
//! | Variant (numeric) | Variant | Type preserved |
//! | Variant (Null) | Null | Returns Null |
//!
//! ## Performance Notes
//!
//! - `Abs` is a very fast intrinsic function
//! - No function call overhead in compiled code
//! - Optimized to CPU instructions where possible
//! - Prefer `Abs` over manual `If`/`Then` checks for performance
//!
//! `Abs` is parsed as a regular function call (`CallExpression`)
//! This module serves as documentation and reference for the Abs function
