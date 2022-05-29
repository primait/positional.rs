use super::analyze::FieldAlignment;
use super::analyze::Model;
use crate::analyze::StructModel;
use proc_macro_error::abort;

pub enum Ir {
    Struct(StructIr),
}

pub struct StructIr {
    pub container_identity: syn::Ident,
    pub fields: Vec<Field>,
}

pub enum ImplBlockType {
    From,
    To,
}

pub struct Field {
    pub ident: syn::Ident,
    pub attributes: RowAttributes,
    pub optional: bool,
}

pub struct RowAttributes {
    pub size: usize,
    pub filler: char,
    pub align: FieldAlignment,
}

pub fn lower(model: Model) -> Ir {
    match model {
        Model::Struct(struct_model) => Ir::Struct(lower_struct(struct_model)),
        Model::Enum(_) => unimplemented!(),
    }
}

fn lower_struct(model: StructModel) -> StructIr {
    let mut fields: Vec<Field> = vec![];
    for model_field in model.fields {
        fields.push(Field {
            ident: model_field.field.ident.unwrap(),
            optional: extract_option_type(&model_field.field.ty).is_some(),
            attributes: RowAttributes {
                size: match model_field.size.base10_parse() {
                    Ok(size) => size,
                    Err(_) => abort!(
                        model_field.size,
                        "wrong field configuration";
                        help = "you need to provide at least a size configuration to the field"
                    ),
                },
                filler: model_field
                    .filler
                    .map(|lit_filler| lit_filler.value())
                    .unwrap_or(' '),
                align: model_field
                    .align
                    .map(|lit_align| {
                        let field_align: Result<FieldAlignment, _> = lit_align.value().parse();
                        match field_align {
                            Ok(align) => align,
                            Err(_) => {
                                abort!(
                                    lit_align,
                                    "wrong field configuration";
                                    help = "align value should be 'left', 'right', 'l' or 'r'"
                                )
                            }
                        }
                    })
                    .unwrap_or(FieldAlignment::Left),
            },
        })
    }

    StructIr {
        container_identity: model.container_identity,
        fields,
    }
}

fn extract_option_type(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push('|');
            acc
        });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .iter()
            .find(|s| &idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(extract_option_segment)
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}
