/**
 * Embedded test forms for E2E testing (Step 11).
 *
 * Each test form is defined as a VB6 .frm source string that can be
 * loaded directly into the playground. They cover the key scenarios
 * described in step_by_step.md Step 11:
 *
 *   1. Simple form — Label + TextBox + CommandButton
 *   2. Form with containers — Frame + PictureBox with nested controls
 *   3. Form with Form_Load / Form_Unload events
 *   4. Form referencing a module with global procedures
 *   5. Full controls — ComboBox, ListBox, CheckBox, OptionButton, Shape
 *
 * These correspond to the real fixtures under
 * `projects/vb6runtime/tests/fixtures/` and the real-world forms in
 * `test-data/` (e.g. Bitrate-calculator, Levels-effect).
 */

export const testForms = {
    /**
     * Test form 1: Simple form with Label, TextBox, and CommandButton.
     * Equivalent to `simple.frm` fixture but with a TextBox added.
     */
    "simple-form": {
        name: "Simple Form (Label + TextBox + Button)",
        description: "A basic form with a label, text box, and command button. Tests basic control rendering and click event dispatch.",
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

    /**
     * Test form 2: Form with containers (Frame + PictureBox with nested controls).
     * Equivalent to `container.frm` fixture extended with a PictureBox.
     */
    "form-with-containers": {
        name: "Form with Containers (Frame + PictureBox)",
        description: "Tests nested controls inside Frame and PictureBox containers. Verifies the layout engine handles hierarchy correctly.",
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

    /**
     * Test form 3: Form with Form_Load and Form_Unload events.
     * Uses Debug.Print in form-level events to verify they execute.
     * Corresponds to real-world forms like frmMain.frm from Bitrate-calculator.
     */
    "form-with-events": {
        name: "Form with Form_Load / Form_Unload Events",
        description: "Tests form-level event handlers. Debug.Print in Form_Load should appear in the output panel when the form loads.",
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

    /**
     * Test form 4: Form with module reference (global procedures).
     * This form calls a global Sub from a module. The module defines
     * a public procedure that the form's button calls.
     */
    "form-with-module-reference": {
        name: "Form with Module Reference",
        description: "Tests form calling a global procedure from a module. Simulates the pattern where a form button calls a shared utility function.",
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
        moduleCode: `Attribute VB_Name = "DataModule"

Public Sub ProcessData(data As String)
    Dim result As String
    result = UCase(data)
    Debug.Print "Processed: " & result
    Debug.Print "Length: " & Str(Len(result))
End Sub
`,
    },

    /**
     * Test form 5: Full controls test.
     * Equivalent to `full_controls.frm` fixture with event handlers added.
     * Includes ComboBox, ListBox, CheckBox, OptionButton, Shape, Frame.
     */
    "full-controls": {
        name: "Full Controls Test",
        description: "Tests rendering of many VB6 control types: Label, CommandButton, Frame, CheckBox, OptionButton, PictureBox, Shape, ComboBox, ListBox.",
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
    ' Populate the combo box
    cboItems.AddItem "Item 1"
    cboItems.AddItem "Item 2"
    cboItems.AddItem "Item 3"
    Debug.Print "Combo box has " & cboItems.ListCount & " items"
End Sub
`,
    },

    /**
     * Test form 6: TextBox with Change event.
     * Tests real-time input handling via the Change event mapped to
     * the DOM 'input' event.
     */
    "textbox-change-event": {
        name: "TextBox Change Event",
        description: "Tests the TextBox Change event (mapped to DOM 'input' event). Each keystroke should trigger the handler.",
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
    },
};

export function getTestForm(id) {
    return testForms[id] ?? null;
}

export function getTestFormIds() {
    return Object.keys(testForms);
}

export function getTestFormNames() {
    return Object.entries(testForms).map(([id, form]) => ({ id, name: form.name }));
}
