use crate::command_prelude::*;
use crate::errors::CliResult;

use xtap_util::Shell;

pub mod list;

pub fn builtin() -> Vec<App> {
    vec![list::cli()]
}

pub fn builtin_exec(
    cmd: &str,
) -> Option<fn(&mut Shell, &ArgMatches<'_>) -> CliResult> {
    let f = match cmd {
        "list" => list::exec,
        _ => return None,
    };
    Some(f)
}
