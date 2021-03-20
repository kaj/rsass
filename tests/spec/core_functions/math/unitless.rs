//! Tests auto-converted from "sass-spec/spec/core_functions/math/unitless.hrx"

#[test]
fn denominator() {
    assert_eq!(
        crate::rsass(
            "a {b: unitless(1/1px)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: unitless($number: 100)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
#[test]
fn numerator() {
    assert_eq!(
        crate::rsass(
            "a {b: unitless(1px)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
#[test]
fn numerator_and_denominator() {
    assert_eq!(
        crate::rsass(
            "a {b: unitless(1px/1em)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        crate::rsass(
            "a {b: unitless(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
