use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use positional::*;

#[derive(ToPositionalRow, FromPositionalRow, Dummy)]
struct Data {
    #[field(size = 5)]
    name: String,

    #[field(size = 40, filler = '_')]
    created: DateTime<Utc>,
}

pub fn main() {
    let mut rows: Vec<Data> = vec![];
    for _ in 1..100 {
        rows.push(Faker.fake())
    }
    let positional_file = PositionalFile::new(rows);
    let output = positional_file
        .to_positional_file()
        .expect("invalid format");
    println!("{}", &output);
}
