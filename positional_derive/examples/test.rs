use positional_derive::*;

#[derive(ToPositionalRow)]
struct Data {
    #[field(size = 10, filler = '-', align = "r")]
    name: String,
}

fn main() {}
