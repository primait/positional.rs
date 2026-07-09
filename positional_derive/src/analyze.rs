use manyhow::{bail, Result};
use syn::{Data, DataStruct, Fields};

mod field;
mod field_alignment;
mod variant;

use crate::analyze::{field::Field, variant::Variant};
use crate::Ast;
pub use field_alignment::FieldAlignment;
pub use variant::Matcher;

#[cfg(test)]
#[path = "./test/analyze.rs"]
mod analyze_test;

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
    pub variants: Vec<Variant>,
}

pub fn analyze(ast: Ast) -> Result<Model> {
    match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields_unnamed),
            ..
        }) => {
            bail!(
                fields_unnamed,
                "only structs with named fields";
                help = "`#[derive(ToPositionalRow)]` can only be used on structs with named fields, this is a struct with unnamed fields"
            )
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            bail!(
                ast,
                "only structs with named fields";
                help = "`#[derive(ToPositionalRow)]` can only be used on structs with named fields, this is a unit struct"
            )
        }
        Data::Enum(data_enum) => {
            let mut variants = vec![];
            for syn_variant in data_enum.variants {
                match Variant::new(syn_variant.clone())? {
                    None => bail!(syn_variant, "only enum variants with one unnamed field"),
                    Some(v) => variants.push(v),
                }
            }

            Ok(Model::Enum(EnumModel {
                container_identity: ast.ident,
                variants,
            }))
        }
        Data::Struct(DataStruct {
            fields: Fields::Named(fields_named),
            ..
        }) => {
            let mut fields = vec![];
            for field in fields_named.named {
                if let Some(field) = Field::new(field)? {
                    fields.push(field);
                }
            }
            Ok(Model::Struct(StructModel {
                container_identity: ast.ident,
                fields,
            }))
        }
        // this is blocked at the parsing phase
        Data::Union(_) => unreachable!(),
    }
}
