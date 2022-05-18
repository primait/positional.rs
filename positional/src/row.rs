use std::error::Error;

/// this trait is implemented by the PositionalRow macros from the positional_derive crate
pub trait ToPositionalRow {
    fn to_positional_row(&self) -> String;
}

pub trait FromPositionalRow {
    fn from_positional_row(row: impl ToString) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}
