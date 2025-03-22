//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/invert.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("invert")
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
fn with_color() {
    assert_eq!(
        runner().ok("a {b: invert(#abcdef)}\n"),
        "a {\
         \n  b: #543210;\
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
fn with_number() {
    assert_eq!(
        runner().ok("a {b: invert(10%)}\n"),
        "a {\
         \n  b: invert(10%);\
         \n}\n"
    );
}
#[test]
fn with_unquoted_calc() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \na {b: invert(string.unquote(\'calc(1)\'))}\n"),
        "a {\
         \n  b: invert(calc(1));\
         \n}\n"
    );
}
