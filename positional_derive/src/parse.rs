use manyhow::{bail, Result};
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

pub type Ast = DeriveInput;

pub fn parse(tokens: TokenStream) -> Result<Ast> {
    match syn::parse2::<DeriveInput>(tokens)? {
        // the derivation is applied to a struct
        item @ DeriveInput {
            data: Data::Struct(_),
            ..
        } => Ok(item),
        // the derivation is applied to an enum
        item @ DeriveInput {
            data: Data::Enum(_),
            ..
        } => Ok(item),
        // the derivation is applied to a union
        item @ DeriveInput {
            data: Data::Union(_),
            ..
        } => {
            bail!(
                item,
                "item is not a struct or an enum";
                help = "derives can only be used on structs or enums"
            )
        }
    }
}
