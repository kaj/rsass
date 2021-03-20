//! Tests auto-converted from "sass-spec/spec/core_functions/math/abs.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: abs($number: -3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}
mod negative {
    #[test]
    fn decimal() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(-123.456)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 123.456;\
        \n}\
        \n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(-17)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 17;\
        \n}\
        \n"
        );
    }
}
mod positive {
    #[test]
    fn decimal() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(5.6)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 5.6;\
        \n}\
        \n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
}
#[test]
fn preserves_units() {
    assert_eq!(
        crate::rsass(
            "a {b: abs(-7px / 4em) * 1em}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1.75px;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "a {b: abs(0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
