//! Tests auto-converted from "sass-spec/spec/core_functions/math/log.hrx"

mod base {
    #[test]
    fn between_zero_and_one() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 0.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -1;\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, -1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: NaN;\
        \n}\
        \n"
        );
    }
    #[test]
    fn null() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, null)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
        );
    }
    #[test]
    fn one() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
    #[test]
    fn one_fuzzy() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 1.000000000001)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.3010299957;\
        \n}\
        \n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 0)}\
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
    fn zero_fuzzy() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log(2, 0.000000000001)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
}
mod error {

    // Ignoring "base_has_units", error tests are not supported yet.

    // Ignoring "number_has_units", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "zero_args", error tests are not supported yet.
}
#[test]
fn infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(1 / 0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: Infinity;\
        \n}\
        \n"
    );
}
mod named_arg {
    #[test]
    fn number() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log($number: 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
        );
    }
}
mod named_args {
    #[test]
    fn number_with_base() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.log($number: 2, $base: 10)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.3010299957;\
        \n}\
        \n"
        );
    }
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(-1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaN;\
        \n}\
        \n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.6931471806;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -Infinity;\
        \n}\
        \n"
    );
}
#[test]
fn zero_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.log(0.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -Infinity;\
        \n}\
        \n"
    );
}
