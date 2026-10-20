use std::fmt::{Display, Write};

/// a trait to represent a type that could be converted to a positional field
///
/// There is a generic implementation for types that implements Display
/// If you want to customize the output on the file you should implement this trait
pub trait ToPositionalField {
    fn to_positional_field(&self) -> String;
}

impl<T: ToString + ?Sized> ToPositionalField for T {
    fn to_positional_field(&self) -> String {
        self.to_string()
    }
}

#[doc(hidden)]
/// a single field ready to be serialized in a positional row
#[derive(Debug)]
pub struct PositionalField {
    value: String,
    size: usize,
    filler: char,
    left_aligned: bool,
}

impl PositionalField {
    pub fn new<T: ToPositionalField + ?Sized>(
        value: Option<&T>,
        size: usize,
        filler: char,
        left_aligned: bool,
    ) -> Self {
        Self {
            value: value.map(|v| v.to_positional_field()).unwrap_or_default(),
            size,
            filler,
            left_aligned,
        }
    }
}

impl Display for PositionalField {
    /// The size is measured in chars, as it is when parsing the row: every char takes one
    /// position, whatever its display width or UTF-8 length. Longer values are truncated.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = self
            .value
            .char_indices()
            .nth(self.size)
            .map_or(self.value.len(), |(index, _)| index);
        let value = &self.value[..end];
        let padding = self.size - value.chars().count();

        if self.left_aligned {
            f.write_str(value)?;
            write_filler(f, self.filler, padding)
        } else {
            write_filler(f, self.filler, padding)?;
            f.write_str(value)
        }
    }
}

fn write_filler(f: &mut std::fmt::Formatter<'_>, filler: char, count: usize) -> std::fmt::Result {
    (0..count).try_for_each(|_| f.write_char(filler))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = "scottex";

    #[test]
    fn empty_value_size_zero_to_string() {
        let field = PositionalField::new(Some(""), 0, ' ', true);
        assert!(field.to_string().is_empty());
    }

    #[test]
    fn non_empty_value_size_zero_to_string() {
        let field = PositionalField::new(Some(TEST_STRING), 0, ' ', true);
        assert!(field.to_string().is_empty());
    }

    #[test]
    fn empty_value_size_one_to_string() {
        let field = PositionalField::new(Some(""), 1, ' ', true);
        assert_eq!(field.to_string(), " ");
    }

    #[test]
    fn non_empty_value_size_one_to_string() {
        let field = PositionalField::new(Some(TEST_STRING), 1, ' ', true);
        assert_eq!(field.to_string(), "s");
    }

    #[test]
    fn non_empty_value_size_one_more_than_value_left_align_to_string() {
        let field = PositionalField::new(Some(TEST_STRING), 8, ' ', true);
        assert_eq!(field.to_string(), format!("{TEST_STRING} "));
    }

    #[test]
    fn non_empty_value_size_one_more_than_value_right_align_to_string() {
        let field = PositionalField::new(Some(TEST_STRING), 8, ' ', false);
        assert_eq!(field.to_string(), format!(" {TEST_STRING}"));
    }

    #[test]
    fn wide_char_takes_one_position() {
        let field = PositionalField::new(Some("李😀"), 4, ' ', true);
        assert_eq!(field.to_string(), "李😀  ");
    }

    #[test]
    fn zero_width_char_takes_one_position() {
        let field = PositionalField::new(Some("e\u{301}\u{200B}"), 4, ' ', true);
        assert_eq!(field.to_string(), "e\u{301}\u{200B} ");
    }

    #[test]
    fn multi_byte_value_filling_the_field_is_kept_whole() {
        let field = PositionalField::new(Some("Ñandú"), 5, ' ', true);
        assert_eq!(field.to_string(), "Ñandú");
    }

    #[test]
    fn multi_byte_value_longer_than_size_is_truncated_by_chars() {
        let field = PositionalField::new(Some("abcÑdef"), 4, ' ', true);
        assert_eq!(field.to_string(), "abcÑ");
    }

    #[test]
    fn multi_byte_value_right_align_to_string() {
        let field = PositionalField::new(Some("Ñ"), 3, '0', false);
        assert_eq!(field.to_string(), "00Ñ");
    }
}
