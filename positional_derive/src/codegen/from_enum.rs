use crate::codegen::Rust;
use crate::lower::from_enum::{EnumIr, Variant};
use crate::ImplBlockType;
use quote::quote;

pub fn codegen_enum(ir: EnumIr, impl_block_type: ImplBlockType) -> Rust {
    let container_identity = ir.container_identity;

    match impl_block_type {
        ImplBlockType::From => {
            let variants_stream = create_variants_for_from_positional_row(ir.variants.as_slice());
            quote! {
                impl ::positional::FromPositionalRow for #container_identity {
                    fn from_positional_row(row: &str) -> ::positional::PositionalResult<Self> {
                        #(#variants_stream)*
                        Err(::positional::PositionalError::UnparsableFile)
                    }
                }
            }
        }
        ImplBlockType::To => {
            let variants_stream = create_variants_for_to_positional_row(ir.variants.as_slice());
            quote! {
                impl ::positional::ToPositionalRow for #container_identity {
                    fn to_positional_row(&self) -> String {
                        match self {
                            #(#variants_stream),*
                        }
                    }
                }
            }
        }
    }
}

fn create_variants_for_from_positional_row(variants: &[Variant]) -> Vec<proc_macro2::TokenStream> {
    let mut variants_stream = vec![];
    for variant in variants {
        let expr = variant.matcher.expr.clone();
        let ident = variant.ident.ident.clone();
        let sub_variant = variant.sub_variant_type.clone();
        let stream = quote! {
            if #expr {
                return Ok(Self::#ident(<#sub_variant as ::positional::FromPositionalRow>::from_positional_row(row)?));
            }
        };
        variants_stream.push(stream);
    }
    variants_stream
}

fn create_variants_for_to_positional_row(variants: &[Variant]) -> Vec<proc_macro2::TokenStream> {
    let mut variants_stream = vec![];
    for variant in variants {
        let ident = variant.ident.ident.clone();
        let stream = quote! {
            Self::#ident(sub_variant) => ::positional::ToPositionalRow::to_positional_row(sub_variant)
        };
        variants_stream.push(stream);
    }
    variants_stream
}
