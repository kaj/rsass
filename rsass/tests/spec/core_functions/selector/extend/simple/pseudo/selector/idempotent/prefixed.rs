//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/prefixed.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("prefixed")
}

#[test]
#[ignore] // wrong result
fn different_prefix_in_extender() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":-ms-matches(.c)\", \".c\", \":-moz-matches(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :-ms-matches(.c);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":-ms-matches(.c)\", \".c\", \".d, .e\")}\n"
        ),
        "a {\
         \n  b: :-ms-matches(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn same_prefix_in_extender() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":-ms-matches(.c)\", \".c\", \":-ms-matches(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :-ms-matches(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":-ms-matches(.c)\", \".c\", \".d\")}\n"
        ),
        "a {\
         \n  b: :-ms-matches(.c, .d);\
         \n}\n"
    );
}
