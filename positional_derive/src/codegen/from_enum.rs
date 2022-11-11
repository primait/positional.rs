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
                impl FromPositionalRow for #container_identity {
                    fn from_positional_row(row: &str) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
                        #(#variants_stream)*
                        Err(Box::new(positional::PositionalError::UnparsableFile))
                    }
                }
            }
        }
        ImplBlockType::To => {
            let variants_stream = create_variants_for_to_positional_row(ir.variants.as_slice());
            quote! {
                impl ToPositionalRow for #container_identity {
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
                return Ok(Self::#ident(#sub_variant::from_positional_row(row)?));
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
            Self::#ident(sub_variant) => sub_variant.to_positional_row()
        };
        variants_stream.push(stream);
    }
    variants_stream
}
