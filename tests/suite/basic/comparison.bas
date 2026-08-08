Attribute VB_Name = "Comparison"
Option Explicit

' TEST: Comparison and logical operators
' CATEGORY: basic
' DESCRIPTION: Relational operators and boolean logic.

Sub Main()
    Print #1, 1 = 1
    Print #1, 1 = 2
    Print #1, 1 <> 2
    Print #1, 3 > 2
    Print #1, 2 < 3
    Print #1, 2 <= 2
    Print #1, 3 >= 4
    Print #1, 3.5 > 3
    Print #1, True And False
    Print #1, True Or False
    Print #1, Not True
    Print #1, True Xor True
    Print #1, True Eqv True
    Print #1, False Imp True
    Print #1, True Imp False
    Print #1, "a" = "A"
    Print #1, "abc" < "abd"
    Print #1, 5 > 4 And 3 < 4
    Print #1, Not (1 = 2)
    Print #1, (1 + 1) = 2
    Print #1, 7 Mod 2 = 1
End Sub
