use std::iter;

/// this represent a single field in a positional row
pub struct PositionalField {
    value: String,
    size: usize,
    filler: char,
    align_left: bool,
}

impl PositionalField {
    pub fn new(value: impl Into<String>, size: usize, filler: char, align_left: bool) -> Self {
        Self {
            value: value.into(),
            size,
            filler,
            align_left,
        }
    }
}

impl ToString for PositionalField {
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
