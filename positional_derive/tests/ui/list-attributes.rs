use positional_derive::ToPositionalRow;

#[derive(ToPositionalRow)]
struct MyData {
    #[field(a)]
    name: String,
}

fn main() {}
