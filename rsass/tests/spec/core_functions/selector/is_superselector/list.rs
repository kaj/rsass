//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/list.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("list")
}

mod three {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn match_one() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c, d, e\", \"d\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn match_three() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c, d, e\", \"d, c, e\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn match_two() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c, d, e\", \"e, c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn miss_one() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c, d, e\", \"c, f\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
mod two {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn both_satisfied_by_one_superselector() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\".c\", \"d.c, e.c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod in_both {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn equal() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c, d\", \"c, d\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn subset() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c, d\", \"c.e, d.f\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn superset() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c.e, d.f\", \"c, d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
    #[test]
    fn in_sub() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"c, d\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn in_super() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c, d\", \"c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
