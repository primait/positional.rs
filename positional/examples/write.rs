use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use positional::*;

#[derive(ToPositionalRow, FromPositionalRow, Dummy, Debug)]
struct Data {
    #[field(size = 25)]
    name: String,

    #[field(size = 25)]
    surname: String,

    #[field(size = 40, filler = '_', align = "right")]
    created: DateTime<Utc>,

    #[field(size = 40, filler = '+', align = "right")]
    updated_at: Option<DateTime<Utc>>,
}

pub fn main() {
    let mut rows: Vec<Data> = vec![];
    for _ in 1..=100 {
        rows.push(Faker.fake())
    }
    let positional_file = Writer::new(rows);
    let output = positional_file.to_string();
    println!("{}", &output);
}
