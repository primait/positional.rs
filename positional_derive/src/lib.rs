use darling::{ast, util, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse, parse_macro_input, Attribute, Data, DeriveInput, NestedMeta};
use syn::{Ident, Type};

#[derive(Debug, FromField)]
#[darling(attributes(positional))]
struct PositionalRowField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    skip: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(positional), supports(struct_named))]
struct PositionalRow {
    size: i32,
    filler: String,
}

// // Generate a compile error to output struct name
// #[proc_macro_derive(PositionalRow, attributes(positional))]
// pub fn positional_row(tokens: TokenStream) -> TokenStream {
//     //dbg!(&tokens);
//     // convert the input tokens into an ast, specially from a derive
//     let ast = parse_macro_input!(tokens as DeriveInput);
//
//     //panic!("My struct name is: <{}>", ast.ident.to_string());
//
//     let type_name = ast.ident;
//
//     match ast.data {
//         Data::Struct(data_struct) => {
//             for field in data_struct.fields {
//                 let mut nested_meta = vec![];
//                 for attr in field.attrs {
//                     if attr.path.is_ident("positional") {
//                         dbg!(attr.parse_args::<Positional>());
//                     }
//                     //nested_meta.push(NestedMeta::from(attr.parse_meta().unwrap()));
//                 }
//                 let args = match Positional::from_list(&nested_meta) {
//                     Ok(v) => v,
//                     Err(e) => {
//                         return TokenStream::from(e.write_errors());
//                     }
//                 };
//             }
//         }
//         Data::Enum(_) => panic!("only structs! This is an enum"),
//         Data::Union(_) => panic!("only structs! This is a union type."),
//     };
//
//     quote! {
//         impl PositionalRow for #type_name {
//             fn to_positional_row(&self) -> String {
//                 "test".to_string()
//             }
//         }
//     }
//     .into()
// }
