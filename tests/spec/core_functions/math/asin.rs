//! Tests auto-converted from "sass-spec/spec/core_functions/math/asin.hrx"

#[test]
fn decimal() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 30deg;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "units", error tests are not supported yet.

    // Ignoring "zero_args", error tests are not supported yet.
}
#[test]
fn greater_than_one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
    );
}
#[test]
fn less_than_negative_one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(-2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: NaNdeg;\
        \n}\
        \n"
    );
}
#[test]
fn negative_decimal() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(-0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -30deg;\
        \n}\
        \n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(-0.0)}\
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
fn negative_zero_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(-0.000000000001)}\
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
fn one() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 90deg;\
        \n}\
        \n"
    );
}
#[test]
fn one_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(1.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 90deg;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(0)}\
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
fn zero_fuzzy() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.asin(0.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0deg;\
        \n}\
        \n"
    );
}
