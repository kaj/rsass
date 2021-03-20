//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan.hrx"

mod error {

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.

    // Ignoring "units", error tests are not supported yet.

    // Ignoring "zero_args", error tests are not supported yet.
}
#[test]
fn infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.atan(1 / 0)}\
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
fn negative() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.atan(-1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -45deg;\
        \n}\
        \n"
    );
}
#[test]
fn negative_infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.atan(-1 / 0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -90deg;\
        \n}\
        \n"
    );
}
#[test]
fn negative_zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.atan(-0.0)}\
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
            \na {b: math.atan(-0.000000000001)}\
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
fn positive() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.atan(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 45deg;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.atan(0)}\
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
            \na {b: math.atan(0.000000000001)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0deg;\
        \n}\
        \n"
    );
}
