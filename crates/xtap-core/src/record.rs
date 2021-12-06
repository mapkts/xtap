use std::{fmt, slice};

use bstr::BString;
use calamine::{CellErrorType, DataType};
use csv::{ByteRecord, ByteRecordIter};

/// A single xlsx/csv record.
#[derive(Clone, PartialEq)]
pub struct Record(RecordInner);

#[derive(Clone, PartialEq)]
enum RecordInner {
    Csv(ByteRecord),
    Xlsx(Vec<DataType>),
}

impl fmt::Debug for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            RecordInner::Csv(ref record) => {
                let mut fields = vec![];
                for field in record {
                    fields.push(BString::from(field));
                }
                write!(f, "Record::Csv({:?})", fields)
            }
            RecordInner::Xlsx(ref record) => {
                write!(f, "Record::Xlsx({:?})", record)
            }
        }
    }
}

impl From<ByteRecord> for Record {
    fn from(record: ByteRecord) -> Record {
        Record(RecordInner::Csv(record))
    }
}

impl From<Vec<DataType>> for Record {
    fn from(record: Vec<DataType>) -> Self {
        Record(RecordInner::Xlsx(record))
    }
}

impl From<&[DataType]> for Record {
    fn from(record: &[DataType]) -> Self {
        Record(RecordInner::Xlsx(record.to_vec()))
    }
}

impl Record {
    /// Returns the field at index `i`.
    ///
    /// If no field at index `i` exists, then this returns `None`.
    #[inline]
    pub fn get(&self, i: usize) -> Option<Field<'_>> {
        match self.0 {
            RecordInner::Csv(ref record) => record.get(i).map(|x| x.into()),
            RecordInner::Xlsx(ref record) => record.get(i).map(|x| x.into()),
        }
    }

    /// Returns an iterator over all fields in this record.
    #[inline]
    pub fn iter(&self) -> RecordIter {
        match &self.0 {
            RecordInner::Csv(record) => RecordIter(IterInner::CsvRecord(record.iter())),
            RecordInner::Xlsx(record) => RecordIter(IterInner::XlsxRecord(record.iter())),
        }
    }

    /// Returns the number of fields in this record.
    #[inline]
    pub fn len(&self) -> usize {
        self.iter().len()
    }

    /// Returns true if this record is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the underlying xlsx/csv record stored in this struct.
    ///
    /// If `self` is a csv record, then this will return `(Some(ByteRecord), None)`.
    /// If `self` is a xlsx record, then this will return `(None, Some(Vec<DataType>)`.
    pub fn into_inner(self) -> (Option<ByteRecord>, Option<Vec<DataType>>) {
        match self.0 {
            RecordInner::Csv(record) => (Some(record), None),
            RecordInner::Xlsx(record) => (None, Some(record)),
        }
    }
}

/// A double-ended iterator over all fields in a record.
///
/// The `'r` lifetime refers to the lifetime of the `Record` that is being iterated over.
pub struct RecordIter<'r>(IterInner<'r>);

enum IterInner<'r> {
    CsvRecord(ByteRecordIter<'r>),
    XlsxRecord(slice::Iter<'r, DataType>),
}

impl<'r> Iterator for RecordIter<'r> {
    type Item = Field<'r>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IterInner::CsvRecord(iter) => iter.next().map(Field::Binary),
            IterInner::XlsxRecord(iter) => iter.next().map(|x| x.into()),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.0 {
            IterInner::CsvRecord(iter) => iter.size_hint(),
            IterInner::XlsxRecord(iter) => iter.size_hint(),
        }
    }

    #[inline]
    fn count(self) -> usize {
        match self.0 {
            IterInner::CsvRecord(iter) => iter.len(),
            IterInner::XlsxRecord(iter) => iter.len(),
        }
    }
}

impl<'r> ExactSizeIterator for RecordIter<'r> {}

impl<'r> DoubleEndedIterator for RecordIter<'r> {
    #[inline]
    fn next_back(&mut self) -> Option<Field<'r>> {
        match &mut self.0 {
            IterInner::CsvRecord(iter) => iter.next_back().map(Field::Binary),
            IterInner::XlsxRecord(iter) => iter.next_back().map(|x| x.into()),
        }
    }
}

impl<'r> IntoIterator for &'r Record {
    type Item = Field<'r>;
    type IntoIter = RecordIter<'r>;

    #[inline]
    fn into_iter(self) -> RecordIter<'r> {
        self.iter()
    }
}

/// A borrowed view into a field in a `Record`.
#[derive(Debug, Clone, PartialEq)]
pub enum Field<'r> {
    Binary(&'r [u8]),
    String(&'r str),
    Int(i64),
    Float(f64),
    Bool(bool),
    DateTime(f64),
    Error(&'r CellErrorType),
    Empty,
}

macro_rules! field_partial_eq {
    ($variant:ident, $other:ty) => {
        impl<'r> PartialEq<$other> for Field<'r> {
            fn eq(&self, other: &$other) -> bool {
                match *self {
                    Field::$variant(ref x) if *x == *other => true,
                    _ => false,
                }
            }
        }
    };
}

field_partial_eq!(Binary, &'r [u8]);
field_partial_eq!(String, &'r str);
field_partial_eq!(Int, i64);
field_partial_eq!(Float, f64);
field_partial_eq!(Bool, bool);
field_partial_eq!(Error, &'r CellErrorType);

impl<'r> PartialEq<()> for Field<'r> {
    fn eq(&self, _: &()) -> bool {
        matches!(*self, Field::Empty)
    }
}

macro_rules! field_from_type {
    ($variant:ident, $ty:ty) => {
        impl<'r> From<$ty> for Field<'r> {
            fn from(v: $ty) -> Self {
                Field::$variant(v)
            }
        }
    };
}

field_from_type!(Binary, &'r [u8]);
field_from_type!(String, &'r str);
field_from_type!(Int, i64);
field_from_type!(Float, f64);
field_from_type!(Bool, bool);
field_from_type!(Error, &'r CellErrorType);

impl<'r> From<()> for Field<'r> {
    fn from(_: ()) -> Self {
        Field::Empty
    }
}

impl<'r, T> From<Option<T>> for Field<'r>
where
    Field<'r>: From<T>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => From::from(v),
            None => Field::Empty,
        }
    }
}

impl<'r> From<&'r DataType> for Field<'r> {
    fn from(v: &'r DataType) -> Field<'r> {
        match v {
            DataType::Int(int) => Field::Int(*int),
            DataType::Float(float) => Field::Float(*float),
            DataType::String(string) => Field::String(string.as_str()),
            DataType::Bool(boolean) => Field::Bool(*boolean),
            DataType::Error(err) => Field::Error(err),
            DataType::DateTime(datetime) => Field::DateTime(*datetime),
            DataType::Empty => Field::Empty,
        }
    }
}
