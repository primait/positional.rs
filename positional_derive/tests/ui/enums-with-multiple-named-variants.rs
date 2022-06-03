use positional_derive::FromPositionalRow;

#[derive(FromPositionalRow)]
enum MyData {
    #[matcher(row[0..2] == "11")]
    Row1(String, String),
}

fn main() {}
