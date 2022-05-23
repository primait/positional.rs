use positional_derive::ToPositionalRow;

#[derive(ToPositionalRow)]
struct MyData {
    #[field(align = "")]
    name: String,
}

fn main() {}
