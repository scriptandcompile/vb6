use anyhow::Error;
use std::path::{Path, PathBuf};

use anyhow::Result;
use rayon::prelude::*;
use vb6parse::files::project::ProjectReference;
use vb6parse::lint::LintSettings;
use vb6parse::{ProjectFile, SourceFile};

use walkdir::WalkDir;

pub struct CheckSettings {
    pub project_path: PathBuf,
    pub lint: LintSettings,
}

/// Runs the selected lint rules over the files a project refers to.
///
/// The same file can be shared by several projects -- in the code base this
/// was written against one module is referenced by eight of them -- so a
/// finding is reported once per project that includes it. Deduplication is
/// left to the summary, where the whole run is visible.
fn run_lint_rules(paths: &[PathBuf], settings: &LintSettings) -> Vec<String> {
    let mut findings: Vec<String> = paths
        .par_iter()
        .flat_map(|path| {
            let Ok(source) = SourceFile::from_file(path) else {
                // Unreadable files are already reported as missing or as a
                // parse failure; do not say it twice.
                return Vec::new();
            };

            vb6parse::lint::lint_source(source.as_ref(), settings)
                .into_iter()
                .map(|finding| {
                    format!(
                        "{}:{}:{}: {} {}",
                        path.display(),
                        finding.line,
                        finding.column,
                        finding.code,
                        finding.message
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    findings.sort();
    findings
}

pub struct CheckResults {
    pub project_path: String,
    pub parsing_errors: Vec<Error>,
    pub non_english_files: Vec<String>,
    pub missing_files: Vec<String>,
    pub warnings: Vec<String>,
    /// Findings from the lint rules, already formatted for display.
    pub lint_findings: Vec<String>,
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

    let lint = check_settings.lint.clone();

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
                        lint_findings: Vec::new(),
                    };

                    return check_result;
                }

                let check_settings = CheckSettings {
                    project_path: project_path.as_ref().unwrap().path().to_path_buf(),
                    lint: lint.clone(),
                };

                match check_project(&check_settings) {
                    Ok(result) => result,
                    Err(e) => CheckResults {
                        project_path: check_settings.project_path.to_str().unwrap().to_string(),
                        parsing_errors: vec![e],
                        non_english_files: Vec::new(),
                        missing_files: Vec::new(),
                        warnings: Vec::new(),
                        lint_findings: Vec::new(),
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
                lint_findings: Vec::new(),
            },
        };
        check_summary.push(check_result);
    }

    for check_result in &check_summary {
        report_check(check_result);
    }

    let anything_found = check_summary.iter().any(|result| {
        !result.parsing_errors.is_empty()
            || !result.missing_files.is_empty()
            || !result.non_english_files.is_empty()
            || !result.lint_findings.is_empty()
    });

    report_check_summary(check_summary);

    // Reporting problems and then exiting zero makes `check` useless as a CI
    // gate: the gate passes in exactly the case where it should fail. The
    // convention is ruff's -- 1 means the run worked and found something.
    if anything_found {
        std::process::exit(1);
    }

    Ok(())
}

fn report_check(check_results: &CheckResults) {
    if check_results.parsing_errors.is_empty()
        && check_results.non_english_files.is_empty()
        && check_results.missing_files.is_empty()
        && check_results.warnings.is_empty()
        && check_results.lint_findings.is_empty()
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
    if !check_results.lint_findings.is_empty() {
        println!("Lint:");
        for finding in &check_results.lint_findings {
            println!("  {}", finding);
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
    if !summary.lint_findings.is_empty() {
        parts.push(format!("{} lint findings", summary.lint_findings.len()));
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

    let total_lint_count = summary.iter().fold(0, |acc, x| acc + x.lint_findings.len());

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
    if total_lint_count != 0 {
        parts.push(format!("{} lint findings", total_lint_count));
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
        lint_findings: Vec::new(),
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

    // Every source file the project refers to, for the lint rules to run over.
    let mut source_paths: Vec<PathBuf> = Vec::new();

    for class_reference in project.classes() {
        let class_path = join_parent_project_path(project_directory, class_reference.path);

        if std::fs::metadata(&class_path).is_err() {
            source_files_missing = true;
            check_results
                .missing_files
                .push(format!("Class not found: {}", class_path.to_str().unwrap()));
        } else {
            source_paths.push(class_path);
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
        } else {
            source_paths.push(module_path);
        }
    }

    for form_reference in project.forms() {
        let form_path = join_parent_project_path(project_directory, form_reference);

        if std::fs::metadata(&form_path).is_err() {
            source_files_missing = true;
            check_results
                .missing_files
                .push(format!("Form not found: {}", form_path.to_str().unwrap()));
        } else {
            source_paths.push(form_path);
        }
    }

    check_results.lint_findings = run_lint_rules(&source_paths, &check_settings.lint);

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

/// The `[lint]` section of `.aspen.toml`, alongside the `[fmt]` section that
/// is already read from the same file.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct LintConfig {
    #[serde(default)]
    pub select: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct AspenConfig {
    lint: Option<LintConfig>,
}

/// Reads the `[lint]` section next to the given path, then from the working
/// directory, the same way the formatter finds its own settings.
#[must_use]
pub fn load_lint_settings(project_path: &Path) -> LintConfig {
    let config_root = if project_path.is_dir() {
        project_path.to_path_buf()
    } else {
        project_path
            .parent()
            .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
    };

    let candidates = [
        config_root.join(".aspenfmt.toml"),
        config_root.join(".aspen.toml"),
        PathBuf::from(".aspenfmt.toml"),
        PathBuf::from(".aspen.toml"),
    ];

    for path in &candidates {
        if !path.exists() {
            continue;
        }

        let Ok(contents) = std::fs::read_to_string(path) else {
            continue;
        };

        match toml::from_str::<AspenConfig>(&contents) {
            Ok(config) => return config.lint.unwrap_or_default(),
            Err(e) => {
                // A malformed config that is silently ignored looks exactly
                // like a config that selected nothing.
                eprintln!("Ignoring {}: {}", path.display(), e);
            }
        }
    }

    LintConfig::default()
}

/// Prints every rule with its default and fixability.
pub fn explain_rules() {
    println!(
        "{:<6} {:<24} {:<8} {:<8} {}",
        "CODE", "NAME", "DEFAULT", "FIX", "SUMMARY"
    );

    for rule in vb6parse::lint::RULES {
        println!(
            "{:<6} {:<24} {:<8} {:<8} {}",
            rule.code,
            rule.name,
            if rule.default_on { "on" } else { "off" },
            match rule.fixability {
                vb6parse::lint::Fixability::Safe => "safe",
                vb6parse::lint::Fixability::Unsafe => "unsafe",
                vb6parse::lint::Fixability::None => "none",
            },
            rule.summary
        );
    }
}
