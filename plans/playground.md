# VB6Parse Playground Implementation Plan

## Overview
Create an interactive web-based playground for VB6Parse that allows users to parse VB6 code in real-time and visualize the results. The playground will be compiled to WebAssembly (WASM) and integrated into the existing GitHub Pages documentation site.

## 1. WASM Compilation Setup

### 1.1 Dependencies
Add to `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib", "rlib"]

# Note: serde is already a dependency with derive feature, serde_json may need to be added
[dependencies]
serde_json = "1.0"

# WASM-specific dependencies - only compiled for wasm32 target
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
console_error_panic_hook = "0.1"
getrandom = { version = "0.2", features = ["js"] }

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Enable Link Time Optimization
codegen-units = 1   # Reduce parallel code generation units for better optimization
```

### 1.2 Build Tools
Install required tools:
```bash
# Install wasm-pack for building WASM packages
cargo install wasm-pack

# Optional: Install wasm-opt for further optimization
cargo install wasm-opt
```

### 1.3 WASM Module Structure
Create a new module `src/wasm.rs` or `src/playground.rs`:
```rust
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct PlaygroundOutput {
    tokens: Option<Vec<TokenInfo>>,
    cst: Option<String>,
    errors: Vec<ErrorInfo>,
    parse_time_ms: f64,
}

#[wasm_bindgen]
pub fn parse_vb6_code(
    code: &str,
    file_type: &str, // "project", "class", "module", "form"
) -> Result<JsValue, JsValue> {
    // Implementation that calls appropriate parser
    // Returns serialized PlaygroundOutput
}

#[wasm_bindgen]
pub fn tokenize_vb6_code(code: &str) -> Result<JsValue, JsValue> {
    // Returns just tokens for quick preview
}
```

### 1.4 Build Script
Create `scripts/build-wasm.py`:
```python
#!/usr/bin/env python3
"""
Cross-platform WASM build script for VB6Parse playground.
Works on Windows, macOS, and Linux, both locally and in GitHub Actions.

Requirements:
- Python 3.7+
- wasm-pack (installed via: cargo install wasm-pack)
- wasm-opt (optional, installed via: cargo install wasm-opt)

Usage:
    python scripts/build-wasm.py [--optimize] [--no-typescript]
"""

import argparse
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path


def find_executable(name: str) -> str | None:
    """
    Find an executable in PATH, handling Windows .exe extension.
    Returns the full path to the executable or None if not found.
    """
    # On Windows, check with .exe extension
    if platform.system() == "Windows":
        executable = shutil.which(f"{name}.exe")
        if executable:
            return executable
    
    # Try without extension (works on all platforms)
    executable = shutil.which(name)
    return executable


def check_requirements() -> tuple[str, str | None]:
    """
    Check if required tools are installed.
    Returns tuple of (wasm_pack_path, wasm_opt_path)
    """
    wasm_pack = find_executable("wasm-pack")
    if not wasm_pack:
        print("❌ Error: wasm-pack not found in PATH", file=sys.stderr)
        print("   Install with: cargo install wasm-pack", file=sys.stderr)
        sys.exit(1)
    
    wasm_opt = find_executable("wasm-opt")
    if not wasm_opt:
        print("⚠️  Warning: wasm-opt not found (optional optimization will be skipped)")
        print("   Install with: cargo install wasm-opt", file=sys.stderr)
    
    return wasm_pack, wasm_opt


def run_command(cmd: list[str], description: str) -> None:
    """Run a command and handle errors."""
    print(f"🔨 {description}...")
    try:
        result = subprocess.run(
            cmd,
            check=True,
            capture_output=True,
            text=True
        )
        if result.stdout:
            print(result.stdout)
    except subprocess.CalledProcessError as e:
        print(f"❌ Error: {description} failed", file=sys.stderr)
        if e.stdout:
            print(e.stdout, file=sys.stderr)
        if e.stderr:
            print(e.stderr, file=sys.stderr)
        sys.exit(1)


def build_wasm(wasm_pack: str, output_dir: Path, no_typescript: bool) -> None:
    """Build the WASM module using wasm-pack."""
    cmd = [
        wasm_pack,
        "build",
        "--target", "web",
        "--out-dir", str(output_dir),
        "--release"
    ]
    
    # Add --no-typescript flag if requested (reduces output files)
    if no_typescript:
        cmd.append("--no-typescript")
    
    run_command(cmd, "Building WASM module with wasm-pack")


def optimize_wasm(wasm_opt: str | None, wasm_file: Path) -> None:
    """Optimize WASM binary using wasm-opt if available."""
    if not wasm_opt:
        print("⏩ Skipping wasm-opt optimization (not installed)")
        return
    
    if not wasm_file.exists():
        print(f"⚠️  Warning: {wasm_file} not found, skipping optimization")
        return
    
    # Create backup
    backup_file = wasm_file.with_suffix(".wasm.bak")
    shutil.copy2(wasm_file, backup_file)
    
    try:
        cmd = [
            wasm_opt,
            "-Oz",  # Optimize aggressively for size
            "-o", str(wasm_file),
            str(backup_file)
        ]
        run_command(cmd, "Optimizing WASM binary with wasm-opt")
        
        # Show size comparison
        original_size = backup_file.stat().st_size
        optimized_size = wasm_file.stat().st_size
        savings = original_size - optimized_size
        percent = (savings / original_size) * 100
        
        print(f"   Original size: {original_size:,} bytes")
        print(f"   Optimized size: {optimized_size:,} bytes")
        print(f"   Saved: {savings:,} bytes ({percent:.1f}%)")
        
        # Remove backup
        backup_file.unlink()
        
    except Exception as e:
        print(f"⚠️  Warning: Optimization failed: {e}")
        print("   Restoring original file...")
        shutil.move(backup_file, wasm_file)


def main():
    parser = argparse.ArgumentParser(
        description="Build VB6Parse WASM module for playground"
    )
    parser.add_argument(
        "--optimize",
        action="store_true",
        help="Run wasm-opt optimization (requires wasm-opt to be installed)"
    )
    parser.add_argument(
        "--no-typescript",
        action="store_true",
        help="Skip TypeScript definition generation"
    )
    args = parser.parse_args()
    
    # Determine project root (parent of scripts directory)
    script_dir = Path(__file__).parent.resolve()
    project_root = script_dir.parent
    output_dir = project_root / "docs" / "assets" / "wasm"
    
    print("=" * 60)
    print("VB6Parse WASM Build Script")
    print("=" * 60)
    print(f"Platform: {platform.system()} {platform.machine()}")
    print(f"Python: {sys.version.split()[0]}")
    print(f"Project root: {project_root}")
    print(f"Output directory: {output_dir}")
    print("=" * 60)
    
    # Change to project root
    os.chdir(project_root)
    
    # Check requirements
    wasm_pack, wasm_opt = check_requirements()
    print(f"✅ wasm-pack found: {wasm_pack}")
    if wasm_opt:
        print(f"✅ wasm-opt found: {wasm_opt}")
    
    # Ensure output directory exists
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Build WASM
    build_wasm(wasm_pack, output_dir, args.no_typescript)
    
    # Optimize if requested and tool is available
    if args.optimize:
        wasm_file = output_dir / "vb6parse_bg.wasm"
        optimize_wasm(wasm_opt, wasm_file)
    
    print("=" * 60)
    print("✅ WASM build complete!")
    print(f"📦 Output files in: {output_dir}")
    
    # List generated files
    if output_dir.exists():
        files = sorted(output_dir.iterdir())
        if files:
            print("\n📄 Generated files:")
            for file in files:
                size = file.stat().st_size
                print(f"   - {file.name} ({size:,} bytes)")
    
    print("=" * 60)


if __name__ == "__main__":
    main()
```

**Platform-specific notes:**

1. **Windows**:
   - Script automatically handles `.exe` extensions for executables
   - Works with both CMD and PowerShell
   - Use `python scripts/build-wasm.py` or `python3 scripts/build-wasm.py`
   - Path separators handled automatically by `pathlib.Path`

2. **macOS/Linux**:
   - Make script executable: `chmod +x scripts/build-wasm.py`
   - Can run as: `./scripts/build-wasm.py` or `python3 scripts/build-wasm.py`
   - Shebang (`#!/usr/bin/env python3`) allows direct execution

3. **GitHub Actions**:
   - Python 3 is pre-installed on all GitHub-hosted runners
   - Works identically across ubuntu-latest, windows-latest, macos-latest
   - Example workflow step:
     ```yaml
     - name: Build WASM
       run: python scripts/build-wasm.py --optimize --no-typescript
     ```

4. **Common Issues**:
   - **cargo not in PATH**: Ensure `~/.cargo/bin` (Linux/macOS) or `%USERPROFILE%\.cargo\bin` (Windows) is in PATH
   - **wasm-pack not found**: Run `cargo install wasm-pack` first
   - **Permission denied** (Linux/macOS): Run `chmod +x scripts/build-wasm.py`
   - **Python version**: Requires Python 3.7+ (for `pathlib` and type hints with `|` requires 3.10+, but script is compatible with 3.7+ using `Optional` if needed)

## 2. Playground UI Design

### 2.1 Layout Structure
```
┌─────────────────────────────────────────────────────┐
│  VB6Parse Playground                    [File Type▼]│
├──────────────────┬──────────────────────────────────┤
│                  │  ┌────────────────────────────┐  │
│                  │  │ Tokens │ CST │ Tree │ Info │  │
│                  │  ├────────────────────────────┤  │
│  Code Editor     │  │                            │  │
│  (Monaco/Ace)    │  │  Output Display Area       │  │
│                  │  │                            │  │
│  [VB6 Syntax     │  │  (Content changes based    │  │
│   Highlighting]  │  │   on selected tab)         │  │
│                  │  │                            │  │
│                  │  │                            │  │
│                  │  └────────────────────────────┘  │
│  [Parse Button]  │  Parse Time: 42ms                │
└──────────────────┴──────────────────────────────────┘
```

### 2.2 Left Panel: Code Editor
- **Editor**: Monaco Editor (used by VS Code) or Ace Editor
- **Features**:
  - VB6 syntax highlighting (custom language definition)
  - Line numbers
  - Auto-save to localStorage
  - Sample code dropdown (pre-populated examples)
  - File type selector: Project (.vbp), Class (.cls), Module (.bas), Form (.frm)
  - Auto-parse on change (with 500ms debounce) or manual parse button

### 2.3 Right Panel: Output Tabs

#### Tab 1: Tokens View
- **Format**: Table or syntax-highlighted list
- **Columns**:
  - Token Type (Keyword, Identifier, Literal, Operator, Whitespace, Comment)
  - Value
  - Position (line:column)
  - Length
- **Features**:
  - Click token to highlight in source
  - Filter by token type
  - Search tokens

#### Tab 2: CST (Concrete Syntax Tree) View
- **Format**: Indented text tree
- **Example**:
  ```
  CompilationUnit [0..256]
    VersionStatement [0..18]
      Keyword(VERSION) [0..7]
      Whitespace [7..8]
      Float(1.0) [8..11]
      Newline [11..13]
    AttributeStatement [13..45]
      ...
  ```
- **Features**:
  - Expandable/collapsible nodes
  - Click node to highlight in source
  - Show byte ranges
  - Color-coded by node type

#### Tab 3: Tree Visualization
- **Library**: D3.js, vis.js, or Cytoscape.js
- **Display**: Interactive tree diagram
  - Horizontal or vertical layout (user toggle)
  - Zoom and pan controls
  - Node colors based on type (statement, expression, literal, etc.)
  - Click node to show details in sidebar
  - Highlight corresponding source code
  - Collapse/expand subtrees
- **Legend**: Node type color key

#### Tab 4: Info/Errors View
- **Sections**:
  - **Statistics**:
    - Total tokens: N
    - Parse time: Nms
    - Tree depth: N
    - Node count: N
    - File type detected: X
  - **Errors** (if any):
    - List of parsing errors with line numbers
    - Click to jump to error location
    - Error type and message
  - **Warnings** (if any)

### 2.4 Additional UI Elements
- **Header Bar**:
  - Title: "VB6Parse Playground"
  - File type selector dropdown
  - Share button (generates URL with encoded code)
  - GitHub link
  - Documentation link
  
- **Footer**:
  - Version info
  - Link to source code
  - "Powered by VB6Parse vX.X.X"

## 3. Technical Architecture

### 3.1 Frontend Stack
```
playground/
├── index.html          # Main playground page
├── styles/
│   ├── playground.css  # Main styles
│   └── tree-viz.css    # Tree visualization styles
├── js/
│   ├── main.js         # Main application logic
│   ├── editor.js       # Editor initialization and handling
│   ├── parser.js       # WASM wrapper and parser calls
│   ├── renderer.js     # Output rendering for each tab
│   ├── tree-viz.js     # Tree visualization logic
│   └── examples.js     # Sample code snippets
└── assets/
    └── wasm/           # Built WASM files (from wasm-pack)
```

### 3.2 Key JavaScript Modules

#### main.js
```javascript
import init, { parse_vb6_code, init_panic_hook } from './assets/wasm/vb6parse.js';

class VB6Playground {
    constructor() {
        this.editor = null;
        this.currentOutput = null;
        this.currentTab = 'tokens';
    }
    
    async init() {
        // Initialize WASM
        await init();
        init_panic_hook();
        
        // Initialize editor
        this.editor = initEditor('editor-container');
        
        // Set up event listeners
        this.setupEventListeners();
        
        // Load default example
        this.loadExample('simple-class');
    }
    
    async parse() {
        const code = this.editor.getValue();
        const fileType = document.getElementById('file-type').value;
        
        try {
            const output = await parse_vb6_code(code, fileType);
            this.currentOutput = output;
            this.renderOutput();
        } catch (err) {
            this.showError(err);
        }
    }
}
```

#### tree-viz.js
```javascript
import * as d3 from 'd3';

export class TreeVisualizer {
    constructor(containerId) {
        this.container = d3.select(`#${containerId}`);
        this.svg = null;
        this.data = null;
    }
    
    render(cstData) {
        // Convert CST to D3 hierarchical data
        const hierarchy = this.cstToHierarchy(cstData);
        
        // Create tree layout
        const treeLayout = d3.tree()
            .size([height, width]);
            
        // Render nodes and links
        // ... D3 visualization code
    }
    
    cstToHierarchy(cstNode) {
        // Convert rowan-style CST to D3 hierarchy
    }
}
```

### 3.3 Integration Points with VB6Parse
- Create serializable output structures
- Convert `TokenStream` to JSON-friendly format
- Convert `CstNode` tree to JSON representation
- Format `ErrorDetails` for web display
- Include source locations for all elements

## 4. GitHub Pages Integration

### 4.1 File Placement
```
docs/
├── playground.html          # New playground page
├── assets/
│   ├── css/
│   │   ├── playground.css   # Playground-specific styles
│   │   └── tree-viz.css
│   ├── js/
│   │   ├── playground/      # Playground JS modules
│   │   │   ├── main.js
│   │   │   ├── editor.js
│   │   │   ├── parser.js
│   │   │   ├── renderer.js
│   │   │   ├── tree-viz.js
│   │   │   └── examples.js
│   │   └── vendor/          # Third-party libraries
│   │       ├── monaco-editor/ (or CDN link)
│   │       └── d3.min.js      (or CDN link)
│   └── wasm/                # WASM build output
│       ├── vb6parse_bg.wasm
│       ├── vb6parse.js
│       └── vb6parse.d.ts
└── index.html               # Update to link to playground
```

### 4.2 Navigation Integration
Update `docs/index.html` to add playground link:
```html
<nav>
  <a href="index.html">Home</a>
  <a href="getting-started.html">Getting Started</a>
  <a href="documentation.html">Documentation</a>
  <a href="playground.html">Playground</a>  <!-- NEW -->
  <a href="benchmarks.html">Benchmarks</a>
  <a href="coverage.html">Coverage</a>
</nav>
```

### 4.3 Build Process

**Local Development:**
```bash
# Build WASM before deploying docs
python scripts/build-wasm.py --optimize

# Commit built WASM files
git add docs/assets/wasm/
git commit -m "Update WASM playground build"

# Push to trigger GitHub Pages deployment
git push origin main
```

**GitHub Actions CI/CD:**
Create `.github/workflows/build-wasm.yml`:
```yaml
name: Build WASM Playground

on:
  push:
    branches: [ main ]
    paths:
      - 'src/**'
      - 'Cargo.toml'
      - 'scripts/build-wasm.py'
  pull_request:
    branches: [ main ]
  workflow_dispatch:

jobs:
  build-wasm:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      
      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo index
        uses: actions/cache@v4
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Install wasm-pack
        run: cargo install wasm-pack
      
      - name: Install wasm-opt
        run: cargo install wasm-opt
      
      - name: Build WASM module
        run: python scripts/build-wasm.py --optimize --no-typescript
      
      - name: Upload WASM artifacts
        uses: actions/upload-artifact@v4
        with:
          name: wasm-${{ matrix.os }}
          path: docs/assets/wasm/
          retention-days: 7
      
      - name: Commit WASM files (main branch only)
        if: github.ref == 'refs/heads/main' && matrix.os == 'ubuntu-latest'
        run: |
          git config --local user.email "github-actions[bot]@users.noreply.github.com"
          git config --local user.name "github-actions[bot]"
          git add docs/assets/wasm/
          git diff --quiet && git diff --staged --quiet || git commit -m "Update WASM build [skip ci]"
          git push
```

**Notes:**
- The workflow runs on all three platforms to verify cross-platform compatibility
- Only the Ubuntu build commits the WASM files back to the repo
- `[skip ci]` in commit message prevents infinite build loops
- Caching speeds up subsequent builds
- Artifacts are uploaded for inspection/debugging

## 5. Implementation Phases

### Phase 1: WASM Foundation (Week 1)
- [ ] Add WASM dependencies to Cargo.toml
- [ ] Create src/wasm.rs module with basic API
- [ ] Implement parse_vb6_code() function
- [ ] Set up build script for WASM compilation
- [ ] Test WASM module in Node.js environment
- [ ] Create minimal HTML page to verify WASM loading

### Phase 2: Basic Playground UI (Week 2)
- [ ] Create playground.html with basic layout
- [ ] Integrate Monaco Editor or Ace Editor
- [ ] Add VB6 syntax highlighting definition
- [ ] Implement file type selector
- [ ] Create tab navigation (Tokens, CST, Tree, Info)
- [ ] Add parse button with loading state
- [ ] Implement basic tokens view (table format)
- [ ] Test end-to-end: code → WASM → token display

### Phase 3: Advanced Output Views (Week 3)
- [ ] Implement CST text tree view with collapsible nodes
- [ ] Add source highlighting (click token/node → highlight code)
- [ ] Create Info/Stats panel
- [ ] Format and display errors with line numbers
- [ ] Add performance metrics display
- [ ] Implement localStorage for code persistence

### Phase 4: Tree Visualization (Week 4)
- [ ] Integrate D3.js or chosen visualization library
- [ ] Convert CST to hierarchical data format
- [ ] Implement tree rendering with zoom/pan
- [ ] Add node coloring by type
- [ ] Implement click interactions (node → code highlight)
- [ ] Add layout toggle (horizontal/vertical)
- [ ] Create legend for node types

### Phase 5: Polish & Integration (Week 5)
- [ ] Add sample code examples dropdown
- [ ] Implement share functionality (URL encoding)
- [ ] Add responsive design for mobile/tablet
- [ ] Optimize WASM bundle size
- [ ] Add loading states and error boundaries
- [ ] Write user documentation for playground
- [ ] Update main documentation site to link playground
- [ ] Performance testing and optimization
- [ ] Cross-browser testing (Chrome, Firefox, Safari, Edge)

### Phase 6: Advanced Features (Optional/Future)
- [ ] Side-by-side diff mode for before/after
- [ ] Export functionality (JSON, image of tree)
- [ ] Permalink generation with compressed code
- [ ] Keyboard shortcuts
- [ ] Dark/light theme toggle
- [ ] Multiple file support (project with dependencies)
- [ ] AST view (when implemented in library)
- [ ] Form resource (.frx) file viewer

## 6. Technical Considerations

### 6.1 Performance
- **Lazy Loading**: Load Monaco Editor and D3.js on demand
- **Web Workers**: Consider running parser in web worker to avoid blocking UI
- **Debouncing**: Auto-parse with 500ms debounce to avoid excessive parsing
- **WASM Optimization**: Use release profile with size optimizations
- **Bundle Size**: Monitor and optimize total page weight (target: <2MB including WASM)

### 6.2 Browser Compatibility
- **Target**: Modern browsers with WASM support (Chrome 57+, Firefox 52+, Safari 11+, Edge 16+)
- **Feature Detection**: Check for WASM support and show message if unavailable
- **Polyfills**: Minimal JS polyfills for older browsers (if needed)

### 6.3 Security
- **Sandboxing**: WASM runs in sandbox, but validate input sizes
- **Resource Limits**: Limit input code size (e.g., 1MB max) to prevent DoS
- **Error Handling**: Catch and display WASM panics gracefully
- **No eval()**: Avoid dynamic code execution in JavaScript

### 6.4 Accessibility
- **Keyboard Navigation**: Full keyboard support for tabs and controls
- **Screen Readers**: ARIA labels for interactive elements
- **Color Contrast**: Meet WCAG AA standards
- **Focus Management**: Clear focus indicators

## 7. Testing Strategy

### 7.1 Unit Tests
- Test WASM module functions independently
- Mock WASM in JS tests for faster iteration
- Test tree conversion functions

### 7.2 Integration Tests
- Test full parse flow: input → WASM → output
- Test with various VB6 code samples
- Test error handling and edge cases

### 7.3 Browser Tests
- Manual testing in all target browsers
- Automated E2E tests with Playwright or Cypress (optional)

### 7.4 Performance Tests
- Measure parse time for various file sizes
- Monitor WASM module memory usage
- Test with large files (10KB, 100KB, 1MB)

## 8. Documentation

### 8.1 User Guide
Create `docs/playground-guide.md`:
- How to use the playground
- Explanation of each output view
- Sample code snippets to try
- Tips for using tree visualization
- Keyboard shortcuts

### 8.2 Developer Guide
Document in CONTRIBUTING.md:
- How to build WASM module
- How to test playground locally
- Architecture overview
- How to add new features

## 9. Deployment Checklist

- [ ] WASM module builds successfully
- [ ] All output views render correctly
- [ ] Examples load and parse
- [ ] Error handling works (bad syntax, large files)
- [ ] Responsive design tested
- [ ] Cross-browser testing complete
- [ ] Documentation written
- [ ] Performance acceptable (parse <500ms for typical files)
- [ ] Bundle size acceptable (<2MB total)
- [ ] GitHub Pages deployment works
- [ ] Links from main site functional
- [ ] Analytics set up (optional)

## 10. Future Enhancements

### Short-term
- Add more example snippets
- Implement advanced filtering in tokens view
- Add keyboard shortcuts
- Create tutorial/walkthrough for first-time users

### Medium-term
- Multi-file project support
- Form designer visualization for .frm files
- Control property inspector for forms
- Export to various formats (JSON, GraphViz DOT)

### Long-term
- AST view when available in library
- Code transformation playground (VB6 → VB.NET snippets)
- Integration with other parsing tools
- Collaborative features (share and comment on code)
- VS Code extension using same WASM module

## 11. Resources & References

### Libraries
- **wasm-bindgen**: https://rustwasm.github.io/wasm-bindgen/
- **wasm-pack**: https://rustwasm.github.io/wasm-pack/
- **Monaco Editor**: https://microsoft.github.io/monaco-editor/
- **Ace Editor**: https://ace.c9.io/
- **D3.js**: https://d3js.org/
- **vis.js**: https://visjs.org/
- **Cytoscape.js**: https://js.cytoscape.org/

### Examples
- **Rust Playground**: https://play.rust-lang.org/
- **Go Playground**: https://go.dev/play/
- **TypeScript Playground**: https://www.typescriptlang.org/play
- **AST Explorer**: https://astexplorer.net/

### Documentation
- **Rust and WebAssembly Book**: https://rustwasm.github.io/book/
- **MDN WebAssembly**: https://developer.mozilla.org/en-US/docs/WebAssembly

---

**Estimated Timeline**: 5-6 weeks for full implementation with one developer
**Maintenance**: Minimal after initial release; rebuild WASM when library updates
