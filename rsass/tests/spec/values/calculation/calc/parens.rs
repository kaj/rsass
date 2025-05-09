//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/parens.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("parens")
}

#[test]
fn calculation() {
    assert_eq!(
        runner().ok("a {b: calc((calc(1px + 1%)))}\n"),
        "a {\
         \n  b: calc(1px + 1%);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn double_preserved() {
    assert_eq!(
        runner().ok("a {b: calc(((var(--c))))}\n"),
        "a {\
         \n  b: calc(((var(--c))));\
         \n}\n"
    );
}
#[test]
fn identifier() {
    assert_eq!(
        runner().ok("a {b: calc((d))}\n"),
        "a {\
         \n  b: calc((d));\
         \n}\n"
    );
}
#[test]
fn interpolation() {
    assert_eq!(
        runner().ok("a {b: calc((#{\"1 + 2\"}))}\n"),
        "a {\
         \n  b: calc((1 + 2));\
         \n}\n"
    );
}
#[test]
fn number() {
    assert_eq!(
        runner().ok("a {b: calc((1px))}\n"),
        "a {\
         \n  b: 1px;\
         \n}\n"
    );
}
#[test]
fn operation() {
    assert_eq!(
        runner().ok("a {b: calc((1px + 1%))}\n"),
        "a {\
         \n  b: calc(1px + 1%);\
         \n}\n"
    );
}
mod var {
    use super::runner;

    #[test]
    fn direct() {
        assert_eq!(
            runner().ok("a {b: calc((var(--c)))}\n"),
            "a {\
         \n  b: calc((var(--c)));\
         \n}\n"
        );
    }
    #[test]
    fn variable() {
        assert_eq!(
            runner().ok("$c: var(--d);\
             \na {b: calc(($c))}\n"),
            "a {\
         \n  b: calc((var(--d)));\
         \n}\n"
        );
    }
}
#[test]
fn variable() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \n$c: string.unquote(\"1 + 2\");\
             \na {b: calc(($c))}\n"),
        "a {\
         \n  b: calc((1 + 2));\
         \n}\n"
    );
}
