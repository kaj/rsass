//! Tests auto-converted from "sass-spec/spec/core_functions/color/hue.hrx"

#[test]
fn above_max() {
    assert_eq!(
        crate::rsass(
            "a {b: hue(hsl(540, 100%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 180deg;\
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
fn fraction() {
    assert_eq!(
        crate::rsass(
            "a {b: hue(hsl(0.5, 100%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5deg;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: hue(hsl(359, 100%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 359deg;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: hue(hsl(123, 100%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 123deg;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: hue(hsl(0, 100%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0deg;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: hue($color: hsl(234, 100%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 234deg;\
        \n}\
        \n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "a {b: hue(hsl(-180, 100%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 180deg;\
        \n}\
        \n"
    );
}
