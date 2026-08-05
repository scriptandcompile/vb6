/**
 * VB6Semantic Playground - Example Code Snippets
 *
 * Example VB6 code snippets for each file type that the semantic analyzer
 * supports. Each example is chosen to exercise the scope hierarchy and the
 * symbol/type model (variables, constants, types, enums, procedures, and
 * properties).
 */

export const examples = {
    'simple-module': {
        name: 'Simple Module',
        fileType: 'module',
        code: `VERSION 1.0 CLASS
Attribute VB_Name = "MathUtils"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = False
Attribute VB_Exposed = False

Option Explicit

Public Const PI As Double = 3.14159

Public Type Point
    X As Double
    Y As Double
End Type

Public Function Distance(ByVal p1 As Point, ByVal p2 As Point) As Double
    Dim dx As Double
    Dim dy As Double

    dx = p1.X - p2.X
    dy = p1.Y - p2.Y
    Distance = Sqr(dx * dx + dy * dy)
End Function

Public Function CircleArea(ByVal radius As Double) As Double
    CircleArea = PI * radius * radius
End Function
`
    },

    'type-and-enum': {
        name: 'Type & Enum Declarations',
        fileType: 'module',
        code: `VERSION 1.0 CLASS
Attribute VB_Name = "ShapeDefinitions"

Option Explicit

Public Enum ShapeType
    Circle = 0
    Square = 1
    Triangle = 2
End Enum

Public Type Shape
    Kind As ShapeType
    Label As String
    Dimensions() As Double
End Type

Private m_shapes() As Shape
Private m_count As Integer

Public Sub AddShape(ByVal kind As ShapeType, ByVal label As String)
    Dim idx As Integer
    idx = m_count

    ReDim Preserve m_shapes(0 To idx)
    m_shapes(idx).Kind = kind
    m_shapes(idx).Label = label

    m_count = m_count + 1
End Sub
`
    },

    'class-with-properties': {
        name: 'Class with Properties',
        fileType: 'class',
        code: `VERSION 1.0 CLASS
BEGIN
  MultiUse = -1  'True
  Persistable = 0  'NotPersistable
  DataBindingBehavior = 0  'vbNone
  DataSourceBehavior  = 0  'vbNone
  MTSTransactionMode  = 0  'NotAnMTSObject
END
Attribute VB_Name = "Person"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = True
Attribute VB_PredeclaredId = False
Attribute VB_Exposed = False

Option Explicit

Private m_Name As String
Private m_Age As Integer

Public Property Get Name() As String
    Name = m_Name
End Property

Public Property Let Name(ByVal value As String)
    m_Name = value
End Property

Public Property Get Age() As Integer
    Age = m_Age
End Property

Public Property Let Age(ByVal value As Integer)
    If value >= 0 And value <= 150 Then
        m_Age = value
    End If
End Property

Public Function GetInfo() As String
    GetInfo = m_Name & " is " & m_Age & " years old"
End Function
`
    },

    'form-with-controls': {
        name: 'Form with Controls',
        fileType: 'form',
        code: `VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Calculator"
   ClientHeight    =   3195
   ClientLeft      =   60
   ClientTop       =   405
   ClientWidth     =   4680
   LinkTopic       =   "Form1"
   ScaleHeight     =   3195
   ScaleWidth      =   4680
   StartUpPosition =   3  'Windows Default
   Begin VB.TextBox txtNumber1
      Height          =   495
      Left            =   1440
      TabIndex        =   0
      Top             =   360
      Width           =   1815
   End
   Begin VB.TextBox txtNumber2
      Height          =   495
      Left            =   1440
      TabIndex        =   1
      Top             =   1080
      Width           =   1815
   End
   Begin VB.CommandButton btnCalculate
      Caption         =   "Calculate"
      Height          =   495
      Left            =   1560
      TabIndex        =   2
      Top             =   2520
      Width           =   1575
   End
End
Attribute VB_Name = "Form1"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False

Option Explicit

Private Sub btnCalculate_Click()
    Dim num1 As Double
    Dim num2 As Double

    num1 = Val(txtNumber1.Text)
    num2 = Val(txtNumber2.Text)

    txtNumber1.Text = CStr(num1 + num2)
End Sub

Private Sub Form_Load()
    Me.Caption = "Simple Calculator"
End Sub
`
    }
};

/**
 * Get an example by ID
 * @param {string} exampleId - The example identifier
 * @returns {object|null} The example object or null if not found
 */
export function getExample(exampleId) {
    return examples[exampleId] || null;
}

/**
 * Get all example IDs
 * @returns {string[]} Array of example IDs
 */
export function getExampleIds() {
    return Object.keys(examples);
}
