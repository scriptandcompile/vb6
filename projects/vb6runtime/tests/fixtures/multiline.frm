VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Multi-Line Test"
   ClientHeight    =   4000
   ClientWidth     =   5000
   Begin VB.TextBox txtMulti
      Height          =   1455
      Left            =   240
      MultiLine       =   -1  'True
      ScrollBars      =   2  'Both
      Top             =   240
      Width           =   3750
   End
   Begin VB.CommandButton cmdSubmit
      Caption         =   "&Submit"
      Height          =   495
      Left            =   240
      Top             =   1920
      Width           =   1215
   End
End
