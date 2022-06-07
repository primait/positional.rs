#[doc(hidden)]

/// a single field ready to be parsed from a positional row
pub struct PositionalParsedField {
    row: String,
    offset: usize,
    size: usize,
    filler: char,
    left_aligned: bool,
}

impl PositionalParsedField {
    pub fn new(row: String, offset: usize, size: usize, filler: char, left_aligned: bool) -> Self {
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
        let slice_start = self.offset;
        let slice_end = self.offset + self.size - 1;
        let raw_value = &self.row[slice_start..=slice_end];
        if self.left_aligned {
            raw_value.trim_end_matches(self.filler).to_string()
        } else {
            raw_value.trim_start_matches(self.filler).to_string()
        }
    }
}
