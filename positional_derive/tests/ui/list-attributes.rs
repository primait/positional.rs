use positional_derive::ToPositionalRow;

#[derive(ToPositionalRow)]
struct MyData {
    #[test(a)]
    name: String,
}

fn main() {}
