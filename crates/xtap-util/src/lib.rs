mod lev_distance;
mod macros;
mod shell;

pub use lev_distance::{closest, closest_msg};
pub use shell::{ColorChoice, Shell, TtyWidth};
