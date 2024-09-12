//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/complex/identical.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("identical")
}

mod three_level {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn inner() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\".s1-1 .d .s1-2\", \".s2-1 .d .s2-2\")}\n"
        ),
        "a {\
         \n  b: .s1-1 .s2-1 .d .s1-2.s2-2, .s2-1 .s1-1 .d .s1-2.s2-2;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn outer() {
        assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.unify(\".c .s1-1 .s1-2\", \".c .s2-1 .s2-2\")}\n"
        ),
        "a {\
         \n  b: .c .s1-1 .s2-1 .s1-2.s2-2, .c .s2-1 .s1-1 .s1-2.s2-2;\
         \n}\n"
    );
    }
}
#[test]
fn two_level() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c .s1\", \".c .s2\")}\n"),
        "a {\
         \n  b: .c .s1.s2;\
         \n}\n"
    );
}
