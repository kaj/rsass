//! Tests auto-converted from "sass-spec/spec/core_functions/color/lightness.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn fraction() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness(hsl(0, 100%, 0.5%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5%;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness(hsl(0, 100%, 100%))}\
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
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness(hsl(0, 100%, 50%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 50%;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness(hsl(0, 100%, 0%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0%;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness($color: hsl(0, 100%, 42%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 42%;\
        \n}\
        \n"
    );
}
