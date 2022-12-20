use crate::{PositionalResult, PositionalError};

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
    pub fn to_value(&self) -> PositionalResult<&'s str> {
        let slice_start = self.offset;
        let slice_end = self.offset + self.size;
        let raw_value = &self.row.get(slice_start..slice_end).ok_or(PositionalError::UnparsableFile)?;
        if self.left_aligned {
            Ok(raw_value.trim_end_matches(self.filler))
        } else {
            Ok(raw_value.trim_start_matches(self.filler))
        }
    }
}
