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
        "Field definition error. Looking for a substring from offset {0} to {1} in the row `{2}`"
    )]
    FieldDefinitionError(usize, usize, String),
}
