use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod parse;
mod struct_impl;

use parse::{parse, Ast};
use struct_impl::ImplBlockType;

/// Add to structs to make them deserializable from positional rows
#[proc_macro_derive(FromPositionalRow, attributes(field))]
pub fn from_positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse(tokens.into());
    let model = struct_impl::analyze(ast);
    let ir = struct_impl::lower(model);
    let rust = struct_impl::codegen(ir, ImplBlockType::From);
    rust.into()
}

/// Add to structs to make them serializable into positional rows
#[proc_macro_derive(ToPositionalRow, attributes(field))]
#[proc_macro_error]
pub fn to_positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse(tokens.into());
    let model = struct_impl::analyze(ast);
    let ir = struct_impl::lower(model);
    let rust = struct_impl::codegen(ir, ImplBlockType::To);
    rust.into()
}
