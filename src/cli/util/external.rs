use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::command_prelude::CommandInfo;
use crate::commands;
use crate::consts::BIN_NAME;
use crate::errors::CliResult;
use crate::util::lev_distance::closest_msg;

pub fn execute_external_command(cmd: &str, args: &[&str]) -> CliResult<()> {
    let path = find_external_subcommand(cmd);
    let command = match path {
        Some(command) => command,
        None => {
            let suggestions = list_commands();
            let did_you_mean = closest_msg(cmd, suggestions.keys(), |c| c);
            let err = anyhow::format_err!(
                "no such subcommand: `{}`{}",
                cmd,
                did_you_mean
            );
            return Err(err);
        }
    };

    match Command::new(&command).args(args).spawn() {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e.into()),
    }
}

fn find_external_subcommand(cmd: &str) -> Option<PathBuf> {
    let command_exe =
        format!("{}-{}{}", BIN_NAME, cmd, env::consts::EXE_SUFFIX);
    search_directories()
        .iter()
        .map(|dir| dir.join(&command_exe))
        .find(|file| is_executable(file))
}

fn search_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(mut p) = env::current_exe() {
        p.pop();
        dirs.push(p);
    }
    if let Some(paths) = env::var_os("PATH") {
        dirs.extend(env::split_paths(&paths));
    }
    dirs
}

#[cfg(unix)]
fn is_executable<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    use std::os::unix::prelude::*;
    fs::metadata(path)
        .map(|metadata| {
            metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
        })
        .unwrap_or(false)
}

#[cfg(windows)]
fn is_executable<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref().is_file()
}

fn list_commands() -> BTreeMap<String, CommandInfo> {
    let prefix = "xtap-";
    let suffix = env::consts::EXE_SUFFIX;
    let mut commands = BTreeMap::new();

    for dir in search_directories() {
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            _ => continue,
        };

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let filename = match path.file_name().and_then(|s| s.to_str()) {
                Some(filename) => filename,
                _ => continue,
            };
            if !filename.starts_with(prefix) || !filename.ends_with(suffix) {
                continue;
            }
            if is_executable(entry.path()) {
                let start = prefix.len();
                let end = filename.len() - suffix.len();
                commands.insert(
                    filename[start..end].to_string(),
                    CommandInfo::External { path: path.clone() },
                );
            }
        }
    }

    // for cmd in commands::builtin() {
    //     commands.insert(
    //         cmd.get_name().to_string(),
    //         CommandInfo::BuiltIn {
    //             about: cmd.p.meta.about.map(|s| s.to_string()),
    //         },
    //     );
    // }

    commands
}
