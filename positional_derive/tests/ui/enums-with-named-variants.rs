use positional_derive::ToPositionalRow;

#[derive(ToPositionalRow)]
enum MyData {
    Row1 { name: String },
}

fn main() {}
