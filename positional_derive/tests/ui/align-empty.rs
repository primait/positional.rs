use positional_derive::ToPositionalRow;

#[derive(ToPositionalRow)]
struct MyData {
    #[field(size = 10, align = "")]
    name: String,
}

fn main() {}
