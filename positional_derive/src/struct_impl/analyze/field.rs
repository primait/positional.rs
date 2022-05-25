use super::row_attributes::RowAttributes;
use proc_macro_error::abort;
use std::collections::HashMap;

pub struct Field {
    pub ident: syn::Field,
    pub optional: bool,
    pub attributes: RowAttributes,
}

impl Field {
    pub fn new(field: syn::Field, attrs: &HashMap<String, syn::Lit>) -> Result<Self, String> {
        let option_type = extract_option_type(&field.ty);
        let attributes: Result<RowAttributes, Option<syn::Lit>> = attrs.try_into();
        match attributes {
            Ok(attrs) => Ok(Self {
                optional: option_type.is_some(),
                ident: field,
                attributes: attrs,
            }),
            Err(lit) => match lit {
                None => {
                    abort!(
                        field,
                        "missing field configuration";
                        help = "missing field configuration"
                    )
                }
                Some(lit) => {
                    abort!(
                        lit,
                        "wrong field configuration";
                        help = "wrong field configuration"
                    )
                }
            },
        }
    }
}

fn extract_option_type(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push('|');
            acc
        });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .iter()
            .find(|s| &idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(extract_option_segment)
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}
