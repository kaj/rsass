//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/type.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type")
}

#[test]
fn and_universal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"*\")}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn equal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"c\")}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod namespace {
    use super::runner;

    mod empty {
        use super::runner;

        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|c\", \"|c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|c\", \"d|c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn and_implicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|c\", \"c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn and_universal() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"|c\", \"*|c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
    mod explicit {
        use super::runner;

        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|d\", \"|d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        mod and_explicit {
            use super::runner;

            #[test]
            fn equal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|d\", \"c|d\")}\n"),
                    "a {\
         \n  b: true;\
         \n}\n"
                );
            }
            #[test]
            fn unequal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|d\", \"e|d\")}\n"),
                    "a {\
         \n  b: false;\
         \n}\n"
                );
            }
        }
        #[test]
        fn and_implicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|d\", \"d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn and_universal() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c|d\", \"*|d\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
    mod universal {
        use super::runner;

        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|c\", \"|c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|c\", \"d|c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn and_implicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|c\", \"c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
        #[test]
        fn and_universal() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"*|c\", \"*|c\")}\n"),
                "a {\
         \n  b: true;\
         \n}\n"
            );
        }
    }
}
#[test]
fn unequal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"c\", \"d\")}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
