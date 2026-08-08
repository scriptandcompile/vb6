PROJECTS = [
    {
    "slug": "vb6parse",
    "name": "vb6parse",
    "category": "Parser",
    "status": "stable",
    "statusLabel": "Stable",
    "statusDetail": "Feature-complete parser and docs site.",
    "summary": "Fast VB6 parser, CST, tokenization, and project-file support.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6parse",
    "docsUrl": "https://scriptandcompile.github.io/vb6/vb6parse/",
    "notes": [
      "Canonical parsing layer for the workspace.",
      "Already has the most mature GitHub Pages site."
    ]
  },
  {
    "slug": "vb6semantic",
    "name": "vb6semantic",
    "category": "Analysis",
    "status": "development",
    "statusLabel": "In Development",
    "statusDetail": "Core semantic analysis framework is in place.",
    "summary": "Symbol tables, scope resolution, and type checks for VB6.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6semantic",
    "docsUrl": "https://scriptandcompile.github.io/vb6/vb6semantic/",
    "notes": [
      "Depends on vb6parse for source structure.",
      "Provides Scope management, type tracking, and reference resolution."
    ]
  },
  {
    "slug": "aspen",
    "name": "aspen",
    "category": "Tooling",
    "status": "stable",
    "statusLabel": "Stable",
    "statusDetail": "Cargo-like tooling for VB6 projects.",
    "summary": "Check, analyze, and manage VB6 projects from the command line.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/aspen",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/aspen/README.md",
    "notes": [
      "aspen check - stable.",
      "aspen fmt - stable",
      "Currently depends on vb6parse and will eventually require vb6semantic"
    ]
  },
    {
    "slug": "vb6grammarfuzz",
    "name": "vb6grammarfuzz",
    "category": "Tooling",
    "status": "stable",
    "statusLabel": "Stable",
    "statusDetail": "Grammar fuzzing and parser comparison tooling.",
    "summary": "Parser exploration and grammar validation utilities.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6grammarfuzz",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6grammarfuzz/README.md",
    "notes": [
      "Useful for regression discovery and grammar edge cases.",
      "Includes the external ProLeap parser submodule."
    ]
  },
    {
    "slug": "vb6harness",
    "name": "vb6harness",
    "category": "Tooling",
    "status": "development",
    "statusLabel": "In Development",
    "statusDetail": "Still building runtime library elements",
    "summary": "Test harness to confirm behavior of runtime elements in compiler vs interpreter vs VB6 IDE",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6harness",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6harness/README.md",
    "notes": [
      "Still working on runtime library elements",
      "Eventually should match across all three components."
    ]
  },
  {
    "slug": "vb6format",
    "name": "vb6format",
    "category": "Tooling",
    "status": "development",
    "statusLabel": "In Development",
    "statusDetail": "Indention, compiler directives, and line ending formatting complete.",
    "summary": "Reformat VB6 source files with correct block indentation.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6format",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6format/README.md",
    "notes": [
      "Used by the aspen fmt subcommand.",
      "Indent formatting supported",
      "Compiler directive formatting supported.",
      "Line ending detection and formatting supported."
    ]
  },
  {
    "slug": "vscode-vb6",
    "name": "vscode-vb6",
    "category": "Tooling",
    "status": "development",
    "statusLabel": "In Development",
    "statusDetail": "VS Code extension scaffold for the VB6 ecosystem.",
    "summary": "Editor integration and language tooling for VB6 developers.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vscode-vb6",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vscode-vb6/README.md",
    "notes": [
      "Should eventually consume the same metadata surface as the hub.",
      "Good place to surface docs and status from the workspace."
    ]
  },
  {
    "slug": "vb6runtime",
    "name": "vb6runtime",
    "category": "Runtime",
    "status": "runtime",
    "statusLabel": "In Development",
    "statusDetail": "Runtime value system and standard library planning.",
    "summary": "VB6 value semantics, type conversion, and standard functions.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6runtime",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6runtime/README.md",
    "notes": [
      "Feeds both the interpreter and compiler paths.",
      "Defines the compatibility surface for generated code."
    ]
  },
  {
    "slug": "vb6core",
    "name": "vb6core",
    "category": "Core",
    "status": "development",
    "statusLabel": "In Development",
    "statusDetail": "Internal Values, Errors, and vb6 types for runtime, compiler, and interpreters.",
    "summary": "Shared primitives and types.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6core",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6core/README.md",
    "notes": [
      "Shared infrastructure multiple base library components.",
      "VBError, VBType, TypeInfo, core data type structures."
    ]
  },
  {
    "slug": "vb6convert",
    "name": "vb6convert",
    "category": "Migration",
    "status": "design",
    "statusLabel": "In Design",
    "statusDetail": "Conversion framework and target-matrix planning.",
    "summary": "Transform VB6 projects into modern Rust-based applications.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6convert",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6convert/docs/",
    "notes": [
      "Needs target-specific generator and validation surfaces.",
      "Depends on vb6codegen and vb6semantic design choices."
    ]
  },
  {
    "slug": "vb6codegen",
    "name": "vb6codegen",
    "category": "Generation",
    "status": "design",
    "statusLabel": "In Design",
    "statusDetail": "Shared code-generation backends are still being defined.",
    "summary": "Backends for Rust, LLVM, and future target emitters.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6codegen",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6codegen/docs/",
    "notes": [
      "Should become the shared target-emitter layer.",
      "Supports both compilation and migration flows."
    ]
  },
  {
    "slug": "vb6libraries",
    "name": "vb6libraries",
    "category": "Libraries",
    "status": "design",
    "statusLabel": "In Design",
    "statusDetail": "Library mapping and integration hooks are still being gathered.",
    "summary": "Win32, Office, data, and UI library mappings for codegen.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6libraries",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6libraries/README.md",
    "notes": [
      "Important for conversion fidelity.",
      "Should be expressed as a shared catalog."
    ]
  },
  {
    "slug": "vb6compile",
    "name": "vb6compile",
    "category": "Compiler",
    "status": "design",
    "statusLabel": "In Design",
    "statusDetail": "Native compiler architecture and targets are being defined.",
    "summary": "Compile VB6 source into native executables or generated code.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6compile",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6compile/docs/",
    "notes": [
      "Likely depends on vb6codegen and vb6core.",
      "Needs a clear release and runtime strategy."
    ]
  },
  {
    "slug": "vb6interpret",
    "name": "vb6interpret",
    "category": "Interpreter",
    "status": "development",
    "statusLabel": "In Development",
    "statusDetail": "Interpreter behavior and runtime model are still being shaped.",
    "summary": "Execute VB6 projects directly without compiling first.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6interpret",
    "docsUrl": "https://scriptandcompile.github.io/vb6/vb6interpret/",
    "notes": [
      "Basic .bas processing, string handling, and Debug.Print",
      "Now, with a playground!"
    ]
  },
  {
    "slug": "vb6lsp",
    "name": "vb6lsp",
    "category": "Tooling",
    "status": "design",
    "statusLabel": "In Design",
    "statusDetail": "Language-server adjacent workspace component.",
    "summary": "Language server and editor integration pieces for VB6.",
    "repoUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6lsp",
    "docsUrl": "https://github.com/scriptandcompile/vb6/tree/master/projects/vb6lsp/README.md",
    "notes": [
      "Likely shares parsing and semantic layers.",
      "Should eventually point back into the same docs hub."
    ]
  }
]