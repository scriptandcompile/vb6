Attribute VB_Name = "ConsolePrint"
Option Explicit

' TEST: Console print extensions
' CATEGORY: basic
' DESCRIPTION: Bare Print and Debug.Print are console-output extensions.
' SKIP_VB6: bare Print does not compile in a standard module; Debug.Print is a no-op in compiled exes.

Sub Main()
    Print "bare print"
    Print 2 + 2
    Debug.Print "debug print"
    Debug.Print 10 - 3
End Sub
