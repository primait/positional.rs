use proc_macro_error::abort;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{Meta, NestedMeta};

use crate::analyze::Field;

const FIELD_ATTRIBUTE: &str = "field";

pub fn create_fields(fields_named: &syn::FieldsNamed) -> Vec<Field> {
    let mut metas = HashMap::new();
    fields_named
        .named
        .iter()
        .map(|field| {
            parse_field_attributes(field, &mut metas)
                .unwrap_or_else(|| Field::new(field, &HashMap::new()))
        })
        .collect()
}

fn parse_field_attributes(
    field: &syn::Field,
    metas: &mut HashMap<String, (syn::Path, syn::Lit)>,
) -> Option<Field> {
    field
        .attrs
        .iter()
        .find(|attribute| attribute.path.is_ident(FIELD_ATTRIBUTE))
        .map(|attribute| parse_field_attribute_meta(field, attribute, metas))
}

fn parse_field_attribute_meta(
    field: &syn::Field,
    attribute: &syn::Attribute,
    metas: &mut HashMap<String, (syn::Path, syn::Lit)>,
) -> Field {
    match attribute.parse_meta() {
        Ok(meta) => {
            parse_meta(&meta, field, metas);
            Field::new(field, metas)
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

fn parse_meta(
    meta: &syn::Meta,
    field: &syn::Field,
    metas: &mut HashMap<String, (syn::Path, syn::Lit)>,
) {
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
                    NestedMeta::Meta(name_value) => parse_meta(&name_value, field, metas),
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
            metas.insert(
                field.ident.as_ref().unwrap().to_string(),
                (name_value.path.clone(), name_value.lit.clone()),
            );
        }
    }
}
