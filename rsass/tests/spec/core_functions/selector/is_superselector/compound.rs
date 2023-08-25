//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/compound.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("compound")
}

#[test]
fn different_order() {
    assert_eq!(
        runner().ok("a {b: is-superselector(\"c.e\", \"c:d.e\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod pseudo_element {
    #[allow(unused)]
    use super::runner;

    mod absent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn in_1() {
            assert_eq!(
                runner().ok("a {b: is-superselector(\"c\", \"c::d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn in_2() {
            assert_eq!(
                runner().ok("a {b: is-superselector(\"c::d\", \"c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
    mod class_syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn after() {
            assert_eq!(
                runner().ok("a {b: is-superselector(\"c\", \"c:after\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn before() {
            assert_eq!(
                runner().ok("a {b: is-superselector(\"c\", \"c:before\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn first_letter() {
            assert_eq!(
                runner().ok(
                    "a {b: is-superselector(\"c\", \"c:first-letter\")}\n"
                ),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn first_line() {
            assert_eq!(
                runner()
                    .ok("a {b: is-superselector(\"c\", \"c:first-line\")}\n"),
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
            runner().ok("a {b: is-superselector(\":e::d\", \"::d:e\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn present() {
        assert_eq!(
            runner().ok("a {b: is-superselector(\"::d\", \"c::d\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn same_order() {
        assert_eq!(
            runner().ok("a {b: is-superselector(\"::d:e\", \"::d:e\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod subset {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn after() {
            assert_eq!(
                runner()
                    .ok("a {b: is-superselector(\"::d:c\", \"::d:c:e\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn before() {
            assert_eq!(
                runner()
                    .ok("a {b: is-superselector(\".c::d\", \".c.e::d\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
    mod superset {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn after() {
            assert_eq!(
                runner()
                    .ok("a {b: is-superselector(\"::d:c:e\", \"::d:c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn before() {
            assert_eq!(
                runner()
                    .ok("a {b: is-superselector(\".c.e::d\", \".c::d\")}\n"),
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
        runner().ok("a {b: is-superselector(\"c\", \"c.d\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn superset() {
    assert_eq!(
        runner().ok("a {b: is-superselector(\"c.d\", \"c\")}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
