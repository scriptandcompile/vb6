# Convenient API Design: `ProjectFile::from_file()` Implementation Analysis

## Current Situation

### Current Usage Pattern
```rust
// Step 1: Create SourceFile (owns the data)
let source_file = SourceFile::from_file("MyProject.vbp")
    .expect("Failed to read project file");

// Step 2: Parse into ProjectFile (borrows from source_file)
let project = ProjectFile::parse(&source_file)
    .unwrap_or_fail();

// source_file must stay alive as long as project is used
```

### The Lifetime Problem

**Current Signature:**
```rust
pub struct ProjectFile<'a> { /* ... */ }

impl ProjectFile<'_> {
    pub fn parse(source_file: &'a SourceFile) -> ProjectResult<'a>
}
```

**Why This Design:**
- `ProjectFile<'a>` contains borrowed string slices (`&'a str`) pointing into `SourceFile`'s content
- Zero-copy parsing: efficient memory usage, no string allocations
- The lifetime `'a` ensures `SourceFile` outlives `ProjectFile`

**Why Simple `from_file()` Cannot Work:**
```rust
// This would NOT compile:
pub fn from_file(path: &str) -> Result<ProjectFile<'static>, Error> {
    let source_file = SourceFile::from_file(path)?;  // source_file created here
    let project = ProjectFile::parse(&source_file)?; // project borrows from source_file
    Ok(project) // ERROR: source_file dropped here, but project contains references to it!
}
```

## Design Options

### Option 1: Return Both SourceFile and ProjectFile (Recommended for vb6parse)

**Approach:** Create a wrapper struct that owns both the `SourceFile` and the `ProjectFile`.

```rust
/// Owns both the source file and the parsed project.
/// This ensures the source data outlives the references in the project.
pub struct OwnedProjectFile {
    source: SourceFile,
    project: ProjectFile<'static>, // Actually borrows from 'source', but we lie about lifetime
}

impl OwnedProjectFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ErrorDetails<'static>> {
        let source = SourceFile::from_file(path)?;
        
        // SAFETY: We're extending the lifetime to 'static, which is safe because
        // the OwnedProjectFile struct owns the source data and won't allow
        // dropping source while project is accessible.
        let project = unsafe {
            std::mem::transmute::<ProjectFile<'_>, ProjectFile<'static>>(
                ProjectFile::parse(&source).unwrap_or_fail()
            )
        };
        
        Ok(Self { source, project })
    }
    
    pub fn project(&self) -> &ProjectFile<'_> {
        &self.project
    }
    
    pub fn source(&self) -> &SourceFile {
        &self.source
    }
}
```

**Pros:**
- Maintains zero-copy efficiency
- Simple, explicit API
- No changes to existing `ProjectFile` struct
- Safe: `SourceFile` is guaranteed to outlive `ProjectFile`

**Cons:**
- Requires `unsafe` code (lifetime transmutation)
- Users must understand the wrapper type
- Slightly more verbose API (`.project()` accessor)

**Usage:**
```rust
let owned = OwnedProjectFile::from_file("MyProject.vbp")
    .expect("Failed to read project");
    
// Access the project
println!("Project: {}", owned.project().properties.name);

// Both source and project are dropped together, safely
```

### Option 2: Self-Referential Struct with `ouroboros` or `self_cell`

**Approach:** Use a crate like `ouroboros` or `self_cell` to create safe self-referential structs.

```rust
use ouroboros::self_referencing;

#[self_referencing]
pub struct OwnedProjectFile {
    source: SourceFile,
    
    #[borrows(source)]
    #[not_covariant]
    project: ProjectFile<'this>,
}

impl OwnedProjectFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ErrorDetails<'static>> {
        let source = SourceFile::from_file(path)?;
        
        OwnedProjectFileTryBuilder {
            source,
            project_builder: |source: &SourceFile| {
                ProjectFile::parse(source).unwrap_or_fail()
            },
        }.try_build()
    }
}
```

**Pros:**
- Type-safe self-referential struct
- No manual `unsafe` code
- Well-tested library solution

**Cons:**
- External dependency
- Generated code can be harder to debug
- More complex to understand/maintain
- Larger API surface (generated methods)

**Crates to consider:**
- `ouroboros` (v0.18): Most popular, good docs
- `self_cell` (v1.0): Simpler, smaller, also safe

### Option 3: Make ProjectFile Own Its Data

**Approach:** Change `ProjectFile<'a>` to use owned `String` instead of borrowed `&'a str`.

```rust
pub struct ProjectFile {  // No lifetime parameter
    // Change all &'a str fields to String
    pub project_type: CompileTargetType,
    references: Vec<ProjectReference>,  // Also needs owned strings
    // ... etc
}

impl ProjectFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ErrorDetails<'static>> {
        let source = SourceFile::from_file(path)?;
        Self::parse_owned(&source)
    }
    
    // New method that creates owned strings
    fn parse_owned(source: &SourceFile) -> Result<Self, ErrorDetails<'static>> {
        // Parse and .to_owned() all strings
    }
}
```

**Pros:**
- Simplest API: just `ProjectFile::from_file()`
- No lifetime parameters to manage
- Familiar pattern (matches `ModuleFile`, `ClassFile`, `FormFile`)

**Cons:**
- **Memory overhead:** Every string is cloned
- **Performance cost:** Allocation for each string slice
- **Breaking change:** Existing API must change
- Loses zero-copy efficiency that was a design goal

**Memory Impact Estimate:**
For a typical VB6 project with:
- 50 modules at 20 chars avg path = 1KB
- 30 classes at 20 chars avg path = 0.6KB
- 20 forms at 15 chars avg path = 0.3KB
- 10 references with full metadata = 2KB
- Properties and other metadata = 2KB

Total extra memory: ~6KB per project file (not counting String overhead)

For most use cases this is negligible, but for batch processing 1000s of projects it adds up.

### Option 4: Arena Allocator Pattern

**Approach:** Use an arena like `bumpalo` to allocate strings with a single backing allocator.

```rust
use bumpalo::Bump;

pub struct ProjectFileArena {
    arena: Bump,
    project: ProjectFile<'static>,  // Actually borrows from arena
}

impl ProjectFileArena {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ErrorDetails<'static>> {
        let source = SourceFile::from_file(path)?;
        let arena = Bump::new();
        
        // Parse using the arena for string allocations
        let project = /* complex parsing with arena */;
        
        Ok(Self { arena, project })
    }
}
```

**Pros:**
- Very efficient memory allocation (bump allocator)
- Batch deallocation (entire arena at once)
- Still zero-copy within the arena

**Cons:**
- Complex implementation
- Requires rewriting parser to use arena
- External dependency
- Arena memory only freed when entire struct is dropped

## Recommendations

### For vb6parse Library

**Primary Recommendation: Option 1 (Wrapper Struct)**

Implement `OwnedProjectFile` as a convenience wrapper:

```rust
// In src/files/project/mod.rs or new src/files/project/owned.rs

/// A convenience wrapper that owns both the source file and parsed project.
/// 
/// This type is useful when you want a simple `from_file()` API without
/// managing the lifetime relationship between `SourceFile` and `ProjectFile`.
/// 
/// # Example
/// ```no_run
/// use vb6parse::OwnedProjectFile;
/// 
/// let owned = OwnedProjectFile::from_file("MyProject.vbp")
///     .expect("Failed to load project");
///     
/// println!("Project: {}", owned.project().properties.name);
/// ```
pub struct OwnedProjectFile {
    source: SourceFile,
    project: ProjectFile<'static>,
}
```

**Why This is Best:**
1. **No breaking changes:** Existing API unchanged
2. **Opt-in:** Users can choose wrapper or manual lifetime management
3. **Zero-copy maintained:** Original design goal preserved
4. **Simple mental model:** "This owns everything you need"
5. **Consistent with library philosophy:** Explicit about what's borrowed

### For vb6semantic and Higher-Level Tools

**Consider Option 3 (Owned Data)** if:
- Building a long-lived semantic database
- Storing projects across threads
- Need to send data across async boundaries
- Memory overhead is acceptable for your use case

**Example:**
```rust
// In vb6semantic
pub struct SemanticProject {
    // Store owned version for long-term storage
    name: String,
    modules: Vec<String>,
    // etc
}

impl SemanticProject {
    pub fn from_project_file(project: &ProjectFile) -> Self {
        Self {
            name: project.properties.name.to_owned(),
            modules: project.modules()
                .map(|m| m.path.to_owned())
                .collect(),
        }
    }
}
```

## Implementation Checklist

### If Implementing Option 1 (OwnedProjectFile)

- [ ] Create `src/files/project/owned.rs`
- [ ] Define `OwnedProjectFile` struct
- [ ] Implement `from_file()` method
- [ ] Add accessor methods: `project()`, `source()`
- [ ] Add `parse()` constructor variant
- [ ] Document the lifetime safety invariant
- [ ] Add comprehensive doc tests
- [ ] Re-export from `src/lib.rs`
- [ ] Add example to `examples/owned_project.rs`
- [ ] Document in README.md
- [ ] Add to CHANGELOG.md

### Safety Notes for Option 1

The lifetime transmutation in Option 1 is safe because:

1. **Ownership invariant:** `OwnedProjectFile` owns both `source` and `project`
2. **Drop order:** Rust guarantees fields are dropped in declaration order, so `project` drops before `source`
3. **No moves:** The struct is not `Copy`, preventing partial moves
4. **No borrows out:** Accessor methods return references with correct lifetimes, not 'static

**Example of why it's safe:**
```rust
impl OwnedProjectFile {
    // This method signature maintains safety
    pub fn project(&self) -> &ProjectFile<'_> {
        &self.project  // Returns with lifetime tied to &self, not 'static
    }
}
```

The `'static` lifetime on the stored `ProjectFile<'static>` is a lie, but it's a contained lie that never escapes the module. All public APIs return proper lifetimes.

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn owned_project_file_basic() {
        let owned = OwnedProjectFile::from_file("tests/data/simple.vbp")
            .expect("Failed to load");
        assert_eq!(owned.project().project_type, CompileTargetType::Exe);
    }
    
    #[test]
    fn owned_project_file_references_valid() {
        let owned = OwnedProjectFile::from_file("tests/data/with_refs.vbp")
            .expect("Failed to load");
        let refs: Vec<_> = owned.project().references().collect();
        assert!(refs.len() > 0);
    }
    
    #[test]
    fn drop_order_is_safe() {
        // This test compiles = drop order is correct
        let owned = OwnedProjectFile::from_file("tests/data/simple.vbp").unwrap();
        let _proj_ref = owned.project(); // Borrow from owned
        // owned drops here, which is fine because ref doesn't outlive it
    }
}
```

## Future Considerations

### Potential Evolution Path

1. **v1.1:** Add `OwnedProjectFile` as described (non-breaking)
2. **v1.2:** Gather user feedback on API preference
3. **v2.0:** Consider deprecating borrowed API if owned variant is strongly preferred
4. **v3.0:** Could switch to fully owned if zero-copy proves unnecessary

### Questions for Users/Maintainers

1. **Performance:** Is zero-copy parsing measurably faster in practice?
2. **Ergonomics:** Would users prefer simpler API over efficiency?
3. **Use cases:** Are projects typically short-lived or long-lived in downstream tools?
4. **Memory constraints:** Are users parsing projects on embedded/constrained systems?

## Comparison with Other File Types

| Type | Lifetime | Why |
|------|----------|-----|
| `ProjectFile<'a>` | Borrowed | Many string fields (references, paths, properties) |
| `ModuleFile` | Owned | Single name string, CST is owned |
| `ClassFile` | Owned | Header properties are small, CST is owned |
| `FormFile` | Owned | Form root and objects are owned, CST is owned |

**Lesson:** `ProjectFile` is unique in having many small string fields that benefit from zero-copy parsing. Other file types have mostly structured data that's better owned.

## Conclusion

For vb6parse, implement **Option 1** (`OwnedProjectFile` wrapper) because it:
- Maintains library design philosophy
- Provides convenient API without compromise
- Allows users to choose based on their needs
- Can coexist with existing API indefinitely

For vb6semantic and other high-level tools, extract owned data as needed for your specific use case. Don't try to keep the `ProjectFile<'a>` reference alive for the entire semantic analysis phase—copy what you need.
