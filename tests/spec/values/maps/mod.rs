//! Tests auto-converted from "sass-spec/spec/values/maps"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/values/maps/duplicate-keys.hrx"

// Ignoring "duplicate_keys", error tests are not supported yet.

// From "sass-spec/spec/values/maps/errors.hrx"

// Ignoring "errors", error tests are not supported yet.

// From "sass-spec/spec/values/maps/invalid-key.hrx"

// Ignoring "invalid_key", error tests are not supported yet.

// From "sass-spec/spec/values/maps/key_equality.hrx"
mod key_equality {
    #[allow(unused)]
    use super::rsass;
    mod infinity {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn negative() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-get(((-1/0): b), -1/0))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: null;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn positive() {
            assert_eq!(
                rsass(
                    "a {b: inspect(map-get(((1/0): b), 1/0))}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: null;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn nan() {
        assert_eq!(
            rsass(
                "a {b: inspect(map-get(((0/0): b), 0/0))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: null;\
        \n}\
        \n"
        );
    }
}

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
