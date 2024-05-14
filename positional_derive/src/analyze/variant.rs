use proc_macro_error::abort;
use syn::spanned::Spanned;
use syn::{Expr, Fields};

const MATCHER_ATTRIBUTE: &str = "matcher";

pub struct Variant {
    pub variant: syn::Variant,
    pub sub_variant_type: syn::Type,
    pub matcher: Matcher,
}

#[derive(Debug, Clone)]
pub struct Matcher {
    pub expr: Expr,
}

impl Variant {
    pub fn new(variant: syn::Variant) -> Option<Self> {
        match variant.clone().fields {
            Fields::Named(_) => {
                abort!(variant, "wrong enum variant"; help = "named variants are not supported")
            }
            Fields::Unnamed(fields_unnamed) => {
                if fields_unnamed.unnamed.len() > 1 {
                    abort!(
                        variant,
                        "wrong enum variant";
                        help = "Only one unnamed field (implementing the relative ToPositionalRow/ToPositionalRow trait) is accepted"
                    )
                };
                let first_field = fields_unnamed.unnamed.first().cloned();
                let sub_variant_type = first_field.unwrap().ty;

                match parse_variant_attributes(&variant) {
                    None => {
                        abort!(
                            variant,
                            "matcher missing";
                            help = "You need to define a matcher for every enum variant. Like #[matcher(row[0..2] == \"00\")]"
                        )
                    }
                    Some(matcher) => Some(Self {
                        variant,
                        sub_variant_type,
                        matcher,
                    }),
                }
            }
            Fields::Unit => {
                abort!(variant, "wrong enum variant"; help = "unit variants are not supported")
            }
        }
    }
}

fn parse_variant_attributes(variant: &syn::Variant) -> Option<Matcher> {
    variant
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident(MATCHER_ATTRIBUTE))
        .map(parse_matcher_expression)
}

fn parse_matcher_expression(attribute: &syn::Attribute) -> Matcher {
    match attribute.parse_args::<Expr>() {
        Ok(expr) => Matcher { expr },
        Err(err) => {
            abort!(
                err.span(),
                "expected an expression as matcher";
                help = "example syntax: `#[matcher(row[0..2] == \"00\")]`"
            )
        }
    }
}
