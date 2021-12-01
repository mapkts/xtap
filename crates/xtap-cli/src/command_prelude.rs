use clap::SubCommand;
use std::ffi::OsString;
use std::path::PathBuf;

pub use crate::errors::{CliError, CliResult};

pub use clap::{AppSettings, Arg, ArgMatches};

pub type App = clap::App<'static, 'static>;

pub fn subcommand(name: &'static str) -> App {
    SubCommand::with_name(name).settings(&[
        AppSettings::UnifiedHelpMessage,
        AppSettings::DeriveDisplayOrder,
        AppSettings::DontCollapseArgsInUsage,
        AppSettings::AllowExternalSubcommands,
    ])
}

pub fn opt(name: &'static str, help: &'static str) -> Arg<'static, 'static> {
    Arg::with_name(name).long(name).help(help)
}

pub fn optional_opt(
    name: &'static str,
    help: &'static str,
) -> Arg<'static, 'static> {
    opt(name, help).min_values(0)
}

pub fn multi_opt(
    name: &'static str,
    value_name: &'static str,
    help: &'static str,
) -> Arg<'static, 'static> {
    opt(name, help).multiple(true).value_name(value_name).number_of_values(1)
}

pub fn optional_multi_opt(
    name: &'static str,
    value_name: &'static str,
    help: &'static str,
) -> Arg<'static, 'static> {
    opt(name, help)
        .multiple(true)
        .value_name(value_name)
        .min_values(0)
        .number_of_values(1)
}

// pub fn values(args: &ArgMatches<'_>, name: &str) -> Vec<String> {
//     args._values_of(name)
// }

// pub fn values_os(args: &ArgMatches<'_>, name: &str) -> Vec<OsString> {
//     args._values_of_os(name)
// }

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandInfo {
    BuiltIn { about: Option<String> },
    External { path: PathBuf },
}
