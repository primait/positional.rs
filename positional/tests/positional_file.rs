use positional::*;
use std::fmt::{Display, Formatter};

#[derive(PositionalRow)]
struct Data {
    #[positional(size = 5)]
    name: SubData,
    #[positional(size = 5, filler = '-')]
    age: i32,
    #[positional(size = 20)]
    address: String,
}

struct SubData {
    id: i32,
}

impl Display for SubData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Data {
    pub fn new(id: i32, age: i32, address: impl Into<String>) -> Self {
        Self {
            name: SubData { id },
            age,
            address: address.into(),
        }
    }
}

#[test]
fn test_simple_positional_file() {
    let rows = vec![Data::new(1, 10, "the address is this")];
    let positional_file: PositionalFile<Data> = PositionalFile::new(rows);
    assert_eq!(
        "1    10---the address is this ",
        positional_file.to_positional_file()
    );
}
