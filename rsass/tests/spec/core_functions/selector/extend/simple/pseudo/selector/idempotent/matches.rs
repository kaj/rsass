//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/matches.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("matches")
}

#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":matches(.c)\", \".c\", \".d, .e\")}\n"
        ),
        "a {\
         \n  b: :matches(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn matches_in_extender() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":matches(.c)\", \".c\", \":matches(.d, .e)\")}\n"
        ),
        "a {\
         \n  b: :matches(.c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":matches(.c)\", \".c\", \".d\")}\n"),
        "a {\
         \n  b: :matches(.c, .d);\
         \n}\n"
    );
}
