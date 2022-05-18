use crate::row::ToPositionalRow;
use std::fmt::Write;

/// a positional file is a collection of rows
pub struct PositionalFile<T: ToPositionalRow> {
    rows: Vec<T>,
}

impl<T: ToPositionalRow> PositionalFile<T> {
    pub fn new(rows: impl IntoIterator<Item = T>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }

    pub fn to_positional_file(&self) -> Result<String, std::fmt::Error> {
        let mut out = String::new();
        for row in &self.rows {
            writeln!(&mut out, "{}", row.to_positional_row())?;
        }
        Ok(out)
    }
}
