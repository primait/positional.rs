use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod analyze;
mod codegen;
mod lower;
mod parse;

use analyze::analyze;
use codegen::codegen;
use lower::{lower, ImplBlockType};
use parse::{parse, Ast};

/// Add to structs to make them deserializable from positional rows
#[proc_macro_derive(FromPositionalRow, attributes(field))]
pub fn from_positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse(tokens.into());
    let model = analyze(ast);
    let ir = lower(model);
    let rust = codegen(ir, ImplBlockType::From);
    rust.into()
}

/// Add to structs to make them serializable into positional rows
#[proc_macro_derive(ToPositionalRow, attributes(field))]
#[proc_macro_error]
pub fn to_positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse(tokens.into());
    let model = analyze(ast);
    let ir = lower(model);
    let rust = codegen(ir, ImplBlockType::To);
    rust.into()
}
