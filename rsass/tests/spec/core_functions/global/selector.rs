//! Tests auto-converted from "sass-spec/spec/core_functions/global/selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("selector")
}

#[test]
fn append() {
    assert_eq!(
        runner().ok("a {b: selector-append(c, d)}\n"),
        "a {\
         \n  b: cd;\
         \n}\n"
    );
}
#[test]
fn extend() {
    assert_eq!(
        runner().ok("a {b: selector-extend(c, c, d)}\n"),
        "a {\
         \n  b: c, d;\
         \n}\n"
    );
}
#[test]
fn is_superselector() {
    assert_eq!(
        runner().ok("a {b: is-superselector(c, d)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn nest() {
    assert_eq!(
        runner().ok("a {b: selector-nest(c, d)}\n"),
        "a {\
         \n  b: c d;\
         \n}\n"
    );
}
#[test]
fn parse() {
    assert_eq!(
        runner().ok("a {b: selector-parse(\".c, .d\")}\n"),
        "a {\
         \n  b: .c, .d;\
         \n}\n"
    );
}
#[test]
fn replace() {
    assert_eq!(
        runner().ok("a {b: selector-replace(c, c, d)}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
#[test]
fn simple_selectors() {
    assert_eq!(
        runner().ok("a {b: simple-selectors(\".c.d\")}\n"),
        "a {\
         \n  b: .c, .d;\
         \n}\n"
    );
}
#[test]
fn unify() {
    assert_eq!(
        runner().ok("a {b: selector-unify(\".c\", \".d\")}\n"),
        "a {\
         \n  b: .c.d;\
         \n}\n"
    );
}
