use xtap_util::Shell;

pub type CliError = anyhow::Error;

pub type CliResult<T> = Result<T, CliError>;

pub fn exit_with_error(err: CliError, shell: &mut Shell) -> ! {
    debug!("exit_with_error; err={:?}", err);
    if let Some(clap_err) = err.downcast_ref::<clap::Error>() {
        clap_err.exit()
    }
    display_error(&err, shell);
    std::process::exit(1)
}

/// Displays an error to stderr.
pub fn display_error(err: &CliError, shell: &mut Shell) {
    debug!("display_error; err={:?}", err);
    _display_error(err, shell, true);
}

fn _display_error(err: &CliError, shell: &mut Shell, as_err: bool) -> bool {
    if as_err {
        drop(shell.error(&err));
    } else {
        drop(writeln!(shell.err(), "{}", err));
    }
    false
}
