//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/operator/times.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("times")
}

#[test]
fn no_whitespace() {
    assert_eq!(
        runner().ok("a {b: calc(1px*2)}\n"),
        "a {\
         \n  b: 2px;\
         \n}\n"
    );
}
#[test]
fn preserved() {
    assert_eq!(
        runner().ok("a {b: calc(1px * var(--c))}\n"),
        "a {\
         \n  b: calc(1px * var(--c));\
         \n}\n"
    );
}
#[test]
fn simplified() {
    assert_eq!(
        runner().ok("a {b: calc(1px * 2)}\n"),
        "a {\
         \n  b: 2px;\
         \n}\n"
    );
}
