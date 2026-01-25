//! Tests auto-converted from "sass-spec/spec/css/functions/special_variable.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("special_variable")
}

#[test]
fn attr() {
    assert_eq!(
        runner().ok("a {b: rgb(attr(c))}\n"),
        "a {\
         \n  b: rgb(attr(c));\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn test_if() {
    assert_eq!(
        runner().ok("a {b: rgb(if(css(): c))}\n"),
        "a {\
         \n  b: rgb(if(css(): c));\
         \n}\n"
    );
}
#[test]
fn var() {
    assert_eq!(
        runner().ok("a {b: rgb(var(--c))}\n"),
        "a {\
         \n  b: rgb(var(--c));\
         \n}\n"
    );
}
