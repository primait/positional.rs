use chrono::{DateTime, Utc};
use criterion::{criterion_group, criterion_main, Criterion};
use fake::{Fake, Faker};
use positional::*;

#[derive(FromPositionalRow, ToPositionalRow, Debug)]
struct Data {
    #[field(size = 2)]
    code: String,
    #[field(size = 10)]
    name: String,
    #[field(size = 40, filler = '_', align = "right")]
    created: DateTime<Utc>,
    #[field(size = 10, filler = '0')]
    field: String,
}

impl Data {
    pub fn random() -> Self {
        let mut name = Faker.fake::<String>();
        name.truncate(10);
        let mut field = Faker.fake::<String>();
        field.truncate(10);
        Self {
            code: "00".to_string(),
            name,
            created: Faker.fake::<DateTime<Utc>>(),
            field,
        }
    }
}

#[derive(FromPositionalRow, ToPositionalRow, Debug)]
struct Data2 {
    #[field(size = 2)]
    code: String,
    #[field(size = 10)]
    name: String,
    #[field(size = 40, filler = '_', align = "right")]
    created: DateTime<Utc>,
    #[field(size = 10, filler = '0')]
    field: String,
}

impl Data2 {
    pub fn random() -> Self {
        let mut name = Faker.fake::<String>();
        name.truncate(10);
        let mut field = Faker.fake::<String>();
        field.truncate(10);
        Self {
            code: "01".to_string(),
            name,
            created: Faker.fake::<DateTime<Utc>>(),
            field,
        }
    }
}

#[derive(FromPositionalRow, ToPositionalRow)]
enum AllData {
    #[matcher(&row[0..2] == "00")]
    First(Data),
    #[matcher(&row[0..2] == "01")]
    Second(Data2),
}

fn deserialize_enum(c: &mut Criterion) {
    c.bench_function("deserialize 100.000 structs", |b| {
        b.iter(|| {
            for _ in 1..=100_000 {
                let data = if rand::random::<bool>() {
                    AllData::First(Data::random())
                } else {
                    AllData::Second(Data2::random())
                };
                let writer = Writer::new(vec![data]);
                let row = writer.to_string();
                let _data = AllData::parse(&row).unwrap();
            }
        })
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(60);
    targets = deserialize_enum
}
criterion_main!(benches);
