use positional_derive::FromPositionalRow;

#[derive(FromPositionalRow)]
enum MyData {
    Row1(String),
}

fn main() {}
