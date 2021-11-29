#![warn(rust_2018_idioms)]
#![allow(clippy::all)]

#[macro_use]
extern crate log;

pub mod util;
pub mod consts;
pub mod errors;
pub mod commands;
pub mod command_prelude;

fn main() {
    pretty_env_logger::init_custom_env("XTAP_LOG");

    if let Err(err) = util::external::execute_external_command("fcd", &["dl"]) {
        let mut shell = util::Shell::new();
        errors::exit_with_error(err, &mut shell);
    }
}
