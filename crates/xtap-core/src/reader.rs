use std::io;

/// A xlsx/csv file reader.
pub struct Reader<R> {
    rdr: io::BufReader<R>,
    fmt: Format,
}

/// Represents the file format.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Csv,
    Xlsx,
    Xls,
    Ods,
    Other,
}
