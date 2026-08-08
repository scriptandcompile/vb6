Attribute VB_Name = "Strings"
Option Explicit

' TEST: String functions
' CATEGORY: basic
' DESCRIPTION: Len, Left, Right, Mid, case, trim, reverse, InStr, Chr/Asc.

Sub Main()
    Print #1, Len("Hello")
    Print #1, Len("")
    Print #1, Left("Hello", 2)
    Print #1, Right("Hello", 2)
    Print #1, Mid("Hello", 2, 3)
    Print #1, Mid("Hello", 2)
    Print #1, Mid("Hello", 10, 3)
    Print #1, LCase("HeLLo")
    Print #1, UCase("HeLLo")
    Print #1, Trim("  hi  ")
    Print #1, LTrim("  hi  ")
    Print #1, RTrim("  hi  ")
    Print #1, StrReverse("Hello")
    Print #1, InStr("Hello world", "world")
    Print #1, InStr("Hello world", "xyz")
    Print #1, InStr(4, "Hello Hello", "He")
    Print #1, InStrRev("Hello world", "o")
    Print #1, InStrRev("Hello world", "o", 5)
    Print #1, Space(4) & "x"
    Print #1, Chr(65)
    Print #1, Chr(66) & Chr(67)
    Print #1, ChrW(68)
    Print #1, Asc("A")
    Print #1, AscW("B")
    Print #1, Left("abcdef", 3) & Right("abcdef", 3)
    Print #1, UCase(Left("hello", 4)) & LCase(Right("WORLD", 3))
End Sub
