use super::{analyze, Model};
use syn::parse_quote;

#[test]
fn can_extract_structs() {
    let model = analyze(parse_quote!(
        #[derive(ToPositionalRow)]
        struct Data {
            #[field(size = 20)]
            name: String,
        }
    ));
    assert!(matches!(model, Model::Struct(_)));
}

//#[test]
fn can_extract_enums() {
    let model = analyze(parse_quote!(
        #[derive(ToPositionalRow)]
        enum EnumData {
            Row1(RowData1),
        }
    ));
    assert!(matches!(model, Model::Enum(_)));
}
