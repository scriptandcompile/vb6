use anyhow::Error;
use std::path::{Path, PathBuf};

use anyhow::Result;
use rayon::prelude::*;
use vb6parse::files::project::ProjectReference;
use vb6parse::{ProjectFile, SourceFile};

use walkdir::WalkDir;

pub struct CheckSettings {
    pub project_path: PathBuf,
}

pub struct CheckResults {
    pub project_path: String,
    pub parsing_errors: Vec<Error>,
    pub non_english_files: Vec<String>,
    pub missing_files: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn check_subcommand(check_settings: CheckSettings) -> Result<()> {
    if !check_settings.project_path.exists() {
        println!(
            "No project file found at '{:?}'",
            check_settings.project_path
        );
        return Ok(());
    }

    let mut check_summary = Vec::new();

    if check_settings.project_path.is_dir() {
        let search_path = check_settings.project_path.to_str().unwrap();
        let walker = WalkDir::new(search_path).into_iter();

        println!("Searching '{}' for .vbp project files.", search_path);

        let found_projects: Vec<_> = walker.into_iter().filter(is_project_file).collect();

        found_projects
            .par_iter()
            .map(|project_path| {
                if project_path.is_err() {
                    let check_result = CheckResults {
                        project_path: project_path
                            .as_ref()
                            .unwrap()
                            .path()
                            .to_str()
                            .unwrap()
                            .to_string(),
                        parsing_errors: Vec::new(),
                        non_english_files: Vec::new(),
                        missing_files: vec![format!(
                            "Failed to load {}",
                            project_path.as_ref().err().unwrap()
                        )],
                        warnings: Vec::new(),
                    };

                    return check_result;
                }

                let check_settings = CheckSettings {
                    project_path: project_path.as_ref().unwrap().path().to_path_buf(),
                };

                match check_project(&check_settings) {
                    Ok(result) => result,
                    Err(e) => CheckResults {
                        project_path: check_settings.project_path.to_str().unwrap().to_string(),
                        parsing_errors: vec![e],
                        non_english_files: Vec::new(),
                        missing_files: Vec::new(),
                        warnings: Vec::new(),
                    },
                }
            })
            .collect_into_vec(&mut check_summary);
    } else {
        let check_result = match check_project(&check_settings) {
            Ok(result) => result,
            Err(e) => CheckResults {
                project_path: check_settings.project_path.to_str().unwrap().to_string(),
                parsing_errors: vec![e],
                non_english_files: Vec::new(),
                missing_files: Vec::new(),
                warnings: Vec::new(),
            },
        };
        check_summary.push(check_result);
    }

    for check_result in &check_summary {
        report_check(check_result);
    }

    report_check_summary(check_summary);

    Ok(())
}

fn report_check(check_results: &CheckResults) {
    if check_results.parsing_errors.is_empty()
        && check_results.non_english_files.is_empty()
        && check_results.missing_files.is_empty()
        && check_results.warnings.is_empty()
    {
        return;
    }

    println!("Issues found in '{}':", check_results.project_path);
    if !check_results.missing_files.is_empty() {
        println!("Missing Files:");
        for missing_file in &check_results.missing_files {
            println!("  {}", missing_file);
        }
    }
    if !check_results.parsing_errors.is_empty() {
        println!("Parsing Errors:");
        for error in &check_results.parsing_errors {
            println!("  {}", error);
        }
    }
    if !check_results.non_english_files.is_empty() {
        println!("Non-English Files:");
        for non_english_file in &check_results.non_english_files {
            println!("  {}", non_english_file);
        }
    }
    if !check_results.warnings.is_empty() {
        println!("Warnings:");
        for warning in &check_results.warnings {
            println!("  {}", warning);
        }
    }
}

fn report_single_check_summary(summary: &CheckResults) {
    let mut parts = Vec::new();

    if !summary.missing_files.is_empty() {
        parts.push(format!("{} missing files", summary.missing_files.len()));
    }
    if !summary.non_english_files.is_empty() {
        parts.push(format!(
            "{} unprocessed non-English files",
            summary.non_english_files.len()
        ));
    }
    if !summary.parsing_errors.is_empty() {
        parts.push(format!("{} errors", summary.parsing_errors.len()));
    }
    if !summary.warnings.is_empty() {
        parts.push(format!("{} warnings", summary.warnings.len()));
    }

    if parts.is_empty() {
        println!("No errors found in {}.", summary.project_path);
    } else {
        println!("{} found in {}.", parts.join(", "), summary.project_path);
    }
}

fn report_check_summary(summary: Vec<CheckResults>) {
    if summary.len() == 1 {
        report_single_check_summary(&summary[0]);
        return;
    }

    let project_count = summary.len();

    let total_error_count = summary
        .iter()
        .fold(0, |acc, x| acc + x.parsing_errors.len());

    let total_missed_file_count = summary.iter().fold(0, |acc, x| acc + x.missing_files.len());

    let total_non_english_file_count = summary
        .iter()
        .fold(0, |acc, x| acc + x.non_english_files.len());

    let total_warning_count = summary.iter().fold(0, |acc, x| acc + x.warnings.len());

    let mut parts = Vec::new();

    if total_missed_file_count != 0 {
        parts.push(format!("{} missing files", total_missed_file_count));
    }
    if total_non_english_file_count != 0 {
        parts.push(format!(
            "{} unprocessed non-English files",
            total_non_english_file_count
        ));
    }
    if total_error_count != 0 {
        parts.push(format!("{} errors", total_error_count));
    }
    if total_warning_count != 0 {
        parts.push(format!("{} warnings", total_warning_count));
    }

    if parts.is_empty() {
        println!("No errors found in {} projects.", project_count);
    } else {
        println!("{} found in {} projects.", parts.join(", "), project_count);
    }
}

fn is_project_file(entry: &Result<walkdir::DirEntry, walkdir::Error>) -> bool {
    if entry.is_err() {
        return false;
    }

    let entry = entry.as_ref().unwrap();
    entry.path().extension() == Some("vbp".as_ref())
}

fn join_parent_project_path(parent_project_path: &Path, file_path: &str) -> PathBuf {
    let path = PathBuf::from(parent_project_path);

    if cfg!(target_os = "windows") {
        path.join(file_path)
    } else {
        path.join(file_path.replace("\\", "/"))
    }
}

// TODO: Eventually we should be returning an object that contains the errors and the project information.
// This will allow us to display the errors in a more structured way.
// For now we just print the errors to the console and return the error count.
fn check_project(check_settings: &CheckSettings) -> Result<CheckResults> {
    let mut check_results = CheckResults {
        project_path: check_settings.project_path.to_str().unwrap().to_string(),
        parsing_errors: Vec::new(),
        non_english_files: Vec::new(),
        missing_files: Vec::new(),
        warnings: Vec::new(),
    };

    let project_contents = std::fs::read(&check_settings.project_path).unwrap();

    let file_name = check_settings
        .project_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let source_file = SourceFile::decode_with_replacement(file_name, &project_contents)
        .expect("Unable to decode project file");

    let parse_result = ProjectFile::parse(&source_file);
    let (project, failures) = parse_result.unpack();

    let Some(project) = project else {
        check_results
            .parsing_errors
            .push(anyhow::anyhow!("Failed to parse project file"));
        return Ok(check_results);
    };

    for failure in failures {
        match failure.print_to_string() {
            Ok(text) => check_results
                .parsing_errors
                .push(anyhow::anyhow!("{}", text)),
            Err(_) => check_results
                .parsing_errors
                .push(anyhow::anyhow!("Parse failure: {}", failure)),
        }
    }

    //remove filename from path
    let project_directory = std::path::Path::new(&check_settings.project_path)
        .parent()
        .unwrap();

    let mut source_files_missing = false;

    for reference in project.references() {
        match reference {
            ProjectReference::SubProject { path } => {
                let reference_path = join_parent_project_path(project_directory, path);
                if std::fs::metadata(&reference_path).is_err() {
                    check_results.missing_files.push(format!(
                        "Sub-Project Reference not found: {}",
                        reference_path.to_str().unwrap()
                    ));
                }
            }
            // this should be unreachable, but if it is reached, we just skip it.
            _ => continue,
        }
    }

    for class_reference in project.classes() {
        let class_path = join_parent_project_path(project_directory, class_reference.path);

        if std::fs::metadata(&class_path).is_err() {
            source_files_missing = true;
            check_results
                .missing_files
                .push(format!("Class not found: {}", class_path.to_str().unwrap()));
        }
    }

    for module_reference in project.modules() {
        let module_path = join_parent_project_path(project_directory, module_reference.path);

        if std::fs::metadata(&module_path).is_err() {
            source_files_missing = true;
            check_results.missing_files.push(format!(
                "Module not found: {}",
                module_path.to_str().unwrap()
            ));
        }
    }

    for form_reference in project.forms() {
        let form_path = join_parent_project_path(project_directory, form_reference);

        if std::fs::metadata(&form_path).is_err() {
            source_files_missing = true;
            check_results
                .missing_files
                .push(format!("Form not found: {}", form_path.to_str().unwrap()));
        }
    }

    // Analyze the project with vb6semantic. This resolves names, builds symbol
    // tables, and reports semantic errors and warnings across all of the
    // project's source files.
    if !source_files_missing {
        let mut analyzer = vb6semantic::SemanticAnalyzer::new();
        analyzer.set_base_dir(project_directory);

        match analyzer.analyze_project(&project) {
            Ok(analysis_result) => {
                for error in analysis_result.errors {
                    check_results
                        .parsing_errors
                        .push(anyhow::anyhow!("{}", error));
                }
                check_results.warnings.extend(analysis_result.warnings);
            }
            Err(vb6semantic::SemanticError::FileReadError { file, message }) => {
                check_results
                    .missing_files
                    .push(format!("Failed to read {}: {}", file, message));
            }
            Err(error) => {
                check_results
                    .parsing_errors
                    .push(anyhow::anyhow!("{}", error));
            }
        }
    }

    Ok(check_results)
}
