//! These are from the `output_styles/compressed/number-functions`
//! directory in the sass specification.
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

#[test]
fn max() {
    check(b"foo {\n  foo: max(1, 2, 3);\n  foo: max(3, 2px, 1px);\n  \
            foo: max(4em);\n  foo: max(10cm, 6in);\n}\n",
          "foo{foo:3;foo:3;foo:4em;foo:6in}\n")
}

#[test]
fn min() {
    check(b"foo {\n  foo: min(1, 2, 3);\n  foo: min(3px, 2px, 1);\n  \
            foo: min(4em);\n  foo: min(10cm, 6in);\n}\n",
          "foo{foo:1;foo:1;foo:4em;foo:10cm}\n")
}

#[test]
fn percentage() {
    check(b"foo {\n  foo: percentage(.5);\n  foo: percentage(1);\n  \
            foo: percentage(25px / 100px);\n  \
            foo: percentage($number: 0.5);\n}\n",
          "foo{foo:50%;foo:100%;foo:25%;foo:50%}\n")
}

#[test]
fn random() {
    check(b"foo {\n  $number: random();\n  \
            foo: $number >= 0 and $number <= 1;\n  $number: random(1.0);\n  \
            foo: $number >= 0 and $number <= 1;\n  foo: random(1) == 1;\n  \
            foo: type-of(random()) == number;\n  \
            foo: type-of(random(1)) == number;\n  \
            foo: type-of(random(1.0)) == number;\n}\n",
          "foo{foo:true;foo:true;foo:true;foo:true;foo:true;foo:true}\n")
}

#[test]
fn round() {
    check(b"foo {\n  foo: round(4.8);\n  foo: round(4.8px);\n  \
            foo: round(5.49px);\n  foo: round($number: 5.49px);\n}\n",
          "foo{foo:5;foo:5px;foo:5px;foo:5px}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Compressed)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
