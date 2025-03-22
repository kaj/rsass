//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/compound.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("compound")
}

#[test]
fn different_order() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c.e\", \"c:d.e\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod pseudo_element {
    use super::runner;

    mod absent {
        use super::runner;

        #[test]
        fn in_1() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"c::d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn in_2() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c::d\", \"c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
    mod class_syntax {
        use super::runner;

        #[test]
        fn after() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"c:after\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn before() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"c:before\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn first_letter() {
            assert_eq!(
                runner().ok(
                    "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"c:first-letter\")}\n"
                ),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn first_line() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"c:first-line\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn different_order() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":e::d\", \"::d:e\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn present() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::d\", \"c::d\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn same_order() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::d:e\", \"::d:e\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod subset {
        use super::runner;

        #[test]
        fn after() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::d:c\", \"::d:c:e\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn before() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\".c::d\", \".c.e::d\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    mod superset {
        use super::runner;

        #[test]
        fn after() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::d:c:e\", \"::d:c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn before() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\".c.e::d\", \".c::d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
}
#[test]
fn same_order() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"c.d\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn superset() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c.d\", \"c\")}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
