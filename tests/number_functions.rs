//! These are from the "output_styles/compressed/number-functions"
//! directory in the sass specification.
//! See https://github.com/sass/sass-spec for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn abs() {
    check(b"foo {\n  foo: abs(-5);\n  foo: abs(-5px);\n  foo: abs(5);\n  \
            foo: abs(5px);\n  foo: abs($number: 5px);\n}\n",
          "foo{foo:5;foo:5px;foo:5;foo:5px;foo:5px}\n")
}

#[test]
fn ceil() {
    check(b"foo {\n  foo: ceil(4.1);\n  foo: ceil(4.8px);\n  \
            foo: ceil($number: 4.8px);\n}\n",
          "foo{foo:5;foo:5px;foo:5px}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Compressed).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
