use positional::{FromPositionalRow, PositionalError, ToPositionalRow};

#[derive(ToPositionalRow, FromPositionalRow, PartialEq, Debug)]
struct Data {
    #[field(size = 5)]
    name: String,
    #[field(size = 5, filler = '-', align = "right")]
    age: i32,
    #[field(size = 20)]
    address: String,
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
    let row = FromPositionalRow::from_positional_row("1    ---10the address is this ")
        .expect("error converting from positional row");
    assert_eq!(Data::new(1, 10, "the address is this"), row);
}

#[test]
fn ser_de() {
    let data = Data::new(1, 100, "the address is this");
    let row = data.to_positional_row();
    let original_data: Data =
        FromPositionalRow::from_positional_row(&row).expect("error converting from positional row");
    assert_eq!(original_data, data);
}

#[test]
fn empty_string() {
    let row = <Data as FromPositionalRow>::from_positional_row("");
    assert_eq!(row, Err(PositionalError::RowSizeError(30, "".to_string())));
}

#[test]
fn string_smaller_than_field_definition() {
    let row_content = "1    ---10";
    let row = Data::from_positional_row(row_content);
    assert_eq!(
        row,
        Err(PositionalError::RowSizeError(30, row_content.to_string()))
    );
}

#[test]
fn unicode_chars() {
    let row = Data::from_positional_row("Noël ---10the address is this ")
        .expect("error converting from positional row");
    assert_eq!(Data::new("Noël", 10, "the address is this"), row);
}
