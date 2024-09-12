//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/complex/descendant.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("descendant")
}

mod and_child {
    #[allow(unused)]
    use super::runner;

    mod multiple {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"d c\", \"d > e > c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn neither() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"f c\", \"d > e > c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"e c\", \"d > e > c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    #[test]
    fn sub() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"d > c\", \"d c\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn test_super() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"d c\", \"d > c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod multiple {
    #[allow(unused)]
    use super::runner;

    mod extra_middle {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn child() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"a b c\", \"a x > b c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn descendant() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"a b c\", \"a x b c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn following_sibling() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"a b c\", \"a x ~ b c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn next_sibling() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"a b c\", \"a x + b c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    #[test]
    fn in_sub() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"d e c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn match_first() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"d c\", \"d e c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn match_neither() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"f c\", \"d e c\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn match_second() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"e c\", \"d e c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod single {
    #[allow(unused)]
    use super::runner;

    mod in_both {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn equal() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c d\", \"c d\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn subset() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c d\", \"c.e d.f\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn superset() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c.e d.f\", \"c d\")}\n"),
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
             \na {b: selector.is-superselector(\"c\", \"d c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn in_super() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c d\", \"d\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
