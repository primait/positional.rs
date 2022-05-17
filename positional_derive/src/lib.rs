use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta, Path};

// Generate a compile error to output struct name
#[proc_macro_derive(PositionalRow, attributes(positional))]
pub fn positional_row(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as DeriveInput);
    let type_name = ast.ident;
    let mut all_fields = vec![];

    match ast.data {
        Data::Struct(data_struct) => {
            match data_struct.fields {
                Fields::Named(fields) => {
                    for field in fields.named {
                        // it's ok to unwrap here, because we are in a Struct with named fields
                        let field_ident = field.ident.unwrap();
                        for attr in field.attrs {
                            if attr.path.is_ident("positional") {
                                let meta = attr.parse_meta().expect("unable to parse meta");
                                let mut attrs = vec![];
                                parse_meta(&mut attrs, meta);
                                let row_attributes = create_row_attributes(attrs);
                                let size = row_attributes.size;
                                let filler = row_attributes.filler;
                                let align = row_attributes.align == FieldAlignment::Left;
                                let output = quote! {
                                    PositionalField::new(self.#field_ident.to_string(), #size, #filler, #align)
                                };
                                all_fields.push(output);
                            }
                        }
                    }
                }
                Fields::Unnamed(_) => {
                    panic!("only structs with named fields! This is an unnamed struct")
                }
                Fields::Unit => panic!("only structs with named fields! This is a unit struct"),
            }
        }
        Data::Enum(_) => panic!("only structs! This is an enum"),
        Data::Union(_) => panic!("only structs! This is a union type."),
    };

    quote! {
        impl PositionalRow for #type_name {
            fn to_positional_row(&self) -> String {
                let out = vec![#(#all_fields),*];
                out.iter().map(|positional_field| positional_field.to_string()).collect::<Vec<String>>().join("")
            }
        }
    }
    .into()
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

#[derive(PartialEq)]
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

struct RowAttributes {
    size: usize,
    filler: char,
    align: FieldAlignment,
}

fn create_row_attributes(attrs: Vec<(Path, Lit)>) -> RowAttributes {
    let mut size = 10;
    let mut filler = ' ';
    let mut align = FieldAlignment::Left;
    for (path, lit) in attrs {
        if path.is_ident("size") {
            size = match &lit {
                Lit::Int(lit_int) => lit_int.base10_parse().unwrap(),
                _ => panic!("size should be an int"),
            };
        }

        if path.is_ident("filler") {
            filler = match &lit {
                Lit::Char(lit_char) => lit_char.value(),
                _ => panic!("filler should be a char"),
            };
        }

        if path.is_ident("align") {
            align = match &lit {
                Lit::Str(lit_str) => lit_str.value().parse().unwrap(),
                _ => panic!("align should be either left or right"),
            };
        }
    }
    RowAttributes {
        size,
        filler,
        align,
    }
}
