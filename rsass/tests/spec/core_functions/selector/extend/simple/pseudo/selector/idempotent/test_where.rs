//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/where.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("where")
}

#[test]
#[ignore] // wrong result
fn is_in_extender() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":where(.c)\", \".c\", \":where(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :where(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":where(.c)\", \".c\", \".d, .e\")}\n"
        ),
        "a {\
         \n  b: :where(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner()
            .ok("a {b: selector-extend(\":where(.c)\", \".c\", \".d\")}\n"),
        "a {\
         \n  b: :where(.c, .d);\
         \n}\n"
    );
}
