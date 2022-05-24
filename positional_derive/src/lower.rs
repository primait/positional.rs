use crate::analyze::FieldAlignment;
use crate::analyze::Model;

pub struct Ir {
    pub container_identity: syn::Ident,
    pub fields: Vec<Field>,
}

pub enum ImplBlockType {
    From,
    To,
}

pub struct Field {
    pub ident: syn::Field,
    pub attributes: RowAttributes,
    pub optional: bool,
}

pub struct RowAttributes {
    pub size: usize,
    pub filler: char,
    pub align: FieldAlignment,
}

pub fn lower(model: Model) -> Ir {
    let mut fields: Vec<Field> = vec![];
    for model_field in model.fields {
        fields.push(Field {
            ident: model_field.ident,
            optional: model_field.optional,
            attributes: RowAttributes {
                size: model_field.attributes.size,
                filler: model_field.attributes.filler,
                align: model_field.attributes.align,
            },
        })
    }

    Ir {
        container_identity: model.container_identity,
        fields,
    }
}
