use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, Lit, Meta, NestedMeta, Path};

const FIELD_ATTR_NAME: &str = "field";

#[proc_macro_derive(PositionalRow, attributes(field))]
pub fn positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    positional_for_struct(ast).into()
}

fn positional_for_struct(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let type_name = ast.ident;
    let type_span = type_name.span();

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let to_positional = match create_to_positional(&fields) {
                    Ok(to_positional_stream) => to_positional_stream,
                    Err(error) => return error,
                };
                let from_positional = match create_from_positional(&fields) {
                    Ok(from_positional_stream) => from_positional_stream,
                    Err(error) => return error,
                };
                quote! {
                    impl PositionalRow for #type_name {
                        #to_positional
                        #from_positional
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

fn create_from_positional(
    fields: &FieldsNamed,
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    parse_fields_into_struct_builder_stream(fields).map(|fields| {
        quote! {
            fn from_positional_row(row: impl ToString) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
                let row_string = row.to_string();
                Ok(Self {
                    #(#fields),*
                })
            }
        }
    })
}

fn parse_fields_into_struct_builder_stream(
    fields: &FieldsNamed,
) -> Result<Vec<proc_macro2::TokenStream>, proc_macro2::TokenStream> {
    let mut field_token_streams = vec![];
    let mut offset = 0;
    for field in &fields.named {
        // it's ok to unwrap here, because we are in a Struct with named fields
        let field_ident = field.ident.as_ref().unwrap();
        for attr in &field.attrs {
            if attr.path.is_ident(FIELD_ATTR_NAME) {
                let meta = attr.parse_meta().expect("unable to parse meta");
                let mut attrs = vec![];
                parse_meta(&mut attrs, meta);
                let row_attributes = create_row_attributes(attrs)?;
                let size = row_attributes.size;
                let filler = row_attributes.filler;
                let align = row_attributes.align == FieldAlignment::Left;
                let output = quote! {
                    #field_ident: PositionalParsedField::new(row.to_string(), #offset, #size, #filler, #align).to_value().parse()?
                };
                offset += size;
                field_token_streams.push(output);
            }
        }
    }
    Ok(field_token_streams)
}

fn parse_fields_into_positional_field_stream(
    fields: &FieldsNamed,
) -> Result<Vec<proc_macro2::TokenStream>, proc_macro2::TokenStream> {
    let mut field_token_streams = vec![];
    for field in &fields.named {
        // it's ok to unwrap here, because we are in a Struct with named fields
        let field_ident = field.ident.as_ref().unwrap();
        for attr in &field.attrs {
            if attr.path.is_ident(FIELD_ATTR_NAME) {
                let meta = attr.parse_meta().expect("unable to parse meta");
                let mut attrs = vec![];
                parse_meta(&mut attrs, meta);
                let row_attributes = create_row_attributes(attrs)?;
                let size = row_attributes.size;
                let filler = row_attributes.filler;
                let align = row_attributes.align == FieldAlignment::Left;
                let output = quote! {
                    PositionalField::new(self.#field_ident.to_string(), #size, #filler, #align)
                };
                field_token_streams.push(output);
            }
        }
    }
    Ok(field_token_streams)
}

fn parse_meta(attrs: &mut Vec<(Path, Lit)>, meta: Meta) {
    match meta {
        Meta::Path(_) => {}
        Meta::List(meta_list) => {
            for nested_meta in meta_list.nested {
                match nested_meta {
                    NestedMeta::Meta(name_value) => parse_meta(attrs, name_value),
                    NestedMeta::Lit(_) => {}
                }
            }
        }
        Meta::NameValue(name_value) => attrs.push((name_value.path, name_value.lit)),
    }
}

#[derive(PartialEq, Debug)]
enum FieldAlignment {
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
                "align value should be 'left' or 'right', {} provided",
                s
            )),
        }
    }
}

#[derive(Debug)]
struct RowAttributes {
    size: usize,
    filler: char,
    align: FieldAlignment,
}

fn create_row_attributes(
    attrs: Vec<(Path, Lit)>,
) -> Result<RowAttributes, proc_macro2::TokenStream> {
    let mut size = 10;
    let mut filler = ' ';
    let mut align = FieldAlignment::Left;
    for (path, lit) in attrs {
        if path.is_ident("size") {
            size = match &lit {
                Lit::Int(lit_int) => lit_int.base10_parse().unwrap(),
                _ => {
                    let span = path.span();
                    return Err(quote_spanned!(span=> compile_error!("size should be an int")));
                }
            };
        }

        if path.is_ident("filler") {
            filler = match &lit {
                Lit::Char(lit_char) => lit_char.value(),
                _ => {
                    let span = path.span();
                    return Err(quote_spanned!(span=> compile_error!("filler should be a char")));
                }
            };
        }

        if path.is_ident("align") {
            align = match &lit {
                Lit::Str(lit_str) => lit_str.value().parse().unwrap(),
                _ => {
                    let span = path.span();
                    return Err(
                        quote_spanned!(span=> compile_error!("align should be either left or right")),
                    );
                }
            };
        }
    }
    Ok(RowAttributes {
        size,
        filler,
        align,
    })
}
