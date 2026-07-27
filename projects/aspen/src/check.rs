use anyhow::Error;
use std::path::{Path, PathBuf};

use anyhow::Result;
use rayon::prelude::*;
use vb6parse::files::project::ProjectReference;
use vb6parse::{ClassFile, FormFile, ModuleFile, ProjectFile, SourceFile};

use walkdir::WalkDir;

pub struct CheckSettings {
    pub project_path: PathBuf,
    pub check_forms: bool,
    pub check_modules: bool,
    pub check_classes: bool,
    pub check_references: bool,
}

pub struct CheckResults {
    pub project_path: String,
    pub parsing_errors: Vec<Error>,
    pub non_english_files: Vec<String>,
    pub missing_files: Vec<String>,
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
                    };

                    return check_result;
                }

                let check_settings = CheckSettings {
                    project_path: project_path.as_ref().unwrap().path().to_path_buf(),
                    check_forms: check_settings.check_forms,
                    check_modules: check_settings.check_modules,
                    check_classes: check_settings.check_classes,
                    check_references: check_settings.check_references,
                };

                let check_result = match check_project(&check_settings) {
                    Ok(result) => result,
                    Err(e) => {
                        let check_result = CheckResults {
                            project_path: check_settings.project_path.to_str().unwrap().to_string(),
                            parsing_errors: vec![e],
                            non_english_files: Vec::new(),
                            missing_files: Vec::new(),
                        };

                        return check_result;
                    }
                };
                check_result
            })
            .collect_into_vec(&mut check_summary);
    } else {
        let check_result = match check_project(&check_settings) {
            Ok(result) => result,
            Err(e) => {
                let check_result = CheckResults {
                    project_path: check_settings.project_path.to_str().unwrap().to_string(),
                    parsing_errors: vec![e],
                    non_english_files: Vec::new(),
                    missing_files: Vec::new(),
                };

                check_result
            }
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
    {
        return;
    }

    println!("Errors found in '{}':", check_results.project_path);
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
}

fn report_single_check_summary(summary: &CheckResults) {
    // 0, 0, 0
    if summary.parsing_errors.is_empty()
        && summary.non_english_files.is_empty()
        && summary.missing_files.is_empty()
    {
        println!("No errors found in {}.", summary.project_path);
        return;
    }

    // 0, 0, 1
    if summary.parsing_errors.is_empty()
        && summary.non_english_files.is_empty()
        && !summary.missing_files.is_empty()
    {
        println!(
            "{} missing files in {}.",
            summary.missing_files.len(),
            summary.project_path
        );
        return;
    }

    // 0, 1, 0
    if summary.parsing_errors.is_empty()
        && !summary.non_english_files.is_empty()
        && summary.missing_files.is_empty()
    {
        println!(
            "{} unprocessed non-English files found in the project.",
            summary.non_english_files.len()
        );
        return;
    }

    // 0, 1, 1
    if summary.parsing_errors.is_empty()
        && !summary.non_english_files.is_empty()
        && !summary.missing_files.is_empty()
    {
        println!(
            "{} missing files, {} unprocessed non-English files found in the project.",
            summary.missing_files.len(),
            summary.non_english_files.len()
        );
        return;
    }

    // 1, 0, 0
    if !summary.parsing_errors.is_empty()
        && summary.non_english_files.is_empty()
        && summary.missing_files.is_empty()
    {
        println!(
            "{} errors found in the project.",
            summary.parsing_errors.len()
        );
        return;
    }

    // 1, 0, 1
    if !summary.parsing_errors.is_empty()
        && summary.non_english_files.is_empty()
        && !summary.missing_files.is_empty()
    {
        println!(
            "{} missing files, {} errors found in the project.",
            summary.missing_files.len(),
            summary.parsing_errors.len()
        );
        return;
    }

    // 1, 1, 0
    if !summary.parsing_errors.is_empty()
        && !summary.non_english_files.is_empty()
        && summary.missing_files.is_empty()
    {
        println!(
            "{} errors found in project with {} unprocessed non-English files found in the project.",
            summary.parsing_errors.len(),
            summary.non_english_files.len()
        );
        return;
    }

    // 1, 1, 1
    if !summary.parsing_errors.is_empty()
        && !summary.non_english_files.is_empty()
        && !summary.missing_files.is_empty()
    {
        println!(
            "{} missing files, {} errors found in project with {} unprocessed non-English files found in the project.",
            summary.missing_files.len(),
            summary.parsing_errors.len(),
            summary.non_english_files.len()
        );
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

    // 0, 0, 0
    if total_error_count == 0 && total_non_english_file_count == 0 && total_missed_file_count == 0 {
        println!("No errors found in {} projects.", project_count);
        return;
    }

    // 0, 0, 1
    if total_error_count == 0 && total_non_english_file_count == 0 && total_missed_file_count != 0 {
        println!(
            "{} missing files in {} projects",
            total_non_english_file_count, project_count
        );
        return;
    }

    // 0, 1, 0
    if total_error_count == 0 && total_non_english_file_count != 0 && total_missed_file_count == 0 {
        println!(
            "{} unprocessed non-English files found in {} projects",
            total_non_english_file_count, project_count
        );
        return;
    }

    // 0, 1, 1
    if total_error_count == 0 && total_non_english_file_count != 0 && total_missed_file_count != 0 {
        println!(
            "{} missing files, {} unprocessed non-English files found in {} projects",
            total_missed_file_count, total_non_english_file_count, project_count
        );
        return;
    }

    // 1, 0, 0
    if total_error_count != 0 && total_non_english_file_count == 0 && total_missed_file_count == 0 {
        println!(
            "{} errors found in {} projects.",
            total_error_count, project_count
        );
        return;
    }

    // 1, 0, 1
    if total_error_count != 0 && total_non_english_file_count == 0 && total_missed_file_count != 0 {
        println!(
            "{} missing files, {} errors found in {} projects.",
            total_missed_file_count, total_error_count, project_count
        );
        return;
    }

    // 1, 1, 0
    if total_error_count != 0 && total_non_english_file_count != 0 && total_missed_file_count == 0 {
        println!(
            "{} errors, {} unprocessed non-English files found in {} projects.",
            total_error_count, total_non_english_file_count, project_count
        );
        return;
    }

    // 1, 1, 1
    if total_error_count != 0 && total_non_english_file_count != 0 && total_missed_file_count != 0 {
        println!(
            "{} missing files, {} errors, {} unprocessed non-English files found in {} projects.",
            total_missed_file_count, total_error_count, total_non_english_file_count, project_count
        );
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
        check_results
            .parsing_errors
            .push(anyhow::anyhow!("Parse failure: {}", failure));
    }

    //remove filename from path
    let project_directory = std::path::Path::new(&check_settings.project_path)
        .parent()
        .unwrap();

    if check_settings.check_references {
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
    }

    if check_settings.check_classes {
        for class_reference in project.classes() {
            let class_path = join_parent_project_path(project_directory, class_reference.path);

            if std::fs::metadata(&class_path).is_err() {
                check_results
                    .missing_files
                    .push(format!("Class not found: {}", class_path.to_str().unwrap()));

                continue;
            }

            let file_name = class_path.file_name().unwrap().to_str().unwrap();
            let class_contents = std::fs::read(&class_path).unwrap();
            let source_file = match SourceFile::decode_with_replacement(file_name, &class_contents)
            {
                Ok(sf) => sf,
                Err(err) => {
                    check_results.parsing_errors.push(anyhow::anyhow!(
                        "Failed to decode class file {}: {}",
                        file_name,
                        err
                    ));
                    continue;
                }
            };

            let parse_result = ClassFile::parse(&source_file);
            let (_, failures) = parse_result.unpack();

            for failure in failures {
                check_results
                    .parsing_errors
                    .push(anyhow::anyhow!("Class parse error: {}", failure));
            }
        }
    }

    if check_settings.check_modules {
        for module_reference in project.modules() {
            let module_path = join_parent_project_path(project_directory, module_reference.path);

            if std::fs::metadata(&module_path).is_err() {
                check_results.missing_files.push(format!(
                    "Module not found: {}",
                    module_path.to_str().unwrap()
                ));

                continue;
            }

            let file_name = module_path.file_name().unwrap().to_str().unwrap();
            let module_contents = std::fs::read(&module_path).unwrap();
            let source_file = match SourceFile::decode_with_replacement(file_name, &module_contents)
            {
                Ok(sf) => sf,
                Err(err) => {
                    check_results.parsing_errors.push(anyhow::anyhow!(
                        "Failed to decode module file {}: {}",
                        file_name,
                        err
                    ));
                    continue;
                }
            };

            let parse_result = ModuleFile::parse(&source_file);
            let (_, failures) = parse_result.unpack();

            for failure in failures {
                check_results
                    .parsing_errors
                    .push(anyhow::anyhow!("Module parse error: {}", failure));
            }
        }
    }

    if check_settings.check_forms {
        for form_reference in project.forms() {
            let form_path = join_parent_project_path(project_directory, form_reference);

            if std::fs::metadata(&form_path).is_err() {
                check_results
                    .missing_files
                    .push(format!("Form not found: {}", form_path.to_str().unwrap()));

                continue;
            }

            let file_name = form_path.file_name().unwrap().to_str().unwrap();
            let form_contents = std::fs::read(&form_path).unwrap();
            let source_file = match SourceFile::decode_with_replacement(file_name, &form_contents) {
                Ok(sf) => sf,
                Err(err) => {
                    check_results.parsing_errors.push(anyhow::anyhow!(
                        "Failed to decode form file {}: {}",
                        file_name,
                        err
                    ));
                    continue;
                }
            };

            let parse_result = FormFile::parse(&source_file);
            let (_, failures) = parse_result.unpack();

            for failure in failures {
                check_results
                    .parsing_errors
                    .push(anyhow::anyhow!("Form parse error: {}", failure));
            }
        }
    }

    Ok(check_results)
}
