use positional::*;

#[derive(PositionalRow)]
struct Data {
    #[positional(size = 5, filler = " ", alignment: LEFT)]
    id: String,
    #[positional(size = 20, filler = " ", alignment: LEFT)]
    value: String,
    #[positional(size = 10, filler = "0", alignment: RIGHT)]
    count: i32,
}

pub fn main() {
    let data = Data {
        id: "1".to_string(),
        value: "test value".to_string(),
        count: 10,
    };

    println!("{}", data.to_positional_row());
}
