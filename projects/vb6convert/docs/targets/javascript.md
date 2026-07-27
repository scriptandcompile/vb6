# JavaScript/TypeScript Conversion Backend

## Overview

The JavaScript/TypeScript conversion backend transforms VB6 projects into modern JavaScript or TypeScript code. TypeScript is preferred for better type safety and tooling support.

## Goals

1. **Modern JavaScript**: Use ES6+ features
2. **Type Safety**: Leverage TypeScript's type system
3. **Browser Compatibility**: Generate code that runs in modern browsers
4. **Node.js Support**: Support server-side execution
5. **Maintainability**: Produce clean, idiomatic code

## Type Mapping

### JavaScript (Untyped)

| VB6 Type | JavaScript Type | Notes |
|----------|-----------------|-------|
| Integer | Number | No distinction between integer types |
| Long | Number | |
| Single | Number | |
| Double | Number | |
| String | String | |
| Boolean | Boolean | |
| Byte | Number | |
| Currency | Number or BigDecimal | May need library for precision |
| Date | Date | |
| Variant | any type | JavaScript is dynamically typed |
| Object | Object | |
| Array | Array | |
| Nothing/Null | null | |
| Empty | undefined | |

### TypeScript (Typed)

| VB6 Type | TypeScript Type | Notes |
|----------|-----------------|-------|
| Integer | number | |
| Long | number | |
| Single | number | |
| Double | number | |
| String | string | |
| Boolean | boolean | |
| Byte | number | |
| Currency | Decimal | Use decimal.js library |
| Date | Date | |
| Variant | any | Use sparingly |
| Object | object or any | |
| Array | T[] or Array<T> | |
| Nothing/Null | null | |
| Empty | undefined | |
| User Type | interface or type | |
| Enum | enum | TypeScript native enum |

### Variant Type (TypeScript)

```typescript
type Variant = 
    | number
    | string
    | boolean
    | Date
    | object
    | null
    | undefined
    | Variant[];

// Or with discriminated union:
type VariantValue =
    | { type: 'number'; value: number }
    | { type: 'string'; value: string }
    | { type: 'boolean'; value: boolean }
    | { type: 'date'; value: Date }
    | { type: 'object'; value: object }
    | { type: 'null'; value: null }
    | { type: 'array'; value: Variant[] };
```

## Module Conversion

### VB6 Module

```vb6
' Module1.bas
Option Explicit

Public Const APP_NAME As String = "MyApp"
Private m_counter As Integer

Public Sub Initialize()
    m_counter = 0
End Sub

Public Function GetNextId() As Integer
    m_counter = m_counter + 1
    GetNextId = m_counter
End Function
```

### Converted JavaScript

```javascript
// module1.js

/** Application name constant */
export const APP_NAME = "MyApp";

/** Module-level counter */
let counter = 0;

/** Initialize the module */
export function initialize() {
    counter = 0;
}

/** Get next ID */
export function getNextId() {
    counter++;
    return counter;
}
```

### Converted TypeScript

```typescript
// module1.ts

/** Application name constant */
export const APP_NAME: string = "MyApp";

/** Module-level counter */
let counter: number = 0;

/** Initialize the module */
export function initialize(): void {
    counter = 0;
}

/** Get next ID */
export function getNextId(): number {
    counter++;
    return counter;
}
```

## Class Conversion

### VB6 Class

```vb6
' Person.cls
Option Explicit

Private m_name As String
Private m_age As Integer

Public Property Get Name() As String
    Name = m_name
End Property

Public Property Let Name(ByVal value As String)
    m_name = value
End Property

Public Property Get Age() As Integer
    Age = m_age
End Property

Public Property Let Age(ByVal value As Integer)
    If value >= 0 Then
        m_age = value
    End If
End Property

Public Sub Greet()
    MsgBox "Hello, " & m_name
End Sub
```

### Converted JavaScript (ES6 Class)

```javascript
// person.js

export class Person {
    constructor() {
        this._name = '';
        this._age = 0;
    }

    get name() {
        return this._name;
    }

    set name(value) {
        this._name = value;
    }

    get age() {
        return this._age;
    }

    set age(value) {
        if (value >= 0) {
            this._age = value;
        }
    }

    greet() {
        alert(`Hello, ${this._name}`);
    }
}
```

### Converted TypeScript

```typescript
// person.ts

export class Person {
    private _name: string = '';
    private _age: number = 0;

    get name(): string {
        return this._name;
    }

    set name(value: string) {
        this._name = value;
    }

    get age(): number {
        return this._age;
    }

    set age(value: number) {
        if (value >= 0) {
            this._age = value;
        }
    }

    greet(): void {
        alert(`Hello, ${this._name}`);
    }
}
```

## Statement Conversion

### Control Flow

| VB6 | JavaScript/TypeScript |
|-----|----------------------|
| `If...Then...Else...End If` | `if...else` |
| `Select Case...End Select` | `switch` or `if/else if` |
| `For...Next` | `for` loop |
| `For Each...Next` | `for...of` loop |
| `While...Wend` | `while` loop |
| `Do While...Loop` | `while` loop |
| `Do Until...Loop` | `while (!condition)` |
| `Exit Sub/Function` | `return` |

### Examples

**If Statement**

```vb6
If x > 0 Then
    MsgBox "Positive"
ElseIf x < 0 Then
    MsgBox "Negative"
Else
    MsgBox "Zero"
End If
```

```javascript
if (x > 0) {
    alert("Positive");
} else if (x < 0) {
    alert("Negative");
} else {
    alert("Zero");
}
```

**Select Case**

```vb6
Select Case value
    Case 1
        result = "One"
    Case 2, 3
        result = "Two or Three"
    Case Else
        result = "Other"
End Select
```

```javascript
switch (value) {
    case 1:
        result = "One";
        break;
    case 2:
    case 3:
        result = "Two or Three";
        break;
    default:
        result = "Other";
}
```

**For Loop**

```vb6
For i = 1 To 10
    Debug.Print i
Next i
```

```javascript
for (let i = 1; i <= 10; i++) {
    console.log(i);
}
```

**For Each**

```vb6
For Each item In collection
    Debug.Print item.Name
Next item
```

```javascript
for (const item of collection) {
    console.log(item.name);
}
```

## Error Handling

### VB6 Error Handling

```vb6
Public Function ReadFile(path As String) As String
    On Error GoTo ErrorHandler
    
    Dim fso As New FileSystemObject
    Dim file As TextStream
    Set file = fso.OpenTextFile(path, ForReading)
    ReadFile = file.ReadAll
    file.Close
    Exit Function

ErrorHandler:
    MsgBox "Error: " & Err.Description
    ReadFile = ""
End Function
```

### JavaScript (Node.js)

```javascript
import { readFileSync } from 'fs';

export function readFile(path) {
    try {
        return readFileSync(path, 'utf8');
    } catch (error) {
        console.error(`Error: ${error.message}`);
        return '';
    }
}
```

### TypeScript with Error Handling

```typescript
import { readFileSync } from 'fs';

export function readFile(path: string): string {
    try {
        return readFileSync(path, 'utf8');
    } catch (error) {
        if (error instanceof Error) {
            console.error(`Error: ${error.message}`);
        }
        return '';
    }
}

// Or with Result type:
type Result<T, E = Error> = 
    | { ok: true; value: T }
    | { ok: false; error: E };

export function readFileSafe(path: string): Result<string> {
    try {
        const content = readFileSync(path, 'utf8');
        return { ok: true, value: content };
    } catch (error) {
        return { 
            ok: false, 
            error: error instanceof Error ? error : new Error(String(error))
        };
    }
}
```

## Async Operations

VB6 operations are synchronous, but modern JavaScript often uses async/await:

```typescript
// Synchronous VB6 style
export function getData(url: string): string {
    // Synchronous fetch (blocking)
    return fetchDataSync(url);
}

// Modern async/await style
export async function getData(url: string): Promise<string> {
    try {
        const response = await fetch(url);
        return await response.text();
    } catch (error) {
        console.error('Error fetching data:', error);
        throw error;
    }
}
```

## Collection Types

### VB6 Collection

```vb6
Dim col As New Collection
col.Add "Item1", "Key1"
col.Add "Item2", "Key2"
MsgBox col.Item("Key1")
```

### JavaScript Map

```javascript
const col = new Map();
col.set("Key1", "Item1");
col.set("Key2", "Item2");
alert(col.get("Key1"));
```

### TypeScript with Type Safety

```typescript
const col = new Map<string, string>();
col.set("Key1", "Item1");
col.set("Key2", "Item2");
const value = col.get("Key1");
if (value !== undefined) {
    alert(value);
}
```

## Project Structure

### JavaScript

```
converted_project/
├── package.json
├── src/
│   ├── index.js            # Entry point
│   ├── modules/            # Converted modules
│   │   ├── module1.js
│   │   └── module2.js
│   ├── classes/            # Converted classes
│   │   ├── person.js
│   │   └── account.js
│   └── utils/              # Utility functions
│       ├── variant.js
│       └── vb6compat.js
└── tests/
    └── integration.test.js
```

### TypeScript

```
converted_project/
├── package.json
├── tsconfig.json
├── src/
│   ├── index.ts            # Entry point
│   ├── modules/            # Converted modules
│   │   ├── module1.ts
│   │   └── module2.ts
│   ├── classes/            # Converted classes
│   │   ├── person.ts
│   │   └── account.ts
│   ├── types/              # Type definitions
│   │   ├── variant.ts
│   │   └── vb6types.ts
│   └── utils/              # Utility functions
│       └── vb6compat.ts
├── dist/                   # Compiled output
└── tests/
    └── integration.test.ts
```

### Generated package.json

```json
{
  "name": "converted-vb6-project",
  "version": "0.1.0",
  "type": "module",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc",
    "start": "node dist/index.js",
    "test": "jest",
    "dev": "tsc --watch"
  },
  "dependencies": {
    "decimal.js": "^10.0.0"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "typescript": "^5.0.0",
    "jest": "^29.0.0",
    "@types/jest": "^29.0.0"
  }
}
```

### Generated tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "lib": ["ES2020"],
    "moduleResolution": "node",
    "outDir": "./dist",
    "rootDir": "./src",
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "tests"]
}
```

## Conversion Challenges

### 1. Dynamic Typing

VB6's Variant type is very flexible but hard to type-check:

**Solution**: Use TypeScript's union types or `any` with runtime checks:

```typescript
function processVariant(value: any): string {
    if (typeof value === 'number') {
        return value.toFixed(2);
    } else if (typeof value === 'string') {
        return value.toUpperCase();
    } else if (value instanceof Date) {
        return value.toISOString();
    }
    return String(value);
}
```

### 2. ByRef Parameters

JavaScript passes objects by reference but primitives by value:

```vb6
Sub ModifyValue(ByRef x As Integer)
    x = x + 1
End Sub
```

**Solution**: Return modified values or use objects:

```typescript
// Option 1: Return value
function modifyValue(x: number): number {
    return x + 1;
}
let x = 5;
x = modifyValue(x);

// Option 2: Use object wrapper
function modifyValue(ref: { value: number }): void {
    ref.value += 1;
}
const x = { value: 5 };
modifyValue(x);
```

### 3. Optional Parameters

VB6 allows optional parameters with defaults:

```vb6
Function Calculate(x As Integer, Optional y As Integer = 10) As Integer
    Calculate = x + y
End Function
```

```typescript
function calculate(x: number, y: number = 10): number {
    return x + y;
}
```

### 4. Named Parameters

VB6 supports named parameters; JavaScript/TypeScript doesn't directly:

```vb6
result = Calculate(x:=5, y:=10)
```

**Solution**: Use object parameter:

```typescript
interface CalculateParams {
    x: number;
    y?: number;
}

function calculate({ x, y = 10 }: CalculateParams): number {
    return x + y;
}

const result = calculate({ x: 5, y: 10 });
```

### 5. Events

VB6 events can be converted to EventEmitter pattern:

```typescript
import { EventEmitter } from 'events';

export class MyClass extends EventEmitter {
    doSomething(): void {
        // Do work...
        this.emit('completed', { result: 'success' });
    }
}

// Usage:
const obj = new MyClass();
obj.on('completed', (data) => {
    console.log('Completed:', data);
});
```

## Implementation Checklist

- [ ] Basic type conversion
- [ ] Module conversion (to ES modules)
- [ ] Class conversion (to ES6 classes)
- [ ] Property conversion (getters/setters)
- [ ] Method conversion
- [ ] Expression conversion
- [ ] Statement conversion
- [ ] Control flow conversion
- [ ] Error handling conversion (try/catch)
- [ ] Array handling
- [ ] Collection handling (Map, Set)
- [ ] String operations
- [ ] Date/Time operations
- [ ] Variant support
- [ ] Optional parameters
- [ ] ParamArray conversion (rest parameters)
- [ ] Events (EventEmitter)
- [ ] Enums
- [ ] Constants
- [ ] Type definitions (TypeScript)
- [ ] JSDoc comments (JavaScript)

## Browser vs Node.js

Different runtime environments require different APIs:

| VB6 Function | Browser | Node.js |
|--------------|---------|---------|
| MsgBox | `alert()` | `console.log()` |
| InputBox | `prompt()` | `readline` module |
| Dir | N/A | `fs.readdirSync()` |
| Open file | Fetch API | `fs` module |
| Timer | `setTimeout` | `setTimeout` |

Converter should detect target environment and adapt accordingly.

## Testing Strategy

1. Unit test each conversion rule
2. Test both JavaScript and TypeScript outputs
3. Test in both browser and Node.js environments
4. Verify type safety in TypeScript
5. Test bundle size and performance

## Future Enhancements

- [ ] Generate Web Workers for background tasks
- [ ] Support for WebAssembly for performance-critical code
- [ ] Progressive Web App (PWA) support for forms
- [ ] Automatic polyfill injection for older browsers
- [ ] Bundle optimization
- [ ] Tree-shaking support
