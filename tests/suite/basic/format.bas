Attribute VB_Name = "Formatting"
Option Explicit

' TEST: Format function
' CATEGORY: basic
' DESCRIPTION: Numeric format patterns.

Sub Main()
    Print #1, Format(1234.5, "###,##0.00")
    Print #1, Format(7, "000")
    Print #1, Format(123.456, "0.00")
    Print #1, Format(255, "00000")
    Print #1, Format(0, "0.0")
    Print #1, Format(1.5, "0.0")
End Sub
