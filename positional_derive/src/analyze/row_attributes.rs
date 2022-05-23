use std::collections::HashMap;
use std::str::FromStr;
use syn::{Lit, Path};

const ATTR_NAME_SIZE: &str = "size";
const ATTR_NAME_FILLER: &str = "filler";
const ATTR_NAME_ALIGN: &str = "align";

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

impl TryFrom<&HashMap<String, syn::Lit>> for RowAttributes {
    type Error = String;

    fn try_from(attrs: &HashMap<String, Lit>) -> Result<Self, Self::Error> {
        let size = match attrs.get(ATTR_NAME_SIZE) {
            None => Err("no size attribute".to_string()),
            Some(lit) => match lit {
                Lit::Int(lit_int) => Ok(lit_int.base10_parse().map_err(|err| err.to_string())?),
                _ => Err("size attribute is not a number".to_string()),
            },
        }
        .map_err(|_| "unable to find size attribute".to_string())?;

        let filler = match attrs.get(ATTR_NAME_FILLER) {
            None => Ok(' '),
            Some(lit) => match lit {
                Lit::Char(lit_char) => Ok(lit_char.value()),
                _ => Err("filler is not a char".to_string()),
            },
        }?;

        let align = match attrs.get(ATTR_NAME_ALIGN) {
            None => Ok(FieldAlignment::Left),
            Some(lit) => match lit {
                Lit::Str(lit_str) => lit_str
                    .value()
                    .parse()
                    .map_err(|_| "alignment is not valid".to_string()),
                _ => Err("alignment is not a string".to_string()),
            },
        }?;

        Ok(Self {
            size,
            filler,
            align,
        })
    }
}
