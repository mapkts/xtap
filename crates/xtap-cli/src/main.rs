#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate xtap_util;

pub mod command_prelude;
pub mod commands;
pub mod errors;
pub mod external;

use crate::command_prelude::*;

use xtap_util::Shell;

fn main() {
    pretty_env_logger::init_custom_env("XTAP_LOG");

    let mut shell = Shell::new();

    if let Err(err) = app(&mut shell) {
        errors::exit_with_error(err, &mut shell);
    }
}

fn app(shell: &mut Shell) -> CliResult {
    let args = match cli().get_matches_safe() {
        Ok(args) => args,
        Err(e) => {
            if e.kind == clap::ErrorKind::UnrecognizedSubcommand {
                // An unrecognized subcommand might be an external subcommand.
                let cmd = &e.info.as_ref().unwrap()[0].to_owned();
                return external::execute_external_subcommand(
                    cmd,
                    &[cmd, "--help"],
                )
                .map_err(|_| e.into());
            } else {
                return Err(e.into());
            }
        }
    };

    let (cmd, subcommand_args) = match args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            cli().print_help()?;
            return Ok(());
        }
    };

    execute_subcommand(shell, cmd, subcommand_args)
}

fn cli() -> App {
    App::new("xtap")
        .about(crate_description!())
        .version(crate_version!())
        .subcommands(commands::builtin())
        .settings(&[
            AppSettings::UnifiedHelpMessage,
            AppSettings::DeriveDisplayOrder,
            AppSettings::VersionlessSubcommands,
            AppSettings::AllowExternalSubcommands,
        ])
        .template(
            "\
{bin} {version}
{about}

USAGE:
    {usage}

OPTIONS:
{unified}

SUBCOMMANDS:
{subcommands}",
        )
        .version_message("Print version infomation")
        .help_message("Print help information")
}

fn execute_subcommand(
    shell: &mut Shell,
    cmd: &str,
    subcommand_args: &ArgMatches<'_>,
) -> CliResult {
    if let Some(exec) = commands::builtin_exec(cmd) {
        return exec(shell, subcommand_args);
    }

    let ext_args: Vec<&str> =
        subcommand_args.values_of("").unwrap_or_default().collect();
    external::execute_external_subcommand(cmd, &ext_args)
}
