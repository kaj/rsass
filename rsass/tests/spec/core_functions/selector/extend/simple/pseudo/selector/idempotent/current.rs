//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/current.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("current")
}

#[test]
#[ignore] // wrong result
fn current_in_extender() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":current(.c)\", \".c\", \":current(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :current(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":current(.c)\", \".c\", \".d, .e\")}\n"
        ),
        "a {\
         \n  b: :current(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner()
            .ok("a {b: selector-extend(\":current(.c)\", \".c\", \".d\")}\n"),
        "a {\
         \n  b: :current(.c, .d);\
         \n}\n"
    );
}
