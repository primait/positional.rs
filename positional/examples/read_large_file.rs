use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use positional::*;

#[derive(FromPositionalRow, Debug)]
struct Data {
    #[field(size = 25)]
    name: String,

    #[field(size = 25)]
    surname: String,

    #[field(size = 40, filler = '_', align = "right")]
    created: DateTime<Utc>,
}

pub fn main() {
    let file_reader: positional::FileReader<Data> =
        FileReader::from_path("output.txt").expect("unable to open the file");

    for data in file_reader {
        dbg!(data);
    }
}
