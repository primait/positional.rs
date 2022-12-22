use chrono::{DateTime, Utc};
use criterion::{criterion_group, criterion_main, Criterion};
use fake::{Dummy, Fake, Faker};
use positional::{FromPositionalRow, ToPositionalRow};
use std::fs::File;
use std::io::{LineWriter, Write};

#[derive(ToPositionalRow, FromPositionalRow, Dummy, Debug)]
struct Data {
    #[field(size = 30)]
    name: String,

    #[field(size = 40, filler = '_', align = "right")]
    created: DateTime<Utc>,
}

fn serialize_to_file(c: &mut Criterion) {
    c.bench_function("serialize 1.000 structs to file", |b| {
        let dir = std::env::temp_dir();
        let file = File::create(format!("{}/positional.txt", dir.to_str().unwrap()))
            .expect("unable to open a file");
        let mut file = LineWriter::new(file);
        b.iter(move || {
            let iter = std::iter::repeat_with(|| Faker.fake::<Data>()).take(1_000);
            for row in iter {
                file.write_all(format!("{}\n", row.to_positional_row()).as_bytes())
                    .unwrap();
            }
        })
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(30);
    targets = serialize_to_file
}
criterion_main!(benches);
