Attribute VB_Name = "Variables"
Option Explicit

' TEST: Variables and constants
' CATEGORY: basic
' DESCRIPTION: Declarations, assignment, and typed arithmetic.

Const GREETING = "hello"

Sub Main()
    Dim x As Integer
    Dim y As Long
    Dim firstName As String
    x = 5
    y = 100000
    firstName = "world"
    Print #1, x + y
    Print #1, firstName
    Print #1, GREETING & " " & firstName
    Dim a As Double
    Dim b As Double
    a = 1.5
    b = 2.25
    Print #1, a * b
    Print #1, a + b
    Dim flag As Boolean
    flag = True
    Print #1, flag
    flag = False
    Print #1, flag
    Dim count As Long
    count = 1
    count = count + 1
    count = count + 1
    Print #1, count
End Sub
