//! Tests auto-converted from "sass-spec/spec/core_functions/global/math.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("math")
}

#[test]
fn abs() {
    assert_eq!(
        runner().ok("a {b: abs(-1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn ceil() {
    assert_eq!(
        runner().ok("a {b: ceil(0.5)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn comparable() {
    assert_eq!(
        runner().ok("a {b: comparable(1px, 1in)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn floor() {
    assert_eq!(
        runner().ok("a {b: floor(0.5)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("a {b: max(1, 2, 3)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: min(1, 2, 3)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn percentage() {
    assert_eq!(
        runner().ok("a {b: percentage(0.5)}\n"),
        "a {\
         \n  b: 50%;\
         \n}\n"
    );
}
#[test]
fn random() {
    assert_eq!(
        runner().ok("a {b: random(5) <= 5}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn round() {
    assert_eq!(
        runner().ok("a {b: round(0.5)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn unit() {
    assert_eq!(
        runner().ok("a {b: unit(5px)}\n"),
        "a {\
         \n  b: \"px\";\
         \n}\n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        runner().ok("a {b: unitless(1)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
