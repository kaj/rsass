//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/selector/idempotent/nth_child.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nth_child")
}

#[test]
fn different_arg_in_extender() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \n// This should produce `:nth-child(2n + 1 of .c, :nth-child(2n + 1 of .d, .e))`.\
             \n// See sass/sass#2828.\
             \na {\
             \n  b: selector.extend(\
             \n      \":nth-child(2n + 1 of .c)\",\
             \n      \".c\",\
             \n      \":nth-child(2n + 2 of .d, .e)\");\
             \n}\n"
        ),
        "a {\
         \n  b: :nth-child(2n+1 of .c);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn list() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":nth-child(2n + 1 of .c)\", \".c\", \".d, .e\")}\n"
        ),
        "a {\
         \n  b: :nth-child(2n+1 of .c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn same_arg_in_extender() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {\
             \n  b: selector.extend(\
             \n      \":nth-child(2n + 1 of .c)\",\
             \n      \".c\",\
             \n      \":nth-child(2n + 1 of .d, .e)\");\
             \n}\n"),
        "a {\
         \n  b: :nth-child(2n+1 of .c, .d, .e);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simple() {
    assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\":nth-child(2n + 1 of .c)\", \".c\", \".d\")}\n"
        ),
        "a {\
         \n  b: :nth-child(2n+1 of .c, .d);\
         \n}\n"
    );
}
