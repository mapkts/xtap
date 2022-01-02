/// The columns and rows drop hehaviour.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Drop {
    /// Drops a number of rows from the head of each source. The second field of this tuple
    /// indicates whether the head of the first source should be preserved or not.
    head: usize,
    /// Drops a number of rows from the tail of each source. The second field of this tuple
    /// indicates whether the tail of the last source should be preserved or not.
    tail: usize,
    /// Drops any row if its length is not the longest.
    non_max_length: bool,
    /// Drops any row if its length is shorter than the given one.
    length_less_than: Option<usize>,
    /// Drops any row if its `nth` (zero-based) field is empty.
    ///
    /// Both `Field::Binary([])` and `Field::Empty` are considered empty.
    fields_empty: Option<Vec<usize>>,
    /// Drops the given columns.
    columns: Option<Vec<usize>>,
}

impl Drop {
    /// Creates a new instance of [`Drop`] that allows setting various options.
    pub fn new() -> Drop {
        Default::default()
    }

    /// Drops a given number of rows from the head of each file. If `trailing_only` was given
    /// `true`, then the head of the the first file is preserved.
    pub fn drop_head(self, count: usize) -> Self {
        Drop { head: count, ..self }
    }

    /// Drops a given number of rows from the tail of each file. If `leading_only` was given
    /// `true`, then the tail of the last file is preserved.
    pub fn drop_tail(self, count: usize) -> Self {
        Drop { tail: count, ..self }
    }

    /// Drops any row if its length is not the longest.
    pub fn drop_non_max_length(self, yes: bool) -> Self {
        Drop { non_max_length: yes, ..self }
    }

    /// Drops any row if its length is shorter than the given `threshold`.
    pub fn drop_length_less_than(self, threshold: usize) -> Self {
        Drop { length_less_than: Some(threshold), ..self }
    }

    /// Drops any row if its the given `indexes` of fields is empty.
    ///
    /// Both `Field::Bytes([])` and `Field::Empty` are considered empty.
    pub fn drop_fields_empty(self, indexes: Vec<usize>) -> Self {
        Drop { fields_empty: Some(indexes), ..self }
    }

    /// Drops the given columns.
    pub fn drop_columns(self, indexes: Vec<usize>) -> Self {
        Drop { columns: Some(indexes), ..self }
    }
}
