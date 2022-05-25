use crate::lower::{Field, Ir};
use crate::ImplBlockType;
use proc_macro2::TokenStream;
use quote::quote;

pub type Rust = TokenStream;

pub fn codegen(ir: Ir, impl_block_type: ImplBlockType) -> Rust {
    let container_identity = ir.container_identity;

    let mut fields_stream = vec![];
    let mut offset = 0;
    for field in ir.fields {
        let stream = match impl_block_type {
            ImplBlockType::From => {
                let stream = generate_from_field(&field, offset);
                offset += field.attributes.size;
                stream
            }
            ImplBlockType::To => generate_to_field(&field),
        };
        fields_stream.push(stream);
    }

    match impl_block_type {
        ImplBlockType::From => {
            quote! {
                impl FromPositionalRow for #container_identity {
                    fn parse(row: impl ToString) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
                        let row_string = row.to_string();
                        Ok(Self {
                            #(#fields_stream),*
                        })
                    }
                }
            }
        }
        ImplBlockType::To => {
            quote! {
                impl ToPositionalRow for #container_identity {
                    fn to_positional_row(&self) -> String {
                        vec![#(#fields_stream),*].into_iter().map(|field| field.to_string()).collect::<Vec<String>>().join("")
                    }
                }
            }
        }
    }
}

fn generate_to_field(field: &Field) -> TokenStream {
    let field_ident = field.ident.ident.as_ref().unwrap();
    let size = field.attributes.size;
    let filler = field.attributes.filler;
    let align = &field.attributes.align;
    if field.optional {
        quote! {
            positional::PositionalField::new(self.#field_ident.as_ref(), #size, #filler, #align)
        }
    } else {
        quote! {
            positional::PositionalField::new(Some(&self.#field_ident), #size, #filler, #align)
        }
    }
}

fn generate_from_field(field: &Field, offset: usize) -> TokenStream {
    let field_ident = field.ident.ident.as_ref().unwrap();
    let size = field.attributes.size;
    let filler = field.attributes.filler;
    let align = &field.attributes.align;
    if field.optional {
        quote! {
            #field_ident: positional::PositionalParsedField::new(row.to_string(), #offset, #size, #filler, #align).to_value().parse().ok()
        }
    } else {
        quote! {
            #field_ident: positional::PositionalParsedField::new(row.to_string(), #offset, #size, #filler, #align).to_value().parse()?
        }
    }
}
