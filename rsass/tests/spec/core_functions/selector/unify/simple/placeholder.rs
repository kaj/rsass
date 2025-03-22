//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/simple/placeholder.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("placeholder")
}

#[test]
fn different() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"%c\", \"%d\")}\n"),
        "a {\
         \n  b: %c%d;\
         \n}\n"
    );
}
#[test]
fn same() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"%c\", \"%c\")}\n"),
        "a {\
         \n  b: %c;\
         \n}\n"
    );
}
