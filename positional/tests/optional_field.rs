use positional::*;

#[derive(FromPositionalRow, ToPositionalRow, PartialEq, Debug)]
struct Data {
    #[field(size = 5)]
    name: String,
    #[field(size = 5, filler = '-', align = "right")]
    age: Option<i32>,
    #[field(size = 20)]
    address: String,
}

impl Data {
    pub fn new(name: impl ToString, age: Option<i32>, address: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            age,
            address: address.to_string(),
        }
    }
}

#[test]
fn parse() {
    let row = FromPositionalRow::from_positional_row("1    -----the address is this ")
        .expect("error converting from positional row");
    assert_eq!(Data::new(1, None, "the address is this"), row);
}

#[test]
fn write() {
    let rows = vec![Data::new(1, None, "the address is this")];
    let positional_file: Writer<Data> = Writer::new(rows);
    assert_eq!(
        "1    -----the address is this ",
        positional_file.to_string()
    );
}
