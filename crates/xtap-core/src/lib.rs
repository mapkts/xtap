pub mod csv;
pub mod reader;
pub mod xlsx;

mod merge;
mod record;

pub use crate::record::{Field, Record, RecordIter};
