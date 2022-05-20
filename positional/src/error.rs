use thiserror::Error;

/// a handy type to represent results with positional errors
pub type PositionalResult<T> = Result<T, PositionalError>;

/// library error type
#[derive(Error, Debug)]
pub enum PositionalError {
    #[error("unparsable file")]
    UnparsableFile,
}
