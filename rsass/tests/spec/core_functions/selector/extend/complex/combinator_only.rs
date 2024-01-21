//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/complex/combinator_only.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("combinator_only")
}

#[test]
#[ignore] // wrong result
fn extender() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\".c\", \".c\", \">\")}\n"),
        "a {\
         \n  b: .c, >;\
         \n}\n"
    );
}
#[test]
fn selector() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\"+\", \".c\", \".d\")}\n"),
        "a {\
         \n  b: +;\
         \n}\n"
    );
}
