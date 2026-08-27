# vb6libraries

VB6 third-party library and Windows API integration hooks for code generation and compilation.

## Implementation Status: PLANNING

> This crate is in the planning phase. The documentation below describes
> the intended design. The actual crate only has `src/lib.rs` and all described
> types (`Win32ApiMapper`, `ExcelMapper`, `LibraryAnalyzer`, `LibraryLinker`)
> are not yet implemented.

## Overview

`vb6libraries` provides modular support for common VB6 third-party libraries, Windows API calls, Office automation, and database integrations. This crate is used by `vb6codegen`, `vb6convert`, and `vb6compile` to handle library-specific code generation.

**Note:** All described functionality is planned but not yet implemented.

### Purpose

VB6 applications commonly use:
- **Win32 API**: Direct Windows API calls (GetWindowText, SendMessage, registry, etc.)
- **UI Libraries**: Common controls (ListView, TreeView, DataGrid, RichTextBox)
- **Office Automation**: Excel, Word, Outlook, Access automation
- **Database Libraries**: DAO, ADO, ODBC for SQL Server, Oracle, MySQL, Access
- **Third-Party Controls**: Crystal Reports, DevExpress, Infragistics, ComponentOne

This crate provides:
- **Library Detection**: Identify which libraries a VB6 project uses
- **API Mapping**: Map VB6 library calls to modern equivalents
- **Code Generation Hooks**: Generate appropriate code for each library
- **Cross-Cutting Support**: Handle libraries that affect both backend and frontend

## Architecture

**Note:** The architecture and directory structure below describe the planned design.

```
```
┌─────────────────────────────────────────────┐
│          vb6libraries                       │
│                                             │
│  ┌────────────────────────────────────────┐ │
│  │  Library Traits & Detection            │ │
│  └────────────────────────────────────────┘ │
│                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Win32   │  │ Office   │  │ Database │   │
│  │   APIs   │  │Automation│  │ Libraries│   │
│  └──────────┘  └──────────┘  └──────────┘   │
│                                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │   UI     │  │  Third   │  │  Common  │   │
│  │Libraries │  │  Party   │  │ Controls │   │
│  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────┘
         │              │              │
         ▼              ▼              ▼
    ┌──────────┐  ┌──────────┐  ┌──────────┐
    │vb6codegen│  │vb6convert│  │vb6compile│
    └──────────┘  └──────────┘  └──────────┘
```

## Library Categories

*(All categories below are planned but not yet implemented.)*

### 1. Windows APIs (`win32-*`)

Direct Windows API integration:

```toml
[dependencies]
vb6libraries = { path = "../vb6libraries", features = ["win32-core", "win32-gdi"] }
```

**Features:**
- `win32-core`: Core APIs (GetWindowText, SendMessage, PostMessage, etc.)
- `win32-gdi`: Graphics Device Interface (BitBlt, CreateDC, etc.)
- `win32-shell`: Shell APIs (SHBrowseForFolder, ShellExecute, etc.)
- `win32-registry`: Registry access (RegOpenKey, RegQueryValue, etc.)
- `win32-networking`: Network APIs (WinSock, WinHTTP, etc.)

**VB6 Example:**
```vb
Declare Function GetWindowText Lib "user32" Alias "GetWindowTextA" _
    (ByVal hwnd As Long, ByVal lpString As String, ByVal cch As Long) As Long
```

**Generated Rust (via vb6codegen):**
```rust
use windows::Win32::UI::WindowsAndMessaging::GetWindowTextA;
```

### 2. Common UI Libraries

Standard VB6 UI controls:

```toml
[dependencies]
vb6libraries = { path = "../vb6libraries", features = ["mscomctl", "datagrid"] }
```

**Features:**
- `mscomctl`: ListView, TreeView, TabStrip, Toolbar, StatusBar, ProgressBar, ImageList
- `mscomctl2`: MonthView, DateTimePicker, UpDown, Animation
- `richtextbox`: Rich text editing control
- `datagrid`: DataGrid, MSFlexGrid, MSHFlexGrid
- `webview`: WebBrowser control

**Conversion Strategy:**
- **Backend**: Data structures remain in Rust
- **Frontend**: Map to modern UI equivalents (React Table, TreeView components, etc.)

### 3. Office Automation

COM automation for Office applications:

```toml
[dependencies]
vb6libraries = { path = "../vb6libraries", features = ["excel-automation"] }
```

**Features:**
- `office-automation`: Base Office COM support
- `excel-automation`: Excel object model
- `word-automation`: Word object model
- `outlook-automation`: Outlook object model
- `access-automation`: Access object model

**VB6 Example:**
```vb
Dim xlApp As Excel.Application
Set xlApp = CreateObject("Excel.Application")
xlApp.Workbooks.Open "data.xlsx"
```

**Generated Rust (via vb6codegen):**
```rust
use vb6libraries::office::excel::ExcelApp;
let xl_app = ExcelApp::new()?;
xl_app.workbooks().open("data.xlsx")?;
```

### 4. Database Libraries

Database access layers:

```toml
[dependencies]
vb6libraries = { path = "../vb6libraries", features = ["ado", "sql-server"] }
```

**Features:**
- `dao`: Data Access Objects (Jet/Access)
- `ado`: ActiveX Data Objects (universal)
- `rdo`: Remote Data Objects (legacy)
- `odbc`: Direct ODBC access

**Vendor-Specific:**
- `sql-server`: SQL Server optimizations
- `oracle-db`: Oracle-specific features
- `mysql-db`: MySQL support
- `postgresql-db`: PostgreSQL support
- `access-db`: MS Access local databases

**VB6 Example:**
```vb
Dim conn As ADODB.Connection
Set conn = New ADODB.Connection
conn.Open "Provider=SQLOLEDB;Data Source=localhost;Initial Catalog=mydb"
```

**Generated Rust (via vb6codegen):**
```rust
use vb6libraries::database::ado::Connection;
let conn = Connection::new()?;
conn.open("sqlserver://localhost/mydb")?;
```

### 5. Third-Party Controls

Commercial UI control libraries:

```toml
[dependencies]
vb6libraries = { path = "../vb6libraries", features = ["crystalreports", "devexpress"] }
```

**Features:**
- `crystalreports`: Crystal Reports integration
- `activereports`: Active Reports
- `devexpress`: DevExpress control suite
- `infragistics`: Infragistics controls
- `componentone`: ComponentOne Studio

**Strategy**: Map to modern equivalents or provide compatibility layer.

## Usage

### In vb6codegen

**Note:** The API below is planned. Types `Win32ApiMapper`, `ExcelMapper`, `AdoMapper` are not yet implemented.

```rust
```
use vb6libraries::win32::Win32ApiMapper;
use vb6libraries::office::ExcelMapper;
use vb6libraries::database::AdoMapper;

// Detect which libraries are used
let libraries = detect_libraries(&vb6_project)?;

// Configure code generation
if libraries.uses_win32_apis() {
    codegen.add_mapper(Win32ApiMapper::new());
}

if libraries.uses_excel() {
    codegen.add_mapper(ExcelMapper::new());
}

if libraries.uses_ado() {
    codegen.add_mapper(AdoMapper::new());
}
```

### In vb6convert

**Note:** `LibraryAnalyzer` is not yet implemented.

```rust
```
use vb6libraries::analysis::LibraryAnalyzer;

// Analyze project for library usage
let analyzer = LibraryAnalyzer::new();
let report = analyzer.analyze(&project)?;

println!("Win32 API Calls: {}", report.win32_calls.len());
println!("Office Automation: {:?}", report.office_usage);
println!("Database Libraries: {:?}", report.database_libs);

// Recommend features for conversion
let features = report.recommend_features();
println!("Suggested Cargo features: {:?}", features);
```

### In vb6compile

**Note:** `LibraryLinker` is not yet implemented.

```rust
```
use vb6libraries::linking::LibraryLinker;

// Link against required libraries
let linker = LibraryLinker::new();
linker.add_library("user32")?;  // For Win32 APIs
linker.add_library("ole32")?;   // For Office automation
linker.link_database("sqlserver")?;  // For SQL Server
```

## Cross-Cutting Concerns

Some libraries affect both backend and frontend:

### Example: DataGrid with Database

**VB6:**
```vb
' Backend: Database query
Dim rs As ADODB.Recordset
Set rs = conn.Execute("SELECT * FROM customers")

' Frontend: Display in grid
Set DataGrid1.DataSource = rs
```

**Generated Architecture:**

**Backend (Rust):**
```rust
pub async fn get_customers() -> Result<Vec<Customer>> {
    let conn = get_connection().await?;
    conn.query("SELECT * FROM customers").await
}
```

**Frontend (TypeScript/React):**
```typescript
const [customers, setCustomers] = useState<Customer[]>([]);

useEffect(() => {
    api.getCustomers().then(setCustomers);
}, []);

<DataGrid data={customers} />
```

**Library Hook Coordination:**
- `vb6libraries::database::ado` generates backend query code
- `vb6libraries::ui::datagrid` generates frontend table component
- Both coordinate through generated API layer

## Feature Flags

Use feature flags to include only what you need:

```toml
```

## Implementation Status

| Category | Status | Notes |
|----------|--------|-------|
| Win32 APIs | 🚧 Planned | Core, GDI, Shell, Registry |
| Common Controls | 🚧 Planned | ListView, TreeView, DataGrid |
| Office Automation | 🚧 Planned | Excel, Word, Outlook |
| Database (ADO) | 🚧 Planned | SQL Server, Oracle, MySQL |
| Database (DAO) | 🚧 Planned | Access integration |
| Third-Party | 🚧 Planned | Crystal Reports, DevExpress |

## Directory Structure

**Note:** The directory structure below describes the planned layout. Currently only `src/lib.rs` exists.

```
```
vb6libraries/
├── src/
│   ├── lib.rs              # Public API
│   ├── detection.rs        # Library detection
│   ├── traits.rs           # Common traits
│   ├── win32/
│   │   ├── mod.rs
│   │   ├── core.rs         # Core Win32 APIs
│   │   ├── gdi.rs          # Graphics APIs
│   │   ├── shell.rs        # Shell APIs
│   │   └── registry.rs     # Registry APIs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── mscomctl.rs     # Common controls
│   │   ├── datagrid.rs     # Grid controls
│   │   └── richtextbox.rs  # Rich text
│   ├── office/
│   │   ├── mod.rs
│   │   ├── excel.rs        # Excel automation
│   │   ├── word.rs         # Word automation
│   │   └── outlook.rs      # Outlook automation
│   ├── database/
│   │   ├── mod.rs
│   │   ├── dao.rs          # Data Access Objects
│   │   ├── ado.rs          # ActiveX Data Objects
│   │   └── vendors/
│   │       ├── sqlserver.rs
│   │       ├── oracle.rs
│   │       └── access.rs
│   └── thirdparty/
│       ├── mod.rs
│       ├── crystalreports.rs
│       └── devexpress.rs
├── docs/
│   ├── WIN32_APIS.md       # Win32 API mapping
│   ├── OFFICE_AUTOMATION.md # Office integration
│   ├── DATABASE_LIBRARIES.md # Database access
│   └── UI_CONTROLS.md       # UI control mapping
└── tests/
    └── integration/
```

## See Also

- [vb6codegen](../vb6codegen) - Code generation framework
- [vb6convert](../vb6convert) - Conversion tool
- [vb6compile](../vb6compile) - Compilation tool
