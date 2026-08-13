mod check;
mod fmt;

use check::check_subcommand;
use fmt::fmt_subcommand;

use anyhow::Result;

use std::{env::current_dir, path::PathBuf};

use clap::{Arg, Command, builder::PossibleValue, command, value_parser};

fn main() -> Result<()> {
    let matches = command!()
        .subcommand(
            Command::new("check")
                .about("Check the project")
                .arg(
                    Arg::new("select")
                        .long("select")
                        .required(false)
                        // One value per flag, repeatable, comma-separated. Not
                        // `num_args(1..)`: that swallows the positional path.
                        .num_args(1)
                        .action(clap::ArgAction::Append)
                        .value_delimiter(',')
                        .help("lint rule codes or code prefixes to run, e.g. N001 or N"),
                )
                .arg(
                    Arg::new("ignore")
                        .long("ignore")
                        .required(false)
                        .num_args(1)
                        .action(clap::ArgAction::Append)
                        .value_delimiter(',')
                        .help("lint rule codes or code prefixes to skip"),
                )
                .arg(
                    Arg::new("explain")
                        .long("explain")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                        .help("list every lint rule with its default and fixability"),
                )
                .arg(
                    Arg::new("project path")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("fmt")
                .about("Format VB6 source files")
                .arg(
                    Arg::new("check")
                        .long("check")
                        .short('c')
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                        .help("only check if files are formatted, don't write"),
                )
                .arg(
                    Arg::new("keyword")
                        .long("keyword")
                        .short('K')
                        .required(false)
                        .value_parser([
                            PossibleValue::new("upper")
                                .help("format keywords in uppercase: ELSEIF"),
                            PossibleValue::new("lower")
                                .help("format keywords in lowercase: elseif"),
                            PossibleValue::new("camel")
                                .help("format keywords in camel case: ElseIf"),
                            PossibleValue::new("first")
                                .help("format keywords with the first letter in uppercase: Elseif"),
                        ])
                        .help("format keywords in the source files"),
                )
                .arg(
                    Arg::new("indent-size")
                        .long("indent-size")
                        .short('i')
                        .required(false)
                        .value_parser(value_parser!(usize))
                        .help("indentation size in spaces"),
                )
                .arg(
                    Arg::new("blank-lines-around-directives")
                        .long("blank-lines-around-directives")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                        .help("insert blank line before #If and after #End If"),
                )
                .arg(
                    Arg::new("blank-lines-inside-directives")
                        .long("blank-lines-inside-directives")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                        .help(
                            "insert blank lines between #If/#ElseIf/#Else/#End If and their bodies",
                        ),
                )
                .arg(
                    Arg::new("blank-lines-around-top-level")
                        .long("blank-lines-around-top-level")
                        .required(false)
                        .action(clap::ArgAction::SetTrue)
                        .help(
                            "insert blank lines between procedures and other top-level constructs",
                        ),
                )
                .arg(
                    Arg::new("project path")
                        .required(false)
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .arg_required_else_help(true)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("check") {
        let current_dir = current_dir()?;

        let project_path = matches
            .get_one::<PathBuf>("project path")
            .unwrap_or(&current_dir)
            .to_path_buf();

        if matches.get_flag("explain") {
            check::explain_rules();
            return Ok(());
        }

        let configured = check::load_lint_settings(&project_path);
        let from_cli = |name: &str| -> Option<Vec<String>> {
            matches
                .get_many::<String>(name)
                .map(|values| values.cloned().collect())
        };

        let select = from_cli("select").unwrap_or(configured.select);
        let ignore = from_cli("ignore").unwrap_or(configured.ignore);

        let unknown: Vec<&String> = select
            .iter()
            .chain(ignore.iter())
            .filter(|code| {
                !vb6parse::lint::RULES
                    .iter()
                    .any(|rule| rule.code.starts_with(code.as_str()))
            })
            .collect();

        if !unknown.is_empty() {
            // A typo in a rule code must not quietly select nothing.
            anyhow::bail!(
                "unknown rule code(s): {}. `aspen check --explain` lists them.",
                unknown
                    .iter()
                    .map(|code| code.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        let check_settings = check::CheckSettings {
            project_path,
            lint: vb6parse::lint::LintSettings::from_selection(&select, &ignore),
        };

        check_subcommand(check_settings)?;

        return Ok(());
    }

    if let Some(matches) = matches.subcommand_matches("fmt") {
        let current_dir = current_dir()?;

        let project_path = matches
            .get_one::<PathBuf>("project path")
            .unwrap_or(&current_dir)
            .to_path_buf();

        let cli_check = matches.get_flag("check");
        let cli_indent_size = matches.get_one::<usize>("indent-size").copied();
        let cli_blank_around = matches.get_flag("blank-lines-around-directives");
        let cli_blank_inside = matches.get_flag("blank-lines-inside-directives");
        let cli_blank_around_top_level = matches.get_flag("blank-lines-around-top-level");
        let cli_keyword_case = matches.get_one::<String>("keyword").cloned();
        let settings = fmt::load_fmt_settings(&project_path);

        let cmd = fmt::FmtCommand {
            cli: fmt::CliSettings {
                project_path,
                check: cli_check,
                keyword_case: cli_keyword_case,
                indent_size: cli_indent_size,
                blank_lines_around_directives: cli_blank_around.then_some(true),
                blank_lines_inside_directives: cli_blank_inside.then_some(true),
                blank_lines_around_top_level: cli_blank_around_top_level.then_some(true),
            },
            settings,
        };

        fmt_subcommand(cmd)?;

        return Ok(());
    }

    println!("Unknown subcommand");

    Ok(())
}
