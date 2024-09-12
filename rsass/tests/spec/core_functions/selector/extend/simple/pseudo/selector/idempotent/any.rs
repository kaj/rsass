//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/any.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("any")
}

#[test]
#[ignore] // wrong result
fn any_in_extender() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":any(.c)\", \".c\", \":any(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :any(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":any(.c)\", \".c\", \".d, .e\")}\n"),
        "a {\
         \n  b: :any(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":any(.c)\", \".c\", \".d\")}\n"),
        "a {\
         \n  b: :any(.c, .d);\
         \n}\n"
    );
}
