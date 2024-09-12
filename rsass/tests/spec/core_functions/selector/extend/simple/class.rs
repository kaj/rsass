//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/class.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("class")
}

#[test]
fn equal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c\", \".c\", \"e\")}\n"),
        "a {\
         \n  b: .c, e;\
         \n}\n"
    );
}
#[test]
fn unequal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c\", \".d\", \"e\")}\n"),
        "a {\
         \n  b: .c;\
         \n}\n"
    );
}
