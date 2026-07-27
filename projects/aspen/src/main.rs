mod check;

use check::check_subcommand;

use anyhow::Result;

use std::{env::current_dir, path::PathBuf};

use clap::{command, value_parser, Arg, Command};

fn main() -> Result<()> {
    let matches = command!()
        .subcommand(
            Command::new("check")
                .about("Check the project")
                .arg(
                    Arg::new("ignore forms")
                        .short('f')
                        .long("form")
                        .alias("forms")
                        .required(false)
                        .value_parser(value_parser!(bool))
                        .action(clap::ArgAction::SetFalse)
                        .help("skip checking the forms listed in the project"),
                )
                .arg(
                    Arg::new("ignore modules")
                        .short('m')
                        .long("module")
                        .alias("modules")
                        .required(false)
                        .value_parser(value_parser!(bool))
                        .action(clap::ArgAction::SetFalse)
                        .help("skip checking the modules listed in the project"),
                )
                .arg(
                    Arg::new("ignore classes")
                        .short('c')
                        .long("class")
                        .alias("classes")
                        .required(false)
                        .value_parser(value_parser!(bool))
                        .action(clap::ArgAction::SetFalse)
                        .help("skip checking the classes listed in the project"),
                )
                .arg(
                    Arg::new("ignore references")
                        .short('r')
                        .long("reference")
                        .alias("references")
                        .required(false)
                        .value_parser(value_parser!(bool))
                        .action(clap::ArgAction::SetFalse)
                        .help("skip checking the references listed in the project"),
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

        let check_forms = *matches.get_one::<bool>("ignore forms").unwrap_or(&false);
        let check_modules = *matches.get_one::<bool>("ignore modules").unwrap_or(&false);
        let check_classes = *matches.get_one::<bool>("ignore classes").unwrap_or(&false);
        let check_references = *matches
            .get_one::<bool>("ignore references")
            .unwrap_or(&false);

        let check_settings = check::CheckSettings {
            project_path,
            check_forms,
            check_modules,
            check_classes,
            check_references,
        };

        check_subcommand(check_settings)?;

        return Ok(());
    }

    println!("Unknown subcommand");

    Ok(())
}
