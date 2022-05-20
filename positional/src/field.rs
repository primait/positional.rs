use std::fmt::Display;
use std::iter;

/// a trait to represent a type that could be converted to a positional field
/// There is a handy generic implementation for every type that implements Display,
/// but if you want to customize the output on the file you should implement this trait
pub trait ToPositionalField {
    fn to_positional_field(&self) -> String;
}

impl<T: ToString> ToPositionalField for T {
    fn to_positional_field(&self) -> String {
        self.to_string()
    }
}

/// this represent a single field in a positional row
#[derive(Debug)]
pub struct PositionalField {
    value: String,
    size: usize,
    filler: char,
    align_left: bool,
}

impl PositionalField {
    pub fn new<T: ToPositionalField>(
        value: Option<&T>,
        size: usize,
        filler: char,
        align_left: bool,
    ) -> Self {
        Self {
            value: value
                .map(|v| v.to_positional_field())
                .unwrap_or(String::new()),
            size,
            filler,
            align_left,
        }
    }
}

impl ToString for PositionalField {
    fn to_string(&self) -> String {
        let value_size = self.value.len();
        let fill = if value_size >= self.size {
            "".to_string()
        } else {
            iter::repeat(self.filler)
                .take(self.size - value_size)
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
