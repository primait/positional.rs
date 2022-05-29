mod analyze;
mod codegen;
mod lower;

pub use analyze::{analyze, FieldAlignment};
pub use codegen::{codegen, Rust};
pub use lower::{lower, Field, ImplBlockType};
