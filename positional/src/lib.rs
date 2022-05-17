pub use positional_derive::PositionalRow;

use proc_macro2::{Ident, Literal, Punct, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};

pub trait PositionalRow {
    fn to_positional_row(&self) -> String;
}

pub struct PositionalFile<T: PositionalRow> {
    rows: Vec<T>,
}

impl<T: PositionalRow> PositionalFile<T> {
    pub fn new(rows: impl Iterator<Item = T>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
        }
    }
}

pub struct PositionalField<T> {
    value: T,
    size: String,
    filler: char,
}

impl<T> PositionalField<T> {
    pub fn new(value: T, size: usize, filler: char) -> Self {
        Self {
            value,
            size: size.to_string(),
            filler,
        }
    }
}

impl<P: ToString> ToTokens for PositionalField<P> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = Ident::new(&self.value.to_string(), Span::call_site());
        tokens.append(value);
    }
}
