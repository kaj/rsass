//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/is.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("is")
}

#[test]
#[ignore] // wrong result
fn is_in_extender() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":is(.c)\", \".c\", \":is(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :is(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":is(.c)\", \".c\", \".d, .e\")}\n"),
        "a {\
         \n  b: :is(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":is(.c)\", \".c\", \".d\")}\n"),
        "a {\
         \n  b: :is(.c, .d);\
         \n}\n"
    );
}
