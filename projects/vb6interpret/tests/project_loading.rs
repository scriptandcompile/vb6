//! Integration tests for VBP project loading and `run_project()`.

use std::fs;
use tempfile::TempDir;
use vb6interpret::{Interpreter, LoadedProject};

fn temp_dir() -> TempDir {
    tempfile::tempdir().expect("create temp dir")
}

fn write_file(dir: &TempDir, name: &str, content: &str) {
    let path = dir.path().join(name);
    fs::write(&path, content).unwrap_or_else(|_| panic!("write {}", name));
}

fn write_vbp(dir: &TempDir, content: &str) {
    write_file(dir, "project.vbp", content);
}

fn load_project(dir: &TempDir) -> LoadedProject {
    let vbp_path = dir.path().join("project.vbp");
    LoadedProject::load(&vbp_path).expect("load project")
}

#[test]
fn load_single_module_vbp() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Module1.bas",
        "Attribute VB_Name = \"Module1\"\nSub Main()\nDebug.Print \"hello\"\nEnd Sub\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Module=Module1; Module1.bas\n",
    );

    let project = load_project(&dir);
    assert_eq!(project.modules.len(), 1);
    assert_eq!(project.modules[0].name, "Module1");
    assert_eq!(project.forms.len(), 0);
}

#[test]
fn load_form_vbp() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Form1.frm",
        "VERSION 5.00\nBegin VB.Form Form1\nEnd\nAttribute VB_Name = \"Form1\"\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Form=Form1.frm\n\
         Startup=\"Form1\"\n",
    );

    let project = load_project(&dir);
    assert_eq!(project.forms.len(), 1);
    assert_eq!(project.forms[0].name, "Form1");
}

#[test]
fn load_submain_vbp() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Module1.bas",
        "Attribute VB_Name = \"Module1\"\nSub Main()\nDebug.Print \"main\"\nEnd Sub\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Module=Module1; Module1.bas\n\
         Startup=\"Sub Main\"\n",
    );

    let project = load_project(&dir);
    assert_eq!(project.modules.len(), 1);
}

#[test]
fn run_project_console_output() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Module1.bas",
        "Attribute VB_Name = \"Module1\"\nSub Main()\nDebug.Print \"hello\"\nEnd Sub\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Module=Module1; Module1.bas\n\
         Startup=\"Sub Main\"\n",
    );

    let project = load_project(&dir);
    let mut interpreter = Interpreter::new();
    let _ = interpreter.run_project(&project);
    assert_eq!(interpreter.output(), &vec!["hello".to_string()]);
}

#[test]
fn load_project_with_multiple_modules() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Module1.bas",
        "Attribute VB_Name = \"Module1\"\nSub Main()\nDebug.Print \"mod1\"\nEnd Sub\n",
    );
    write_file(
        &dir,
        "Module2.bas",
        "Attribute VB_Name = \"Module2\"\nSub Helper()\nDebug.Print \"mod2\"\nEnd Sub\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Module=Module1; Module1.bas\n\
         Module=Module2; Module2.bas\n\
         Startup=\"Sub Main\"\n",
    );

    let project = load_project(&dir);
    assert_eq!(project.modules.len(), 2);
    let names: Vec<&str> = project.modules.iter().map(|m| m.name.as_str()).collect();
    assert!(names.contains(&"Module1"));
    assert!(names.contains(&"Module2"));
}

#[test]
fn missing_file_returns_error() {
    let dir = temp_dir();
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Module=Module1; NonExistent.bas\n",
    );

    let result = LoadedProject::load(&dir.path().join("project.vbp"));
    assert!(result.is_err());
}

#[test]
fn load_class_vbp() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Class1.cls",
        "VERSION 5.00\nAttribute VB_Name = \"Class1\"\nSub DoIt()\nEnd Sub\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Class=Class1; Class1.cls\n",
    );

    let project = load_project(&dir);
    assert_eq!(project.classes.len(), 1);
    assert_eq!(project.classes[0].name, "Class1");
}

#[test]
fn startup_object_form_detection() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Form1.frm",
        "VERSION 5.00\nBegin VB.Form Form1\nEnd\nAttribute VB_Name = \"Form1\"\n",
    );
    write_file(
        &dir,
        "Module1.bas",
        "Attribute VB_Name = \"Module1\"\nSub Main()\nEnd Sub\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Form=Form1.frm\n\
         Module=Module1; Module1.bas\n\
         Startup=\"Form1\"\n",
    );

    let project = load_project(&dir);
    match &project.startup_object {
        vb6interpret::StartupObject::Form { form_name } => {
            assert_eq!(form_name, "Form1");
        }
        _ => panic!("Expected Form startup"),
    }
}

#[test]
fn startup_object_submain_detection() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Module1.bas",
        "Attribute VB_Name = \"Module1\"\nSub Main()\nEnd Sub\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Module=Module1; Module1.bas\n\
         Startup=\"Sub Main\"\n",
    );

    let project = load_project(&dir);
    match &project.startup_object {
        vb6interpret::StartupObject::SubMain { sub_name, .. } => {
            assert_eq!(sub_name, "Main");
        }
        _ => panic!("Expected SubMain startup"),
    }
}

#[test]
fn run_project_multiple_modules_merged() {
    let dir = temp_dir();
    write_file(
        &dir,
        "Module1.bas",
        "Attribute VB_Name = \"Module1\"\nSub Main()\nCall Helper\nDebug.Print \"done\"\nEnd Sub\n",
    );
    write_file(
        &dir,
        "Module2.bas",
        "Attribute VB_Name = \"Module2\"\nSub Helper()\nDebug.Print \"helper called\"\nEnd Sub\n",
    );
    write_vbp(
        &dir,
        "TYPE=Exe\n\
         Module=Module1; Module1.bas\n\
         Module=Module2; Module2.bas\n\
         Startup=\"Sub Main\"\n",
    );

    let project = load_project(&dir);
    let mut interpreter = Interpreter::new();
    let _ = interpreter.run_project(&project);
    let output = interpreter.output();
    assert_eq!(output.len(), 2);
    assert_eq!(output[0], "helper called");
    assert_eq!(output[1], "done");
}
