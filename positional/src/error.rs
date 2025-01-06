use thiserror::Error;

/// a handy type to represent results with positional errors
pub type PositionalResult<T> = Result<T, PositionalError>;

/// library error type
#[derive(Error, Debug, PartialEq, Eq)]
pub enum PositionalError {
    #[error("Unable to parse the file")]
    UnparsableFile,

    #[error("Unable to find a matcher for row `{0}`")]
    UnmatchedVariant(String),

    #[error(
        "Given row `{1}` has length {n}; expected length: {0}", n = .1.len()
    )]
    RowSizeError(usize, String),

    #[error("Parsing field `{field}` in row `{row}` failed.")]
    ParsingFailed { field: String, row: String },
}
