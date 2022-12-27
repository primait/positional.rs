use proc_macro_error::abort;
use std::collections::HashMap;
use syn::{Lit, LitChar, LitInt, LitStr, Meta, NestedMeta};

const FIELD_ATTRIBUTE: &str = "field";

pub struct Field {
    pub field: syn::Field,
    pub size: LitInt,
    pub filler: Option<LitChar>,
    pub align: Option<LitStr>,
}

impl Field {
    pub fn new(field: syn::Field) -> Option<Self> {
        parse_field_attributes(&field).map(|(size, filler, align)| Self {
            field,
            size,
            filler,
            align,
        })
    }
}

fn parse_field_attributes(field: &syn::Field) -> Option<(LitInt, Option<LitChar>, Option<LitStr>)> {
    field
        .attrs
        .iter()
        .find(|attribute| attribute.path.is_ident(FIELD_ATTRIBUTE))
        .map(parse_field_attribute_meta)
}

fn parse_field_attribute_meta(
    attribute: &syn::Attribute,
) -> (LitInt, Option<LitChar>, Option<LitStr>) {
    match attribute.parse_meta() {
        Ok(meta) => {
            let mut attrs = HashMap::new();
            parse_meta(&meta, &mut attrs);

            let size = match attrs.get("size") {
                None => {
                    abort!(
                        attribute,
                        "wrong field configuration";
                        help = "you need to provide at least a size configuration to the field"
                    )
                }
                Some(size_lit) => match size_lit {
                    Lit::Int(lit_int) => lit_int,
                    _ => {
                        abort!(
                            attribute,
                            "wrong field configuration";
                            help = "the size configuration should be a number"
                        )
                    }
                },
            };

            let filler = match attrs.get("filler") {
                None => None,
                Some(filler_lit) => match filler_lit {
                    Lit::Char(lit_char) => Some(lit_char),
                    _ => {
                        abort!(
                            attribute,
                            "wrong field configuration";
                            help = "the filler configuration should be a char"
                        )
                    }
                },
            };

            let align = match attrs.get("align") {
                None => None,
                Some(filler_align) => match filler_align {
                    Lit::Str(lit_str) => Some(lit_str),
                    _ => {
                        abort!(
                            attribute,
                            "wrong field configuration";
                            help = "the align configuration should be a string"
                        )
                    }
                },
            };

            (size.clone(), filler.cloned(), align.cloned())
        }
        Err(_) => {
            abort!(
                attribute,
                "wrong field configuration";
                help = "unable to parse field configuration"
            )
        }
    }
}

fn parse_meta(meta: &syn::Meta, attrs: &mut HashMap<String, syn::Lit>) {
    match meta {
        Meta::Path(path) => {
            abort!(
                path,
                "wrong field configuration";
                help = "there should only be name = value couple inside the field configuration"
            )
        }
        Meta::List(meta_list) => {
            for nested_meta in &meta_list.nested {
                match nested_meta {
                    NestedMeta::Meta(name_value) => parse_meta(name_value, attrs),
                    NestedMeta::Lit(lit) => {
                        abort!(
                            lit,
                            "wrong field configuration";
                            help = "there should only be name = value couple inside the field configuration"
                        )
                    }
                }
            }
        }
        Meta::NameValue(name_value) => {
            attrs.insert(
                name_value.path.get_ident().unwrap().to_string(),
                name_value.lit.clone(),
            );
        }
    }
}
