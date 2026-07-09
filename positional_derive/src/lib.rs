use manyhow::manyhow;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod analyze;
mod codegen;
mod lower;
mod parse;

use analyze::analyze;
use codegen::codegen;
use lower::{lower, ImplBlockType};
use parse::{parse, Ast};

/// Add to structs to make them deserializable from positional rows
#[proc_macro_derive(FromPositionalRow, attributes(field, matcher))]
#[manyhow]
pub fn from_positional_row(tokens: TokenStream) -> manyhow::Result<TokenStream2> {
    let ast = parse(tokens.into())?;
    let model = analyze(ast)?;
    let ir = lower(model)?;
    let rust = codegen(ir, ImplBlockType::From);
    Ok(rust)
}

/// Add to structs to make them serializable into positional rows
#[proc_macro_derive(ToPositionalRow, attributes(field))]
#[manyhow]
pub fn to_positional_row(tokens: TokenStream) -> manyhow::Result<TokenStream2> {
    let ast = parse(tokens.into())?;
    let model = analyze(ast)?;
    let ir = lower(model)?;
    let rust = codegen(ir, ImplBlockType::To);
    Ok(rust)
}
