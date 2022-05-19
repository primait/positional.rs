use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attributes_parsing;
mod from_positional_rows;
mod to_positional_row;
mod type_parsing;

use from_positional_rows::from_positional_for_struct;
use to_positional_row::to_positional_for_struct;

const FIELD_ATTR_NAME: &str = "field";

#[proc_macro_derive(FromPositionalRow, attributes(field))]
pub fn from_positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    from_positional_for_struct(ast).into()
}

#[proc_macro_derive(ToPositionalRow, attributes(field))]
pub fn to_positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    to_positional_for_struct(ast).into()
}
