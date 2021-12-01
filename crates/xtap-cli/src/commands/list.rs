use crate::command_prelude::*;
use crate::external;

use xtap_util::Shell;

pub fn cli() -> App {
    subcommand("list").about("List all subcommands")
}

pub fn exec(shell: &mut Shell, _: &ArgMatches<'_>) -> CliResult {
    let (builtin_commands, external_commands): (Vec<_>, Vec<_>) =
        external::list_commands().into_iter().partition(|(_, v)| match v {
            CommandInfo::BuiltIn { .. } => true,
            CommandInfo::External { .. } => false,
        });

    drop_println!(shell, "Built-in Commands:");
    builtin_commands.into_iter().for_each(|(name, info)| {
        if let CommandInfo::BuiltIn { about } = info {
            let summary = about.unwrap_or_default();
            // display only the first line
            let summary = summary.lines().next().unwrap_or(&summary);
            drop_println!(shell, "    {:<20} {}", name, summary);
        }
    });

    drop_println!(shell, "Found External Commands:");
    external_commands.into_iter().for_each(|(name, info)| {
        if let CommandInfo::External { path } = info {
            drop_println!(shell, "    {:<20} {}", name, path.display());
        }
    });

    Ok(())
}
