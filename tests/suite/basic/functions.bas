Attribute VB_Name = "Functions"
Option Explicit

' TEST: User-defined procedures and functions
' CATEGORY: basic
' DESCRIPTION: Sub and Function declarations, parameters, recursion.

Sub Main()
    Print #1, Dbl(4)
    Print #1, Add(2, 3)
    Print #1, Factorial(5)
    SayHello
    Print #1, JoinWords("a", "b")
End Sub

Function Dbl(ByVal n As Integer) As Integer
    Dbl = n * 2
End Function

Function Add(ByVal a As Integer, ByVal b As Integer) As Integer
    Add = a + b
End Function

Function Factorial(ByVal n As Integer) As Integer
    If n <= 1 Then
        Factorial = 1
    Else
        Factorial = n * Factorial(n - 1)
    End If
End Function

Function JoinWords(ByVal left As String, ByVal right As String) As String
    JoinWords = left & "-" & right
End Function

Sub SayHello()
    Print #1, "hello from sub"
End Sub
