use std::error::Error;

/// implement this trait to mark something as serializable into a row of a positional file
pub trait ToPositionalRow {
    fn to_positional_row(&self) -> String;
}

/// implement this trait to mark something as parsable from a row of a positional file
pub trait FromPositionalRow {
    fn parse(row: impl ToString) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}
