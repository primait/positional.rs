use std::error::Error;

/// this trait is implemented by the PositionalRow macros from the positional_derive crate
pub trait PositionalRow {
    fn to_positional_row(&self) -> String;
    fn from_positional_row(row: impl ToString) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}
