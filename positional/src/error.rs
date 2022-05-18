use thiserror::Error;

pub type PositionalResult<T> = Result<T, PositionalError>;

#[derive(Error, Debug)]
pub enum PositionalError {
    #[error("unparsable file")]
    UnparsableFile,
}
