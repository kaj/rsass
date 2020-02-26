//! Tests auto-converted from "sass-spec/spec/values/maps"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/values/maps/duplicate-keys.hrx"

// Ignoring "duplicate_keys", error tests are not supported yet.

// From "sass-spec/spec/values/maps/errors.hrx"

// Ignoring "errors", error tests are not supported yet.

// From "sass-spec/spec/values/maps/invalid-key.hrx"

// Ignoring "invalid_key", error tests are not supported yet.

// From "sass-spec/spec/values/maps/length.hrx"
#[test]
fn length() {
    assert_eq!(
        rsass(
            "$map: (aaa: 100, bbb: 200, ccc: 300);\
            \n\
            \na {\
            \n  b: length($map);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/values/maps/map-values.hrx"
#[test]
fn map_values() {
    assert_eq!(
        rsass(
            "div {\
            \n  foo: map-values((foo: 1, bar: 2));\
            \n  foo: map-values((foo: 1, bar: 2, baz: 2));\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: 1, 2;\
        \n  foo: 1, 2, 2;\
        \n}\
        \n"
    );
}
