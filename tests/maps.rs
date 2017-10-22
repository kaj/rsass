//! These are from the "values/maps" directory in the sass specification.
//! See https://github.com/sass/sass-spec for source material.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn map_get() {
    check("div {\n  foo: map-get((foo: 1, bar: 2), foo);\n  \
           foo: map-get((foo: 1, bar: 2), bar);\n  \
           foo: map-get((foo: 1, bar: 2), baz);\n  \
           foo: map-get((), foo);\n}\n",
          "div {\n  foo: 1;\n  foo: 2;\n}\n")
}

#[test]
fn map_merge() {
    check("$before-1: map-merge((foo: 1, bar: 2), (baz: 3));\n\
           $before-2: map-merge((), (foo: 1, bar: 2));\n\
           $before-3: map-merge((foo: 1, bar: 2), ());\n\
           $before-4: map-merge((foo: 1, bar: 2), (foo: 3));\n\n\
           $after-1: (foo: 1, bar: 2, baz: 3);\n\
           $after-2: (foo: 1, bar: 2);\n\
           $after-3: (foo: 1, bar: 2);\n\
           $after-4: (foo: 3, bar: 2);\n\n\
           div {\n  foo-1: $before-1 == $after-1;\n  \
           foo-2: $before-2 == $after-2;\n  foo-3: $before-3 == $after-3;\n  \
           foo-4: $before-4 == $after-4;\n}\n",
          "div {\n  foo-1: true;\n  foo-2: true;\n  \
           foo-3: true;\n  foo-4: true;\n}\n")
}

#[test]
fn map_remove() {
    check("$before: map-remove((foo: 1, bar: 2, baz: 3), bar);\n\
           $after: (foo: 1, baz: 3);\n\n\
           div {\n  foo: $before == $after;\n}\n",
          "div {\n  foo: true;\n}\n")
}

#[test]
fn length() {
    check("$map: (aaa: 100, bbb: 200, ccc: 300);\n\n\
           a {\n  b: length($map);\n}\n",
          "a {\n  b: 3;\n}\n")
}

fn check(input: &str, expected: &str) {
    assert_eq!(compile_scss(input.as_bytes(), OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
