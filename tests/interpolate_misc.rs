//! Tests from `spec/parser/interpolate`
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn t01_literal_06_escape_interpolation() {
    check(b"$input: literal;\n\
            .result {\n  output: \"[\\#{literal}]\";\n  \
            output: \"\\#{literal}\";\n  output: '\\#{literal}';\n  \
            output: \"['\\#{literal}']\";\n}\n",
          ".result {\n  output: \"[\\#{literal}]\";\n  \
           output: \"\\#{literal}\";\n  output: '\\#{literal}';\n  \
           output: \"['\\#{literal}']\";\n}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
