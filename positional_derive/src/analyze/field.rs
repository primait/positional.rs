use crate::analyze::row_attributes::RowAttributes;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    ident: String,
    attributes: Option<RowAttributes>,
}

impl Field {
    pub fn new(field: &syn::Field, _metas: &HashMap<String, (syn::Path, syn::Lit)>) -> Self {
        Self {
            ident: field.ident.as_ref().unwrap().to_string(),
            attributes: None,
        }
    }
}
