use positional_derive::ToPositionalRow;

#[derive(ToPositionalRow)]
struct MyData {
    #[field]
    name: String,
}

fn main() {}
