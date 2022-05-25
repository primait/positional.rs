use super::field_alignment::FieldAlignment;
use std::collections::HashMap;
use syn::Lit;

const ATTR_NAME_SIZE: &str = "size";
const ATTR_NAME_FILLER: &str = "filler";
const ATTR_NAME_ALIGN: &str = "align";

pub struct RowAttributes {
    pub size: usize,
    pub filler: char,
    pub align: FieldAlignment,
}

impl TryFrom<&HashMap<String, syn::Lit>> for RowAttributes {
    type Error = Option<syn::Lit>;

    fn try_from(attrs: &HashMap<String, Lit>) -> Result<Self, Self::Error> {
        let size = match attrs.get(ATTR_NAME_SIZE) {
            None => Err(None),
            Some(lit) => match lit {
                Lit::Int(lit_int) => Ok(lit_int.base10_parse().map_err(|_| Some(lit.clone()))?),
                _ => Err(Some(lit.clone())),
            },
        }
        .map_err(|_| None)?;

        let filler = match attrs.get(ATTR_NAME_FILLER) {
            None => Ok(' '),
            Some(lit) => match lit {
                Lit::Char(lit_char) => Ok(lit_char.value()),
                _ => Err(lit.clone()),
            },
        }?;

        let align = match attrs.get(ATTR_NAME_ALIGN) {
            None => Ok(FieldAlignment::Left),
            Some(lit) => match lit {
                Lit::Str(lit_str) => lit_str.value().parse().map_err(|_| Some(lit.clone())),
                _ => Err(Some(lit.clone())),
            },
        }?;

        Ok(Self {
            size,
            filler,
            align,
        })
    }
}
