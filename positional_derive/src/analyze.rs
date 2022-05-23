use proc_macro_error::{abort, abort_call_site};
use std::collections::HashMap;
use std::str::FromStr;
use syn::{Attribute, Data, DataStruct, Fields, FieldsNamed, Lit, Meta};

use crate::Ast;

pub struct Model<'a> {
    fields: Vec<Field<'a>>,
}

pub struct Field<'a> {
    ident: &'a syn::Ident,
    attribute: RowAttributes,
}

pub struct RowAttributes {
    pub size: usize,
    pub filler: char,
    pub align: FieldAlignment,
}

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

pub fn analyze<'a>(ast: Ast) -> Model<'a> {
    let fields = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields_unnamed),
            ..
        }) => {
            abort!(
                fields_unnamed,
                "only structs with named fields";
                help = "`#[derive(ToPositionalRow)]` can only be used on structs with named fields, this is a struct with unnamed fields"
            )
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            abort_call_site!(
                "only structs with named fields";
                help = "`#[derive(ToPositionalRow)]` can only be used on structs with named fields, this is a unit struct"
            )
        }
        Data::Enum(_) => {
            abort_call_site!(
                "only structs with named fields";
                help = "`#[derive(ToPositionalRow)]` can only be used on structs with named fields, this is an enum"
            )
        }
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields_named),
            ..
        }) => create_fields_from_named_fields(fields_named),
        // this is blocked in the parsing phase
        Data::Union(_) => unreachable!(),
    };
    Model { fields: vec![] }
}

fn create_fields_from_named_fields(named_fields: &FieldsNamed) -> Vec<Field> {
    let mut fields = vec![];
    for field in &named_fields.named {
        let field = Field {
            // safe to unwrap, since it's a FieldsNamed struct
            ident: field.ident.as_ref().unwrap(),
            attribute: create_row_attribute_from_attrs(&field.attrs),
        };

        fields.push(field);
    }

    fields
}

fn create_row_attribute_from_attrs(attributes: &Vec<Attribute>) -> RowAttributes {
    let mut metas = HashMap::new();
    for attribute in attributes {
        match attribute.parse_meta() {
            Ok(meta) => match meta {
                Meta::Path(path) => {
                    abort!(
                        path,
                        "wrong attribute";
                        help = "the field attribute of this macro accepts only name value attributes. Here you are using a path attribute. You should have something like #[field(size = \"10\")]"
                    )
                }
                Meta::List(list) => {
                    abort!(
                        list,
                        "wrong attribute";
                        help = "the field attribute of this macro accepts only name value attributes. Here you are using a list attribute. You should have something like #[field(size = \"10\")]"
                    )
                }
                Meta::NameValue(name_value) => {
                    metas.insert(name_value.path.get_ident().unwrap().to_string(), name_value);
                }
            },
            Err(_error) => {
                abort!(
                    attribute,
                    "wrong attribute";
                    help = "there is something wrong with your attribute definition"
                )
            }
        }
    }

    let field_alignment: FieldAlignment = metas
        .get("align")
        .map(|meta_name_value| match &meta_name_value.lit {
            Lit::Str(lit_str) => match lit_str.value().parse() {
                Ok(parsed) => parsed,
                Err(_) => abort!(
                    meta_name_value,
                    "wrong attribute";
                    help = "there is something wrong with your attribute definition"
                ),
            },
            _ => abort!(
                meta_name_value,
                "wrong attribute";
                help = "there is something wrong with your attribute definition"
            ),
        })
        .unwrap_or(FieldAlignment::Left);

    RowAttributes {
        align: field_alignment,
        size: 20,
        filler: ' ',
    }
}
