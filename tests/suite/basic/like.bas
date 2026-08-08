Attribute VB_Name = "LikeOps"
Option Explicit

' TEST: Like operator
' CATEGORY: basic
' DESCRIPTION: Pattern matching with ?, *, # and character classes.

Sub Main()
    Print #1, "Hello" Like "H*"
    Print #1, "Hello" Like "H?llo"
    Print #1, "Hello" Like "H?xxx"
    Print #1, "abc" Like "[a-c]*"
    Print #1, "def" Like "[a-c]*"
    Print #1, "123" Like "###"
    Print #1, "abc123" Like "abc###"
    Print #1, "test" Like "*es*"
End Sub
