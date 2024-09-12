//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/distinct.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("distinct")
}

#[test]
#[ignore] // wrong result
fn three_level() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c .d .e\", \".f .g .h\")}\n"),
        "a {\
         \n  b: .c .d .f .g .e.h, .f .g .c .d .e.h;\
         \n}\n"
    );
}
#[test]
fn two_level() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c .d\", \".e .f\")}\n"),
        "a {\
         \n  b: .c .e .d.f, .e .c .d.f;\
         \n}\n"
    );
}
