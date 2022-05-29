#![allow(dead_code)]

use positional::*;

#[derive(ToPositionalRow)]
struct JohnData {
    #[field(size = 10)]
    name: String,
}

#[derive(ToPositionalRow)]
struct PaulData {
    #[field(size = 10)]
    name: String,
}

#[derive(ToPositionalRow)]
struct GeorgeData {
    #[field(size = 10)]
    name: String,
}

#[derive(ToPositionalRow)]
struct RingoData {
    #[field(size = 10)]
    name: String,
}

enum Beatles {
    John(JohnData),
    Paul(PaulData),
    George(GeorgeData),
    Ringo(RingoData),
}

// impl ToPositionalRow for Beatles {
//     fn to_positional_row(&self) -> String {
//         match self {
//             Beatles::John(john) => john.to_positional_row(),
//             Beatles::Paul(paul) => paul.to_positional_row(),
//             Beatles::George(george) => george.to_positional_row(),
//             Beatles::Ringo(ringo) => ringo.to_positional_row(),
//         }
//     }
// }

fn main() {
    let _john = Beatles::John(JohnData {
        name: "john".to_string(),
    });
    //assert_eq!("john      ", john.to_positional_row());
}
