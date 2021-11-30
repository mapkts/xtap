use std::path::PathBuf;

pub use clap::{AppSettings, Arg, ArgMatches};

pub type App = clap::App<'static, 'static>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandInfo {
    BuiltIn { about: Option<String> },
    External { path: PathBuf },
}
