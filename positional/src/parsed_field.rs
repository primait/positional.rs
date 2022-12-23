#[doc(hidden)]

/// a single field ready to be parsed from a positional row
pub struct PositionalParsedField<'s> {
    row: &'s str,
    offset: usize,
    size: usize,
    filler: char,
    left_aligned: bool,
}

impl<'s> PositionalParsedField<'s> {
    pub fn new(row: &'s str, offset: usize, size: usize, filler: char, left_aligned: bool) -> Self {
        Self {
            row,
            offset,
            size,
            filler,
            left_aligned,
        }
    }

    /// output a string representation of the parsed value
    /// trimming is done by the library based on the declared positional row configurations
    pub fn to_value(&self) -> String {
        // we don't take into consideration the empty string (or generally an out of bound string)
        // because we are already checking for string correctness in the main `from_positional_row`
        // function
        let raw_value = self
            .row
            .chars()
            .skip(self.offset)
            .take(self.size)
            .collect::<String>();
        if self.left_aligned {
            raw_value.trim_end_matches(self.filler).to_string()
        } else {
            raw_value.trim_start_matches(self.filler).to_string()
        }
    }
}
