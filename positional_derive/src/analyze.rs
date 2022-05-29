use proc_macro_error::abort;
use syn::{Data, DataStruct, Fields};

mod field;
mod field_alignment;

pub use field_alignment::FieldAlignment;

use super::analyze::field::Field;
use crate::Ast;

pub enum Model {
    Struct(StructModel),
    Enum(EnumModel),
}

pub struct StructModel {
    pub container_identity: syn::Ident,
    pub fields: Vec<Field>,
}

pub struct EnumModel {
    pub container_identity: syn::Ident,
    pub variants: Vec<syn::Variant>,
}

pub fn analyze(ast: Ast) -> Model {
    match ast.data {
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
            abort!(
                ast,
                "only structs with named fields";
                help = "`#[derive(ToPositionalRow)]` can only be used on structs with named fields, this is a unit struct"
            )
        }
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.into_iter().collect();
            Model::Enum(EnumModel {
                container_identity: ast.ident,
                variants,
            })
        }
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => {
            let fields = fields_named
                .named
                .into_iter()
                .filter_map(Field::new)
                .collect();
            Model::Struct(StructModel {
                container_identity: ast.ident,
                fields,
            })
        }
        // this is blocked at the parsing phase
        Data::Union(_) => unreachable!(),
    }
}
