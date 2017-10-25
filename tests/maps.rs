//! These are from the `values/maps` directory in the sass specification.
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
fn map_keys() {
    check("div {\n  foo: map-keys((foo: 1, bar: 2));\n}\n",
          "div {\n  foo: foo, bar;\n}\n")
}

#[test]
fn map_values() {
    check("div {\n  foo: map-values((foo: 1, bar: 2));\n  \
           foo: map-values((foo: 1, bar: 2, baz: 2));\n}\n",
          "div {\n  foo: 1, 2;\n  foo: 1, 2, 2;\n}\n")
}

#[test]
fn append() {
    check("$map1: (aaa: 100, bbb: 200, ccc: 300);\n\
           $map2: (ddd: 400, eee: 500, fff: 600);\n\n\
           a {\n  $new-list: append($map1, $map2);\n  b: length($new-list);\n  \
           b: type-of($new-list);\n  b: nth($new-list, 1);\n  \
           b: nth($new-list, 2);\n  b: nth($new-list, 3);\n  \
           b: nth(nth($new-list, 4), 1);\n  b: nth(nth($new-list, 4), 2);\n  \
           b: nth(nth($new-list, 4), 3);\n\n  \
           $new-list: append($map2, $map1);\n  c: length($new-list);\n  \
           c: type-of($new-list);\n  c: nth($new-list, 1);\n  \
           c: nth($new-list, 2);\n  c: nth($new-list, 3);\n  \
           c: nth(nth($new-list, 4), 1);\n  c: nth(nth($new-list, 4), 2);\n  \
           c: nth(nth($new-list, 4), 3);\n}\n",
          "a {\n  b: 4;\n  b: list;\n  b: aaa 100;\n  b: bbb 200;\n  \
           b: ccc 300;\n  b: ddd 400;\n  b: eee 500;\n  b: fff 600;\n  \
           c: 4;\n  c: list;\n  c: ddd 400;\n  c: eee 500;\n  c: fff 600;\n  \
           c: aaa 100;\n  c: bbb 200;\n  c: ccc 300;\n}\n")
}

#[test]
fn index() {
    check("$map: (aaa: 100, bbb: 200, ccc: 300);\n\n\
           a {\n  b: index($map, aaa 100);\n  b: index($map, bbb 200);\n  \
           b: index($map, ccc 300);\n\n  \
           c: index($map, (aaa 100));\n  c: index($map, (bbb 200));\n  \
           c: index($map, (ccc 300));\n\n  \
           d: index($map, (aaa, 100));\n  d: index($map, (bbb, 200));\n  \
           d: index($map, (ccc, 300));\n}\n",
          "a {\n  b: 1;\n  b: 2;\n  b: 3;\n  c: 1;\n  c: 2;\n  c: 3;\n}\n")
}

#[test]
fn join() {
    check("$map1: (aaa: 100, bbb: 200, ccc: 300);\n\
           $map2: (ddd: 400, eee: 500, fff: 600);\n\n\
           a {\n  $new-list: join($map1, $map2);\n  b: $new-list;\n  \
           b: length($new-list);\n  b: type-of($new-list);\n  \
           b: nth($new-list, 1);\n  b: nth($new-list, 2);\n  \
           b: nth($new-list, 3);\n  b: nth($new-list, 4);\n  \
           b: nth($new-list, 5);\n  b: nth($new-list, 6);\n\n\n  \
           $new-list: join($map2, $map1);\n  \
           c: $new-list;\n  c: length($new-list);\n  \
           c: type-of($new-list);\n  c: nth($new-list, 1);\n  \
           c: nth($new-list, 2);\n  c: nth($new-list, 3);\n  \
           c: nth($new-list, 4);\n  c: nth($new-list, 5);\n  \
           c: nth($new-list, 6);\n}\n",
          "a {\n  b: aaa 100, bbb 200, ccc 300, ddd 400, eee 500, fff 600;\n  \
           b: 6;\n  b: list;\n  b: aaa 100;\n  b: bbb 200;\n  b: ccc 300;\n  \
           b: ddd 400;\n  b: eee 500;\n  b: fff 600;\n  \
           c: ddd 400, eee 500, fff 600, aaa 100, bbb 200, ccc 300;\n  \
           c: 6;\n  c: list;\n  c: ddd 400;\n  c: eee 500;\n  c: fff 600;\n  \
           c: aaa 100;\n  c: bbb 200;\n  c: ccc 300;\n}\n")
}

#[test]
fn length() {
    check("$map: (aaa: 100, bbb: 200, ccc: 300);\n\n\
           a {\n  b: length($map);\n}\n",
          "a {\n  b: 3;\n}\n")
}

#[test]
fn nth() {
    check("$map: (aaa: 100, bbb: 200, ccc: 300);\n\n\
           a {\n  b: nth($map, 1);\n  b: length(nth($map, 1));\n  \
           b: type-of(nth($map, 1));\n  b: nth(nth($map, 1), 1);\n  \
           b: nth(nth($map, 1), 2);\n\n  \
           c: nth($map, 2);\n  c: length(nth($map, 2));\n  \
           c: type-of(nth($map, 2));\n  c: nth(nth($map, 2), 1);\n  \
           c: nth(nth($map, 2), 2);\n\n  \
           d: nth($map, 3);  d: length(nth($map, 3));\n  \
           d: type-of(nth($map, 3));\n  d: nth(nth($map, 3), 1);\n  \
           d: nth(nth($map, 3), 2);\n}\n",
          "a {\n  b: aaa 100;\n  b: 2;\n  b: list;\n  b: aaa;\n  b: 100;\n  \
           c: bbb 200;\n  c: 2;\n  c: list;\n  c: bbb;\n  c: 200;\n  \
           d: ccc 300;\n  d: 2;\n  d: list;\n  d: ccc;\n  d: 300;\n}\n")
}

#[test]
fn set_nth() {
    check("$map: (aaa: 100, bbb: 200, ccc: 300);\n\n\
           a {\n  b: set-nth($map, 2, ddd 220);\n  b: nth($map, 2);\n  \
           b: length(nth($map, 2));\n  b: type-of(nth($map, 2));\n  \
           b: nth(nth($map, 2), 1);\n  b: nth(nth($map, 2), 2);\n\n  \
           c: set-nth($map, 2, (ddd 240));\n  c: nth($map, 2);\n  \
           c: length(nth($map, 2));\n  c: type-of(nth($map, 2));\n  \
           c: nth(nth($map, 2), 1);\n  c: nth(nth($map, 2), 2);\n\n  \
           d: set-nth($map, 2, (ddd, 260));\n  d: nth($map, 2);\n  \
           d: length(nth($map, 2));\n  d: type-of(nth($map, 2));\n  \
           d: nth(nth($map, 2), 1);\n  d: nth(nth($map, 2), 2);\n}\n",
          "a {\n  b: aaa 100, ddd 220, ccc 300;\n  b: bbb 200;\n  b: 2;\n  \
           b: list;\n  b: bbb;\n  b: 200;\n  \
           c: aaa 100, ddd 240, ccc 300;\n  c: bbb 200;\n  c: 2;\n  \
           c: list;\n  c: bbb;\n  c: 200;\n  \
           d: aaa 100, ddd, 260, ccc 300;\n  d: bbb 200;\n  d: 2;\n  \
           d: list;\n  d: bbb;\n  d: 200;\n}\n")
}

#[test]
fn zip() {
    check("$map1: (aaa: 100, bbb: 200, ccc: 300);\n\
           $map2: (ddd: 400, eee: 500, fff: 600);\n\n\
           a {\n  $new-list: zip($map1, $map2);\n  b: $new-list;\n  \
           b: length($new-list);\n  b: type-of($new-list);\n  \
           b: nth($new-list, 1);\n  b: length(nth($new-list, 1));\n  \
           b: nth(nth($new-list, 1), 1);\n  b: nth(nth($new-list, 1), 2);\n  \
           b: nth($new-list, 2);\n  b: length(nth($new-list, 2));\n  \
           b: nth(nth($new-list, 2), 1);\n  b: nth(nth($new-list, 2), 2);\n  \
           b: nth($new-list, 3);\n  b: length(nth($new-list, 3));\n  \
           b: nth(nth($new-list, 3), 1);\n  b: nth(nth($new-list, 3), 2);\n\
           \n  \
           $new-list: zip($map2, $map1);\n  c: $new-list;\n  \
           c: length($new-list);\n  c: type-of($new-list);\n  \
           c: nth($new-list, 1);\n  c: length(nth($new-list, 1));\n  \
           c: nth(nth($new-list, 1), 1);\n  c: nth(nth($new-list, 1), 2);\n  \
           c: nth($new-list, 2);\n  c: length(nth($new-list, 2));\n  \
           c: nth(nth($new-list, 2), 1);\n  c: nth(nth($new-list, 2), 2);\n  \
           c: nth($new-list, 3);\n  c: length(nth($new-list, 3));\n  \
           c: nth(nth($new-list, 3), 1);\n  c: nth(nth($new-list, 3), 2);\n}\n",
          "a {\n  b: aaa 100 ddd 400, bbb 200 eee 500, ccc 300 fff 600;\n  \
           b: 3;\n  b: list;\n  b: aaa 100 ddd 400;\n  b: 2;\n  \
           b: aaa 100;\n  b: ddd 400;\n  b: bbb 200 eee 500;\n  b: 2;\n  \
           b: bbb 200;\n  b: eee 500;\n  b: ccc 300 fff 600;\n  b: 2;\n  \
           b: ccc 300;\n  b: fff 600;\n  \
           c: ddd 400 aaa 100, eee 500 bbb 200, fff 600 ccc 300;\n  \
           c: 3;\n  c: list;\n  c: ddd 400 aaa 100;\n  c: 2;\n  \
           c: ddd 400;\n  c: aaa 100;\n  c: eee 500 bbb 200;\n  c: 2;\n  \
           c: eee 500;\n  c: bbb 200;\n  c: fff 600 ccc 300;\n  c: 2;\n  \
           c: fff 600;\n  c: ccc 300;\n}\n")
}

fn check(input: &str, expected: &str) {
    assert_eq!(compile_scss(input.as_bytes(), OutputStyle::Normal)
                   .and_then(|s| Ok(String::from_utf8(s)?))
                   .unwrap(),
               expected);
}
