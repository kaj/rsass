//! Tests auto-converted from "sass-spec/spec/core_functions/math/ceil.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn high() {
    assert_eq!(
        crate::rsass(
            "a {b: ceil(2.9)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}
#[test]
fn integer() {
    assert_eq!(
        crate::rsass(
            "a {b: ceil(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn low() {
    assert_eq!(
        crate::rsass(
            "a {b: ceil(6.000000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 7;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: ceil($number: 1.6)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "a {b: ceil(-7.6)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -7;\
        \n}\
        \n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        crate::rsass(
            "a {b: ceil(7px / 4em) * 1em}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2px;\
        \n}\
        \n"
    );
}
