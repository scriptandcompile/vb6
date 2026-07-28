use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rayon::prelude::*;

use walkdir::WalkDir;

pub use vb6format::FmtSettings;

pub struct FmtCommand {
    pub project_path: PathBuf,
    pub check: bool,
    pub fmt_settings: FmtSettings,
    pub cli_blank_around: Option<bool>,
    pub cli_blank_inside: Option<bool>,
}

pub fn fmt_subcommand(cmd: FmtCommand) -> Result<()> {
    let project_path = &cmd.project_path;

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
    let cfg_fmt = config.and_then(|c| c.fmt);

    let indent_size = cfg_fmt
        .as_ref()
        .and_then(|f| f.indent_size)
        .unwrap_or(cmd.fmt_settings.indent_size);

    let blank_around = cmd
        .cli_blank_around
        .or(cfg_fmt
            .as_ref()
            .and_then(|f| f.blank_lines_around_directives))
        .unwrap_or(false);

    let blank_inside = cmd
        .cli_blank_inside
        .or(cfg_fmt
            .as_ref()
            .and_then(|f| f.blank_lines_inside_directives))
        .unwrap_or(false);

    let fmt_settings = FmtSettings {
        indent_size,
        blank_lines_around_directives: blank_around,
        blank_lines_inside_directives: blank_inside,
    };

    let results: Vec<(PathBuf, bool)> = files_to_format
        .par_iter()
        .map(|file| {
            let result = process_file(file, &fmt_settings, cmd.check);
            (file.clone(), result)
        })
        .map(|(file, result)| match result {
            Ok(changed) => {
                if changed {
                    if cmd.check {
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

    if cmd.check {
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

    let source_file =
        match vb6parse::SourceFile::decode_with_replacement(file_name, &project_contents) {
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

fn process_file(path: &Path, fmt_settings: &FmtSettings, check_only: bool) -> Result<bool> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let formatted = vb6format::fmt_source(&source, fmt_settings)
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

#[derive(Debug, Clone, serde::Deserialize)]
struct FmtConfig {
    fmt: Option<FmtSection>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct FmtSection {
    indent_size: Option<usize>,
    blank_lines_around_directives: Option<bool>,
    blank_lines_inside_directives: Option<bool>,
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
