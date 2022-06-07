//! This is a library to parse and write positional files
//!
//! # Getting Started
//! You start by defining your own struct that represent a single row in the positional file
//! ```
//! struct RowData {
//!     name: String,
//!     surname: String,
//!     age: i32,
//! }
//! ```
//! If you have the data in memory and want to serialize the struct in a positional file
//! you need to annotate the struct with the `ToPositionalRow` derive.
//! ```
//! use positional::*;
//!
//! #[derive(ToPositionalRow)]
//! struct RowData {
//!     #[field(size = 10)]
//!     name: String,
//!     #[field(size = 10, filler = '-')]
//!     surname: String,
//!     #[field(size = 5, align = "right")]
//!     age: i32,
//! }
//! let row_data = RowData {
//!     name: "test".to_string(),
//!     surname: "test".to_string(),
//!     age: 20,
//! };
//! assert_eq!("test      test------   20", row_data.to_positional_row());
//! ```
//!
//! If you are parsing a file you can use the `FromPositionalRow` derive
//! ```
//! use positional::*;
//!
//! #[derive(FromPositionalRow)]
//! struct RowData {
//!     #[field(size = 20)]
//!     name: String,
//!     #[field(size = 20, filler = '-')]
//!     surname: String,
//!     #[field(size = 20, align = "right")]
//!     age: i32,
//! }
//! ```
//!
//! You can use both on the same struct if that makes sense in your domain model.
//!
//! We annotate the struct to be serializable to/deserializable from a positional row.
//! We also need to annotate every field in the struct to configure the field specification.
//!
//!
//! Possible attributes are:
//!
//! | attribute name | mandatory | type   | default      | description                                                      |
//! |----------------|-----------|--------|--------------|------------------------------------------------------------------|
//! | size           | yes       | number | ---          | define the size of the field in the positional row               |
//! | filler         | no        | char   | `whitespace` | define what represent the empty space in the field               |
//! | align          | no        | string | `"left"`     | define the alignment of the field. It could be *left* or *right* |
//!
//! ### Use your own types
//!
//! Fields are not limited to simple types like `String` or `i32`, you can use any type as long as
//! it implements the trait `FromStr` for parsing and `ToString` for serializing.
//! For the ToString implementation the library will take care of fill and trim the values for
//! the positional representation. You just need to take care of converting the value to a string.
//!
//! # Files with multiple row types
//!
//! It could happen that positional files contains more than one line type. In those cases normally
//! you can tell one row from the other by looking at one particular position in the row that identifies
//! the row type. This is useful only for parsing, serializing is basically the same.
//! You can use an enum to represent all rows inside a file.
//!
//! ```
//! use positional::*;
//!
//! #[derive(FromPositionalRow, ToPositionalRow)]
//! enum Rows {
//!     #[matcher(&row[0..2] == "10")]
//!     Row1(Row1Data),
//!     #[matcher(&row[0..2] == "20")]
//!     Row2(Row2Data),
//! }
//!
//! #[derive(FromPositionalRow, ToPositionalRow)]
//! struct Row1Data {
//!     #[field(size = 2)]
//!     row_type: String,
//!     #[field(size = 20)]
//!     name: String,
//!     #[field(size = 20, align = "right")]
//!     age: i32,
//! }
//!
//! #[derive(FromPositionalRow, ToPositionalRow)]
//! struct Row2Data {
//!     #[field(size = 2)]
//!     row_type: String,
//!     #[field(size = 20)]
//!     name: String,
//!     #[field(size = 20, align = "right")]
//!     age: i32,
//! }
//! ```
//!
//! The enum should have variants with one (and only one) anonymous parameter. To tell the row you
//! annotate the enum variants with `matcher` attribute and provide an expression.
//! The expression can access the `row` variable, which contains the full row as a string.
//! In the example we are matching the two starting chars from the string with a given value
//!
//! # How it works
//!
//! Under the hood, the library just deals with 2 traits: [FromPositionalRow], and [ToPositionalRow]
//! You could use those traits and just use the positional library to handle the actual
//! parsing/creation of the positional files.
//!
//! The procedural macros `FromPositionalRow` and `ToPositionalRow` just do the implementation for
//! you, by leveraging on annotations and rust type system.

mod error;
mod field;
mod file;
#[doc(hidden)]
mod parsed_field;
mod row;

pub use self::{
    error::{PositionalError, PositionalResult},
    field::{PositionalField, ToPositionalField},
    file::{Reader, Writer},
    parsed_field::PositionalParsedField,
    row::{FromPositionalRow, ToPositionalRow},
};
pub use positional_derive::{FromPositionalRow, ToPositionalRow};
