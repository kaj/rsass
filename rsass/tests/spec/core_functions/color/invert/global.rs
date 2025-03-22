//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert/global.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("global")
}

#[test]
fn legacy() {
    assert_eq!(
        runner().ok("a {b: invert(red)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn modern() {
    assert_eq!(
        runner().ok("a {b: invert(color(srgb 0 0.4 1), $space: srgb)}\n"),
        "a {\
         \n  b: color(srgb 1 0.6 0);\
         \n}\n"
    );
}
#[test]
fn number() {
    assert_eq!(
        runner().ok("a {b: invert(10%)}\n"),
        "a {\
         \n  b: invert(10%);\
         \n}\n"
    );
}
#[test]
fn with_calc() {
    assert_eq!(
        runner().ok("a {b: invert(calc(1 + 2))}\n"),
        "a {\
         \n  b: invert(3);\
         \n}\n"
    );
}
#[test]
fn with_css_var() {
    assert_eq!(
        runner().ok("a {b: invert(var(--c))}\n"),
        "a {\
         \n  b: invert(var(--c));\
         \n}\n"
    );
}
#[test]
fn with_unquoted_calc() {
    assert_eq!(
        runner().ok("a {b: invert(unquote(\'calc(1)\'))}\n"),
        "a {\
         \n  b: invert(calc(1));\
         \n}\n"
    );
}
