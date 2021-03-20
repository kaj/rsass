//! Tests auto-converted from "sass-spec/spec/core_functions/math/percentage.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "unit", error tests are not supported yet.
}
#[test]
fn integer() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(42)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 4200%;\
        \n}\
        \n"
    );
}
#[test]
fn large() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(123.456)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12345.6%;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage($number: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 100%;\
        \n}\
        \n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(-0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -40%;\
        \n}\
        \n"
    );
}
#[test]
fn small() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(0.246)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 24.6%;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0%;\
        \n}\
        \n"
    );
}
