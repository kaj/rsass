//! Tests auto-converted from "sass-spec/spec/core_functions/math/tan.hrx"

mod asymptote {
    #[test]
    fn radian() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(0.5rad * math.$pi)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
}
#[test]
fn deg() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(1deg)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.0174550649;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "unit", error tests are not supported yet.

    // Ignoring "zero_args", error tests are not supported yet.
}
#[test]
fn grad() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(1grad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.0157092553;\
        \n}\
        \n"
    );
}
#[test]
fn infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(1 / 0)}\
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
fn named_arg() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan($number: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1.5574077247;\
        \n}\
        \n"
    );
}
mod negative_asymptote {
    #[test]
    fn radian() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.tan(-0.5rad * math.$pi)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -Infinity;\
        \n}\
        \n"
        );
    }
}
#[test]
fn negative_infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(-1 / 0)}\
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
fn negative_zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(-0.0)}\
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
fn negative_zero_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(-0.000000000001)}\
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
fn rad() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(1rad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1.5574077247;\
        \n}\
        \n"
    );
}
#[test]
fn turn() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(1turn)}\
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
fn unitless() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1.5574077247;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.tan(0)}\
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
            \na {b: math.tan(0.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
