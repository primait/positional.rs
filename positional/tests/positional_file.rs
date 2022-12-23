use positional::{ToPositionalRow, Writer};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(ToPositionalRow)]
struct Data {
    #[field(size = 5)]
    name: SubData,
    #[field(size = 5, filler = '-')]
    age: i32,
    #[field(size = 20)]
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

impl FromStr for SubData {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: i32 = s.parse()?;
        Ok(Self { id })
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
fn simple() {
    let rows = vec![Data::new(1, 10, "the address is this")];
    let positional_file: Writer<Data> = Writer::new(rows);
    assert_eq!(
        "1    10---the address is this ",
        positional_file.to_string()
    );
}
