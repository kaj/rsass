//! Tests auto-converted from "sass-spec/spec/core_functions/color/saturation.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn fraction() {
    assert_eq!(
        crate::rsass(
            "a {b: saturation(hsl(0, 0.5%, 100%))}\
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
            "a {b: saturation(hsl(0, 100%, 100%))}\
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
            "a {b: saturation(hsl(0, 50%, 100%))}\
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
            "a {b: saturation(hsl(0, 0%, 100%))}\
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
            "a {b: saturation($color: hsl(0, 42%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 42%;\
        \n}\
        \n"
    );
}
