use positional_derive::FromPositionalRow;

#[derive(FromPositionalRow)]
enum MyData {
    #[matcher(struct)]
    Row1(String),
}

fn main() {}
