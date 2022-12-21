use crate::PositionalResult;

/// implement this trait to mark something as serializable into a row of a positional file
pub trait ToPositionalRow {
    fn to_positional_row(&self) -> String;
}

/// implement this trait to mark something as parsable from a row of a positional file
pub trait FromPositionalRow {
    fn from_positional_row(row: &str) -> PositionalResult<Self>
    where
        Self: Sized;
}
