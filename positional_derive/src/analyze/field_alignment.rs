use proc_macro2::Ident;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use quote::TokenStreamExt;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub enum FieldAlignment {
    Left,
    Right,
}

impl FromStr for FieldAlignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" => Ok(Self::Left),
            "l" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "r" => Ok(Self::Right),
            _ => Err(format!(
                "align value should be 'left', 'right', 'l' or 'r', \"{}\" provided",
                s
            )),
        }
    }
}

impl ToTokens for FieldAlignment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let output = if self == &FieldAlignment::Left {
            "true"
        } else {
            "false"
        };
        let identity = Ident::new(output, Span::call_site());
        tokens.append(identity);
    }
}
