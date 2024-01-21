//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/placeholder.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("placeholder")
}

#[test]
#[ignore] // wrong result
fn equal() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\"%c\", \"%c\", \"e\")}\n"),
        "a {\
         \n  b: %c, e;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn unequal() {
    assert_eq!(
        runner().ok("a {b: selector-extend(\"%c\", \"%d\", \"e\")}\n"),
        "a {\
         \n  b: %c;\
         \n}\n"
    );
}
