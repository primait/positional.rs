use proc_macro_error::abort;
use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use syn::{Meta, NestedMeta};

use crate::analyze::Field;

const FIELD_ATTRIBUTE: &str = "field";
pub type FieldWithParsedAttributes = HashMap<syn::Ident, HashMap<String, syn::Lit>>;

pub fn create_fields(fields_named: &syn::FieldsNamed) -> Vec<Field> {
    fields_named
        .named
        .iter()
        .map(|field| {
            parse_field_attributes(field).unwrap_or_else(|| Field::new(field, &HashMap::new()))
        })
        .collect()
}

fn parse_field_attributes(field: &syn::Field) -> Option<Field> {
    field
        .attrs
        .iter()
        .find(|attribute| attribute.path.is_ident(FIELD_ATTRIBUTE))
        .map(|attribute| parse_field_attribute_meta(field, attribute))
}

fn parse_field_attribute_meta(field: &syn::Field, attribute: &syn::Attribute) -> Field {
    match attribute.parse_meta() {
        Ok(meta) => {
            let mut attrs = HashMap::new();
            parse_meta(&meta, field, &mut attrs);
            match Field::new(field, &attrs) {
                Ok(field) => field,
                Err(_) => {
                    abort!(
                        field,
                        "wrong field configuration";
                        help = "something went wrong while parsing your field configuration"
                    )
                }
            }
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

fn parse_meta(meta: &syn::Meta, field: &syn::Field, attrs: &mut HashMap<String, syn::Lit>) {
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
                    NestedMeta::Meta(name_value) => parse_meta(&name_value, field, attrs),
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
