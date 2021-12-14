use std::fs::File;
use std::io;
use std::path::Path;

use crate::{Record, Trim};

/// A xlsx/csv file merger.
#[derive(Debug)]
pub struct Merger {
    /// The files to merge.
    sources: Vec<File>,
    /// The underlying csv data reader.
    csv_rdr: Option<io::BufReader<File>>,
    /// The underlying xlsx data reader.
    xlsx_rdr: Option<io::BufReader<File>>,
    /// The tracking state.
    state: MergerState,
}

#[derive(Debug)]
struct MergerState {
    /// This contains the headers of any xlsx/csv data, if any.
    ///
    /// Note that the `headers` here aren't necessary corresponding to the first row in a xlsx/csv
    /// data source, especially when the `skip` option has been set.
    headers: Option<Record>,
    /// Indicates whether the `sources` contain headers.
    has_headers: bool,
    /// The maximum number of fields found.
    max_field_count: Option<u32>,
    /// The number of fields in the first record in each source.
    first_field_count: Option<u32>,
    /// Various skip options.
    skip: Option<Skip>,
    /// Indicates whether the presence of ending newline in each source should be forced.
    force_ending_newline: Option<Newline>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skip {
    /// Skips a number of rows from the head of each source.
    head: usize,
    /// Skips a number of rows from the tail of each source.
    tail: usize,
    /// Skips any row if its length is not the longest.
    non_max_length: bool,
    /// Skips any row if its length is shorter than the given one.
    length_less_than: Option<usize>,
    /// Skips any row if its `nth` (zero-based) field is empty.
    ///
    /// Both `Field::Binary([])` and `Field::Empty` are considered empty.
    field_nth_is_empty: Option<usize>,
}

/// The style of a newline, either unix-style `\n` or dos-style `\r\n`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Newline {
    /// Unix-style `\n`.
    Lf,
    /// Dos-style `\r\n`.
    Crlf,
}

/// A builder used for building a custom merger.
#[derive(Debug)]
pub struct MergerBuilder {
    /// The capacity of `io::BufReader`.
    capacity: usize,
    /// Whether data contains headers.
    has_headers: bool,
    /// The whitespace trim behaviour.
    trim: Trim,
    /// Various skip options.
    skip: Option<Skip>,
    /// Newline style.
    newline: Newline,
}

impl Default for MergerBuilder {
    fn default() -> Self {
        MergerBuilder {
            capacity: 8 * (1 << 10),
            has_headers: true,
            trim: Trim::default(),
            skip: Default::default(),
            #[cfg(windows)]
            newline: Newline::Crlf,
            #[cfg(not(windows))]
            newline: Newline::Lf,
        }
    }
}

impl MergerBuilder {
    /// Create a new merger builder.
    ///
    /// To convert a builder into a merger, call one of the methods starting with `from_`.
    pub fn new() -> MergerBuilder {
        MergerBuilder::default()
    }
}
