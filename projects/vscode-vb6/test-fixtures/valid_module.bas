Option Explicit

Public Const APP_TITLE As String = "VB6 Diagnostics Demo"

Private Type Point
    X As Integer
    Y As Integer
End Type

Public Function AddNumbers(ByVal a As Integer, ByVal b As Integer) As Integer
    AddNumbers = a + b
End Function

Public Sub Main()
    Dim total As Integer
    total = AddNumbers(1, 2)
    Debug.Print APP_TITLE & ": " & total
End Sub
