use crate::{Record, Trim};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

/// A xlsx/csv file merger.
#[derive(Debug)]
pub struct Merger<R> {
    /// The files to merge.
    sources: Vec<R>,
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
    /// Newline style.
    newline: Newline,
    /// Indicates whether the presence of ending newline in each source should be forced.
    force_ending_newline: bool,
    /// Capacity of the `rdr`.
    capacity: usize,
}

impl Default for MergerState {
    fn default() -> MergerState {
        MergerState {
            capacity: 8 * (1 << 10),
            has_headers: true,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Skip {
    /// Skips a number of rows from the head of each source. The second field of this tuple
    /// indicates whether the head of the first source should be preserved or not.
    head: (usize, bool),
    /// Skips a number of rows from the tail of each source. The second field of this tuple
    /// indicates whether the tail of the last source should be preserved or not.
    tail: (usize, bool),
    /// Skips any row if its length is not the longest.
    non_max_length: bool,
    /// Skips any row if its length is shorter than the given one.
    length_less_than: Option<usize>,
    /// Skips any row if its `nth` (zero-based) field is empty.
    ///
    /// Both `Field::Binary([])` and `Field::Empty` are considered empty.
    fields_is_empty: Option<Vec<usize>>,
}

/// The style of a newline, either unix-style `\n` or dos-style `\r\n`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Newline {
    /// Unix-style `\n`.
    Lf,
    /// Dos-style `\r\n`.
    Crlf,
}

impl<R: Read> Merger<R> {
    /// Creates a new merger with default configuration for the given readers.
    ///
    /// To build a custom merger, use `MergerBuilder`.
    pub fn from_readers(readers: Vec<R>) -> Merger<R> {
        Merger { sources: readers, state: Default::default() }
    }

    /// Returns a new [`MergerBuilder`] for configuring a custom merger.
    pub fn builder() -> MergerBuilder {
        MergerBuilder::new()
    }

    /// Merges the contents of the underlying readers into the given `wtr` in the given `format`.
    ///
    /// Note that the given `wtr` is buffered automatically, so you should not wrap `wtr` in a
    /// buffered writer like `io::BufWriter`.
    pub fn into_writer<W: Write>(self, wtr: W, format: Format) -> Result<()> {
        todo!()
    }

    /// Merges the contents of the underlying readers into the given file path.
    ///
    /// This function will overwrite the contents of the given file path.
    pub fn into_path<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let mut file =
            OpenOptions::new().write(true).truncate(true).open(path)?;
        todo!()
    }
}

impl Merger<File> {
    pub fn from_paths<P: AsRef<Path>>(paths: Vec<P>) -> Result<Merger<File>> {
        let files: Result<Vec<File>> =
            paths.into_iter().map(|p| File::open(p)).collect();
        Ok(Merger { sources: files?, state: Default::default() })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Csv,
    Xlsx,
    Bytes,
}

/// A builder used for configuring a custom merger.
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
    /// Whether the presence of ending newline in each source should be forced.
    force_ending_newline: bool,
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
            force_ending_newline: false,
        }
    }
}

impl MergerBuilder {
    /// Create a new merger builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use xtap_core::{Trim, MergerBuilder};
    ///
    /// let readers = vec!["from reader 1".as_bytes(), "from reader 2".as_bytes()];
    ///
    /// let mut merger = MergerBuilder::new()
    ///     .trim(Trim::All)
    ///     .force_ending_newline(true)
    ///     .from_readers(readers);
    /// ```
    pub fn new() -> MergerBuilder {
        MergerBuilder::default()
    }

    /// Sets the capacity (in bytes) of the buffer used in reading xlsx/csv files.
    ///
    /// The default buffer capacity is 8KB currently.
    pub fn buffer_capacity(&mut self, capacity: usize) -> &mut Self {
        self.capacity = capacity;
        self
    }

    /// Whether to treat the first row as a special header row.
    ///
    /// By default, the first row is treated as a special header row, which means the header is
    /// never returned by any of the record reading methods or record iterators.
    pub fn has_headers(&mut self, has_headers: bool) -> &mut Self {
        self.has_headers = has_headers;
        self
    }

    /// Whether fields are trimmed of leading and trailing whitespace.
    ///
    /// By default. no trimming is performed. When reading, only characters meeting the definition
    /// of ASCII whitespace (`[\t\n\v\f\r]`) are trimmed.
    pub fn trim(&mut self, trim: Trim) -> &mut Self {
        self.trim = trim;
        self
    }

    /// Skips a given number of rows from the head of each file. If `trailing_only` was given
    /// `true`, then the head of the the first file is preserved.
    pub fn skip_head(
        &mut self,
        count: usize,
        trailing_only: bool,
    ) -> &mut Self {
        match self.skip.as_mut() {
            Some(skip) => skip.head = (count, trailing_only),
            None => {
                let skip = Skip {
                    head: (count, trailing_only),
                    ..Default::default()
                };
                self.skip = Some(skip);
            }
        }
        self
    }

    /// Skips a given number of rows from the tail of each file. If `leading_only` was given
    /// `true`, then the tail of the last file is preserved.
    pub fn skip_tail(
        &mut self,
        count: usize,
        leading_only: bool,
    ) -> &mut Self {
        match self.skip.as_mut() {
            Some(skip) => skip.tail = (count, leading_only),
            None => {
                let skip =
                    Skip { tail: (count, leading_only), ..Default::default() };
                self.skip = Some(skip);
            }
        }
        self
    }

    /// Skips any row if its length is not the longest.
    pub fn skip_non_max_length(&mut self, yes: bool) -> &mut Self {
        match self.skip.as_mut() {
            Some(skip) => skip.non_max_length = yes,
            None => {
                let skip = Skip { non_max_length: yes, ..Default::default() };
                self.skip = Some(skip);
            }
        }
        self
    }

    /// Skips any row if its length is shorter than the given `threshold`.
    pub fn skip_length_less_than(&mut self, threshold: usize) -> &mut Self {
        match self.skip.as_mut() {
            Some(skip) => skip.length_less_than = Some(threshold),
            None => {
                let skip = Skip {
                    length_less_than: Some(threshold),
                    ..Default::default()
                };
                self.skip = Some(skip);
            }
        }
        self
    }

    /// Skips any row if its the given `indexes` of fields is empty.
    ///
    /// Both `Field::Bytes([])` and `Field::Empty` are considered empty.
    pub fn skip_fields_empty(&mut self, indexes: Vec<usize>) -> &mut Self {
        match self.skip.as_mut() {
            Some(skip) => skip.fields_is_empty = Some(indexes),
            None => {
                let skip = Skip {
                    fields_is_empty: Some(indexes),
                    ..Default::default()
                };
                self.skip = Some(skip);
            }
        }
        self
    }

    /// Sets the style of newline when writing csv files.
    pub fn newline(&mut self, newline: Newline) -> &mut Self {
        self.newline = newline;
        self
    }

    /// Whether the presense of ending newline between each file should be forced.
    pub fn force_ending_newline(&mut self, yes: bool) -> &mut Self {
        self.force_ending_newline = yes;
        self
    }

    /// Builds a [`Merger`] from this configuration that reads data from the given readers.
    ///
    /// Note that the readers are buffered automatically, so you should not wrap any reader in a
    /// buffered reader like `io::BufReader`.
    pub fn from_readers<R: Read>(&self, readers: Vec<R>) -> Merger<R> {
        let state = MergerState {
            headers: None,
            has_headers: self.has_headers,
            max_field_count: None,
            first_field_count: None,
            skip: self.skip.clone(),
            capacity: self.capacity,
            newline: self.newline,
            force_ending_newline: self.force_ending_newline,
        };

        Merger { state, sources: readers }
    }

    /// Builds a [`Merger`] from this configuration that reads data from the given file paths.
    ///
    /// # Errors
    ///
    /// If there was any problem opening the given file paths, then this returns the corresponding
    /// error.
    pub fn from_paths<P: AsRef<Path>>(
        &self,
        paths: Vec<P>,
    ) -> Result<Merger<File>> {
        let files: Result<Vec<File>> =
            paths.into_iter().map(|p| File::open(p)).collect();

        let state = MergerState {
            headers: None,
            has_headers: self.has_headers,
            max_field_count: None,
            first_field_count: None,
            skip: self.skip.clone(),
            capacity: self.capacity,
            newline: self.newline,
            force_ending_newline: self.force_ending_newline,
        };

        Ok(Merger { sources: files?, state })
    }
}
