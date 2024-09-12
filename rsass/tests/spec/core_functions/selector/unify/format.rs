//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/format.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("format")
}

mod input {
    #[allow(unused)]
    use super::runner;

    mod non_string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn selector1() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify((c, d c), \".e\")}\n"),
                "a {\
         \n  b: c.e, d c.e;\
         \n}\n"
            );
        }
        #[test]
        fn selector2() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".e\", (c, d c))}\n"),
                "a {\
         \n  b: c.e, d c.e;\
         \n}\n"
            );
        }
    }
    #[test]
    fn two_lists() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\".c, .d\", \".e, .f\")}\n"),
            "a {\
         \n  b: .c.e, .c.f, .d.e, .d.f;\
         \n}\n"
        );
    }
}
#[test]
fn output() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \n$result: selector.unify(\"c d, e f\", \".g\");\
             \na {\
             \n  result: $result;\
             \n  structure: $result == (\"c\" \"d.g\", \"e\" \"f.g\");\
             \n}\n"),
        "a {\
         \n  result: c d.g, e f.g;\
         \n  structure: true;\
         \n}\n"
    );
}
