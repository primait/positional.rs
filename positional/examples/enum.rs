#![allow(dead_code)]

use positional::*;

#[derive(Debug, PartialEq, FromPositionalRow, ToPositionalRow)]
struct JohnData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, FromPositionalRow, ToPositionalRow)]
struct PaulData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, FromPositionalRow, ToPositionalRow)]
struct GeorgeData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, FromPositionalRow, ToPositionalRow)]
struct RingoData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, FromPositionalRow, ToPositionalRow)]
enum Beatles {
    #[matcher(&row[0..=3] == "john")]
    John(JohnData),
    #[matcher(&row[0..=3] == "paul")]
    Paul(PaulData),
    #[matcher(&row[0..=3] == "george")]
    George(GeorgeData),
    #[matcher(&row[0..=3] == "ringo")]
    Ringo(RingoData),
}

fn main() {
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

    assert_eq!(Beatles::from_positional_row(row_john).unwrap(), john_data);
    assert_eq!(Beatles::from_positional_row(row_paul).unwrap(), paul_data);
    assert!(Beatles::from_positional_row(row_wrong).is_err());

    // Serializing
    let writer = Writer::new(vec![john_data, paul_data]);
    assert_eq!("john      \npaul      ", writer.to_string());
}
