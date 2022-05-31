#![allow(dead_code)]

use positional::*;

#[derive(Debug, PartialEq, FromPositionalRow)]
struct JohnData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, FromPositionalRow)]
struct PaulData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, FromPositionalRow)]
struct GeorgeData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, FromPositionalRow)]
struct RingoData {
    #[field(size = 10)]
    name: String,
}

#[derive(Debug, PartialEq, FromPositionalRow)]
enum Beatles {
    #[matcher(&row_string[0..=3] == "john")]
    John(JohnData),
    #[matcher(&row_string[0..=3] == "paul")]
    Paul(PaulData),
    #[matcher(&row_string[0..=3] == "george")]
    George(GeorgeData),
    #[matcher(&row_string[0..=3] == "ringo")]
    Ringo(RingoData),
}

fn main() {
    let john_data = Beatles::John(JohnData {
        name: "john".to_string(),
    });
    let paul_data = Beatles::Paul(PaulData {
        name: "paul".to_string(),
    });
    let row_john = "john      ";
    let row_paul = "paul      ";

    let row_wrong = "xxxx      ";

    assert_eq!(Beatles::parse(row_john).unwrap(), john_data);
    assert_eq!(Beatles::parse(row_paul).unwrap(), paul_data);
    assert!(Beatles::parse(row_wrong).is_err());
}
