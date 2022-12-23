use chrono::{DateTime, Utc};
use criterion::{criterion_group, criterion_main, Criterion};
use fake::{Dummy, Fake, Faker};
use positional::*;
use std::str::FromStr;

#[derive(ToPositionalRow, FromPositionalRow, Dummy, Debug)]
struct Data {
    #[field(size = 30)]
    name: String,
    #[field(size = 40, filler = '_', align = "right")]
    created: DateTime<Utc>,
    #[field(size = 20, filler = '0')]
    test1: String,
    #[field(size = 20, filler = '0')]
    test2: String,
    #[field(size = 20, filler = '0')]
    test3: String,
    #[field(size = 20, filler = '0')]
    test4: String,
    #[field(size = 20, filler = '0')]
    test5: String,
    #[field(size = 20, filler = '0')]
    test6: String,
    #[field(size = 20, filler = '0')]
    test7: String,
    #[field(size = 20, filler = '0')]
    test8: String,
    #[field(size = 20, filler = '0')]
    test9: CustomType,
}

#[derive(Dummy, Debug)]
struct CustomType(String);

impl ToPositionalField for CustomType {
    fn to_positional_field(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for CustomType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

fn serialize_struct(c: &mut Criterion) {
    let iter = std::iter::repeat_with(|| Faker.fake::<Data>()).take(1_000);
    let positional_file = Writer::new(iter);
    c.bench_function("serialize 1.000 structs", |b| {
        b.iter(|| positional_file.to_string())
    });
}

fn deserialize_struct(c: &mut Criterion) {
    let rows_iter = std::iter::repeat_with(|| Faker.fake::<Data>());
    let rows: Vec<Data> = rows_iter.take(1000).collect();
    let positional_file = Writer::new(rows);
    let line = positional_file.to_string();
    c.bench_function("deserialize a file with 1.000 rows", |b| {
        b.iter(|| {
            let _reader: Reader<Data> = Reader::from_str(&line).unwrap();
        })
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = serialize_struct, deserialize_struct
}
criterion_main!(benches);
