# Cross-Cutting Library Example: DataGrid with Database

This document illustrates how a library can be a cross-cutting concern that affects both backend and frontend code generation.

## VB6 Original Code

```vb
' Form1.frm
Private Sub Form_Load()
    Dim conn As ADODB.Connection
    Dim rs As ADODB.Recordset
    
    ' Backend: Database connection and query
    Set conn = New ADODB.Connection
    conn.Open "Provider=SQLOLEDB;Data Source=localhost;Initial Catalog=mydb;User ID=sa;Password=pass"
    
    Set rs = New ADODB.Recordset
    rs.Open "SELECT CustomerID, CompanyName, ContactName, City FROM Customers ORDER BY CompanyName", conn
    
    ' Frontend: Bind data to grid
    Set DataGrid1.DataSource = rs
    
    ' Configure grid columns
    With DataGrid1
        .Columns(0).Caption = "ID"
        .Columns(0).Width = 500
        .Columns(1).Caption = "Company"
        .Columns(1).Width = 2000
        .Columns(2).Caption = "Contact"
        .Columns(2).Width = 1500
        .Columns(3).Caption = "City"
        .Columns(3).Width = 1000
    End With
End Sub

Private Sub DataGrid1_DblClick()
    If Not IsNull(DataGrid1.Bookmark) Then
        Dim customerId As String
        customerId = DataGrid1.Columns(0).Value
        MsgBox "Selected Customer ID: " & customerId
    End If
End Sub
```

## Library Detection

```rust
use vb6libraries::detection::LibraryUsage;

let usage = detect_libraries(&project)?;

// Detects:
// - ADO database usage (ADODB.Connection, ADODB.Recordset)
// - DataGrid UI control
// - SQL Server connection string
```

## Generated Architecture

### Backend (Rust)

**src/backend/database.rs:**
```rust
use sqlx::{SqlServer, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub customer_id: String,
    pub company_name: String,
    pub contact_name: String,
    pub city: String,
}

pub async fn get_customers() -> Result<Vec<Customer>, sqlx::Error> {
    let pool = get_database_pool().await?;
    
    let customers = sqlx::query_as!(
        Customer,
        "SELECT CustomerID, CompanyName, ContactName, City 
         FROM Customers 
         ORDER BY CompanyName"
    )
    .fetch_all(&pool)
    .await?;
    
    Ok(customers)
}
```

**src/backend/api.rs:**
```rust
use axum::{Router, Json};
use axum::routing::get;

// API endpoint for frontend
pub async fn get_customers_handler() -> Json<Vec<Customer>> {
    match get_customers().await {
        Ok(customers) => Json(customers),
        Err(e) => {
            eprintln!("Database error: {}", e);
            Json(vec![])
        }
    }
}

pub fn customer_routes() -> Router {
    Router::new()
        .route("/api/customers", get(get_customers_handler))
}
```

### Frontend (TypeScript + React)

**src/frontend/types/Customer.ts:**
```typescript
export interface Customer {
  customerId: string;
  companyName: string;
  contactName: string;
  city: string;
}
```

**src/frontend/api/customerApi.ts:**
```typescript
import { Customer } from '../types/Customer';

export async function getCustomers(): Promise<Customer[]> {
  const response = await fetch('/api/customers');
  if (!response.ok) {
    throw new Error('Failed to fetch customers');
  }
  return response.json();
}
```

**src/frontend/components/CustomerGrid.tsx:**
```typescript
import React, { useEffect, useState } from 'react';
import { DataGrid, GridColDef } from '@mui/x-data-grid';
import { Customer } from '../types/Customer';
import { getCustomers } from '../api/customerApi';

const columns: GridColDef[] = [
  { field: 'customerId', headerName: 'ID', width: 100 },
  { field: 'companyName', headerName: 'Company', width: 300 },
  { field: 'contactName', headerName: 'Contact', width: 200 },
  { field: 'city', headerName: 'City', width: 150 },
];

export function CustomerGrid() {
  const [customers, setCustomers] = useState<Customer[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadCustomers();
  }, []);

  async function loadCustomers() {
    try {
      const data = await getCustomers();
      setCustomers(data);
    } catch (error) {
      console.error('Error loading customers:', error);
    } finally {
      setLoading(false);
    }
  }

  function handleRowDoubleClick(params: any) {
    const customerId = params.row.customerId;
    alert(`Selected Customer ID: ${customerId}`);
  }

  return (
    <DataGrid
      rows={customers}
      columns={columns}
      loading={loading}
      onRowDoubleClick={handleRowDoubleClick}
      getRowId={(row) => row.customerId}
      autoHeight
    />
  );
}
```

**src/frontend/pages/Form1.tsx:**
```typescript
import React from 'react';
import { CustomerGrid } from '../components/CustomerGrid';

export function Form1() {
  return (
    <div className="form-container">
      <h1>Customers</h1>
      <CustomerGrid />
    </div>
  );
}
```

## vb6libraries Coordination

### ADO Mapper (Backend)

**vb6libraries::database::ado:**

```rust
pub struct AdoMapper;

impl ApiMapper for AdoMapper {
    fn map_call(&self, api_name: &str, args: &[String]) -> Option<String> {
        match api_name {
            "ADODB.Connection.Open" => {
                Some(format!("sqlx::connect(\"{}\").await?", args[0]))
            }
            "ADODB.Recordset.Open" => {
                Some(format!("sqlx::query(\"{}\").fetch_all(&pool).await?", args[0]))
            }
            _ => None
        }
    }
    
    fn get_imports(&self) -> Vec<String> {
        vec![
            "use sqlx::{{SqlServer, Row}};".to_string(),
            "use serde::{{Serialize, Deserialize}};".to_string(),
        ]
    }
}

impl CodegenHook for AdoMapper {
    fn affects_backend(&self) -> bool { true }
    fn affects_frontend(&self) -> bool { true }  // API types
    
    fn generate_init(&self) -> Option<String> {
        Some(r#"
async fn get_database_pool() -> Result<SqlxPool, sqlx::Error> {
    static POOL: OnceCell<SqlxPool> = OnceCell::new();
    POOL.get_or_try_init(|| async {
        SqlxPoolOptions::new()
            .max_connections(5)
            .connect("sqlserver://localhost/mydb").await
    }).await.cloned()
}
        "#.to_string())
    }
}
```

### DataGrid Mapper (Frontend)

**vb6libraries::ui::datagrid:**

```rust
pub struct DataGridMapper;

impl ApiMapper for DataGridMapper {
    fn map_call(&self, api_name: &str, args: &[String]) -> Option<String> {
        match api_name {
            "DataGrid.DataSource" => {
                // Generate frontend component with data binding
                Some(format!("<DataGrid rows={{{}}} columns={{columns}} />", args[0]))
            }
            _ => None
        }
    }
    
    fn get_imports(&self) -> Vec<String> {
        vec![
            "import { DataGrid, GridColDef } from '@mui/x-data-grid';".to_string(),
        ]
    }
}

impl CrossCuttingLibrary for DataGridMapper {
    fn generate_backend(&self, ctx: &BackendContext) -> String {
        // Generate API endpoint for data
        r#"
pub async fn get_grid_data() -> Json<Vec<RowData>> {
    // ... query database ...
}
        "#.to_string()
    }
    
    fn generate_frontend(&self, ctx: &FrontendContext) -> String {
        match ctx.framework {
            FrontendFramework::React => {
                // Generate React component
                r#"
export function DataGrid({ endpoint }: Props) {
    const [rows, setRows] = useState([]);
    useEffect(() => {
        fetch(endpoint).then(r => r.json()).then(setRows);
    }, [endpoint]);
    return <MuiDataGrid rows={rows} />;
}
                "#.to_string()
            }
            FrontendFramework::Vue => {
                // Generate Vue component
                // ...
                String::new()
            }
            _ => String::new()
        }
    }
}
```

## Code Generation Flow

1. **Detection Phase**: Analyze VB6 code, detect ADO + DataGrid usage
2. **Backend Generation**:
   - ADO mapper generates database query functions
   - ADO mapper generates API endpoints
   - Generates shared data types (Customer struct/interface)
3. **Frontend Generation**:
   - DataGrid mapper generates React DataGrid component
   - Generates API client code
   - Binds component to API endpoint
4. **Integration**:
   - Backend exposes REST API: `GET /api/customers`
   - Frontend calls API and renders in DataGrid
   - Types are shared via generated TypeScript definitions

## Benefits

- **Separation of Concerns**: Backend handles data, frontend handles UI
- **Type Safety**: Shared types ensure consistency
- **Modern Architecture**: REST API, async/await, reactive UI
- **Maintainable**: Clear structure, easy to modify
- **Scalable**: Can add caching, pagination, filtering independently

## Feature Flags Required

```toml
[dependencies]
vb6libraries = { 
    path = "../vb6libraries", 
    features = ["ado", "sql-server", "datagrid"] 
}

vb6codegen = { 
    path = "../vb6codegen",
    features = ["rust-backend", "react-frontend", "database-support", "ui-controls"]
}
```
