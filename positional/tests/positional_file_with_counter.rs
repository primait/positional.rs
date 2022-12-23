use positional::{ToPositionalRow, Writer};

#[derive(ToPositionalRow)]
struct Data {
    #[field(size = 5)]
    id: i32,
    #[field(size = 5)]
    name: String,
    #[field(size = 5, filler = '-')]
    age: i32,
    #[field(size = 20)]
    address: String,
}

impl Data {
    pub fn new(id: i32, name: impl Into<String>, age: i32, address: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            age,
            address: address.into(),
        }
    }
}

#[test]
fn simple() {
    let mut ids = 1..;
    let rows = vec![
        Data::new(ids.next().unwrap(), "test", 10, "the address is this"),
        Data::new(ids.next().unwrap(), "test2", 12, "the address is this"),
    ];
    let positional_file: Writer<Data> = Writer::new(rows);
    assert_eq!(
        "1    test 10---the address is this \n2    test212---the address is this ",
        positional_file.to_string()
    );
}
