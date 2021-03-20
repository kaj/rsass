//! Tests auto-converted from "sass-spec/spec/core_functions/color/green.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: green(rgb(0, 255, 0))}\
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
            "a {b: green(rgb(0, 123, 0))}\
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
            "a {b: green(rgb(0, 0, 0))}\
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
            "a {b: green($color: rgb(0, 234, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 234;\
        \n}\
        \n"
    );
}
