//! # Error
//!
//! Defines the possible errors that occurs with `nqcsv`.

use thiserror::Error;

/// The conventional Result type for `nqcsv`.
///
/// Usage of this type isn't recommended, but it's provided as specified
/// by [Serde's implementer documentation](https://serde.rs/conventions.html).
///
/// Again, use of this type is discouraged. For binaries, you may prefer
/// `std::result::Result` or `anyhow::Result`. Otherwise, library authors may
/// like wrapping this type in a `thiserror::Error`.
pub type Result<T> = std::result::Result<T, CsvError>;

/// An error that occurs during CSV processing.
#[derive(Debug, Error)]
#[non_exhaustive] // TODO: remove this if stable
#[allow(missing_docs)]
pub enum CsvError {
    /// A `std::io::Error`, typically caused by underlying OS filesystem
    /// interactions.
    #[error("std::io::Error: {0}")]
    Io(std::io::Error),

    /// A row doesn't have the expected number of fields.
    #[error("Parsing error on line {line}: expected {expected} rows, but found {actual}.")]
    FieldImbalance {
        /// The expected amount of rows.
        expected: usize,
        /// The number of rows that were found.
        actual: usize,
        /// Line number of the abnormal row.
        line: usize,
    },

    /// Some internal error from serde.
    #[error("An external serializer gave nqcsv an error: `{0}`")]
    Custom(String),
}

impl serde::ser::Error for CsvError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
