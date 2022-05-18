/// this trait is implemented by the PositionalRow macros from the positional_derive crate
pub trait PositionalRow {
    fn to_positional_row(&self) -> String;
}
