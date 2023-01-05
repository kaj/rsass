//! Tests auto-converted from "sass-spec/spec/core_functions/modules/math.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("math")
}

#[test]
fn abs() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.abs(-1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn ceil() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.ceil(0.5)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn compatible() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.compatible(1px, 1in)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comparable() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.comparable(1px, 1in)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: math.comparable(1px, 1in)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.unitless(1)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: math.unitless(1)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn floor() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.floor(0.5)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn is_unitless() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.is-unitless(1)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.max(1, 2, 3)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.min(1, 2, 3)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn percentage() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.percentage(0.5)}\n"),
        "a {\
         \n  b: 50%;\
         \n}\n"
    );
}
#[test]
fn random() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.random(5) <= 5}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn round() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.round(0.5)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn unit() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.unit(5px)}\n"),
        "a {\
         \n  b: \"px\";\
         \n}\n"
    );
}
