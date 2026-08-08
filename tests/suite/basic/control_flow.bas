Attribute VB_Name = "ControlFlow"
Option Explicit

' TEST: Control flow statements
' CATEGORY: basic
' DESCRIPTION: If/ElseIf, Select Case, For, Do, While loops.

Sub Main()
    Dim i As Integer
    Dim sum As Integer

    If 1 < 2 Then
        Print #1, "yes"
    Else
        Print #1, "no"
    End If

    If 2 < 1 Then
        Print #1, "never"
    ElseIf 2 > 1 Then
        Print #1, "elseif"
    Else
        Print #1, "never"
    End If

    sum = 0
    For i = 1 To 5
        sum = sum + i
    Next i
    Print #1, sum

    i = 0
    Do While i < 3
        Print #1, i
        i = i + 1
    Loop

    i = 0
    Do
        i = i + 1
        If i = 2 Then Exit Do
    Loop
    Print #1, i

    i = 0
    While i < 2
        Print #1, i
        i = i + 1
    Wend

    Select Case 3
        Case 1
            Print #1, "one"
        Case 2
            Print #1, "two"
        Case Else
            Print #1, "other"
    End Select

    Select Case 1
        Case Is < 0
            Print #1, "neg"
        Case 0 To 2
            Print #1, "small"
        Case Else
            Print #1, "big"
    End Select
End Sub
