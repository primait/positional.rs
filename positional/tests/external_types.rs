use chrono::NaiveDate;
use positional::{ToPositionalField, ToPositionalRow, Writer};

struct Date(NaiveDate);

#[derive(ToPositionalRow)]
struct Data {
    #[field(size = 10)]
    name: String,
    #[field(size = 20, filler = '-')]
    created: Date,
}

impl ToPositionalField for Date {
    fn to_positional_field(&self) -> String {
        self.0.format("%d/%m/%Y").to_string()
    }
}

impl Data {
    pub fn new(name: String, created: NaiveDate) -> Self {
        Self {
            name,
            created: Date(created),
        }
    }
}

#[test]
fn simple() {
    let rows = vec![Data::new(
        "the name".to_string(),
        NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
    )];
    let positional_file: Writer<Data> = Writer::new(rows);
    assert_eq!(
        "the name  01/01/2022----------",
        positional_file.to_string()
    );
}
