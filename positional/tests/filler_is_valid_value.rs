use std::{fmt::Display, str::FromStr};

use positional::{FromPositionalRow, ToPositionalRow};

#[test]
fn filler_is_valid_number() {
    #[derive(FromPositionalRow, ToPositionalRow, Eq, PartialEq, Debug)]
    pub struct PositionalStruct {
        #[field(size = 2, align = "right", filler = '0')]
        pub u_32: u32,
    }

    let row = PositionalStruct { u_32: 0 };

    let str = row.to_positional_row();
    let parsed = PositionalStruct::from_positional_row(&str);
    assert!(parsed.is_ok());
    assert_eq!(row, parsed.unwrap());
}

#[test]
fn filler_is_valid_string() {
    #[derive(PartialEq, Eq, Debug)]
    enum Data {
        A,
        B,
    }

    impl Display for Data {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Data::A => write!(f, "a"),
                Data::B => write!(f, "b"),
            }
        }
    }

    impl FromStr for Data {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "a" => Ok(Data::A),
                "b" => Ok(Data::B),
                other => Err(format!("Unknown variant: {}", other)),
            }
        }
    }

    #[derive(FromPositionalRow, ToPositionalRow, Eq, PartialEq, Debug)]
    pub struct PositionalStruct {
        #[field(size = 6, align = "left", filler = 'a')]
        pub a_string: Data,
    }

    let row = PositionalStruct { a_string: Data::A };

    let str = row.to_positional_row();
    let parsed = PositionalStruct::from_positional_row(&str);
    assert!(parsed.is_ok());
    assert_eq!(row, parsed.unwrap());

    let str = "aaaaaa";
    let parsed = PositionalStruct::from_positional_row(str);
    assert!(parsed.is_ok());
    assert_eq!(row, parsed.unwrap());
}
