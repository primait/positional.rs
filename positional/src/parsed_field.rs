#[derive(Debug)]
pub struct PositionalParsedField {
    row: String,
    offset: usize,
    size: usize,
    filler: char,
    align_left: bool,
}

impl PositionalParsedField {
    pub fn new(row: String, offset: usize, size: usize, filler: char, align_left: bool) -> Self {
        Self {
            row: row.into(),
            offset,
            size,
            filler,
            align_left,
        }
    }

    pub fn to_value(&self) -> String {
        let slice_start = self.offset;
        let slice_end = self.offset + self.size - 1;
        let raw_value = &self.row[slice_start..=slice_end];
        if self.align_left {
            raw_value.trim_end_matches(self.filler).to_string()
        } else {
            raw_value.trim_start_matches(self.filler).to_string()
        }
    }
}
