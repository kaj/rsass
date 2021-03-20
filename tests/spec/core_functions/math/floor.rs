//! Tests auto-converted from "sass-spec/spec/core_functions/math/floor.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn high() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(2.999999999999999)}\
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
fn integer() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(1)}\
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
            "a {b: floor(6.1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 6;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: floor($number: 1.6)}\
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
fn negative() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(-7.2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -8;\
        \n}\
        \n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(7px / 4em) * 1em}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px;\
        \n}\
        \n"
    );
}
