use proc_macro2::TokenStream;

mod from_enum;
mod from_struct;

use crate::lower::Ir;
use crate::ImplBlockType;

pub type Rust = TokenStream;

pub fn codegen(ir: Ir, impl_block_type: ImplBlockType) -> Rust {
    match ir {
        Ir::Struct(struct_ir) => from_struct::codegen_struct(struct_ir, impl_block_type),
        Ir::Enum(enum_ir) => from_enum::codegen_enum(enum_ir, impl_block_type),
    }
}
