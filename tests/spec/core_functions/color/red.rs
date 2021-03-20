//! Tests auto-converted from "sass-spec/spec/core_functions/color/red.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: red(rgb(255, 0, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 255;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: red(rgb(123, 0, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 123;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: red(rgb(0, 0, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: red($color: rgb(234, 0, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 234;\
        \n}\
        \n"
    );
}
