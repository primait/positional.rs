use crate::PositionalRow;

/// a positional file is a collection of rows
pub struct PositionalFile<T: PositionalRow> {
    rows: Vec<T>,
}

impl<T: PositionalRow> PositionalFile<T> {
    pub fn new(rows: impl IntoIterator<Item = T>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }

    pub fn to_positional_file(&self) -> String {
        let mut output = String::new();
        for row in &self.rows {
            output += &row.to_positional_row();
        }
        output
    }
}
