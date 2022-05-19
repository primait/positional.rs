use quote::quote_spanned;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{Lit, Meta, NestedMeta, Path};

pub fn parse_meta(attrs: &mut Vec<(Path, Lit)>, meta: Meta) {
    match meta {
        Meta::Path(_) => {}
        Meta::List(meta_list) => {
            for nested_meta in meta_list.nested {
                match nested_meta {
                    NestedMeta::Meta(name_value) => parse_meta(attrs, name_value),
                    NestedMeta::Lit(_) => {}
                }
            }
        }
        Meta::NameValue(name_value) => attrs.push((name_value.path, name_value.lit)),
    }
}

#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
pub struct RowAttributes {
    pub size: usize,
    pub filler: char,
    pub align: FieldAlignment,
}

pub fn create_row_attributes(
    attrs: Vec<(Path, Lit)>,
) -> Result<RowAttributes, proc_macro2::TokenStream> {
    let mut size = 10;
    let mut filler = ' ';
    let mut align = FieldAlignment::Left;
    for (path, lit) in attrs {
        if path.is_ident("size") {
            size = match &lit {
                Lit::Int(lit_int) => lit_int.base10_parse().unwrap(),
                _ => {
                    let span = path.span();
                    return Err(quote_spanned!(span=> compile_error!("size should be an int")));
                }
            };
        }

        if path.is_ident("filler") {
            filler = match &lit {
                Lit::Char(lit_char) => lit_char.value(),
                _ => {
                    let span = path.span();
                    return Err(quote_spanned!(span=> compile_error!("filler should be a char")));
                }
            };
        }

        if path.is_ident("align") {
            align = match &lit {
                Lit::Str(lit_str) => lit_str.value().parse().unwrap(),
                _ => {
                    let span = path.span();
                    return Err(
                        quote_spanned!(span=> compile_error!("align should be either left or right")),
                    );
                }
            };
        }
    }
    Ok(RowAttributes {
        size,
        filler,
        align,
    })
}
