#[macro_export]
macro_rules! __shell_print {
    ($shell:expr, $which:ident, $newline:literal, $($arg:tt)*) => ({
        let out = $shell.$which();
        drop(out.write_fmt(format_args!($($arg)*)));
        if $newline {
            drop(out.write_all(b"\n"));
        }
    });
}

#[macro_export]
macro_rules! drop_println {
    ($shell:expr) => ( $crate::drop_print!($shell, "\n") );
    ($shell:expr, $($arg:tt)*) => (
        $crate::__shell_print!($shell, out, true, $($arg)*)
    );
}

#[macro_export]
macro_rules! drop_eprintln {
    ($shell:expr) => ( $crate::drop_eprint!($shell, "\n") );
    ($shell:expr, $($arg:tt)*) => (
        $crate::__shell_print!($shell, err, true, $($arg)*)
    );
}

#[macro_export]
macro_rules! drop_print {
    ($shell:expr, $($arg:tt)*) => (
        $crate::__shell_print!($shell, out, false, $($arg)*)
    );
}

#[macro_export]
macro_rules! drop_eprint {
    ($shell:expr, $($arg:tt)*) => (
        $crate::__shell_print!($shell, err, false, $($arg)*)
    );
}
