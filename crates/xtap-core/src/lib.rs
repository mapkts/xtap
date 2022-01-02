mod drop;
mod merger;
mod reader;
mod record;

pub use crate::drop::Drop;
pub use crate::merger::{Merger, MergerBuilder};
pub use crate::record::{Field, Record, RecordIter};

/// The whitespace preservation behaviour.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Trim {
    /// Preserves fields and headers.
    None,
    /// Trim whitespace from headers.
    Headers,
    /// Trim whitespace from fields, but not headers.
    Fields,
    /// Trim whitespace from fields and headers.
    All,
}

impl Trim {
    fn should_trim_fields(&self) -> bool {
        self == &Trim::Fields || self == &Trim::All
    }

    fn should_trim_headers(&self) -> bool {
        self == &Trim::Headers || self == &Trim::All
    }
}

impl Default for Trim {
    fn default() -> Self {
        Trim::None
    }
}
