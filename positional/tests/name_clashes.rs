// Force name clashes to make sure the macros generate code with fully qualified imports.

pub trait ToPositionalField {}
pub trait ToPositionalRow {}
pub trait FromPositionalRow {}

mod positional {}

#[derive(::positional::ToPositionalRow, ::positional::FromPositionalRow, PartialEq, Debug)]
struct Data {
    #[field(size = 5)]
    name: String,
    #[field(size = 5, filler = '-', align = "right")]
    age: i32,
    #[field(size = 20)]
    address: String,
}

impl Data {
    #[allow(dead_code)]
    fn from_positional_row(_: &str) -> Self {
        unreachable!()
    }

    #[allow(dead_code)]
    fn to_positional_row(self) -> String {
        unreachable!()
    }
}

impl Data {
    pub fn new(name: impl ToString, age: i32, address: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            age,
            address: address.to_string(),
        }
    }
}

#[test]
fn parse() {
    let row =
        ::positional::FromPositionalRow::from_positional_row("1    ---10the address is this ")
            .expect("error converting from positional row");
    assert_eq!(Data::new(1, 10, "the address is this"), row);
}

#[test]
fn ser_de() {
    let data = Data::new(1, 100, "the address is this");
    let row = ::positional::ToPositionalRow::to_positional_row(&data);
    let original_data: Data = ::positional::FromPositionalRow::from_positional_row(&row)
        .expect("error converting from positional row");
    assert_eq!(original_data, data);
}
