# PropertyGroup Implementation Plan

## Overview

This document outlines the plan to properly implement Font and other PropertyGroup objects in the VB6 parser. Currently, BeginProperty/EndProperty pairs are correctly parsed into `PropertyGroup` structures, but they are not being converted into typed objects (like `Font`) or properly associated with controls that need them.

## Current State

### What Works
1. **Parsing**: BeginProperty/EndProperty blocks are correctly parsed into `PropertyGroup` objects
2. **Data Structure**: `PropertyGroup` exists with:
   - `name: String` (e.g., "Font", "Images")
   - `guid: Option<Uuid>`
   - `properties: HashMap<String, Either<String, PropertyGroup>>` (supports nested groups)
3. **Storage for Custom Controls**: Custom controls store property groups in a `Vec<PropertyGroup>`
4. **Font Struct**: The `Font` struct is properly defined with all necessary fields

### What Doesn't Work
1. **No Font Fields in Most Controls**: Only `FormProperties` has a `font: Font` field; standard controls like `CommandButton`, `Label`, `TextBox` don't have font fields
2. **No Conversion**: No `From<PropertyGroup>` or `TryFrom<PropertyGroup>` implementation for `Font`
3. **Property Groups Are Ignored**: When parsing standard controls, property groups are collected but:
   - Only stored for Custom controls
   - Discarded for standard controls (see `build_control_kind` in `parsers/cst/mod.rs`)
4. **No Serialization Back**: No way to serialize a `Font` object back to a `PropertyGroup` format

## Problem Examples

### Example: CommandButton with Font
```vb6
Begin VB.CommandButton multiplybtn
   Caption         =   "×"
   BeginProperty Font 
      Name            =   "Tahoma"
      Size            =   20.25
      Charset         =   0
      Weight          =   700
      Underline       =   0   'False
      Italic          =   0   'False
      Strikethrough   =   0   'False
   EndProperty
   Height          =   1000
End
```

**Current Behavior**: 
- PropertyGroup is parsed correctly
- Then discarded when building `ControlKind::CommandButton`
- `CommandButtonProperties` has no `font` field

**Expected Behavior**:
- PropertyGroup converted to `Font` object
- Stored in `CommandButtonProperties.font: Option<Font>`
- Serialized properly with serde

## Implementation Plan

### Phase 1: Add Font Fields to Control Properties

**Affected Files**: All control property structs in `vb6parse/src/language/controls/`

**Controls That Should Have Font**:
Based on VB6 documentation and common usage:
- ✅ Form (already has it)
- ✅ MDIForm (needs it)
- CommandButton
- Label
- TextBox
- CheckBox
- OptionButton
- Frame
- ListBox
- ComboBox
- PictureBox
- Data (data-bound controls typically have fonts)
- FileListBox
- DirListBox
- DriveListBox

**Changes Required**:
1. Add `pub font: Option<Font>` field to each control's Properties struct
2. Update `Default` implementation to set `font: None` or `font: Some(Font::default())`
3. Update `Serialize` implementation to include font field
4. Update `From<Properties>` implementation to handle Font (placeholder for now)

**Example for CommandButton**:
```rust
pub struct CommandButtonProperties {
    // ... existing fields ...
    pub font: Option<Font>,
    // ... rest of fields ...
}

impl Default for CommandButtonProperties {
    fn default() -> Self {
        CommandButtonProperties {
            // ... existing defaults ...
            font: None, // or Some(Font::default()) depending on VB6 defaults
            // ... rest of defaults ...
        }
    }
}
```

### Phase 2: Create PropertyGroup Conversion Traits

**New File**: `vb6parse/src/files/common/property_group_conversions.rs`

**Purpose**: Convert PropertyGroup to typed objects and vice versa

**Implementation**:

```rust
/// Trait for types that can be created from a PropertyGroup
pub trait FromPropertyGroup: Sized {
    type Error;
    
    /// Convert a PropertyGroup to this type
    fn from_property_group(group: &PropertyGroup) -> Result<Self, Self::Error>;
    
    /// Expected name of the property group (e.g., "Font")
    fn property_group_name() -> &'static str;
}

/// Trait for types that can be converted to a PropertyGroup
pub trait ToPropertyGroup {
    /// Convert this type to a PropertyGroup
    fn to_property_group(&self) -> PropertyGroup;
}
```

**Font Implementation**:
```rust
impl FromPropertyGroup for Font {
    type Error = ErrorKind;
    
    fn from_property_group(group: &PropertyGroup) -> Result<Self, Self::Error> {
        if !group.name.eq_ignore_ascii_case("Font") {
            return Err(ErrorKind::Form(FormError::InvalidPropertyGroupName {
                expected: "Font".to_string(),
                found: group.name.clone(),
            }));
        }
        
        let mut font = Font::default();
        
        // Extract properties with proper type conversion
        for (key, value) in &group.properties {
            match value {
                Either::Left(string_value) => {
                    match key.as_str() {
                        "Name" => font.name = string_value.clone(),
                        "Size" => font.size = string_value.parse()
                            .unwrap_or(font.size),
                        "Charset" => font.charset = string_value.parse()
                            .unwrap_or(font.charset),
                        "Weight" => font.weight = string_value.parse()
                            .unwrap_or(font.weight),
                        "Underline" => font.underline = parse_vb6_bool(string_value),
                        "Italic" => font.italic = parse_vb6_bool(string_value),
                        "Strikethrough" => font.strikethrough = parse_vb6_bool(string_value),
                        _ => {}, // Ignore unknown properties
                    }
                },
                Either::Right(_nested) => {
                    // Font doesn't have nested property groups
                }
            }
        }
        
        Ok(font)
    }
    
    fn property_group_name() -> &'static str {
        "Font"
    }
}

impl ToPropertyGroup for Font {
    fn to_property_group(&self) -> PropertyGroup {
        let mut properties = HashMap::new();
        
        properties.insert(
            "Name".to_string(),
            Either::Left(self.name.clone())
        );
        properties.insert(
            "Size".to_string(),
            Either::Left(self.size.to_string())
        );
        properties.insert(
            "Charset".to_string(),
            Either::Left(self.charset.to_string())
        );
        properties.insert(
            "Weight".to_string(),
            Either::Left(self.weight.to_string())
        );
        properties.insert(
            "Underline".to_string(),
            Either::Left(if self.underline { "-1" } else { "0" }.to_string())
        );
        properties.insert(
            "Italic".to_string(),
            Either::Left(if self.italic { "-1" } else { "0" }.to_string())
        );
        properties.insert(
            "Strikethrough".to_string(),
            Either::Left(if self.strikethrough { "-1" } else { "0" }.to_string())
        );
        
        PropertyGroup {
            name: "Font".to_string(),
            guid: None, // Font can have GUID in VB6, could extract if needed
            properties,
        }
    }
}

// Helper function
fn parse_vb6_bool(s: &str) -> bool {
    matches!(s, "-1" | "True" | "true")
}
```

### Phase 3: Modify Control Building to Handle PropertyGroups

**File**: `vb6parse/src/parsers/cst/mod.rs`

**Current Code** (in `build_control_kind`):
```rust
"VB.CommandButton" => ControlKind::CommandButton {
    properties: properties.into(),
},
```

**New Approach**:

1. Create a helper function to extract and convert property groups:
```rust
/// Extract typed property groups from a Vec<PropertyGroup>
fn extract_property_groups(groups: &[PropertyGroup]) -> ExtractedGroups {
    let mut font = None;
    
    for group in groups {
        if group.name.eq_ignore_ascii_case("Font") {
            if let Ok(f) = Font::from_property_group(group) {
                font = Some(f);
            }
        }
        // Future: handle other property group types (Images, etc.)
    }
    
    ExtractedGroups { font }
}

struct ExtractedGroups {
    font: Option<Font>,
    // Future: add other property group types
}
```

2. Pass property groups through to control construction:
```rust
fn build_control_kind(
    control_type: &str,
    properties: Properties,
    child_controls: Vec<Control>,
    menus: Vec<MenuControl>,
    property_groups: Vec<PropertyGroup>,
) -> ControlKind {
    // Extract typed property groups
    let groups = extract_property_groups(&property_groups);
    
    match control_type {
        "VB.CommandButton" => {
            let mut props: CommandButtonProperties = properties.into();
            // Override with property group if present
            if let Some(font) = groups.font {
                props.font = Some(font);
            }
            ControlKind::CommandButton { properties: props }
        },
        // Similar for other controls...
        _ => ControlKind::Custom {
            properties: properties.into(),
            property_groups, // Custom controls keep raw groups
        },
    }
}
```

### Phase 4: Update From<Properties> Implementations

**Files**: All `vb6parse/src/language/controls/*.rs` files

The `From<Properties>` implementation should initialize the font field to None, since the actual Font will come from PropertyGroups in Phase 3.

**Example**:
```rust
impl From<Properties> for CommandButtonProperties {
    fn from(prop: Properties) -> Self {
        let mut command_button_prop = CommandButtonProperties::default();
        
        // ... existing property parsing ...
        
        // Font is handled separately via property groups
        // Leave as None/default
        command_button_prop.font = None;
        
        command_button_prop
    }
}
```

### Phase 5: Handle Form and MDIForm PropertyGroups

**Files**: 
- `vb6parse/src/language/controls/form.rs`
- `vb6parse/src/language/controls/mdiform.rs`
- `vb6parse/src/parsers/cst/mod.rs` (in `parse_properties_block_to_form_root`)

**Current Issue**: Form parsing comment says "Property groups are not used in Form/MDIForm, but we parse them anyway"

**Solution**: 
1. Forms and MDIForms should also process property groups for Font
2. Modify `parse_properties_block_to_form_root` to extract property groups
3. Pass them to Form/MDIForm construction
4. Update `From<Properties>` to handle Font property group

**Example for Form**:
```rust
// In parse_properties_block_to_form_root
let groups = extract_property_groups(&property_groups);
let mut form_properties: FormProperties = properties.into();

// Override font if property group provided
if let Some(font) = groups.font {
    form_properties.font = font;
}
```

### Phase 6: Serialization Support

**Purpose**: Ensure Font objects can be serialized properly, both to JSON (via serde) and back to VB6 format

#### 6.1: JSON Serialization (Already Working)

**Current Font Serialization**: Already implemented via `#[derive(Serialize)]`
   - This handles JSON serialization correctly
   - No changes needed
   - Font fields will appear in JSON output automatically

#### 6.2: VB6 Format Serialization (PropertyGroup → VB6 Text)

**New File**: `vb6parse/src/files/common/property_group_formatter.rs`

**Purpose**: Format PropertyGroup objects as VB6 BeginProperty/EndProperty blocks

**Implementation**:

```rust
use std::fmt::Write as FmtWrite;
use std::io::{Write, Result as IoResult};
use crate::files::common::PropertyGroup;
use either::Either;

/// Configuration for formatting PropertyGroups
#[derive(Debug, Clone)]
pub struct FormatConfig {
    /// Base indentation level (number of spaces)
    pub indent: usize,
    /// Include GUID in BeginProperty line if present
    pub include_guid: bool,
    /// Include inline comments (e.g., 'False after boolean 0)
    pub include_comments: bool,
    /// Line ending style
    pub line_ending: LineEnding,
}

#[derive(Debug, Clone, Copy)]
pub enum LineEnding {
    /// Windows style (\r\n)
    CrLf,
    /// Unix style (\n)
    Lf,
}

impl Default for FormatConfig {
    fn default() -> Self {
        FormatConfig {
            indent: 3, // VB6 uses 3 spaces for property indentation
            include_guid: true,
            include_comments: true,
            line_ending: LineEnding::CrLf, // VB6 default
        }
    }
}

/// Format a PropertyGroup as VB6 text
pub fn format_property_group(
    group: &PropertyGroup,
    writer: &mut dyn Write,
    config: &FormatConfig,
) -> IoResult<()> {
    format_property_group_with_depth(group, writer, config, 0)
}

/// Internal function that tracks nesting depth for indentation
fn format_property_group_with_depth(
    group: &PropertyGroup,
    writer: &mut dyn Write,
    config: &FormatConfig,
    depth: usize,
) -> IoResult<()> {
    let indent_str = " ".repeat(config.indent * (depth + 1));
    let line_end = match config.line_ending {
        LineEnding::CrLf => "\r\n",
        LineEnding::Lf => "\n",
    };
    
    // Write BeginProperty line
    write!(writer, "{}BeginProperty {}", indent_str, group.name)?;
    
    // Add GUID if present and configured
    if config.include_guid {
        if let Some(guid) = &group.guid {
            write!(writer, " {{{}}}", guid.to_string().to_uppercase())?;
        }
    }
    
    write!(writer, "{}", line_end)?;
    
    // Write properties in canonical VB6 order
    let ordered_keys = order_properties_for_font(&group.name, group.properties.keys());
    
    for key in ordered_keys {
        if let Some(value) = group.properties.get(&key) {
            match value {
                Either::Left(string_value) => {
                    // Format simple property
                    let formatted_value = format_property_value(
                        &key,
                        string_value,
                        config.include_comments,
                    );
                    writeln!(
                        writer,
                        "{}   {} = {}{}",
                        indent_str,
                        key,
                        formatted_value,
                        line_end
                    )?;
                }
                Either::Right(nested_group) => {
                    // Recursively format nested property group
                    format_property_group_with_depth(
                        nested_group,
                        writer,
                        config,
                        depth + 1,
                    )?;
                }
            }
        }
    }
    
    // Write EndProperty line
    writeln!(writer, "{}EndProperty{}", indent_str, line_end)?;
    
    Ok(())
}

/// Format a property value with appropriate VB6 formatting
fn format_property_value(key: &str, value: &str, include_comments: bool) -> String {
    // Check if this is a boolean property
    if is_boolean_property(key) {
        return format_boolean_value(value, include_comments);
    }
    
    // Check if value needs quotes
    if needs_quotes(value) {
        return format!("\"{}\"", value);
    }
    
    // Return as-is for numeric values
    value.to_string()
}

/// Check if a property is a boolean type
fn is_boolean_property(key: &str) -> bool {
    matches!(
        key,
        "Underline" | "Italic" | "Strikethrough" | "Bold" | "Visible" | "Enabled"
    )
}

/// Format a boolean value with optional comment
fn format_boolean_value(value: &str, include_comments: bool) -> String {
    match value {
        "-1" | "True" | "true" => {
            if include_comments {
                "-1   'True".to_string()
            } else {
                "-1".to_string()
            }
        }
        "0" | "False" | "false" => {
            if include_comments {
                "0   'False".to_string()
            } else {
                "0".to_string()
            }
        }
        _ => value.to_string(),
    }
}

/// Check if a value needs to be quoted
fn needs_quotes(value: &str) -> bool {
    // If it's not a number and not a special VB6 constant, it needs quotes
    // Empty strings don't need quotes in VB6
    if value.is_empty() {
        return false;
    }
    
    // Check if it's a numeric value
    if value.parse::<f64>().is_ok() || value.starts_with('-') && value[1..].parse::<f64>().is_ok() {
        return false;
    }
    
    // Check if it's a VB6 constant or resource reference
    if value.starts_with('&') || value.contains(':') {
        return false;
    }
    
    true
}

/// Order properties for Font in canonical VB6 order
fn order_properties_for_font(group_name: &str, keys: impl Iterator<Item = impl AsRef<str>>) -> Vec<String> {
    if !group_name.eq_ignore_ascii_case("Font") {
        // For non-Font groups, return keys in collected order
        return keys.map(|k| k.as_ref().to_string()).collect();
    }
    
    // VB6 Font property order
    const FONT_ORDER: &[&str] = &[
        "Name",
        "Size",
        "Charset",
        "Weight",
        "Underline",
        "Italic",
        "Strikethrough",
    ];
    
    let mut result = Vec::new();
    let key_set: std::collections::HashSet<String> = 
        keys.map(|k| k.as_ref().to_string()).collect();
    
    // Add properties in canonical order
    for &prop in FONT_ORDER {
        if key_set.contains(prop) {
            result.push(prop.to_string());
        }
    }
    
    // Add any remaining properties not in canonical order
    for key in key_set {
        if !FONT_ORDER.contains(&key.as_str()) {
            result.push(key);
        }
    }
    
    result
}
```

#### 6.3: Update PropertyGroup ToPropertyGroup Implementation

**File**: `vb6parse/src/files/common/property_group_conversions.rs`

Update the `ToPropertyGroup` implementation for Font to optionally preserve GUID:

```rust
impl ToPropertyGroup for Font {
    fn to_property_group(&self) -> PropertyGroup {
        self.to_property_group_with_guid(None)
    }
}

impl Font {
    /// Convert Font to PropertyGroup with optional GUID
    /// 
    /// VB6 Font GUIDs are typically {0BE35203-8F91-11CE-9DE3-00AA004BB851}
    /// but can vary by VB6 version
    pub fn to_property_group_with_guid(&self, guid: Option<Uuid>) -> PropertyGroup {
        let mut properties = HashMap::new();
        
        properties.insert(
            "Name".to_string(),
            Either::Left(self.name.clone())
        );
        properties.insert(
            "Size".to_string(),
            Either::Left(self.size.to_string())
        );
        properties.insert(
            "Charset".to_string(),
            Either::Left(self.charset.to_string())
        );
        properties.insert(
            "Weight".to_string(),
            Either::Left(self.weight.to_string())
        );
        properties.insert(
            "Underline".to_string(),
            Either::Left(if self.underline { "-1" } else { "0" }.to_string())
        );
        properties.insert(
            "Italic".to_string(),
            Either::Left(if self.italic { "-1" } else { "0" }.to_string())
        );
        properties.insert(
            "Strikethrough".to_string(),
            Either::Left(if self.strikethrough { "-1" } else { "0" }.to_string())
        );
        
        PropertyGroup {
            name: "Font".to_string(),
            guid,
            properties,
        }
    }
}
```

#### 6.4: Control Serialization Integration

**File**: `vb6parse/src/files/form/writer.rs` (new file for VB6 form writing)

```rust
use std::io::{Write, Result as IoResult};
use crate::language::controls::{Control, ControlKind};
use crate::files::common::property_group_formatter::{format_property_group, FormatConfig};

/// Write a control in VB6 format
pub fn write_control(
    control: &Control,
    writer: &mut dyn Write,
    indent: usize,
    config: &FormatConfig,
) -> IoResult<()> {
    let indent_str = " ".repeat(indent);
    let line_end = "\r\n"; // VB6 standard
    
    // Write Begin line
    writeln!(
        writer,
        "{}Begin {} {}{}",
        indent_str,
        control_type_string(control),
        control.name(),
        line_end
    )?;
    
    // Write simple properties
    write_control_properties(control, writer, indent + 3, config)?;
    
    // Write property groups (e.g., Font)
    write_control_property_groups(control, writer, indent + 3, config)?;
    
    // Write child controls recursively
    if let Some(children) = control.children() {
        for child in children {
            write_control(child, writer, indent + 3, config)?;
        }
    }
    
    // Write End line
    writeln!(writer, "{}End{}", indent_str, line_end)?;
    
    Ok(())
}

/// Write control's property groups (Font, etc.)
fn write_control_property_groups(
    control: &Control,
    writer: &mut dyn Write,
    indent: usize,
    config: &FormatConfig,
) -> IoResult<()> {
    match control.kind() {
        ControlKind::CommandButton { properties } => {
            if let Some(font) = &properties.font {
                let prop_group = font.to_property_group();
                format_property_group(&prop_group, writer, config)?;
            }
        }
        ControlKind::Label { properties } => {
            if let Some(font) = &properties.font {
                let prop_group = font.to_property_group();
                format_property_group(&prop_group, writer, config)?;
            }
        }
        ControlKind::TextBox { properties } => {
            if let Some(font) = &properties.font {
                let prop_group = font.to_property_group();
                format_property_group(&prop_group, writer, config)?;
            }
        }
        // Add similar cases for other controls...
        ControlKind::Custom { property_groups, .. } => {
            // Custom controls: write all property groups
            for group in property_groups {
                format_property_group(group, writer, config)?;
            }
        }
        _ => {}
    }
    
    Ok(())
}

/// Helper to get control type string (e.g., "VB.CommandButton")
fn control_type_string(control: &Control) -> &'static str {
    match control.kind() {
        ControlKind::CommandButton { .. } => "VB.CommandButton",
        ControlKind::Label { .. } => "VB.Label",
        ControlKind::TextBox { .. } => "VB.TextBox",
        // ... other control types ...
        _ => "VB.Control",
    }
}
```

#### 6.5: Example Output

**Input Font object**:
```rust
let font = Font {
    name: "Courier New".to_string(),
    size: 10.5,
    charset: 0,
    weight: 700,
    underline: true,
    italic: false,
    strikethrough: false,
};
```

**Serialized VB6 output** (with comments):
```vb6
   BeginProperty Font 
      Name            =   "Courier New"
      Size            =   10.5
      Charset         =   0
      Weight          =   700
      Underline       =   -1   'True
      Italic          =   0   'False
      Strikethrough   =   0   'False
   EndProperty
```

**Serialized VB6 output** (without comments):
```vb6
   BeginProperty Font 
      Name            =   "Courier New"
      Size            =   10.5
      Charset         =   0
      Weight          =   700
      Underline       =   -1
      Italic          =   0
      Strikethrough   =   0
   EndProperty
```

**With GUID**:
```vb6
   BeginProperty Font {0BE35203-8F91-11CE-9DE3-00AA004BB851}
      Name            =   "Arial"
      Size            =   12
      Charset         =   0
      Weight          =   400
      Underline       =   0   'False
      Italic          =   0   'False
      Strikethrough   =   0   'False
   EndProperty
```

#### 6.6: Complete Control Example

**Input**:
```rust
let control = Control::new(
    "Command1",
    "MyButton",
    0,
    ControlKind::CommandButton {
        properties: CommandButtonProperties {
            caption: "Click Me".to_string(),
            height: 495,
            left: 120,
            top: 120,
            width: 1215,
            font: Some(Font {
                name: "Arial".to_string(),
                size: 12.0,
                charset: 0,
                weight: 700,
                underline: false,
                italic: true,
                strikethrough: false,
            }),
            ..Default::default()
        }
    }
);
```

**Serialized VB6 output**:
```vb6
   Begin VB.CommandButton Command1
      Caption         =   "Click Me"
      BeginProperty Font 
         Name            =   "Arial"
         Size            =   12
         Charset         =   0
         Weight          =   700
         Underline       =   0   'False
         Italic          =   -1   'True
         Strikethrough   =   0   'False
      EndProperty
      Height          =   495
      Left            =   120
      Top             =   120
      Width           =   1215
   End
```

### Phase 7: Testing

**Test Cases Needed**:

1. **Unit Tests**: `vb6parse/src/files/common/property_group_conversions.rs`
   ```rust
   #[test]
   fn test_font_from_property_group() {
       let mut properties = HashMap::new();
       properties.insert("Name".to_string(), Either::Left("Arial".to_string()));
       properties.insert("Size".to_string(), Either::Left("12".to_string()));
       properties.insert("Weight".to_string(), Either::Left("700".to_string()));
       properties.insert("Italic".to_string(), Either::Left("-1".to_string()));
       
       let group = PropertyGroup {
           name: "Font".to_string(),
           guid: None,
           properties,
       };
       
       let font = Font::from_property_group(&group).unwrap();
       assert_eq!(font.name, "Arial");
       assert_eq!(font.size, 12.0);
       assert_eq!(font.weight, 700);
       assert_eq!(font.italic, true);
   }
   
   #[test]
   fn test_font_to_property_group() {
       let font = Font {
           name: "Courier New".to_string(),
           size: 10.0,
           charset: 0,
           weight: 400,
           underline: true,
           italic: false,
           strikethrough: false,
       };
       
       let group = font.to_property_group();
       assert_eq!(group.name, "Font");
       assert_eq!(group.properties.get("Name"), Some(&Either::Left("Courier New".to_string())));
       assert_eq!(group.properties.get("Underline"), Some(&Either::Left("-1".to_string())));
   }
   
   #[test]
   fn test_font_serialization_with_guid() {
       let guid = Uuid::parse_str("0BE35203-8F91-11CE-9DE3-00AA004BB851").unwrap();
       let font = Font {
           name: "Arial".to_string(),
           size: 12.0,
           charset: 0,
           weight: 400,
           underline: false,
           italic: false,
           strikethrough: false,
       };
       
       let group = font.to_property_group_with_guid(Some(guid));
       assert_eq!(group.guid, Some(guid));
       assert_eq!(group.name, "Font");
   }
   ```

2. **Integration Tests**: Parse actual VB6 forms with Font properties
   ```rust
   #[test]
   fn test_parse_form_with_fonts() {
       let form_content = r#"
       Begin VB.Form Form1
          BeginProperty Font 
             Name            =   "Arial"
             Size            =   12
             Charset         =   0
             Weight          =   400
             Underline       =   0   'False
             Italic          =   0   'False
             Strikethrough   =   0   'False
          EndProperty
          Begin VB.CommandButton Command1
             Caption         =   "Click Me"
             BeginProperty Font 
                Name            =   "Courier New"
                Size            =   10
                Weight          =   700
             EndProperty
          End
       End
       "#;
       
       let parsed = parse_form(form_content).unwrap();
       
       // Check form font
       let form_font = parsed.form.properties.font;
       assert_eq!(form_font.name, "Arial");
       assert_eq!(form_font.size, 12.0);
       
       // Check control font
       let button = &parsed.form.controls[0];
       if let ControlKind::CommandButton { properties } = &button.kind {
           let button_font = properties.font.as_ref().unwrap();
           assert_eq!(button_font.name, "Courier New");
           assert_eq!(button_font.weight, 700);
       }
   }
   ```

3. **Round-trip Tests**: Parse → Serialize → Parse → Compare
   ```rust
   #[test]
   fn test_font_round_trip() {
       let original = Font {
           name: "Verdana".to_string(),
           size: 8.25,
           charset: 0,
           weight: 400,
           underline: false,
           italic: true,
           strikethrough: false,
       };
       
       let group = original.to_property_group();
       let recovered = Font::from_property_group(&group).unwrap();
       
       assert_eq!(original, recovered);
   }
   ```

4. **VB6 Format Serialization Tests**: `vb6parse/src/files/common/property_group_formatter.rs`
   ```rust
   #[test]
   fn test_format_font_property_group() {
       let mut properties = HashMap::new();
       properties.insert("Name".to_string(), Either::Left("Arial".to_string()));
       properties.insert("Size".to_string(), Either::Left("12".to_string()));
       properties.insert("Charset".to_string(), Either::Left("0".to_string()));
       properties.insert("Weight".to_string(), Either::Left("700".to_string()));
       properties.insert("Underline".to_string(), Either::Left("0".to_string()));
       properties.insert("Italic".to_string(), Either::Left("-1".to_string()));
       properties.insert("Strikethrough".to_string(), Either::Left("0".to_string()));
       
       let group = PropertyGroup {
           name: "Font".to_string(),
           guid: None,
           properties,
       };
       
       let mut output = Vec::new();
       let config = FormatConfig::default();
       format_property_group(&group, &mut output, &config).unwrap();
       
       let result = String::from_utf8(output).unwrap();
       
       // Verify structure
       assert!(result.contains("BeginProperty Font"));
       assert!(result.contains("EndProperty"));
       assert!(result.contains("Name            =   \"Arial\""));
       assert!(result.contains("Size            =   12"));
       assert!(result.contains("Italic          =   -1   'True"));
       assert!(result.contains("Underline       =   0   'False"));
   }
   
   #[test]
   fn test_format_font_with_guid() {
       let guid = Uuid::parse_str("0BE35203-8F91-11CE-9DE3-00AA004BB851").unwrap();
       let font = Font::default();
       let group = font.to_property_group_with_guid(Some(guid));
       
       let mut output = Vec::new();
       let config = FormatConfig::default();
       format_property_group(&group, &mut output, &config).unwrap();
       
       let result = String::from_utf8(output).unwrap();
       assert!(result.contains("BeginProperty Font {0BE35203-8F91-11CE-9DE3-00AA004BB851}"));
   }
   
   #[test]
   fn test_format_without_comments() {
       let font = Font {
           name: "Tahoma".to_string(),
           size: 10.0,
           charset: 0,
           weight: 400,
           underline: true,
           italic: false,
           strikethrough: false,
       };
       
       let group = font.to_property_group();
       let mut output = Vec::new();
       let mut config = FormatConfig::default();
       config.include_comments = false;
       
       format_property_group(&group, &mut output, &config).unwrap();
       
       let result = String::from_utf8(output).unwrap();
       
       // Should not contain inline comments
       assert!(!result.contains("'True"));
       assert!(!result.contains("'False"));
       // Should still contain values
       assert!(result.contains("Underline       =   -1"));
       assert!(result.contains("Italic          =   0"));
   }
   
   #[test]
   fn test_property_ordering() {
       // Create properties in random order
       let mut properties = HashMap::new();
       properties.insert("Strikethrough".to_string(), Either::Left("0".to_string()));
       properties.insert("Name".to_string(), Either::Left("Arial".to_string()));
       properties.insert("Italic".to_string(), Either::Left("0".to_string()));
       properties.insert("Weight".to_string(), Either::Left("400".to_string()));
       properties.insert("Size".to_string(), Either::Left("12".to_string()));
       properties.insert("Underline".to_string(), Either::Left("0".to_string()));
       properties.insert("Charset".to_string(), Either::Left("0".to_string()));
       
       let group = PropertyGroup {
           name: "Font".to_string(),
           guid: None,
           properties,
       };
       
       let mut output = Vec::new();
       let config = FormatConfig::default();
       format_property_group(&group, &mut output, &config).unwrap();
       
       let result = String::from_utf8(output).unwrap();
       
       // Verify canonical order: Name, Size, Charset, Weight, Underline, Italic, Strikethrough
       let name_pos = result.find("Name").unwrap();
       let size_pos = result.find("Size").unwrap();
       let charset_pos = result.find("Charset").unwrap();
       let weight_pos = result.find("Weight").unwrap();
       let underline_pos = result.find("Underline").unwrap();
       let italic_pos = result.find("Italic").unwrap();
       let strikethrough_pos = result.find("Strikethrough").unwrap();
       
       assert!(name_pos < size_pos);
       assert!(size_pos < charset_pos);
       assert!(charset_pos < weight_pos);
       assert!(weight_pos < underline_pos);
       assert!(underline_pos < italic_pos);
       assert!(italic_pos < strikethrough_pos);
   }
   ```

5. **Full Control Serialization Tests**: `vb6parse/src/files/form/writer.rs`
   ```rust
   #[test]
   fn test_write_control_with_font() {
       let control = Control::new(
           "Command1",
           "",
           0,
           ControlKind::CommandButton {
               properties: CommandButtonProperties {
                   caption: "Click Me".to_string(),
                   height: 495,
                   left: 120,
                   top: 120,
                   width: 1215,
                   font: Some(Font {
                       name: "Arial".to_string(),
                       size: 12.0,
                       charset: 0,
                       weight: 700,
                       underline: false,
                       italic: true,
                       strikethrough: false,
                   }),
                   ..Default::default()
               }
           }
       );
       
       let mut output = Vec::new();
       let config = FormatConfig::default();
       write_control(&control, &mut output, 3, &config).unwrap();
       
       let result = String::from_utf8(output).unwrap();
       
       // Verify complete control structure
       assert!(result.contains("Begin VB.CommandButton Command1"));
       assert!(result.contains("Caption         =   \"Click Me\""));
       assert!(result.contains("BeginProperty Font"));
       assert!(result.contains("Name            =   \"Arial\""));
       assert!(result.contains("Size            =   12"));
       assert!(result.contains("Weight          =   700"));
       assert!(result.contains("Italic          =   -1   'True"));
       assert!(result.contains("EndProperty"));
       assert!(result.contains("End"));
   }
   
   #[test]
   fn test_full_form_round_trip() {
       // Parse a complete form with font
       let original_vb6 = r#"VERSION 5.00
Begin VB.Form Form1
   Caption         =   "Test Form"
   BeginProperty Font 
      Name            =   "Courier New"
      Size            =   10.5
      Charset         =   0
      Weight          =   400
      Underline       =   0   'False
      Italic          =   0   'False
      Strikethrough   =   0   'False
   EndProperty
   Begin VB.CommandButton Command1
      Caption         =   "Button"
      BeginProperty Font 
         Name            =   "Arial"
         Size            =   12
         Charset         =   0
         Weight          =   700
         Underline       =   0   'False
         Italic          =   -1   'True
         Strikethrough   =   0   'False
      EndProperty
   End
End
"#;
       
       // Parse
       let parsed = parse_form(original_vb6).unwrap();
       
       // Verify parsed fonts
       assert_eq!(parsed.form.properties.font.name, "Courier New");
       assert_eq!(parsed.form.properties.font.size, 10.5);
       
       let button = &parsed.form.controls[0];
       if let ControlKind::CommandButton { properties } = &button.kind {
           let font = properties.font.as_ref().unwrap();
           assert_eq!(font.name, "Arial");
           assert_eq!(font.size, 12.0);
           assert_eq!(font.italic, true);
       }
       
       // Serialize back to VB6
       let mut output = Vec::new();
       write_form(&parsed, &mut output).unwrap();
       let serialized = String::from_utf8(output).unwrap();
       
       // Parse again
       let reparsed = parse_form(&serialized).unwrap();
       
       // Compare - fonts should be identical
       assert_eq!(parsed.form.properties.font, reparsed.form.properties.font);
       
       if let (
           ControlKind::CommandButton { properties: props1 },
           ControlKind::CommandButton { properties: props2 }
       ) = (&parsed.form.controls[0].kind, &reparsed.form.controls[0].kind) {
           assert_eq!(props1.font, props2.font);
       }
   }
   ```

## Future Extensions

### Other PropertyGroup Types

The same pattern can be applied to other property group types:

1. **Images/ImageList**: Custom control property groups for image collections
   ```vb6
   BeginProperty Images {2C247F25-8591-11D1-B16A-00C0F0283628}
      NumListImages   =   2
      BeginProperty ListImage1 {2C247F27-8591-11D1-B16A-00C0F0283628}
         Picture         =   "Form1.frx":0000
         Key             =   ""
      EndProperty
   EndProperty
   ```

2. **Nested PropertyGroups**: Some custom controls have deeply nested structures
   - Extend `FromPropertyGroup` to handle nested groups recursively
   - Maintain Either::Right(PropertyGroup) in properties HashMap

3. **Generic PropertyGroup Handling**: For unknown/custom property groups
   - Keep as raw PropertyGroup for custom controls
   - Allow pass-through without conversion

### Error Handling Improvements

**New Error Types** (add to `vb6parse/src/errors/form.rs`):
```rust
#[error("Invalid property group name: expected '{expected}', found '{found}'")]
InvalidPropertyGroupName {
    expected: String,
    found: String,
},

#[error("Failed to parse property in group '{group}': {property} = {value}")]
InvalidPropertyInGroup {
    group: String,
    property: String,
    value: String,
},

#[error("Missing required property in group '{group}': {property}")]
MissingPropertyInGroup {
    group: String,
    property: String,
},
```

## Migration Strategy

### Backward Compatibility
- Existing code that doesn't use Fonts will continue to work
- Font fields are `Option<Font>`, so None is valid
- Custom controls still store raw PropertyGroups

### Incremental Implementation
1. Start with Font (most common)
2. Add Font fields to a few controls first (Form, CommandButton, Label, TextBox)
3. Test thoroughly
4. Roll out to remaining controls
5. Later: handle other PropertyGroup types

## Open Questions

1. **GUID Handling**: Should Font PropertyGroups preserve GUIDs?
   - VB6 sometimes includes GUIDs like `{0BE35203-8F91-11CE-9DE3-00AA004BB851}`
   - Currently ignored in Font struct
   - May need to add `pub guid: Option<Uuid>` to Font if round-tripping is important

2. **Default Fonts**: Should controls have `font: None` or `font: Some(Font::default())`?
   - VB6 has a default font cascade (control → form → system)
   - None might better represent "use parent font"
   - Some(default) might better match VB6 behavior

3. **Property Names Case Sensitivity**: 
   - Currently using case-insensitive matching
   - Confirm this matches VB6 behavior

4. **Unknown Property Groups**: 
   - Should we warn/error on unknown property groups?
   - Or silently store them for custom controls?

## Success Criteria

✅ All controls that support fonts have `font: Option<Font>` field  
✅ PropertyGroup with name "Font" correctly converts to Font object  
✅ Controls parsed from .frm files have proper Font objects  
✅ Font objects serialize correctly to JSON (via Serialize trait)  
✅ Font objects can convert back to PropertyGroup format (via ToPropertyGroup trait)  
✅ PropertyGroup objects can format to VB6 text format (BeginProperty/EndProperty blocks)  
✅ VB6 text output matches original formatting (proper indentation, property order, comments)  
✅ GUID preservation in PropertyGroup serialization  
✅ Boolean values format correctly with optional inline comments  
✅ Property ordering follows VB6 canonical order (Name, Size, Charset, Weight, etc.)  
✅ Full form round-trip: Parse VB6 → Font objects → Serialize VB6 → Parse → Compare  
✅ All existing tests pass  
✅ New tests for Font conversion pass  
✅ New tests for VB6 format serialization pass  
✅ Documentation updated

## Estimated Implementation Time

- Phase 1 (Add Font fields): 2-3 hours
- Phase 2 (Conversion traits): 2-3 hours  
- Phase 3 (Modify control building): 3-4 hours
- Phase 4 (Update From<Properties>): 1-2 hours
- Phase 5 (Form/MDIForm handling): 1-2 hours
- Phase 6 (Serialization): 
  - PropertyGroup formatter implementation: 3-4 hours
  - Control writer integration: 2-3 hours
  - VB6 format helpers (ordering, quoting, comments): 2-3 hours
  - GUID handling: 1 hour
- Phase 7 (Testing): 
  - Unit tests for conversions: 2 hours
  - Integration tests: 2 hours
  - VB6 serialization tests: 3-4 hours
  - Round-trip tests: 2-3 hours

**Total: ~25-35 hours for complete Font implementation with full serialization**

**Breakdown by Capability**:
- Basic Font parsing and storage: ~10-15 hours
- Full VB6 serialization support: ~10-15 hours
- Comprehensive testing: ~8-10 hours
- Phase 5 (Form/MDIForm handling): 1-2 hours
- Phase 6 (Serialization): 2-3 hours (initial), more for full VB6 writer
- Phase 7 (Testing): 3-4 hours

**Total: ~15-20 hours for complete Font implementation**

---

## References

- [VB6 Font Object Documentation](https://learn.microsoft.com/en-us/office/vba/language/reference/user-interface-help/font-object-microsoft-forms)
- Current Parser Implementation: `vb6parse/src/parsers/cst/mod.rs`
- PropertyGroup Definition: `vb6parse/src/files/common/properties.rs`
- Font Definition: `vb6parse/src/language/controls/mod.rs`
