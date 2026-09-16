//! VB6 project loading.
//!
//! Loads `.vbp` project files, resolves all file references (`.frm`, `.bas`, `.cls`),
//! parses them, and provides a `LoadedProject` containing everything needed for
//! multi-module execution.

use std::path::Path;

use anyhow::Result;
use vb6parse::files::project::ProjectFile;
use vb6parse::files::project::properties::CompileTargetType;
use vb6parse::files::{ClassFile, FormFile, ModuleFile};
use vb6parse::io::SourceFile;

/// An owned file entry from a `.vbp` file.
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum ProjectFileEntry {
    /// A `Module=` entry (`.bas`).
    Module { name: String, path: String },
    /// A `Class=` entry (`.cls`).
    Class { name: String, path: String },
    /// A `Form=` entry (`.frm`).
    Form(String),
    /// A `UserControl=` entry (`.ctl`).
    UserControl(String),
    /// A `UserDocument=` entry (`.do*`).
    UserDocument(String),
    /// A `Designer=` entry.
    Designer(String),
    /// A `RelatedDoc=` entry.
    RelatedDoc(String),
    /// A `PropertyPage=` entry (`.pag`).
    PropertyPage(String),
}

/// A loaded VB6 project with all files resolved and parsed.
pub struct LoadedProject {
    /// The project type (Exe, Dll, etc.).
    pub project_type: CompileTargetType,
    /// The project name.
    pub project_name: String,
    /// All forms referenced in the project.
    pub forms: Vec<LoadedForm>,
    /// All modules referenced in the project.
    pub modules: Vec<LoadedModule>,
    /// All class modules referenced in the project.
    pub classes: Vec<LoadedClass>,
    /// File entries in VBP order (used for execution order).
    pub file_entries: Vec<ProjectFileEntry>,
    /// The startup object for the project.
    pub startup_object: StartupObject,
}

/// The project's startup object.
pub enum StartupObject {
    /// A `Sub Main` procedure in a module.
    SubMain {
        /// The module containing the sub.
        module_name: String,
        /// The name of the sub (usually "Main").
        sub_name: String,
    },
    /// A form as startup object.
    Form {
        /// The form name (from `Attribute VB_Name`).
        form_name: String,
    },
    /// No startup object found.
    None,
}

/// A loaded form file with parsed AST and raw bytes.
pub struct LoadedForm {
    /// The form name (from `Attribute VB_Name`).
    pub name: String,
    /// The file name (e.g. "Form1.frm").
    pub file_name: String,
    /// The parsed form file.
    pub parsed: FormFile,
    /// The raw file bytes (used for Tauri/WASM rendering).
    pub raw_bytes: Vec<u8>,
}

/// A loaded module file with parsed AST and raw bytes.
pub struct LoadedModule {
    /// The module name (from `Attribute VB_Name`).
    pub name: String,
    /// The file name (e.g. "Module1.bas").
    pub file_name: String,
    /// The parsed module file.
    pub parsed: ModuleFile,
    /// The raw file bytes.
    pub raw_bytes: Vec<u8>,
}

/// A loaded class module file with parsed AST and raw bytes.
pub struct LoadedClass {
    /// The class name (from `Attribute VB_Name`).
    pub name: String,
    /// The file name (e.g. "Class1.cls").
    pub file_name: String,
    /// The parsed class file.
    pub parsed: ClassFile,
    /// The raw file bytes.
    pub raw_bytes: Vec<u8>,
}

impl LoadedProject {
    /// Load a VB6 project from a `.vbp` file path.
    ///
    /// Reads and parses the project file, then resolves and loads all referenced
    /// forms, modules, and classes from disk.
    pub fn load(path: &Path) -> Result<Self> {
        let vbp_path = path
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("Cannot canonicalize path '{}': {}", path.display(), e))?;
        let project_dir = vbp_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Project file has no parent directory"))?;

        let source_file = SourceFile::from_file(&vbp_path).map_err(|e| {
            anyhow::anyhow!("Failed to read project file '{}': {:?}", path.display(), e)
        })?;
        let result = ProjectFile::parse(&source_file);
        let (project_opt, failures) = result.unpack();

        if !failures.is_empty() {
            for failure in &failures {
                eprintln!("Project parse warning: {:?}", failure.kind);
            }
        }

        let project = project_opt
            .ok_or_else(|| anyhow::anyhow!("Failed to parse project file: {}", path.display()))?;

        let project_type = project.project_type;
        let project_name = project.properties.name.to_string();

        let mut forms = Vec::new();
        let mut modules = Vec::new();
        let mut classes = Vec::new();
        let mut file_entries = Vec::new();

        // Convert file entries to owned form and load each file.
        for entry in project.file_entries() {
            let owned = match entry {
                vb6parse::files::project::ProjectFileEntry::Module(mref) => {
                    let name = mref.name.to_string();
                    let path = mref.path.to_string();
                    file_entries.push(ProjectFileEntry::Module {
                        name: name.clone(),
                        path: path.clone(),
                    });
                    let file_path = project_dir.join(&path);
                    let module = Self::load_module(&file_path, &name)?;
                    modules.push(module);
                    continue;
                }
                vb6parse::files::project::ProjectFileEntry::Class(cref) => {
                    let name = cref.name.to_string();
                    let path = cref.path.to_string();
                    file_entries.push(ProjectFileEntry::Class {
                        name: name.clone(),
                        path: path.clone(),
                    });
                    let file_path = project_dir.join(&path);
                    let class = Self::load_class(&file_path, &name)?;
                    classes.push(class);
                    continue;
                }
                vb6parse::files::project::ProjectFileEntry::Form(fname) => {
                    let fname = fname.to_string();
                    file_entries.push(ProjectFileEntry::Form(fname.clone()));
                    let file_path = project_dir.join(&fname);
                    let form = Self::load_form(&file_path, &fname)?;
                    forms.push(form);
                    continue;
                }
                vb6parse::files::project::ProjectFileEntry::UserControl(s) => {
                    ProjectFileEntry::UserControl(s.to_string())
                }
                vb6parse::files::project::ProjectFileEntry::UserDocument(s) => {
                    ProjectFileEntry::UserDocument(s.to_string())
                }
                vb6parse::files::project::ProjectFileEntry::Designer(s) => {
                    ProjectFileEntry::Designer(s.to_string())
                }
                vb6parse::files::project::ProjectFileEntry::RelatedDoc(s) => {
                    ProjectFileEntry::RelatedDoc(s.to_string())
                }
                vb6parse::files::project::ProjectFileEntry::PropertyPage(s) => {
                    ProjectFileEntry::PropertyPage(s.to_string())
                }
            };
            file_entries.push(owned);
        }

        let startup_object = Self::detect_startup_object(&project, &forms, &modules, &classes);

        Ok(LoadedProject {
            project_type,
            project_name,
            forms,
            modules,
            classes,
            file_entries,
            startup_object,
        })
    }

    fn load_form(path: &Path, file_name: &str) -> Result<LoadedForm> {
        let raw_bytes = std::fs::read(path)
            .map_err(|e| anyhow::anyhow!("Failed to read form '{}': {}", path.display(), e))?;
        let source_file = SourceFile::decode_with_replacement(file_name, &raw_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode form '{}': {:?}", file_name, e))?;
        let parsed = FormFile::parse(&source_file).unwrap_or_fail();
        Ok(LoadedForm {
            name: parsed.attributes.name.clone(),
            file_name: file_name.to_string(),
            parsed,
            raw_bytes,
        })
    }

    fn load_module(path: &Path, file_name: &str) -> Result<LoadedModule> {
        let raw_bytes = std::fs::read(path)
            .map_err(|e| anyhow::anyhow!("Failed to read module '{}': {}", path.display(), e))?;
        let source_file = SourceFile::decode_with_replacement(file_name, &raw_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode module '{}': {:?}", file_name, e))?;
        let parsed = ModuleFile::parse(&source_file).unwrap_or_fail();
        Ok(LoadedModule {
            name: parsed.name.clone(),
            file_name: file_name.to_string(),
            parsed,
            raw_bytes,
        })
    }

    fn load_class(path: &Path, file_name: &str) -> Result<LoadedClass> {
        let raw_bytes = std::fs::read(path)
            .map_err(|e| anyhow::anyhow!("Failed to read class '{}': {}", path.display(), e))?;
        let source_file = SourceFile::decode_with_replacement(file_name, &raw_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode class '{}': {:?}", file_name, e))?;
        let parsed = ClassFile::parse(&source_file).unwrap_or_fail();
        Ok(LoadedClass {
            name: parsed.header.attributes.name.clone(),
            file_name: file_name.to_string(),
            parsed,
            raw_bytes,
        })
    }

    fn detect_startup_object(
        project: &ProjectFile<'_>,
        forms: &[LoadedForm],
        modules: &[LoadedModule],
        _classes: &[LoadedClass],
    ) -> StartupObject {
        let startup = project.properties.startup.trim();
        if startup.is_empty() {
            return StartupObject::None;
        }

        let form_names: Vec<&str> = forms.iter().map(|f| f.name.as_str()).collect();
        let module_names: Vec<&str> = modules.iter().map(|m| m.name.as_str()).collect();

        // Check if startup matches a form name
        if form_names.iter().any(|&n| n.eq_ignore_ascii_case(startup)) {
            let matched = form_names
                .iter()
                .find(|&n| n.eq_ignore_ascii_case(startup))
                .unwrap();
            return StartupObject::Form {
                form_name: matched.to_string(),
            };
        }

        // Check if startup contains a dot (e.g. "Module1.SubMain")
        if let Some(dot_pos) = startup.find('.') {
            let mod_name = &startup[..dot_pos];
            let sub_name = &startup[dot_pos + 1..];
            return StartupObject::SubMain {
                module_name: mod_name.to_string(),
                sub_name: sub_name.to_string(),
            };
        }

        // Check if it's "Sub Main" or matches a module name
        if startup.eq_ignore_ascii_case("sub main") || startup.eq_ignore_ascii_case("main") {
            return StartupObject::SubMain {
                module_name: String::new(),
                sub_name: "Main".to_string(),
            };
        }

        // Check if it matches a module name
        if module_names
            .iter()
            .any(|&n| n.eq_ignore_ascii_case(startup))
        {
            return StartupObject::SubMain {
                module_name: startup.to_string(),
                sub_name: "Main".to_string(),
            };
        }

        StartupObject::None
    }
}
