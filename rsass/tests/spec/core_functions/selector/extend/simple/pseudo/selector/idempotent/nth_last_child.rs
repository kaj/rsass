//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/nth_last_child.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nth_last_child")
}

#[test]
#[ignore] // wrong result
fn different_arg_in_extender() {
    assert_eq!(
        runner().ok(
            "// This should produce\
             \n// `:nth-last-child(2n + 1 of .c, :nth-last-child(2n + 1 of .d, .e))`.\
             \n// See sass/sass#2828.\
             \na {\
             \n  b: selector-extend(\
             \n      \":nth-last-child(2n + 1 of .c)\",\
             \n      \".c\",\
             \n      \":nth-last-child(2n + 2 of .d, .e)\");\
             \n}\n"
        ),
        "a {\
         \n  b: :nth-last-child(2n+1 of .c);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":nth-last-child(2n + 1 of .c)\", \".c\", \".d, .e\")}\n"
        ),
        "a {\
         \n  b: :nth-last-child(2n+1 of .c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn same_arg_in_extender() {
    assert_eq!(
        runner().ok("a {\
             \n  b: selector-extend(\
             \n      \":nth-last-child(2n + 1 of .c)\",\
             \n      \".c\",\
             \n      \":nth-last-child(2n + 1 of .d, .e)\");\
             \n}\n"),
        "a {\
         \n  b: :nth-last-child(2n+1 of .c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner().ok(
            "a {b: selector-extend(\":nth-last-child(2n + 1 of .c)\", \".c\", \".d\")}\n"
        ),
        "a {\
         \n  b: :nth-last-child(2n+1 of .c, .d);\
         \n}\n"
    );
}
