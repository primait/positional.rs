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
    fn to_positional_row(&self) -> String {
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

#[derive(Debug, PartialEq, ::positional::FromPositionalRow, ::positional::ToPositionalRow)]
struct JohnData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, ::positional::FromPositionalRow, ::positional::ToPositionalRow)]
struct PaulData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, ::positional::FromPositionalRow, ::positional::ToPositionalRow)]
enum Beatles {
    #[matcher(&row[0..=3] == "john")]
    John(JohnData),
    #[matcher(&row[0..=3] == "paul")]
    Paul(PaulData),
}

impl JohnData {
    #[allow(dead_code)]
    fn from_positional_row(_: &str) -> Self {
        unreachable!()
    }

    #[allow(dead_code)]
    fn to_positional_row(&self) -> String {
        unreachable!()
    }
}

impl PaulData {
    #[allow(dead_code)]
    fn from_positional_row(_: &str) -> Self {
        unreachable!()
    }

    #[allow(dead_code)]
    fn to_positional_row(&self) -> String {
        unreachable!()
    }
}

impl Beatles {
    #[allow(dead_code)]
    fn from_positional_row(_: &str) -> Self {
        unreachable!()
    }

    #[allow(dead_code)]
    fn to_positional_row(&self) -> String {
        unreachable!()
    }
}

#[test]
fn struct_parse_no_name_clashes() {
    let row =
        ::positional::FromPositionalRow::from_positional_row("1    ---10the address is this ")
            .expect("error converting from positional row");
    assert_eq!(Data::new(1, 10, "the address is this"), row);
}

#[test]
fn struct_ser_de_no_name_clashes() {
    let data = Data::new(1, 100, "the address is this");
    let row = ::positional::ToPositionalRow::to_positional_row(&data);
    let original_data: Data = ::positional::FromPositionalRow::from_positional_row(&row)
        .expect("error converting from positional row");
    assert_eq!(original_data, data);
}

#[test]
fn enum_no_name_clashes() {
    // Parsing
    let john_data = Beatles::John(JohnData {
        name: "john".to_string(),
    });
    let paul_data = Beatles::Paul(PaulData {
        name: "paul".to_string(),
    });
    let row_john = "john      ";
    let row_paul = "paul      ";

    let row_wrong = "xxxx      ";

    assert_eq!(
        <Beatles as ::positional::FromPositionalRow>::from_positional_row(row_john).unwrap(),
        john_data
    );
    assert_eq!(
        <Beatles as ::positional::FromPositionalRow>::from_positional_row(row_paul).unwrap(),
        paul_data
    );
    assert!(<Beatles as ::positional::FromPositionalRow>::from_positional_row(row_wrong).is_err());

    // Serializing
    let writer = ::positional::Writer::new(vec![john_data, paul_data]);
    assert_eq!("john      \npaul      ", writer.to_string());
}
