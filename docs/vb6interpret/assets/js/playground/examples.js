export const examples = {
    "hello-world": {
        name: "Hello World",
        code: `Attribute VB_Name = "HelloModule"

Sub Main()
    Debug.Print "Hello from VB6Interpret"
End Sub
`,
    },
    "simple-math": {
        name: "Simple Math",
        code: `Attribute VB_Name = "MathModule"

Sub Main()
    Dim total As Integer
    total = 19 + 23
    Debug.Print total
End Sub
`,
    },
    branching: {
        name: "Branching",
        code: `Attribute VB_Name = "BranchModule"

Sub Main()
    Dim score As Integer
    score = 84

    If score >= 80 Then
        Debug.Print "pass"
    Else
        Debug.Print "retry"
    End If
End Sub
`,
    },
    looping: {
        name: "Looping",
        code: `Attribute VB_Name = "LoopModule"

Sub Main()
    Dim i As Integer

    For i = 1 To 3
        Debug.Print i
    Next i
End Sub
`,
    },
    files: {
        name: "Files",
        code: `Attribute VB_NAME = "FilesModule"

Sub Main()
    Dim i as Integer

    Open "Test.txt" For Append As #1 
        For i = 1 to 3
            Print #1, i
        Next i
    Close #1

End Sub
`,
    },
    "simple-form": {
        name: "Simple Form (Label + TextBox + Button)",
        code: `VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Simple Form"
   ClientHeight    =   3000
   ClientWidth     =   4500
   StartUpPosition =   2  'CenterScreen
   Begin VB.Label Label1
      Caption         =   "Enter text below:"
      Height          =   375
      Left            =   240
      Top             =   240
      Width           =   2500
   End
   Begin VB.TextBox Text1
      Height          =   375
      Left            =   240
      TabIndex        =   1
      Top             =   840
      Width           =   3000
   End
   Begin VB.CommandButton cmdOK
      Caption         =   "&OK"
      Height          =   495
      Left            =   240
      TabIndex        =   2
      Top             =   1800
      Width           =   1215
   End
   Begin VB.CommandButton cmdCancel
      Caption         =   "&Cancel"
      Height          =   495
      Left            =   1800
      TabIndex        =   3
      Top             =   1800
      Width           =   1215
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
Private Sub cmdOK_Click()
    Debug.Print "You entered: " & Text1.Text
End Sub

Private Sub cmdCancel_Click()
    Debug.Print "Cancelled"
End Sub
`,
    },
    "form-with-containers": {
        name: "Form with Containers",
        code: `VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Container Test"
   ClientHeight    =   5000
   ClientWidth     =   6000
   StartUpPosition =   2  'CenterScreen
   Begin VB.Frame Frame1
      Caption         =   "Group Box"
      Height          =   2000
      Left            =   240
      TabIndex        =   3
      Top             =   240
      Width           =   3500
      Begin VB.Label lblInFrame
         Caption         =   "Inside Frame"
         Height          =   375
         Left            =   480
         TabIndex        =   4
         Top             =   600
         Width           =   2000
      End
      Begin VB.OptionButton optA
         Caption         =   "Option A"
         Height          =   375
         Left            =   480
         TabIndex        =   5
         Top             =   1200
         Width           =   2000
      End
      Begin VB.OptionButton optB
         Caption         =   "Option B"
         Height          =   375
         Left            =   480
         TabIndex        =   6
         Top             =   1680
         Width           =   2000
      End
   End
   Begin VB.PictureBox picDisplay
      Height          =   2000
      Left            =   4000
      ScaleHeight     =   1920
      ScaleWidth     =   1740
      TabIndex        =   0
      Top             =   240
      Width           =   1800
      Begin VB.Label lblInPic
         Caption         =   "Inside PictureBox"
         Height          =   375
         Left            =   240
         TabIndex        =   1
         Top             =   480
         Width           =   1200
      End
      Begin VB.CommandButton cmdPicBtn
         Caption         =   "Pic Button"
         Height          =   495
         Left            =   240
         TabIndex        =   2
         Top             =   1200
         Width           =   1215
      End
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
Private Sub cmdPicBtn_Click()
    Debug.Print "PictureBox button clicked!"
End Sub
`,
    },
    "form-with-events": {
        name: "Form with Load/Unload Events",
        code: `VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Form Events Test"
   ClientHeight    =   3000
   ClientWidth     =   4500
   StartUpPosition =   2  'CenterScreen
   Begin VB.Label lblStatus
      Caption         =   "Ready"
      Height          =   375
      Left            =   240
      TabIndex        =   1
      Top             =   240
      Width           =   3000
   End
   Begin VB.CommandButton cmdAction
      Caption         =   "Do Something"
      Height          =   495
      Left            =   240
      TabIndex        =   2
      Top             =   1200
      Width           =   1800
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
Private Sub Form_Load()
    Debug.Print "=== Form Loaded ==="
    Debug.Print "Form caption: " & Me.Caption
    lblStatus.Caption = "Form is loaded"
End Sub

Private Sub Form_Unload(Cancel As Integer)
    Debug.Print "=== Form Unloading ==="
    lblStatus.Caption = "Form is closing"
End Sub

Private Sub cmdAction_Click()
    Debug.Print "Action button clicked at " & Time$
    lblStatus.Caption = "Action executed!"
End Sub
`,
    },
    "form-with-module-reference": {
        name: "Form with Module Reference",
        code: `VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Module Reference Test"
   ClientHeight    =   3000
   ClientWidth     =   5000
   StartUpPosition =   2  'CenterScreen
   Begin VB.Label lblResult
      Caption         =   "Results will appear here"
      Height          =   615
      Left            =   240
      TabIndex        =   1
      Top             =   240
      Width           =   4500
   End
   Begin VB.CommandButton cmdProcess
      Caption         =   "Process Data"
      Height          =   495
      Left            =   240
      TabIndex        =   2
      Top             =   1200
      Width           =   1800
   End
   Begin VB.CommandButton cmdReset
      Caption         =   "Reset"
      Height          =   495
      Left            =   240
      TabIndex        =   3
      Top             =   2000
      Width           =   1800
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
Private Sub cmdProcess_Click()
    Debug.Print "Calling module procedure..."
    Call ProcessData("Test Data")
    lblResult.Caption = "Processing complete"
End Sub

Private Sub cmdReset_Click()
    lblResult.Caption = "Results will appear here"
    Debug.Print "Reset executed"
End Sub
`,
    },
    "full-controls": {
        name: "Full Controls Test",
        code: `VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Full Controls Test"
   ClientHeight    =   7000
   ClientWidth     =   7500
   StartUpPosition =   2  'CenterScreen
   Begin VB.Label lblTitle
      Caption         =   "Title Label"
      Height          =   375
      Left            =   240
      TabIndex        =   16
      Top             =   240
      Width           =   3000
   End
   Begin VB.CommandButton cmdCancel
      Caption         =   "&Cancel"
      Height          =   495
      Left            =   4200
      TabIndex        =   15
      Top             =   6200
      Width           =   1215
   End
   Begin VB.CommandButton cmdApply
      Caption         =   "&Apply"
      Height          =   495
      Left            =   2800
      TabIndex        =   14
      Top             =   6200
      Width           =   1215
   End
   Begin VB.Frame Frame1
      Caption         =   "Options"
      Height          =   2200
      Left            =   240
      TabIndex        =   9
      Top             =   1200
      Width           =   3500
      Begin VB.CheckBox chkOption1
         Caption         =   "Enable Option 1"
         Height          =   375
         Left            =   240
         TabIndex        =   10
         Top             =   600
         Width           =   2000
      End
      Begin VB.CheckBox chkOption2
         Caption         =   "Enable Option 2"
         Height          =   375
         Left            =   240
         TabIndex        =   11
         Top             =   1100
         Width           =   2000
      End
      Begin VB.OptionButton optSingle
         Caption         =   "Single Select"
         Height          =   375
         Left            =   240
         TabIndex        =   12
         Top             =   1600
         Width           =   2000
      End
   End
   Begin VB.PictureBox picDisplay
      Height          =   1800
      Left            =   4000
      ScaleHeight     =   1720
      ScaleWidth     =   3240
      TabIndex        =   0
      Top             =   1200
      Width           =   3300
      Begin VB.Label lblInPic
         Caption         =   "PictureBox Area"
         Height          =   375
         Left            =   480
         TabIndex        =   1
         Top             =   480
         Width           =   2000
      End
   End
   Begin VB.Shape Shape1
      Height          =   800
      Left            =   4200
      Shape           =   3  'Oval
      Top             =   3600
      Width           =   2500
   End
   Begin VB.ComboBox cboItems
      Height          =   375
      Left            =   240
      TabIndex        =   6
      Top             =   4000
      Width           =   3000
   End
   Begin VB.ListBox lstResults
      Height          =   1400
      Left            =   240
      TabIndex        =   7
      Top             =   4800
      Width           =   3500
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
Private Sub chkOption1_Click()
    If chkOption1.Value = 1 Then
        Debug.Print "Option 1 enabled"
    Else
        Debug.Print "Option 1 disabled"
    End If
End Sub

Private Sub chkOption2_Click()
    If chkOption2.Value = 1 Then
        Debug.Print "Option 2 enabled"
    Else
        Debug.Print "Option 2 disabled"
    End If
End Sub

Private Sub cmdApply_Click()
    Debug.Print "Apply clicked"
    Debug.Print "Option 1: " & IIf(chkOption1.Value = 1, "On", "Off")
    Debug.Print "Option 2: " & IIf(chkOption2.Value = 1, "On", "Off")
End Sub

Private Sub cmdCancel_Click()
    Debug.Print "Cancel clicked - discarding changes"
End Sub

Private Sub Form_Load()
    Debug.Print "Full controls form loaded"
    cboItems.AddItem "Item 1"
    cboItems.AddItem "Item 2"
    cboItems.AddItem "Item 3"
    Debug.Print "Combo box has " & cboItems.ListCount & " items"
End Sub
`,
    },
    "textbox-change-event": {
        name: "TextBox Change Event",
        code: `VERSION 5.00
Begin VB.Form Form1
   Caption         =   "TextBox Change Test"
   ClientHeight    =   3000
   ClientWidth     =   5000
   StartUpPosition =   2  'CenterScreen
   Begin VB.Label lblLabel
      Caption         =   "Type something:"
      Height          =   375
      Left            =   240
      TabIndex        =   1
      Top             =   240
      Width           =   2000
   End
   Begin VB.TextBox Text1
      Height          =   375
      Left            =   240
      TabIndex        =   2
      Top             =   720
      Width           =   3500
   End
   Begin VB.Label lblCharCount
      Caption         =   "Character count: 0"
      Height          =   375
      Left            =   240
      TabIndex        =   3
      Top             =   1400
      Width           =   3500
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False
Private Sub Text1_Change()
    lblCharCount.Caption = "Character count: " & Str(Len(Text1.Text))
    Debug.Print "Text changed to: " & Text1.Text
End Sub
`,
    }
};

export function getExample(id) {
    return examples[id] ?? null;
}

export function getDefaultExample() {
    return examples["hello-world"];
}