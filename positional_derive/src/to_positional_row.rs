use quote::{quote, quote_spanned};
use syn::{Data, Fields, FieldsNamed};

use crate::attributes_parsing::{create_row_attributes, parse_meta, FieldAlignment};
use crate::type_parsing::extract_option_type;

pub fn to_positional_for_struct(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let type_name = ast.ident;
    let type_span = type_name.span();

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let to_positional = match create_to_positional(&fields) {
                    Ok(to_positional_stream) => to_positional_stream,
                    Err(error) => return error,
                };
                quote! {
                    impl ToPositionalRow for #type_name {
                        #to_positional
                    }
                }
            }
            Fields::Unnamed(_) => {
                quote_spanned!(type_span=> compile_error!("only structs with named fields! This is an unnamed struct"))
            }
            Fields::Unit => {
                quote_spanned!(type_span=> compile_error!("only structs with named fields! This is a unit struct"))
            }
        },
        Data::Enum(_) => {
            quote_spanned!(type_span=> compile_error!("only structs! This is an enum"))
        }
        Data::Union(_) => {
            quote_spanned!(type_span=> compile_error!("only structs! This is a union type"))
        }
    }
}

fn create_to_positional(
    fields: &FieldsNamed,
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    parse_fields_into_positional_field_stream(fields).map(|fields| {
        quote! {
            fn to_positional_row(&self) -> String {
                let out: Vec<PositionalField> = vec![#(#fields),*];
                let mut fields = vec![];
                for positional_field in out {
                    fields.push(positional_field.to_string());
                }
                fields.join("")
            }
        }
    })
}

fn parse_fields_into_positional_field_stream(
    fields: &FieldsNamed,
) -> Result<Vec<proc_macro2::TokenStream>, proc_macro2::TokenStream> {
    let mut field_token_streams = vec![];
    for field in &fields.named {
        // it's ok to unwrap here, because we are in a Struct with named fields
        let field_ident = field.ident.as_ref().unwrap();
        for attr in &field.attrs {
            if attr.path.is_ident(super::FIELD_ATTR_NAME) {
                let meta = attr.parse_meta().expect("unable to parse meta");
                let field_type = extract_option_type(&field.ty);
                let mut attrs = vec![];
                parse_meta(&mut attrs, meta);
                let row_attributes = create_row_attributes(attrs)?;
                let size = row_attributes.size;
                let filler = row_attributes.filler;
                let align = row_attributes.align == FieldAlignment::Left;

                let output = match field_type {
                    // simple type definition like i32 or String
                    None => {
                        quote! {
                            PositionalField::new(&Some(&self.#field_ident), #size, #filler, #align)
                        }
                    }
                    // optional type definition like Option<i32> or Option<String>
                    Some(_) => {
                        quote! {
                            PositionalField::new(&self.#field_ident.as_ref(), #size, #filler, #align)
                        }
                    }
                };
                field_token_streams.push(output);
            }
        }
    }
    Ok(field_token_streams)
}
