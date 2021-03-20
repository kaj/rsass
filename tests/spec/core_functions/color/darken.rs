//! Tests auto-converted from "sass-spec/spec/core_functions/color/darken.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(rgba(red, 0.2), 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 0, 0, 0.2);\
        \n}\
        \n"
    );
}
mod error {
    mod bounds {

        // Ignoring "too_high", error tests are not supported yet.

        // Ignoring "too_low", error tests are not supported yet.
    }

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "color", error tests are not supported yet.

        // Ignoring "lightness", error tests are not supported yet.
    }
}
#[test]
fn fraction() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 0.5%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #fc0000;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: black;\
        \n}\
        \n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 50%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: black;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 14%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #b80000;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 0%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: darken($color: red, $amount: 14%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #b80000;\
        \n}\
        \n"
    );
}
