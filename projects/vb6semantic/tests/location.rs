use vb6parse::files::{ClassFile, FormFile, ModuleFile};
use vb6parse::io::SourceFile;
use vb6semantic::{SemanticAnalyzer, symbols::SymbolKind};

/// Asserts that symbols with the given (name, kind) exist at the given file-absolute line.
fn assert_locations(analyzer: &SemanticAnalyzer, expected: &[(&str, SymbolKind, usize)]) {
    let mut actual: Vec<(&str, SymbolKind, usize)> = Vec::new();
    for scope in analyzer.scope_manager().all_scopes() {
        for symbol in scope.symbols.values() {
            actual.push((
                symbol.name.as_str(),
                symbol.kind.clone(),
                symbol.location.line,
            ));
        }
    }

    for (name, kind, line) in expected {
        let found = actual
            .iter()
            .any(|(n, k, l)| n == name && k == kind && l == line);
        assert!(
            found,
            "expected symbol {:?} ({:?}) at line {} but got: {:?}",
            name, kind, line, actual
        );
    }
}

#[test]
fn module_locations_are_file_absolute() {
    let code = "Attribute VB_Name = \"MathUtils\"

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
";
    let source = SourceFile::from_string("MathUtils.bas", code);
    let (module_opt, failures) = ModuleFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {:?}", failures);
    let module = module_opt.expect("module should parse");
    assert_eq!(module.line_offset, 1);

    let mut analyzer = SemanticAnalyzer::new();
    analyzer
        .analyze_module(&module)
        .expect("analysis should succeed");

    assert_locations(
        &analyzer,
        &[
            ("PI", SymbolKind::Constant, 5),
            ("Point", SymbolKind::UserType, 7),
            ("X", SymbolKind::TypeMember, 8),
            ("Y", SymbolKind::TypeMember, 9),
            ("Distance", SymbolKind::Function, 12),
            ("p1", SymbolKind::Parameter, 12),
            ("p2", SymbolKind::Parameter, 12),
        ],
    );
}

#[test]
fn module_locations_without_header_are_file_absolute() {
    let code = "Option Explicit

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
";
    let source = SourceFile::from_string("MathUtils.bas", code);
    let (module_opt, _) = ModuleFile::parse(&source).unpack();
    let module = module_opt.expect("module should parse");
    assert_eq!(module.line_offset, 0);

    let mut analyzer = SemanticAnalyzer::new();
    analyzer
        .analyze_module(&module)
        .expect("analysis should succeed");

    assert_locations(
        &analyzer,
        &[
            ("PI", SymbolKind::Constant, 3),
            ("Point", SymbolKind::UserType, 5),
            ("X", SymbolKind::TypeMember, 6),
            ("Y", SymbolKind::TypeMember, 7),
            ("Distance", SymbolKind::Function, 10),
            ("p1", SymbolKind::Parameter, 10),
            ("p2", SymbolKind::Parameter, 10),
        ],
    );
}

#[test]
fn module_locations_with_version_header_are_file_absolute() {
    let code = "VERSION 1.0 CLASS
Attribute VB_Name = \"MathUtils\"
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
";
    let source = SourceFile::from_string("MathUtils.bas", code);
    let (module_opt, _) = ModuleFile::parse(&source).unpack();
    let module = module_opt.expect("module should parse");
    assert_eq!(module.line_offset, 5);

    let mut analyzer = SemanticAnalyzer::new();
    analyzer
        .analyze_module(&module)
        .expect("analysis should succeed");

    assert_locations(
        &analyzer,
        &[
            ("PI", SymbolKind::Constant, 10),
            ("Point", SymbolKind::UserType, 12),
            ("X", SymbolKind::TypeMember, 13),
            ("Y", SymbolKind::TypeMember, 14),
            ("Distance", SymbolKind::Function, 17),
            ("p1", SymbolKind::Parameter, 17),
            ("p2", SymbolKind::Parameter, 17),
        ],
    );
}

#[test]
fn class_locations_are_file_absolute() {
    let code = "VERSION 1.0 CLASS
BEGIN
  MultiUse = -1  'True
  Persistable = 0  'NotPersistable
  DataBindingBehavior = 0  'vbNone
  DataSourceBehavior  = 0  'vbNone
  MTSTransactionMode  = 0  'NotAnMTSObject
END
Attribute VB_Name = \"Person\"
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

Public Function GetInfo() As String
    GetInfo = m_Name & \" is \" & m_Age & \" years old\"
End Function
";
    let source = SourceFile::from_string("Person.cls", code);
    let (class_opt, failures) = ClassFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {:?}", failures);
    let class = class_opt.expect("class should parse");
    assert_eq!(class.line_offset, 13);

    let mut analyzer = SemanticAnalyzer::new();
    analyzer
        .analyze_class(&class)
        .expect("analysis should succeed");

    assert_locations(
        &analyzer,
        &[
            ("m_Name", SymbolKind::Variable, 17),
            ("m_Age", SymbolKind::Variable, 18),
            ("Name", SymbolKind::PropertyGet, 20),
            ("value", SymbolKind::Parameter, 24),
            ("GetInfo", SymbolKind::Function, 28),
        ],
    );
}

#[test]
fn form_locations_are_file_absolute() {
    let code = "VERSION 5.00
Begin VB.Form Form1
   Caption         =   \"Calculator\"
   ClientHeight    =   3195
   ClientLeft      =   60
   ClientTop       =   405
   ClientWidth     =   4680
   LinkTopic       =   \"Form1\"
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
End
Attribute VB_Name = \"Form1\"
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
    Me.Caption = \"Simple Calculator\"
End Sub
";
    let source = SourceFile::from_string("Form1.frm", code);
    let (form_opt, failures) = FormFile::parse(&source).unpack();
    assert!(failures.is_empty(), "parse failures: {:?}", failures);
    let form = form_opt.expect("form should parse");
    assert_eq!(form.line_offset, 25);

    let mut analyzer = SemanticAnalyzer::new();
    analyzer
        .analyze_form(&form)
        .expect("analysis should succeed");

    assert_locations(
        &analyzer,
        &[
            ("btnCalculate_Click", SymbolKind::SubProcedure, 28),
            ("Form_Load", SymbolKind::SubProcedure, 38),
        ],
    );
}
