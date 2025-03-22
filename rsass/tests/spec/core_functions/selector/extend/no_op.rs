//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/no_op.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_op")
}

mod conflict {
    use super::runner;

    mod element {
        use super::runner;

        #[test]
        fn alone() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c.d\", \".d\", \"e\")}\n"),
                "a {\
         \n  b: c.d;\
         \n}\n"
            );
        }
        #[test]
        fn with_class() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c.d\", \".d\", \"e.f\")}\n"),
                "a {\
         \n  b: c.d;\
         \n}\n"
            );
        }
    }
    #[test]
    fn id() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"#c.d\", \".d\", \"#e\")}\n"),
            "a {\
         \n  b: #c.d;\
         \n}\n"
        );
    }
    #[test]
    fn next_sibling() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c + .d\", \".d\", \"e + .f\")}\n"),
            "a {\
         \n  b: c + .d;\
         \n}\n"
        );
    }
    #[test]
    fn parent() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c > .d\", \".d\", \"e > .f\")}\n"),
            "a {\
         \n  b: c > .d;\
         \n}\n"
        );
    }
    mod pseudo_element {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn class_syntax() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":before.c\", \".c\", \":after\")}\n"),
                "a {\
         \n  b: :before.c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unknown() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"::c.d\", \".d\", \"::e\")}\n"),
                "a {\
         \n  b: ::c.d;\
         \n}\n"
            );
        }
    }
    mod universal {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn default_and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*.c\", \".c\", \"|*\")}\n"),
                "a {\
         \n  b: *.c;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn default_and_namespace() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*.c\", \".c\", \"d|*\")}\n"),
                "a {\
         \n  b: *.c;\
         \n}\n"
            );
        }
        #[test]
        fn empty_and_default() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|*.c\", \".c\", \"*\")}\n"),
                "a {\
         \n  b: |*.c;\
         \n}\n"
            );
        }
        #[test]
        fn empty_and_namespace() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|*.c\", \".c\", \"d|*\")}\n"),
                "a {\
         \n  b: |*.c;\
         \n}\n"
            );
        }
        #[test]
        fn namespace_and_default() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*.d\", \".d\", \"*\")}\n"),
                "a {\
         \n  b: c|*.d;\
         \n}\n"
            );
        }
        #[test]
        fn namespace_and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*.d\", \".d\", \"|*\")}\n"),
                "a {\
         \n  b: c|*.d;\
         \n}\n"
            );
        }
        #[test]
        fn namespace_and_namespace() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|*.d\", \".d\", \"e|*\")}\n"),
                "a {\
         \n  b: c|*.d;\
         \n}\n"
            );
        }
    }
}
#[test]
fn missing() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"d\", \"e\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod unification {
    use super::runner;

    mod additional {
        use super::runner;

        #[test]
        fn ancestor() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"c\", \"d c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn next_sibling() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"c\", \"d + c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn parent() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"c\", \"d > c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn sibling() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"c\", \"d ~ c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn simple() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"c\", \"c.d\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
    #[test]
    fn identical_to_extendee() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c.d\", \".d\", \".d\")}\n"),
            "a {\
         \n  b: c.d;\
         \n}\n"
        );
    }
    #[test]
    fn identical_to_selector() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c.d\", \".d\", \"c.d\")}\n"),
            "a {\
         \n  b: c.d;\
         \n}\n"
        );
    }
    mod specificity_modification {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn test_where() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {\
             \n  b: selector.extend(\":where(.x)\", \".x\", \".x .y\");\
             \n}\n"),
                "a {\
         \n  b: :where(.x, .x .y);\
         \n}\n"
            );
        }
    }
    mod subselector_of_target {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn is() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\".c:is(d)\", \":is(d)\", \"d.e\")}\n"),
                "a {\
         \n  b: .c:is(d);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn matches() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\".c:matches(d)\", \":matches(d)\", \"d.e\")}\n"
        ),
        "a {\
         \n  b: .c:matches(d);\
         \n}\n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn test_where() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.extend(\".c:where(d)\", \":where(d)\", \"d.e\")}\n"
        ),
        "a {\
         \n  b: .c:where(d);\
         \n}\n"
    );
        }
    }
}
