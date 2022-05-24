use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::{Data, DeriveInput};

pub type Ast = DeriveInput;

pub fn parse(tokens: TokenStream) -> Ast {
    match syn::parse2::<syn::DeriveInput>(tokens) {
        // the derive is applied to a struct
        Ok(
            item @ DeriveInput {
                data: Data::Struct(_),
                ..
            },
        ) => item,
        // the derive is applied to an enum
        Ok(
            item @ DeriveInput {
                data: Data::Enum(_),
                ..
            },
        ) => item,
        // the derive is applied to a union
        Ok(
            item @ DeriveInput {
                data: Data::Union(_),
                ..
            },
        ) => {
            abort!(
                item,
                "item is not a struct or an enum";
                help = "`#[derive(ToPositionalRow)]` can only be used on structs or enums"
            )
        }
        Err(_) => unreachable!(),
    }
}
