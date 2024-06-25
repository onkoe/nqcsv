//! # Model
//!
//! Types that represent the CSV format and its constituent parts.

pub struct CsvFile {
    /// The optional header row (see: RFC 4180 section 2.3).
    header: Option<Header>,
}

pub struct Header {
    fields: Vec<Field>,
}

/// A single column in a CSV file. This represents one named primitive type.
pub struct Field {
    name: String,
    primitive: Primitive,
}

/// A primitive type in a CSV file.
pub enum Primitive {
    /// A string.
    String,
    /// A number.
    Number,
}
