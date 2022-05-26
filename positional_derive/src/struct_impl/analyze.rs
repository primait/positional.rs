use proc_macro_error::abort;
use syn::{Data, DataStruct, Fields};

mod field;
mod field_alignment;

pub use field_alignment::FieldAlignment;

use super::analyze::field::Field;
use crate::Ast;

pub struct Model {
    pub container_identity: syn::Ident,
    pub fields2: Vec<Field>,
}

pub fn analyze(ast: Ast) -> Model {
    let fields2 = match ast.data {
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
        Data::Enum(_) => {
            abort!(
                ast,
                "only structs with named fields";
                help = "`#[derive(ToPositionalRow)]` can only be used on structs with named fields, this is an enum"
            )
        }
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => fields_named
            .named
            .into_iter()
            .filter_map(Field::new)
            .collect(),
        // this is blocked at the parsing phase
        Data::Union(_) => unreachable!(),
    };

    Model {
        container_identity: ast.ident,

        fields2,
    }
}
