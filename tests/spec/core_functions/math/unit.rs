//! Tests auto-converted from "sass-spec/spec/core_functions/math/unit.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn multiple_denominators() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1 / 1px / 3em / 4rad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"(px*em*rad)^-1\";\
        \n}\
        \n"
    );
}
#[test]
fn multiple_numerators() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1px * 1em * 1rad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"px*em*rad\";\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: unit($number: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"\";\
        \n}\
        \n"
    );
}
#[test]
fn none() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"\";\
        \n}\
        \n"
    );
}
mod numerator_and_denominator {
    #[test]
    fn multiple() {
        assert_eq!(
            crate::rsass(
                "a {b: unit(1px * 1em / 1rad / 1s)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"px*em/rad*s\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: unit(1px / 1em)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"px/em\";\
        \n}\
        \n"
        );
    }
}
#[test]
fn one_denominator() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1/1px)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"px^-1\";\
        \n}\
        \n"
    );
}
#[test]
fn one_numerator() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1px)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"px\";\
        \n}\
        \n"
    );
}
