use calamine::{CellErrorType, DataType};
use csv::{ByteRecord, ByteRecordIter};
use std::slice;

#[derive(Clone, PartialEq)]
pub struct Record(RecordInner);

#[derive(Clone, PartialEq)]
enum RecordInner {
    Csv(ByteRecord),
    Xlsx(Vec<DataType>),
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
    pub fn iter(&self) -> RecordIter {
        match &self.0 {
            RecordInner::Csv(record) => {
                RecordIter(IterInner::CsvRecord(record.iter()))
            }
            RecordInner::Xlsx(record) => {
                RecordIter(IterInner::XlsxRecord(record.iter()))
            }
        }
    }
}

pub struct RecordIter<'r>(IterInner<'r>);

pub enum IterInner<'r> {
    CsvRecord(ByteRecordIter<'r>),
    XlsxRecord(slice::Iter<'r, DataType>),
}

impl<'r> Iterator for RecordIter<'r> {
    type Item = Field<'r>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            IterInner::CsvRecord(iter) => iter.next().map(Field::Binary),
            IterInner::XlsxRecord(iter) => iter.next().map(|x| match x {
                DataType::Int(int) => Field::Int(*int),
                DataType::Float(float) => Field::Float(*float),
                DataType::String(string) => Field::String(string.as_str()),
                DataType::Bool(boolean) => Field::Bool(*boolean),
                DataType::Error(err) => Field::Error(err),
                DataType::DateTime(datetime) => Field::DateTime(*datetime),
                DataType::Empty => Field::Empty,
            }),
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
            IterInner::XlsxRecord(iter) => iter.next_back().map(|x| match x {
                DataType::Int(int) => Field::Int(*int),
                DataType::Float(float) => Field::Float(*float),
                DataType::String(string) => Field::String(string.as_str()),
                DataType::Bool(boolean) => Field::Bool(*boolean),
                DataType::Error(err) => Field::Error(err),
                DataType::DateTime(datetime) => Field::DateTime(*datetime),
                DataType::Empty => Field::Empty,
            }),
        }
    }
}

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
    ($variant:path, $other:ty) => {
        impl<'r> PartialEq<$other> for Field<'r> {
            fn eq(&self, other: &$other) -> bool {
                match *self {
                    $variant(ref x) if *x == *other => true,
                    _ => false,
                }
            }
        }
    };
}

field_partial_eq!(Field::Binary, &'r [u8]);
field_partial_eq!(Field::String, &'r str);
field_partial_eq!(Field::Int, i64);
field_partial_eq!(Field::Float, f64);
field_partial_eq!(Field::Bool, bool);
field_partial_eq!(Field::Error, &'r CellErrorType);

impl<'r> PartialEq<()> for Field<'r> {
    fn eq(&self, _: &()) -> bool {
        matches!(*self, Field::Empty)
    }
}

macro_rules! field_from_type {
    ($variant:path, $ty:ty) => {
        impl<'r> From<$ty> for Field<'r> {
            fn from(v: $ty) -> Self {
                $variant(v)
            }
        }
    };
}

field_from_type!(Field::Binary, &'r [u8]);
field_from_type!(Field::String, &'r str);
field_from_type!(Field::Int, i64);
field_from_type!(Field::Float, f64);
field_from_type!(Field::Bool, bool);
field_from_type!(Field::Error, &'r CellErrorType);

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
