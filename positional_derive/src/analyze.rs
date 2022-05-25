mod field;
mod field_alignment;
mod meta;
mod row_attributes;

pub use field_alignment::FieldAlignment;

use crate::analyze::field::Field;
use crate::analyze::meta::create_fields;
use proc_macro_error::{abort, abort_call_site};
use syn::{Data, DataStruct, Fields};

use crate::Ast;

pub struct Model {
    pub container_identity: syn::Ident,
    pub fields: Vec<Field>,
}

pub fn analyze(ast: Ast) -> Model {
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
            abort!(
                ast,
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
            fields: Fields::Named(fields_named),
            ..
        }) => create_fields(&fields_named),
        // this is blocked at the parsing phase
        Data::Union(_) => unreachable!(),
    };
    Model {
        container_identity: ast.ident,
        fields,
    }
}
