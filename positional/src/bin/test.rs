use positional::*;
use positional_derive::PositionalRow;

#[derive(PositionalRow)]
struct Data {
    #[positional(size = 5, filler = ' ', align = "left")]
    id: String,
    #[positional(size = 20, filler = ' ', align = "left")]
    value: String,
    #[positional(size = 10, filler = '0', align = "right")]
    count: i32,
}

pub fn main() {
    let data = Data {
        id: "1".to_string(),
        value: "test value".to_string(),
        count: 10,
    };

    println!("{}", data.to_positional_row());
    assert_eq!(
        "1    test value          0000000010",
        data.to_positional_row()
    );
}
