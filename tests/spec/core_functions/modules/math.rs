//! Tests auto-converted from "sass-spec/spec/core_functions/modules/math.hrx"

#[test]
fn abs() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.abs(-1)}\
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
fn ceil() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.ceil(0.5)}\
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
fn compatible() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.compatible(1px, 1in)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "comparable", error tests are not supported yet.

    // Ignoring "unitless", error tests are not supported yet.
}
#[test]
fn floor() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.floor(0.5)}\
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
fn is_unitless() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.is-unitless(1)}\
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
fn max() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.max(1, 2, 3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.min(1, 2, 3)}\
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
fn percentage() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.percentage(0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 50%;\
        \n}\
        \n"
    );
}
#[test]
fn random() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.random(5) <= 5}\
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
fn round() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.round(0.5)}\
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
fn unit() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\";\
            \na {b: math.unit(5px)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"px\";\
        \n}\
        \n"
    );
}
