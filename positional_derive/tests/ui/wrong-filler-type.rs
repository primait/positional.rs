use positional_derive::ToPositionalRow;

#[derive(ToPositionalRow)]
struct MyData {
    #[field(size = 10, filler = "a")]
    name: String,
}

fn main() {}
