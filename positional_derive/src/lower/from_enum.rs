use crate::analyze::{EnumModel, Matcher};

#[derive(Debug)]
pub struct EnumIr {
    pub container_identity: syn::Ident,
    pub variants: Vec<Variant>,
}

#[derive(Debug)]
pub struct Variant {
    pub ident: syn::Variant,
    pub sub_variant_type: syn::Type,
    pub matcher: Matcher,
}

pub fn lower_enum(model: EnumModel) -> EnumIr {
    EnumIr {
        container_identity: model.container_identity,
        variants: model
            .variants
            .iter()
            .map(|variant| Variant {
                ident: variant.variant.clone(),
                sub_variant_type: variant.sub_variant_type.clone(),
                matcher: variant.matcher.clone(),
            })
            .collect(),
    }
}
