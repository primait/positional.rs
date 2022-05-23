use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct RowAttributes {
    pub size: usize,
    pub filler: char,
    pub align: FieldAlignment,
}

#[derive(Debug)]
pub enum FieldAlignment {
    Left,
    Right,
}

impl FromStr for FieldAlignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" => Ok(Self::Left),
            "l" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "r" => Ok(Self::Right),
            _ => Err(format!(
                "align value should be 'left' or 'right', {} provided",
                s
            )),
        }
    }
}

impl From<Vec<(syn::Path, syn::Lit)>> for RowAttributes {}
