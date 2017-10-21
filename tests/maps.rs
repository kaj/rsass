//! These are from the "values/maps" directory in the sass specification.
//! See https://github.com/sass/sass-spec for source material.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn map_get() {
    check("div {
  foo: map-get((foo: 1, bar: 2), foo);
  foo: map-get((foo: 1, bar: 2), bar);
  foo: map-get((foo: 1, bar: 2), baz);
  foo: map-get((), foo);
}
",
          "div {\n  foo: 1;\n  foo: 2;\n}\n")
}

fn check(input: &str, expected: &str) {
    assert_eq!(compile_scss(input.as_bytes(), OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
