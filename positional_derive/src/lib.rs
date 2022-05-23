use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attributes_parsing;
mod from_positional_rows;
mod to_positional_row;
mod type_parsing;

mod analyze;
mod parse;

use analyze::analyze;
use parse::{parse, Ast};

use from_positional_rows::from_positional_for_struct;
use to_positional_row::to_positional_for_struct;

const FIELD_ATTR_NAME: &str = "field";

/// Add to structs to make them deserializable from positional strings
#[proc_macro_derive(FromPositionalRow, attributes(field))]
pub fn from_positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    from_positional_for_struct(ast).into()
}

/// Add to structs to make them serializable into positional files
#[proc_macro_derive(ToPositionalRow, attributes(field))]
#[proc_macro_error]
pub fn to_positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse(tokens.into());
    let model = analyze(ast);

    let out = quote! {};
    out.into()
}
