//! Tests auto-converted from "sass-spec/spec/core_functions/math/comparable.hrx"

mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.
    mod test_type {

        // Ignoring "arg_1", error tests are not supported yet.

        // Ignoring "arg_2", error tests are not supported yet.
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: comparable($number1: 1, $number2: 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
mod unit {
    #[test]
    fn to_compatible() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1px, 2in)}\
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
    fn to_different() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1px, 2em)}\
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
    fn to_inverse() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1px, 1/1px)}\
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
    fn to_same() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1px, 2px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
}
mod unitless {
    #[test]
    fn to_unit() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1, 2px)}\
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
    fn to_unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1, 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
}
