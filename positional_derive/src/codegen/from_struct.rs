use proc_macro2::TokenStream;
use quote::quote;

use crate::codegen::Rust;
use crate::lower::from_struct::{Field, StructIr};
use crate::ImplBlockType;

pub fn codegen_struct(ir: StructIr, impl_block_type: ImplBlockType) -> Rust {
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
                impl ::positional::FromPositionalRow for #container_identity {
                    fn from_positional_row(row: &str) -> ::positional::PositionalResult<Self> {
                        if row.len() < #offset {
                            return Err(::positional::PositionalError::RowSizeError(#offset, row.to_string()));
                        }
                        Ok(Self {
                            #(#fields_stream),*
                        })
                    }
                }
            }
        }
        ImplBlockType::To => {
            quote! {
                impl ::positional::ToPositionalRow for #container_identity {
                    fn to_positional_row(&self) -> String {
                        vec![#(#fields_stream),*].into_iter().map(|field| field.to_string()).collect::<Vec<String>>().join("")
                    }
                }
            }
        }
    }
}

fn generate_to_field(field: &Field) -> TokenStream {
    let field_ident = &field.ident;
    let size = field.attributes.size;
    let filler = field.attributes.filler;
    let align = &field.attributes.align;
    if field.optional {
        quote! {
            ::positional::PositionalField::new(self.#field_ident.as_ref(), #size, #filler, #align)
        }
    } else {
        quote! {
            ::positional::PositionalField::new(Some(&self.#field_ident), #size, #filler, #align)
        }
    }
}

fn generate_from_field(field: &Field, offset: usize) -> TokenStream {
    let field_ident = &field.ident;
    let size = field.attributes.size;
    let filler = field.attributes.filler;
    let align = &field.attributes.align;
    if field.optional {
        quote! {
            #field_ident: ::positional::PositionalParsedField::new(row, #offset, #size, #filler, #align).to_value().parse().ok()
        }
    } else {
        let field_name_string = field_ident.to_string();
        quote! {
            #field_ident: ::positional::PositionalParsedField::new(row, #offset, #size, #filler, #align).to_value().parse()
                .map_err(|_e| ::positional::PositionalError::ParsingFailed{
                    // Optimally, we'd pass some information from `_e` into the error;
                    // however, since fields only require a FromStr impl for parsing the data,
                    // the Error type is unrestricted, meaning we can't even extract an error message out of it.
                    // Hence, without further restricting the data model, this is the best we can do to report the error to the user.
                    field: #field_name_string.to_string(),
                    row: row.to_string(),
                })?
        }
    }
}
