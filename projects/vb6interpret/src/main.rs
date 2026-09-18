//! vb6-interpret: VB6 interpreter CLI
//!
//! Execute VB6 code directly without compilation.

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[cfg(all(feature = "tauri", not(target_arch = "wasm32")))]
use std::sync::OnceLock;

#[cfg(feature = "tauri")]
use vb6runtime::layout::scale::twips_to_pixels;

#[cfg(feature = "tauri")]
use vb6interpret::tauri_cmds;

use vb6interpret::{Interpreter, LoadedProject, StartupObject, project};
use vb6parse::errors::{ErrorKind, SourceFileError};
use vb6parse::files::ModuleFile;
use vb6parse::files::project::properties::CompileTargetType;
use vb6parse::io::SourceFile;
use vb6runtime::VBVariant;

#[derive(Parser)]
#[command(name = "vb6-interpret")]
#[command(about = "Visual Basic 6 Interpreter", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show execution trace
    #[arg(short, long)]
    trace: bool,

    /// Enable profiling
    #[arg(short, long)]
    profile: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a VB6 file or project
    Run {
        /// Path to file or project
        path: PathBuf,

        /// Set initial variables (VAR=VALUE)
        #[arg(long)]
        set: Vec<String>,

        /// Execution timeout in seconds (0 = no timeout)
        #[arg(long, default_value = "0")]
        timeout: u64,

        /// Resource (.res) file the LoadRes* functions read from
        #[arg(long, value_name = "FILE")]
        res: Option<PathBuf>,
    },

    /// Start interactive REPL
    Repl,

    /// Run with debugger
    Debug {
        /// Path to file or project
        path: PathBuf,

        /// Initial breakpoints
        #[arg(long)]
        r#break: Vec<String>,
    },

    /// Check syntax without executing
    Check {
        /// Path to file or project
        path: PathBuf,
    },
}

fn main() {
    let result = run_cli();
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run {
            path,
            set,
            timeout,
            res,
        }) => {
            let path = expand_tilde(&path);
            match path.extension().and_then(|e| e.to_str()) {
                Some("bas") => run_bas_file(&path, &set, timeout, res.as_deref(), cli.trace)?,
                Some("vbp") => {
                    let project = project::LoadedProject::load(&path)?;
                    match project.project_type {
                        CompileTargetType::Exe => {}
                        other => bail!("Unsupported project type: {:?}", other),
                    }
                    match &project.startup_object {
                        StartupObject::SubMain { .. } => {
                            return run_console_project(project, &set, timeout, res.as_deref());
                        }
                        StartupObject::None => {
                            bail!("No startup object found in project");
                        }
                        StartupObject::Form { form_name: _ } => {
                            let html = project.render_startup_form()?;
                            run_form_project(project, html)?;
                        }
                    }
                }
                Some(ext) => bail!("Unsupported file type: .{}", ext),
                None => run_vbp_in_cwd(&set, timeout, res.as_deref())?,
            }
        }
        Some(Commands::Repl) => {
            #[cfg(feature = "repl")]
            run_repl()?;
            #[cfg(not(feature = "repl"))]
            eprintln!("REPL not enabled. Compile with --features repl");
        }
        None => run_vbp_in_cwd(&[], 0, None)?,
        Some(Commands::Debug { path, r#break }) => {
            let path = expand_tilde(&path);
            println!("TODO: Debug {}", path.display());
            println!("  Breakpoints: {:?}", r#break);
        }
        Some(Commands::Check { path }) => {
            let path = expand_tilde(&path);
            let source_file = read_source_file(&path)?;
            let module = ModuleFile::parse(&source_file).unwrap_or_fail();
            if cli.verbose {
                println!("Parsed {} OK", module.name);
            } else {
                println!("OK");
            }
        }
    }

    Ok(())
}

/// Tauri entry point — runs the Tauri application with form-rendering commands.
///
/// When the `tauri` feature is enabled, this function is called from `main()`
/// and never returns. The Tauri runtime handles the event loop.

#[cfg(feature = "repl")]
fn run_repl() -> Result<()> {
    let mut editor = rustyline::DefaultEditor::new()
        .map_err(|e| anyhow::anyhow!("Failed to create REPL editor: {}", e))?;
    let mut interpreter = Interpreter::new();

    loop {
        match editor.readline("vb6> ") {
            Ok(line) => {
                let trimmed = line.trim();
                match trimmed {
                    "" => continue,
                    ".quit" | ".exit" => break,
                    ".help" => print_repl_help(),
                    ".clear" => {
                        interpreter.clear();
                        println!("State cleared.");
                    }
                    ".globals" => {
                        for (name, value) in interpreter.globals().iter() {
                            println!("{} = {}", name, value);
                        }
                        if interpreter.globals().is_empty() {
                            println!("(no global variables)");
                        }
                    }
                    ".run" => match interpreter.run_startup() {
                        Ok(()) => print_output(&interpreter),
                        Err(e) => eprintln!("Error: {}", e.error),
                    },
                    s if s.starts_with(".run ") => {
                        let path = expand_tilde(Path::new(&s[5..]));
                        match load_source_into(&mut interpreter, &path) {
                            Ok(()) => {
                                print_output(&interpreter);
                                match interpreter.run_startup() {
                                    Ok(()) => print_output(&interpreter),
                                    Err(e) => eprintln!("Error: {}", e.error),
                                }
                            }
                            Err(e) => eprintln!("Error loading '{}': {}", path.display(), e),
                        }
                    }
                    ".load" => {
                        println!("Usage: .load <path>");
                    }
                    s if s.starts_with(".load ") => {
                        let path_str = &s[6..];
                        let path = expand_tilde(Path::new(path_str));
                        match load_source_into(&mut interpreter, &path) {
                            Ok(()) => println!("Loaded {}", path.display()),
                            Err(e) => eprintln!("Error loading '{}': {}", path.display(), e),
                        }
                    }
                    code => match interpreter.run_source(code) {
                        Ok(()) => print_output(&interpreter),
                        Err(e) => eprintln!("Error: {}", e.error),
                    },
                }
                let _ = editor.add_history_entry(trimmed);
            }
            Err(_) => break,
        }
    }
    Ok(())
}

#[cfg(feature = "repl")]
fn print_repl_help() {
    println!("REPL commands:");
    println!("  .load <path>   Load a .bas or .vbp file");
    println!("  .run [path]    Run the current or loaded file");
    println!("  .clear         Clear all state");
    println!("  .globals       List global variables");
    println!("  .help          Show this help");
    println!("  .quit/.exit    Exit REPL");
    println!();
    println!("Any other input is treated as VB6 code and executed immediately.");
}

#[cfg(feature = "repl")]
fn load_source_into(interpreter: &mut Interpreter, path: &Path) -> Result<()> {
    let source_file = SourceFile::from_file(path)
        .map_err(|e| anyhow::anyhow!("Failed to read '{}': {}", path.display(), e))?;
    match path.extension().and_then(|e| e.to_str()) {
        Some("bas") => {
            let module = ModuleFile::parse(&source_file).unwrap_or_fail();
            interpreter.merge_module(&module)?;
        }
        Some("vbp") => {
            let project = project::LoadedProject::load(path)?;
            interpreter.merge_project(&project)?;
        }
        Some(ext) => bail!("Unsupported file type: .{}", ext),
        None => bail!("No file extension. Expected .bas or .vbp"),
    }
    Ok(())
}

fn run_bas_file(
    path: &Path,
    set: &[String],
    timeout: u64,
    res: Option<&Path>,
    trace: bool,
) -> Result<()> {
    let source_file = read_source_file(path)?;
    let module = ModuleFile::parse(&source_file).unwrap_or_fail();

    let mut interpreter = Interpreter::new();
    if timeout > 0 {
        interpreter.set_step_limit(u64::MAX);
    }
    if let Some(res) = &res {
        interpreter.set_resource_file(expand_tilde(res).to_string_lossy().to_string());
    }
    for assignment in set {
        let (name, raw) = assignment
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("Invalid --set '{assignment}' (expected VAR=VALUE)"))?;
        interpreter.set_global(name, parse_value(raw));
    }

    let started = Instant::now();
    let result = interpreter.run_module(&module);
    if trace {
        eprintln!("{:?} statements executed", interpreter.steps());
    }
    let timed_out = match result {
        Ok(()) => timeout > 0 && started.elapsed() > Duration::from_secs(timeout),
        Err(error) => {
            print_output(&interpreter);
            eprintln!("Runtime error: {}", error.error);
            if let Some(report) = vb6interpret::error::render_error_report(
                &path.display().to_string(),
                source_file.as_ref(),
                &error,
                module.line_offset,
            ) {
                eprintln!();
                eprintln!("{report}");
            }
            std::process::exit(1);
        }
    };
    print_output(&interpreter);
    if timed_out {
        bail!("Execution timed out after {}s", timeout);
    }
    Ok(())
}

#[allow(unused_variables)]
fn run_form_project(project: LoadedProject, startup_form_html: (String, u32)) -> Result<!> {
    #[cfg(feature = "tauri")]
    {
        launch_tauri(project, startup_form_html);
    }
    #[cfg(not(feature = "tauri"))]
    {
        bail!(
            "{} is a Form application and requires the tauri feature. Rebuild with --features tauri",
            project.project_name
        )
    }
}

#[cfg(feature = "tauri")]
fn launch_tauri(project: LoadedProject, startup_form_html: (String, u32)) -> ! {
    use tauri::Manager;
    use tauri::generate_handler;

    // Extract startup form name before the project is moved into the engine,
    // and calculate the window size from the form's total dimensions
    // (including title bar, borders, and scrollbars).
    let (startup_form_name, window_width, window_height) = match &project.startup_object {
        StartupObject::Form { form_name } => {
            let form = project.forms.iter().find(|f| f.name == *form_name);
            let (w, h) = form
                .map(|f| match &f.parsed.form {
                    vb6parse::language::FormRoot::Form(frm) => {
                        let dpi = 96;
                        let w = twips_to_pixels(frm.properties.client_width, dpi) as f64;
                        let h = twips_to_pixels(frm.properties.client_height, dpi) as f64;
                        (w.max(10.0), h.max(10.0))
                    }
                    vb6parse::language::FormRoot::MDIForm(mdi) => {
                        let dpi = 96;
                        let w = twips_to_pixels(mdi.properties.width, dpi) as f64;
                        let h = twips_to_pixels(mdi.properties.height, dpi) as f64;
                        (w.max(10.0), h.max(10.0))
                    }
                })
                .unwrap_or((10.0, 10.0));
            (form_name.clone(), w, h)
        }
        _ => (String::new(), 10.0, 10.0),
    };

    let engine_handle = tauri_cmds::spawn_engine(project);

    // Bake the form HTML, CSS, and inline IPC script into a single HTML page
    // that the webview loads through the `vb6://` custom protocol below.
    let (form_html, form_handle) = startup_form_html;
    let css = vb6runtime::layout::vb6_css::bare_css();
    let _ = FORM_PAGE.set(vb6interpret::tauri_html::build_page(
        &form_html,
        &css,
        &startup_form_name,
        engine_handle,
        form_handle,
    ));

    tauri::Builder::default()
        .register_uri_scheme_protocol("vb6", |_ctx, _request| {
            use tauri::http::{
                StatusCode,
                header::{CONTENT_TYPE, HeaderValue},
            };

            let body = FORM_PAGE.get().cloned().unwrap_or_default();
            tauri::http::Response::builder()
                .header(
                    CONTENT_TYPE,
                    HeaderValue::from_static("text/html; charset=utf-8"),
                )
                .body(body.into_bytes())
                .unwrap_or_else(|e| {
                    tauri::http::Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(format!("failed to build response: {e}").into_bytes())
                        .unwrap()
                })
        })
        .invoke_handler(generate_handler![
            tauri_cmds::update_form,
            tauri_cmds::run_project,
            tauri_cmds::stop_engine,
            tauri_cmds::form_event,
            tauri_cmds::form_event_bindings,
            tauri_cmds::get_output,
            tauri_cmds::set_variable,
            tauri_cmds::get_variable,
        ])
        .setup(move |app| {
            app.manage(engine_handle);

            // Custom protocol URLs are served per platform:
            // - macOS/iOS/Linux (WebKit):  vb6://localhost/index.html
            // - Windows/Android (WebView2): http://vb6.localhost/index.html
            #[cfg(any(windows, target_os = "android"))]
            let page_url = tauri::WebviewUrl::CustomProtocol(
                "http://vb6.localhost/index.html".parse().unwrap(),
            );
            #[cfg(not(any(windows, target_os = "android")))]
            let page_url =
                tauri::WebviewUrl::CustomProtocol("vb6://localhost/index.html".parse().unwrap());

            // Create a new window — there is no pre-created window (windows: [] in tauri.conf.json).
            // The page is served by the `vb6://` custom protocol registered above,
            // so the webview loads a real document (with the form already in the
            // DOM) instead of an `about:blank` shell that needs JS injection.
            let window = tauri::WebviewWindowBuilder::new(app, "vb6interpret", page_url)
                .title("VB6Interpret")
                .inner_size(window_width, window_height)
                .resizable(true)
                .build()
                .expect("failed to create webview window");

            let _ = window.set_size(tauri::PhysicalSize::new(window_width, window_height));
            let _ = window.show();
            let _ = window.set_focus();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri");

    unreachable!()
}

/// The complete HTML page served by the `vb6://` custom protocol.
#[cfg(feature = "tauri")]
static FORM_PAGE: OnceLock<String> = OnceLock::new();

fn run_console_project(
    project: LoadedProject,
    set: &[String],
    timeout: u64,
    res: Option<&Path>,
) -> Result<()> {
    let mut interpreter = Interpreter::new();
    if timeout > 0 {
        interpreter.set_step_limit(u64::MAX);
    }
    if let Some(res) = &res {
        interpreter.set_resource_file(expand_tilde(res).to_string_lossy().to_string());
    }
    for assignment in set {
        let (name, raw) = assignment
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("Invalid --set '{assignment}' (expected VAR=VALUE)"))?;
        interpreter.set_global(name, parse_value(raw));
    }

    let result = interpreter.run_project(&project);
    print_output(&interpreter);
    result.map_err(|e| anyhow::anyhow!("Runtime error: {}", e.error))?;
    Ok(())
}

fn run_vbp_in_cwd(set: &[String], timeout: u64, res: Option<&Path>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let vbps: Vec<_> = cwd
        .read_dir()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some("vbp".as_ref()))
        .collect();

    match vbps.len() {
        0 => bail!("No .vbp file found in current directory"),
        1 => {
            let project = project::LoadedProject::load(&vbps[0].path())?;
            match project.project_type {
                CompileTargetType::Exe => {}
                other => bail!("Unsupported project type: {:?}", other),
            }
            match &project.startup_object {
                StartupObject::SubMain { .. } => run_console_project(project, set, timeout, res),
                StartupObject::None => {
                    bail!("No startup object found in project");
                }
                StartupObject::Form { form_name: _ } => {
                    let html = project.render_startup_form()?;
                    run_form_project(project, html)?;
                }
            }
        }
        _ => {
            let paths: Vec<String> = vbps
                .iter()
                .map(|e| e.path().display().to_string())
                .collect();
            bail!(
                "Multiple .vbp files found: {}\nSpecify a file path.",
                paths.join(", ")
            )
        }
    }
}

/// Write the interpreter's captured output to stdout, ensuring the output ends
/// with a newline (needed even when the last `Print` used a trailing `;`).
fn print_output(interpreter: &Interpreter) {
    let text = interpreter.output_text();
    print!("{text}");
    if !text.is_empty() && !text.ends_with('\n') {
        println!();
    }
    let _ = std::io::stdout().flush();
}

/// Read a source file, producing an actionable error with a "did you mean"
/// suggestion when the path does not exist.
fn read_source_file(path: &Path) -> Result<SourceFile> {
    match SourceFile::from_file(path) {
        Ok(source) => Ok(source),
        Err(e) => match path.try_exists() {
            // The path exists but could not be read (directory, permissions, ...).
            Ok(true) => {
                let reason = match &*e.kind {
                    ErrorKind::SourceFile(SourceFileError::Malformed { message }) => {
                        message.as_str()
                    }
                    _ => "not a readable file",
                };
                Err(anyhow::anyhow!(
                    "Failed to read {}: {reason}",
                    path.display()
                ))
            }
            _ => {
                let mut message = format!("Failed to find file '{}'", path.display());
                let suggestions = suggest_similar_path(path);
                if !suggestions.is_empty() {
                    message.push_str("\nDid you mean:");
                    for suggestion in &suggestions {
                        message.push_str(&format!("\n    {}", clean_path(suggestion)));
                    }
                }
                Err(anyhow::anyhow!(message))
            }
        },
    }
}

/// Candidate paths near `requested` whose names resemble the requested file
/// name, ranked by closeness.
fn suggest_similar_path(requested: &Path) -> Vec<PathBuf> {
    let Some(file_name) = requested.file_name().and_then(|s| s.to_str()) else {
        return Vec::new();
    };
    let dir = match requested.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    };

    let mut candidates: Vec<(usize, PathBuf)> = Vec::new();
    scan_dir_for_similar(&dir, file_name, &mut candidates);

    // Nothing in the target directory: look one level down.
    if candidates.is_empty()
        && dir.is_dir()
        && let Ok(entries) = std::fs::read_dir(&dir)
    {
        for entry in entries.flatten() {
            let sub = entry.path();
            if sub.is_dir() {
                scan_dir_for_similar(&sub, file_name, &mut candidates);
            }
        }
    }

    candidates.sort_by_key(|(distance, _)| *distance);
    candidates
        .into_iter()
        .take(3)
        .map(|(_, path)| path)
        .collect()
}

/// Collect files in `dir` whose names are within an edit-distance budget of
/// `file_name`, ranked by distance.
fn scan_dir_for_similar(dir: &Path, file_name: &str, out: &mut Vec<(usize, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let distance = levenshtein(file_name, &name);
        if distance <= file_name.len().max(name.len()) / 2 {
            out.push((distance, path));
        }
    }
}

/// Case-insensitive Levenshtein edit distance.
fn levenshtein(left: &str, right: &str) -> usize {
    let left: Vec<char> = left.to_lowercase().chars().collect();
    let right: Vec<char> = right.to_lowercase().chars().collect();
    let mut prev: Vec<usize> = (0..=right.len()).collect();
    for (i, left_char) in left.iter().enumerate() {
        let mut curr = vec![i + 1];
        for (j, right_char) in right.iter().enumerate() {
            let cost = if left_char == right_char { 0 } else { 1 };
            curr.push((prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost));
        }
        prev = curr;
    }
    prev[right.len()]
}

/// A path rendered for display, without a leading `./`.
fn clean_path(path: &Path) -> String {
    let text = path.display().to_string();
    text.strip_prefix("./").map(str::to_string).unwrap_or(text)
}

/// Expand a leading `~`/`~/` into the user's home directory, matching what a
/// shell would do for unquoted paths. Bash does not expand `~` inside double
/// quotes, so CLI users often pass it through literally.
fn expand_tilde(path: &Path) -> PathBuf {
    let text = path.to_string_lossy();
    let home = std::env::var_os("HOME");
    match text.strip_prefix("~/") {
        Some(rest) => home
            .map(|h| PathBuf::from(h).join(rest))
            .unwrap_or_else(|| path.to_path_buf()),
        None if text == "~" => home
            .map(PathBuf::from)
            .unwrap_or_else(|| path.to_path_buf()),
        None => path.to_path_buf(),
    }
}

/// Parse a `--set` value into a runtime value (Long, Double, or String).
fn parse_value(raw: &str) -> VBVariant {
    if let Ok(long) = raw.parse::<i64>() {
        return VBVariant::from_i64(long);
    }
    if let Ok(double) = raw.parse::<f64>() {
        return VBVariant::from_double(double);
    }
    VBVariant::from_string(raw.to_string())
}
