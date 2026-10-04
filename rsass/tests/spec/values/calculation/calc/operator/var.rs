//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/operator/var.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("var")
}

#[test]
fn calculation() {
    assert_eq!(
        runner().ok("a {b: calc(1 + calc(var(--c)))}\n"),
        "a {\
         \n  b: calc(1 + (var(--c)));\
         \n}\n"
    );
}
#[test]
fn directly_parenthesized() {
    assert_eq!(
        runner().ok("a {b: calc(1 + (var(--c)))}\n"),
        "a {\
         \n  b: calc(1 + (var(--c)));\
         \n}\n"
    );
}
#[test]
fn indirectly_parenthesized() {
    assert_eq!(
        runner().ok("a {b: calc((1 + var(--c)))}\n"),
        "a {\
         \n  b: calc(1 + var(--c));\
         \n}\n"
    );
}
