use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rayon::prelude::*;

use walkdir::WalkDir;

pub struct FmtSettings {
    pub project_path: PathBuf,
    pub check: bool,
    pub indent_size: usize,
}

pub fn fmt_subcommand(settings: FmtSettings) -> Result<()> {
    let project_path = &settings.project_path;

    if !project_path.exists() {
        println!("No project file found at '{:?}'", project_path);
        return Ok(());
    }

    let files_to_format: Vec<PathBuf> = if project_path.is_dir() {
        let mut files = Vec::new();

        let found_projects: Vec<_> = WalkDir::new(project_path)
            .into_iter()
            .filter(is_project_file)
            .collect();

        if found_projects.is_empty() {
            WalkDir::new(project_path)
                .into_iter()
                .filter(is_source_file)
                .for_each(|e| {
                    if let Ok(entry) = e {
                        files.push(entry.path().to_path_buf());
                    }
                });
        } else {
            let nested: Vec<Vec<PathBuf>> = found_projects
                .par_iter()
                .map(|project_path_entry| {
                    project_path_entry
                        .as_ref()
                        .ok()
                        .map(|e| collect_project_files(e.path()))
                        .unwrap_or_default()
                })
                .collect();
            files = nested.into_iter().flatten().collect();
        }

        files
    } else if is_source_file_inner(project_path) {
        vec![project_path.to_path_buf()]
    } else {
        vec![]
    };

    if files_to_format.is_empty() {
        println!("No VB6 source files found.");
        return Ok(());
    }

    let config = find_config(project_path, &files_to_format);
    let indent_size = config
        .and_then(|c| c.fmt)
        .and_then(|f| f.indent_size)
        .unwrap_or(settings.indent_size);

    let results: Vec<(PathBuf, bool)> = files_to_format
        .par_iter()
        .map(|file| {
            let result = process_file(file, indent_size, settings.check);
            (file.clone(), result)
        })
        .map(|(file, result)| match result {
            Ok(changed) => {
                if changed {
                    if settings.check {
                        println!("Would reformat: {}", file.display());
                    } else {
                        println!("Formatted: {}", file.display());
                    }
                    (file, true)
                } else {
                    (file, false)
                }
            }
            Err(e) => {
                eprintln!("Error formatting {}: {}", file.display(), e);
                (file, false)
            }
        })
        .collect();

    let changed_count = results.iter().filter(|(_, changed)| *changed).count();
    let total = results.len();

    if settings.check {
        println!(
            "{} of {} files would be reformatted.",
            changed_count, total
        );
        if changed_count > 0 {
            std::process::exit(1);
        }
    } else {
        println!("Formatted {} of {} files.", changed_count, total);
    }

    Ok(())
}

fn is_project_file(entry: &Result<walkdir::DirEntry, walkdir::Error>) -> bool {
    entry
        .as_ref()
        .is_ok_and(|e| e.path().extension() == Some("vbp".as_ref()))
}

fn is_source_file(entry: &Result<walkdir::DirEntry, walkdir::Error>) -> bool {
    entry
        .as_ref()
        .is_ok_and(|e| is_source_file_inner(e.path()))
}

fn is_source_file_inner(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("bas" | "cls" | "frm")
    )
}

fn collect_project_files(project_path: &Path) -> Vec<PathBuf> {
    let project_contents = match std::fs::read(project_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let file_name = project_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown.vbp");

    let source_file = match vb6parse::SourceFile::decode_with_replacement(file_name, &project_contents)
    {
        Ok(sf) => sf,
        Err(_) => return Vec::new(),
    };

    let parse_result = vb6parse::ProjectFile::parse(&source_file);
    let (project_opt, _failures) = parse_result.unpack();
    let Some(project) = project_opt else {
        return Vec::new();
    };

    let project_directory = match project_path.parent() {
        Some(d) => d,
        None => return Vec::new(),
    };

    let mut files = Vec::new();

    for form in project.forms() {
        let form_path = join_parent_project_path(project_directory, form);
        if form_path.exists() {
            files.push(form_path);
        }
    }

    for class_ref in project.classes() {
        let class_path = join_parent_project_path(project_directory, class_ref.path);
        if class_path.exists() {
            files.push(class_path);
        }
    }

    for module_ref in project.modules() {
        let module_path = join_parent_project_path(project_directory, module_ref.path);
        if module_path.exists() {
            files.push(module_path);
        }
    }

    files
}

fn join_parent_project_path(parent_project_path: &Path, file_path: &str) -> PathBuf {
    let path = PathBuf::from(parent_project_path);
    if cfg!(target_os = "windows") {
        path.join(file_path)
    } else {
        path.join(file_path.replace("\\", "/"))
    }
}

fn process_file(path: &Path, indent_size: usize, check_only: bool) -> Result<bool> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let formatted = fmt_source(&source, indent_size)
        .with_context(|| format!("Failed to format {}", path.display()))?;

    if formatted == source {
        return Ok(false);
    }

    if !check_only {
        std::fs::write(path, &formatted)
            .with_context(|| format!("Failed to write {}", path.display()))?;
    }

    Ok(true)
}

pub fn fmt_source(source: &str, indent_size: usize) -> Result<String> {
    let _parse_result = vb6parse::ConcreteSyntaxTree::from_text("fmt_input", source);
    let (cst_opt, _failures) = _parse_result.unpack();

    let _cst = cst_opt
        .ok_or_else(|| anyhow::anyhow!("Failed to parse source code"))?;

    Ok(reindent_source(source, indent_size))
}

fn reindent_source(source: &str, indent_size: usize) -> String {
    let mut output = String::new();
    let mut indent = 0usize;

    let lines: Vec<&str> = source.lines().collect();
    let source_ends_with_newline = source.ends_with('\n');

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let is_last = i == lines.len() - 1;

        if trimmed.is_empty() {
            output.push('\n');
            continue;
        }

        let first_word = trimmed.split([' ', '\t']).next().unwrap_or("");

        let is_decrease =
            is_closing_keyword(first_word) && (i == 0 || !is_continuation(lines[i - 1]));

        if is_decrease {
            indent = indent.saturating_sub(1);
        }

        output.push_str(&" ".repeat(indent * indent_size));
        output.push_str(trimmed);
        if !is_last || source_ends_with_newline {
            output.push('\n');
        }

        let is_increase = is_opening_keyword(trimmed, first_word)
            && !is_single_line_if(trimmed)
            && (is_last || !is_continuation(trimmed));

        if is_increase {
            indent += 1;
        }
    }

    output
}

fn is_continuation(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.ends_with('_') && !trimmed.ends_with("__")
}

fn is_closing_keyword(first_word: &str) -> bool {
    matches!(
        first_word,
        "End" | "Next" | "Loop" | "Wend" | "Else" | "ElseIf" | "Case"
    )
}

fn is_opening_keyword(trimmed: &str, first_word: &str) -> bool {
    let upper = first_word;

    if matches!(
        upper,
        "Sub" | "Function" | "Property" | "Type" | "Enum"
    ) {
        return true;
    }

    if matches!(upper, "For" | "Do" | "While" | "With") {
        return true;
    }

    if upper == "If" && trimmed.contains("Then") {
        return true;
    }

    if upper == "Select" && trimmed.contains("Case") {
        return true;
    }

    if upper == "Else" || upper == "ElseIf" {
        return true;
    }

    if upper == "Case" {
        return true;
    }

    if matches!(upper, "Private" | "Public" | "Friend") {
        if let Some(second) = trimmed.split([' ', '\t']).nth(1) {
            if matches!(second, "Sub" | "Function" | "Property" | "Type" | "Enum") {
                return true;
            }
        }
    }

    false
}

fn is_single_line_if(trimmed: &str) -> bool {
    let trimmed = trimmed.trim();
    if !trimmed.starts_with("If ") && !trimmed.starts_with("If\t") {
        return false;
    }

    let has_then = trimmed.contains(" Then ");
    if !has_then {
        return false;
    }

    let after_then = trimmed.split(" Then ").nth(1).unwrap_or("");
    let has_newline_after_then = after_then.contains('\n');

    !has_newline_after_then
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FmtConfig {
    fmt: Option<FmtSection>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FmtSection {
    indent_size: Option<usize>,
}

fn find_config(project_path: &Path, _files: &[PathBuf]) -> Option<FmtConfig> {
    let config_paths = [
        project_path.join(".aspen.toml"),
        PathBuf::from(".aspen.toml"),
    ];

    for path in &config_paths {
        if path.exists() {
            let contents = std::fs::read_to_string(path).ok()?;
            let config: FmtConfig = toml::from_str(&contents).ok()?;
            return Some(config);
        }
    }

    None
}
