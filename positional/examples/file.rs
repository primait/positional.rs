#![allow(dead_code)]

use positional::*;

#[derive(FromPositionalRow)]
struct JohnData {
    #[field(size = 10)]
    name: String,
}

#[derive(FromPositionalRow)]
struct PaulData {
    #[field(size = 10)]
    name: String,
}

#[derive(FromPositionalRow)]
struct GeorgeData {
    #[field(size = 10)]
    name: String,
}

#[derive(FromPositionalRow)]
struct RingoData {
    #[field(size = 10)]
    name: String,
}

#[derive(FromPositionalRow)]
enum Beatles {
    #[matcher(self[0..=3] == "john")]
    John(JohnData),
    #[matcher(self[0..=3] == "paul")]
    Paul(PaulData),
    #[matcher(self[0..=3] == "george")]
    George(GeorgeData),
    #[matcher(self[0..=3] == "ringo")]
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
