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

#[test]
fn floor() {
    check(b"foo {\n  foo: floor(4.8);\n  foo: floor(4.8px);\n  \
            foo: floor($number: 4.8px);\n}\n",
          "foo{foo:4;foo:4px;foo:4px}\n")
}

// TODO Test and implement varargs functions max and min.

#[test]
fn percentage() {
    check(b"foo {\n  foo: percentage(.5);\n  foo: percentage(1);\n  \
            foo: percentage(25px / 100px);\n  \
            foo: percentage($number: 0.5);\n}\n",
          "foo{foo:50%;foo:100%;foo:25%;foo:50%}\n")
}

// TODO Test and implement random() (more boolean logic needed for test)

#[test]
fn round() {
    check(b"foo {\n  foo: round(4.8);\n  foo: round(4.8px);\n  \
            foo: round(5.49px);\n  foo: round($number: 5.49px);\n}\n",
          "foo{foo:5;foo:5px;foo:5px;foo:5px}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Compressed).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
