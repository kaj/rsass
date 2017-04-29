extern crate rsass;
use rsass::{FileContext, GlobalScope, OutputStyle, Scope, Value,
            parse_scss_data};

#[test]
fn simple_value() {
    let mut scope = GlobalScope::new();
    scope.define("color", &Value::black());
    let parsed = parse_scss_data(b"p { color: $color }").unwrap();
    let style = OutputStyle::Compressed;
    let file_context = FileContext::new();
    assert_eq!(style
                   .write_root(&parsed, &mut scope, file_context)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               "p{color:black}\n");
}
