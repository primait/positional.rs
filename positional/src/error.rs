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
        "The row passed is too small to work with the fields definition. The row needs to have at least {0} unicode chars. Passed row is: `{1}` with length {}", .1.len()
    )]
    RowSizeError(usize, String),
}
