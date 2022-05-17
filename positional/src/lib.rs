pub use positional_derive::PositionalRow;
use std::iter;

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

pub struct PositionalField<T: ToString> {
    value: T,
    size: usize,
    filler: char,
    align_left: bool,
}

impl<T: ToString> PositionalField<T> {
    pub fn new(value: T, size: usize, filler: char, align_left: bool) -> Self {
        Self {
            value,
            size,
            filler,
            align_left,
        }
    }
}

impl<T: ToString> ToString for PositionalField<T> {
    fn to_string(&self) -> String {
        let value_size = self.value.to_string().len();
        let fill = if value_size >= self.size {
            "".to_string()
        } else {
            iter::repeat(self.filler)
                .take(&self.size - value_size)
                .collect()
        };
        let value_as_string = if value_size >= self.size {
            let mut v = self.value.to_string();
            v.truncate(self.size);
            v
        } else {
            self.value.to_string()
        };

        if self.align_left {
            format!("{}{}", value_as_string, fill)
        } else {
            format!("{}{}", fill, value_as_string)
        }
    }
}
