VERSION 5.00
Begin VB.Form Form1 
   Caption         =   "Simple Test"
   ClientHeight    =   1680
   ClientLeft      =   120
   ClientTop       =   465
   ClientWidth     =   3840
   LinkTopic       =   "Form1"
   ScaleHeight     =   1680
   ScaleWidth      =   3840
   StartUpPosition =   2  'CenterScreen
   Begin VB.CommandButton cmdClose 
      Caption         =   "Close"
      Height          =   495
      Left            =   1320
      TabIndex        =   0
      Top             =   600
      Width           =   1215
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
Option Explicit

Private Sub cmdClose_Click()
    Unload Me
End Sub
