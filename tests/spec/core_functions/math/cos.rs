//! Tests auto-converted from "sass-spec/spec/core_functions/math/cos.hrx"

#[test]
fn deg() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1deg)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.9998476952;\
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
            \na {b: math.cos(1grad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.9998766325;\
        \n}\
        \n"
    );
}
#[test]
fn infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1 / 0)}\
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
            \na {b: math.cos($number: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
    );
}
#[test]
fn negative_infinity() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(-1 / 0)}\
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
fn rad() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1rad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
    );
}
#[test]
fn turn() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1turn)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.cos(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5403023059;\
        \n}\
        \n"
    );
}
