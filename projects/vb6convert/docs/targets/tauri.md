# Tauri Conversion Backend

## Overview

The Tauri conversion backend transforms VB6 desktop applications into modern cross-platform desktop applications using the Tauri framework. Tauri combines a Rust backend with a web frontend (HTML/CSS/JavaScript), making it an excellent target for VB6 form-based applications.

## Architecture

```
┌─────────────────────────────────┐
│     VB6 Application             │
│  (Forms, Modules, Classes)      │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│    Tauri Application            │
├─────────────────────────────────┤
│  Frontend (Web Technologies)    │
│  - HTML (Form layouts)          │
│  - CSS (Styling)                │
│  - JavaScript (UI logic)        │
├─────────────────────────────────┤
│  IPC Bridge (Tauri Commands)    │
├─────────────────────────────────┤
│  Backend (Rust)                 │
│  - Business logic               │
│  - File operations              │
│  - Database access              │
│  - System integration           │
└─────────────────────────────────┘
```

## Conversion Strategy

### Forms → Frontend
- VB6 forms convert to HTML/CSS/JavaScript
- Control layouts preserved
- Events mapped to JavaScript handlers

### Modules/Classes → Backend
- Business logic converts to Rust
- Runs in native backend (better performance and security)
- Exposes APIs to frontend via Tauri commands

### Communication
- Frontend calls backend via Tauri's command system
- Backend emits events to frontend
- Type-safe IPC with serde

## Component Mapping

### VB6 to Tauri

| VB6 Component | Tauri Component | Notes |
|---------------|-----------------|-------|
| Form (.frm) | HTML page | Full conversion with styling |
| Controls | HTML elements | Native or custom components |
| Form code | JavaScript + Rust | UI logic in JS, business in Rust |
| Module (.bas) | Rust module | Backend logic |
| Class (.cls) | Rust struct/trait | Backend implementation |
| Project (.vbp) | Tauri app | Complete desktop application |

## Form Conversion

### VB6 Form

```vb6
' MainForm.frm
VERSION 5.00
Begin VB.Form MainForm 
   Caption         =   "My Application"
   ClientHeight    =   3000
   ClientWidth     =   4500
   Begin VB.CommandButton btnSave 
      Caption         =   "Save"
      Height          =   495
      Left            =   1680
      TabIndex        =   2
      Top             =   2280
      Width           =   1215
   End
   Begin VB.TextBox txtName 
      Height          =   375
      Left            =   1560
      TabIndex        =   1
      Top             =   600
      Width           =   2415
   End
   Begin VB.Label lblName 
      Caption         =   "Name:"
      Height          =   255
      Left            =   480
      Top             =   660
      Width           =   855
   End
End

Private Sub btnSave_Click()
    MsgBox "Saving: " & txtName.Text
    SaveToDatabase txtName.Text
End Sub

Private Sub SaveToDatabase(name As String)
    ' Save to database logic
End Sub
```

### Converted Tauri Application

#### Frontend (HTML)

```html
<!-- src/index.html -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>My Application</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div class="main-form">
        <h1>My Application</h1>
        
        <div class="form-group">
            <label for="txtName" class="label">Name:</label>
            <input type="text" id="txtName" class="textbox">
        </div>
        
        <button id="btnSave" class="button">Save</button>
    </div>
    
    <script src="main.js"></script>
</body>
</html>
```

#### Frontend (CSS)

```css
/* src/styles.css */
.main-form {
    width: 450px;
    height: 300px;
    padding: 20px;
    font-family: Arial, sans-serif;
}

.form-group {
    margin: 20px 0;
    display: flex;
    align-items: center;
}

.label {
    width: 100px;
    margin-right: 10px;
}

.textbox {
    flex: 1;
    height: 37px;
    padding: 5px;
    border: 1px solid #ccc;
}

.button {
    width: 121px;
    height: 49px;
    margin-left: 168px;
    margin-top: 50px;
    cursor: pointer;
    font-size: 14px;
}
```

#### Frontend (JavaScript)

```javascript
// src/main.js
const { invoke } = window.__TAURI__.tauri;

document.getElementById('btnSave').addEventListener('click', async () => {
    const name = document.getElementById('txtName').value;
    
    try {
        await invoke('save_to_database', { name });
        await invoke('show_message', { 
            message: `Saving: ${name}` 
        });
    } catch (error) {
        console.error('Error saving:', error);
    }
});
```

#### Backend (Rust)

```rust
// src-tauri/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
async fn save_to_database(name: String) -> Result<(), String> {
    // Implement database save logic
    println!("Saving to database: {}", name);
    
    // Simulate database operation
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    Ok(())
}

#[tauri::command]
async fn show_message(message: String) -> Result<(), String> {
    // Cross-platform message box implementation
    // Could use native dialog or web-based dialog
    println!("Message: {}", message);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_to_database,
            show_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Control Mapping

### Standard Controls

| VB6 Control | HTML Element | Notes |
|-------------|--------------|-------|
| Label | `<label>` or `<span>` | Static text |
| TextBox | `<input type="text">` | Single-line text |
| TextBox (multiline) | `<textarea>` | Multi-line text |
| CommandButton | `<button>` | Clickable button |
| CheckBox | `<input type="checkbox">` | Boolean input |
| OptionButton | `<input type="radio">` | Radio button |
| ComboBox | `<select>` | Dropdown list |
| ListBox | `<select multiple>` | Multi-select list |
| Frame | `<fieldset>` | Grouping container |
| PictureBox | `<img>` or `<canvas>` | Image display |
| Image | `<img>` | Image display |
| Timer | `setInterval()` | Periodic execution |
| ScrollBar | `<input type="range">` | Slider control |

### Advanced Controls

| VB6 Control | Web Alternative | Library |
|-------------|-----------------|---------|
| ListView | Custom component | ag-Grid, React Table |
| TreeView | Custom component | react-treeview |
| ProgressBar | `<progress>` | Native HTML5 |
| TabStrip | Tab component | Bootstrap, Material-UI |
| Toolbar | Div with buttons | Custom or framework |
| StatusBar | Footer div | Custom styling |
| DataGrid | Table component | ag-Grid, Handsontable |
| Calendar | Date picker | react-datepicker |

## Event Handling

### VB6 Events

```vb6
Private Sub txtName_Change()
    ' Handle text change
End Sub

Private Sub btnSave_Click()
    ' Handle button click
End Sub

Private Sub Form_Load()
    ' Handle form load
End Sub

Private Sub Form_Unload(Cancel As Integer)
    ' Handle form close
End Sub
```

### Tauri Events (JavaScript)

```javascript
// Input change event
document.getElementById('txtName').addEventListener('input', (e) => {
    console.log('Text changed:', e.target.value);
});

// Button click event
document.getElementById('btnSave').addEventListener('click', async () => {
    await handleSave();
});

// Window load event
window.addEventListener('DOMContentLoaded', () => {
    initializeForm();
});

// Window close event
window.addEventListener('beforeunload', (e) => {
    // Confirm close if needed
    if (hasUnsavedChanges()) {
        e.preventDefault();
        e.returnValue = '';
    }
});
```

### Tauri Events (Rust Backend)

```rust
use tauri::{App, Manager, Window};

#[tauri::command]
async fn initialize_form(window: Window) -> Result<(), String> {
    // Form initialization logic
    Ok(())
}

#[tauri::command]
async fn before_close(window: Window) -> Result<bool, String> {
    // Check for unsaved changes
    Ok(true) // return false to cancel close
}

fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();
    
    // Listen for window events
    window.on_window_event(|event| match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            // Handle close request
        }
        _ => {}
    });
    
    Ok(())
}
```

## Custom Controls

For VB6 UserControls or ActiveX controls, create custom web components:

```javascript
// Custom control as web component
class CustomButton extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = `
            <style>
                button {
                    /* Custom styling */
                }
            </style>
            <button>${this.getAttribute('caption') || 'Button'}</button>
        `;
        
        this.shadowRoot.querySelector('button')
            .addEventListener('click', () => {
                this.dispatchEvent(new CustomEvent('customClick'));
            });
    }
}

customElements.define('custom-button', CustomButton);
```

## Database Access

### VB6 ADO Database Access

```vb6
Dim conn As ADODB.Connection
Dim rs As ADODB.Recordset

Set conn = New ADODB.Connection
conn.ConnectionString = "Provider=SQLOLEDB;Data Source=localhost;..."
conn.Open

Set rs = New ADODB.Recordset
rs.Open "SELECT * FROM Users", conn

While Not rs.EOF
    Debug.Print rs!Name
    rs.MoveNext
Wend
```

### Tauri Backend Database Access (Rust)

```rust
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[tauri::command]
async fn get_users() -> Result<Vec<User>, String> {
    let conn = Connection::open("database.db")
        .map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare("SELECT id, name FROM users")
        .map_err(|e| e.to_string())?;
    
    let users = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(users)
}
```

### Frontend Database Usage

```javascript
// Call backend database function
const users = await invoke('get_users');
console.log('Users:', users);

// Display in UI
const userList = document.getElementById('userList');
users.forEach(user => {
    const li = document.createElement('li');
    li.textContent = user.name;
    userList.appendChild(li);
});
```

## File Operations

### VB6 File Operations

```vb6
' Read file
Open "C:\data.txt" For Input As #1
Dim content As String
content = Input$(LOF(1), #1)
Close #1

' Write file
Open "C:\data.txt" For Output As #1
Print #1, "Hello World"
Close #1
```

### Tauri File Operations (Rust)

```rust
use std::fs;
use tauri::api::dialog;

#[tauri::command]
async fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn write_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn open_file_dialog() -> Result<Option<String>, String> {
    let file_path = dialog::blocking::FileDialogBuilder::new()
        .pick_file();
    
    Ok(file_path.map(|p| p.to_string_lossy().to_string()))
}
```

## Project Structure

```
converted-tauri-app/
├── package.json
├── src/                        # Frontend
│   ├── index.html
│   ├── styles.css
│   ├── main.js
│   ├── forms/                  # Individual forms
│   │   ├── main-form.html
│   │   └── settings-form.html
│   └── assets/                 # Images, icons, etc.
│       └── logo.png
│
├── src-tauri/                  # Backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── icons/
│   │   └── icon.png
│   └── src/
│       ├── main.rs             # Entry point
│       ├── commands/           # Tauri commands
│       │   ├── mod.rs
│       │   ├── database.rs
│       │   └── file_ops.rs
│       ├── modules/            # Converted VB6 modules
│       │   └── module1.rs
│       └── classes/            # Converted VB6 classes
│           └── person.rs
│
└── README.md
```

### Generated tauri.conf.json

```json
{
  "build": {
    "beforeDevCommand": "",
    "beforeBuildCommand": "",
    "devPath": "../src",
    "distDir": "../src"
  },
  "package": {
    "productName": "My Converted VB6 App",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": true,
        "scope": ["$APPDATA/*"]
      },
      "dialog": {
        "all": true
      },
      "window": {
        "all": false,
        "close": true,
        "hide": true,
        "show": true,
        "maximize": true,
        "minimize": true,
        "setTitle": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.example.converted-vb6-app",
      "icon": [
        "icons/icon.png"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 600,
        "resizable": true,
        "title": "My Converted VB6 App",
        "width": 800
      }
    ]
  }
}
```

## Advantages of Tauri for VB6 Conversion

1. **Native Performance**: Rust backend provides near-native performance
2. **Security**: Tauri's security model is more robust than Electron
3. **Small Binary Size**: Much smaller than Electron applications
4. **Cross-Platform**: Single codebase for Windows, macOS, Linux
5. **Modern UI**: Leverage modern web technologies for UI
6. **Native APIs**: Access to system APIs through Rust
7. **Easy Updates**: Web-based UI can be updated easily

## Implementation Checklist

- [ ] Form layout conversion (HTML)
- [ ] Form styling conversion (CSS)
- [ ] Control mapping
- [ ] Event handler conversion
- [ ] Module conversion to Rust
- [ ] Class conversion to Rust
- [ ] Database access layer
- [ ] File operations
- [ ] IPC command generation
- [ ] Menu conversion
- [ ] Toolbar conversion
- [ ] Status bar conversion
- [ ] Icon and resource handling
- [ ] MDI form support (if needed)
- [ ] Print functionality
- [ ] Report generation
- [ ] Settings/configuration
- [ ] Installer generation

## Testing Strategy

1. Test each form individually
2. Test IPC communication
3. Test backend logic independently
4. Test on all target platforms
5. UI/UX testing for consistency
6. Performance testing
7. Security testing
8. End-to-end integration tests

## Future Enhancements

- [ ] Hot reload for development
- [ ] Automatic updates
- [ ] Crash reporting
- [ ] Analytics integration
- [ ] Plugin system
- [ ] Theme support
- [ ] Internationalization
- [ ] Accessibility features
