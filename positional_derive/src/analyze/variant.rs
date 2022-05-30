use proc_macro_error::abort;
use syn::Fields;

pub struct Variant {
    pub variant: syn::Variant,
}

impl Variant {
    pub fn new(variant: syn::Variant) -> Option<Self> {
        dbg!(&variant);
        match variant.clone().fields {
            Fields::Named(_) => {
                abort!(variant, "wrong enum variant"; help = "named variants are not supported")
            }
            Fields::Unnamed(_fields_unnamed) => Some(Self {
                variant: variant.clone(),
            }),
            Fields::Unit => {
                abort!(variant, "wrong enum variant"; help = "unit variants are not supported")
            }
        }
    }
}
