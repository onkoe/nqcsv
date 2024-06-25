//! # nqcsv
//!
//! A Serde format implementation for CSV files, with full support for field
//! flattening and somewhat modernized conventions.
//!
//! It's... not quite `csv`! :D

#![deny(missing_docs)]

mod de;
mod error;
mod model; // TODO: consider making it public?
mod ser;

// pub use de, error, model, ser, etc.
