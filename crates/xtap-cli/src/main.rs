#![warn(rust_2018_idioms)]

#[macro_use]
extern crate log;

pub mod command_prelude;
pub mod commands;
pub mod errors;
pub mod external;

use xtap_util::{self, Shell};

fn main() {
    pretty_env_logger::init_custom_env("XTAP_LOG");

    if let Err(err) = external::execute_external_command("fcd", &["dl"]) {
        let mut shell = Shell::new();
        errors::exit_with_error(err, &mut shell);
    }
}
