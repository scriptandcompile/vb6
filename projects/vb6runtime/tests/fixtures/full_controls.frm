VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Full Controls Test"
   ClientHeight    =   6000
   ClientWidth     =   7000
   Begin VB.Label lblTitle
      Caption         =   "Title Label"
      Height          =   375
      Left            =   240
      Top             =   240
      Width           =   2500
   End
   Begin VB.CommandButton cmdCancel
      Caption         =   "&Cancel"
      Height          =   495
      Left            =   4000
      Top             =   5200
      Width           =   1215
   End
   Begin VB.Frame Frame1
      Caption         =   "Options"
      Height          =   2000
      Left            =   240
      Top             =   1200
      Width           =   3000
      Begin VB.CheckBox chkOption1
         Caption         =   "Option 1"
         Height          =   375
         Left            =   240
         Top             =   480
         Width           =   1500
      End
      Begin VB.OptionButton optSingle
         Caption         =   "Single Select"
         Height          =   375
         Left            =   240
         Top             =   1000
         Width           =   2000
      End
   End
   Begin VB.PictureBox picDisplay
      Height          =   1500
      Left            =   4000
      Top             =   240
      Width           =   2500
   End
   Begin VB.Shape Shape1
      Height          =   615
      Left            =   4000
      Shape           =   3  'Oval
      Top             =   2000
      Width           =   1815
   End
   Begin VB.ComboBox cboItems
      Height          =   375
      Left            =   240
      Top             =   3600
      Width           =   2500
   End
   Begin VB.ListBox lstResults
      Height          =   1200
      Left            =   240
      Top             =   4200
      Width           =   3000
   End
End
