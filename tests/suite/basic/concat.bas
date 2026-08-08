Attribute VB_Name = "Concat"
Option Explicit

' TEST: String concatenation
' CATEGORY: basic
' DESCRIPTION: The & and + concatenation operators.

Sub Main()
    Print #1, "foo" & "bar"
    Print #1, "a" & 1
    Print #1, 1 & "a"
    Print #1, "x" + "y"
    Print #1, "left" & "middle" & "right"
    Print #1, "" & "empty"
    Print #1, Left("abcdef", 3) & Right("abcdef", 3)
End Sub
