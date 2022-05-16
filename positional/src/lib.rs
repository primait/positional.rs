pub use positional_derive::PositionalRow;

pub trait PositionalRow {
    fn to_positional_row(&self) -> String;
}

pub struct PositionalFile<T: PositionalRow> {
    rows: Vec<T>,
}

impl<T: PositionalRow> PositionalFile<T> {
    pub fn new(rows: impl Iterator<Item = T>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }
}
