pub mod csv;
pub mod reader;
pub mod xlsx;

mod merger;
mod record;

pub use crate::merger::{Merger, MergerBuilder};
pub use crate::record::{Field, Record, RecordIter};
