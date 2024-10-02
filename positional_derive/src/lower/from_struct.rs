use super::extract_option_type;
use crate::analyze::{FieldAlignment, StructModel};
use proc_macro_error2::abort;

pub struct StructIr {
    pub container_identity: syn::Ident,
    pub fields: Vec<Field>,
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

pub fn lower_struct(model: StructModel) -> StructIr {
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
