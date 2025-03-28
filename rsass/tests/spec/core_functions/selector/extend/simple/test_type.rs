//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/type.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("type")
}

#[test]
fn and_universal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"*\", \"d\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
fn equal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"c\", \"e\")}\n"),
        "a {\
         \n  b: c, e;\
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
             \na {b: selector.extend(\"|c\", \"|c\", \"d\")}\n"),
                "a {\
         \n  b: |c, d;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|c\", \"d|c\", \"e\")}\n"),
                "a {\
         \n  b: |c;\
         \n}\n"
            );
        }
        #[test]
        fn and_implicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|c\", \"c\", \"d\")}\n"),
                "a {\
         \n  b: |c;\
         \n}\n"
            );
        }
        #[test]
        fn and_universal() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"|c\", \"*|c\", \"d\")}\n"),
                "a {\
         \n  b: |c;\
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
             \na {b: selector.extend(\"c|d\", \"|d\", \"e\")}\n"),
                "a {\
         \n  b: c|d;\
         \n}\n"
            );
        }
        mod and_explicit {
            use super::runner;

            #[test]
            fn equal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|d\", \"c|d\", \"e\")}\n"),
                    "a {\
         \n  b: c|d, e;\
         \n}\n"
                );
            }
            #[test]
            fn unequal() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|d\", \"e|d\", \"e\")}\n"),
                    "a {\
         \n  b: c|d;\
         \n}\n"
                );
            }
        }
        #[test]
        fn and_implicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|d\", \"d\", \"e\")}\n"),
                "a {\
         \n  b: c|d;\
         \n}\n"
            );
        }
        #[test]
        fn and_universal() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c|d\", \"*|d\", \"e\")}\n"),
                "a {\
         \n  b: c|d;\
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
             \na {b: selector.extend(\"*|c\", \"|c\", \"d\")}\n"),
                "a {\
         \n  b: *|c;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|c\", \"d|c\", \"d\")}\n"),
                "a {\
         \n  b: *|c;\
         \n}\n"
            );
        }
        #[test]
        fn and_implicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|c\", \"c\", \"d\")}\n"),
                "a {\
         \n  b: *|c;\
         \n}\n"
            );
        }
        #[test]
        fn and_universal() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"*|c\", \"*|c\", \"d\")}\n"),
                "a {\
         \n  b: *|c, d;\
         \n}\n"
            );
        }
    }
}
#[test]
fn unequal() {
    assert_eq!(
        runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"c\", \"d\", \"e\")}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
