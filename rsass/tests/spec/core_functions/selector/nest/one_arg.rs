//! Tests auto-converted from "sass-spec/spec/core_functions/selector/nest/one_arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_arg")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.nest(\"c\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
