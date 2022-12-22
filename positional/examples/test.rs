use positional::ToPositionalRow;

#[derive(ToPositionalRow)]
struct Data {
    #[field(size = 10)]
    name: String,

    #[field(size = 5, filler = '0', align = "r")]
    age: i32,
}

fn main() {
    let data = Data {
        name: "pippo".to_string(),
        age: 10,
    };

    println!("{}", data.to_positional_row());
}
