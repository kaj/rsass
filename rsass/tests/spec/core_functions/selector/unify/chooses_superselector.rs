//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/chooses_superselector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("chooses_superselector")
}

mod parent {
    use super::runner;

    #[test]
    fn selector1() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c d\", \"c.e .f\")}\n"),
            "a {\
         \n  b: c.e d.f;\
         \n}\n"
        );
    }
    #[test]
    fn selector2() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c.e .f\", \"c d\")}\n"),
            "a {\
         \n  b: c.e d.f;\
         \n}\n"
        );
    }
}
mod whole_selector {
    use super::runner;

    #[test]
    fn selector1() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", \"d c.e\")}\n"),
            "a {\
         \n  b: d c.e;\
         \n}\n"
        );
    }
    #[test]
    fn selector2() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"d c.e\", \"c\")}\n"),
            "a {\
         \n  b: d c.e;\
         \n}\n"
        );
    }
}
