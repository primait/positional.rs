
use proc_macro_error::abort;
use syn::{LitChar, LitInt, LitStr};

const FIELD_ATTRIBUTE: &str = "field";

pub struct Field {
    pub field: syn::Field,
    pub size: LitInt,
    pub filler: Option<LitChar>,
    pub align: Option<LitStr>,
}

impl Field {
    pub fn new(field: syn::Field) -> Option<Self> {
        parse_field_attributes(&field).map(|(size, filler, align)| Self {
            field,
            size,
            filler,
            align,
        })
    }
}

fn parse_field_attributes(field: &syn::Field) -> Option<(LitInt, Option<LitChar>, Option<LitStr>)> {
    field
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident(FIELD_ATTRIBUTE))
        .map(parse_field_attribute_meta)
}

fn parse_field_attribute_meta(
    attribute: &syn::Attribute,
) -> (LitInt, Option<LitChar>, Option<LitStr>) {
    let mut size: Option<LitInt> = None;
    let mut align: Option<LitStr> = None;
    let mut filler: Option<LitChar> = None;

    let parse_result = attribute.parse_nested_meta(|meta| {
        if meta.path.is_ident("size") {
            size = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("align") {
            align = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("filler") {
            filler = Some(meta.value()?.parse()?);
        } else {
            return Err(meta.error("unsupported attribute"));
        }
        Ok(())
    });

    if let Err(err) = parse_result {
        abort!(attribute, "failed to parse field attribute"; note = err.to_string());
    }

    match size {
        Some(size) => (size, filler, align),
        None => abort!(
            attribute,
            "wrong field configuration";
            help = "you need to provide at least a size configuration to the field"
        ),
    }
}
