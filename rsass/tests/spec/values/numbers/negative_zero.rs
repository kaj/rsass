//! Tests auto-converted from "sass-spec/spec/values/numbers/negative_zero.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("negative_zero")
}

#[test]
#[ignore] // unexepected error
fn denominator_unit() {
    assert_eq!(
        runner().ok("@use \'sass:math\';\
             \na {b: math.div(-0, 1px)}\n"),
        "a {\
         \n  b: calc(-0 / 1px);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn multiple_numerator_units() {
    assert_eq!(
        runner().ok("a {b: -0px * 1em}\n"),
        "a {\
         \n  b: calc(-0px * 1em);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn numerator_and_denominator_unit() {
    assert_eq!(
        runner().ok("@use \'sass:math\';\
             \na {b: math.div(-0px, 1em)}\n"),
        "a {\
         \n  b: calc(-0px / 1em);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn single_unit() {
    assert_eq!(
        runner().ok("a {b: -0px}\n"),
        "a {\
         \n  b: -0px;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn unitless() {
    assert_eq!(
        runner().ok("a {b: -0}\n"),
        "a {\
         \n  b: -0;\
         \n}\n"
    );
}
