mod lev_distance;
mod shell;

pub use lev_distance::{closest, closest_msg};
pub use shell::{ColorChoice, Shell, TtyWidth};
