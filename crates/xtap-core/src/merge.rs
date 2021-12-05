// use std::io;
// use std::path::Path;

// /// A xlsx/csv file merger.
// pub struct Merger<R> {
//     sources: Option<Vec<Box<dyn AsRef<Path>>>>,
//     rdr: io::BufReader<R>,
//     state: MergerState,
// }

// #[derive(Debug)]
// struct MergerState {
//     headers: Option<Headers>,
//     has_headers: bool,
//     flexible: bool,
//     max_field_count: Option<u32>,
//     first_field_count: Option<u32>,
// }
