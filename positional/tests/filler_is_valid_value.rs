use positional::{FromPositionalRow, ToPositionalRow};

#[test]
fn filler_is_valid_number() {
    #[derive(FromPositionalRow, ToPositionalRow, Eq, PartialEq, Debug)]
    pub struct PositionalStruct {
        #[field(size = 2, align = "right", filler = '0', no_trim)]
        pub u_32: u32,
    }

    let row = PositionalStruct { u_32: 0 };

    let str = row.to_positional_row();
    let parsed = PositionalStruct::from_positional_row(&str);
    assert!(matches!(parsed, Ok(_)));
    assert_eq!(row, parsed.unwrap());
}

#[test]
fn filler_is_valid_string() {
    #[derive(FromPositionalRow, ToPositionalRow, Eq, PartialEq, Debug)]
    pub struct PositionalStruct {
        #[field(size = 6, align = "left", filler = '*', no_trim)]
        pub a_string: String,
    }

    let row = PositionalStruct {
        a_string: "data**".to_string(),
    };

    let str = row.to_positional_row();
    let parsed = PositionalStruct::from_positional_row(&str);
    assert!(matches!(parsed, Ok(_)));
    assert_eq!(row, parsed.unwrap());
}
