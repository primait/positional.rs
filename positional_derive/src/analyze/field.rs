use crate::analyze::meta::FieldWithParsedAttributes;
use crate::analyze::row_attributes::RowAttributes;
use quote::ToTokens;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    ident: String,
    attributes: Option<RowAttributes>,
}

impl Field {
    pub fn new(field: &syn::Field, attrs: &HashMap<String, syn::Lit>) -> Result<Self, String> {
        Ok(Self {
            ident: field.ident.as_ref().unwrap().to_string(),
            attributes: Some(attrs.try_into()?),
        })
    }
}
