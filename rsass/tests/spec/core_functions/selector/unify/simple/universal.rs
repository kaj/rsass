//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/simple/universal.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("universal")
}

mod and_type {
    use super::runner;

    mod any {
        use super::runner;

        #[test]
        fn and_any() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|*\", \"*|c\")}\n"),
                "a {\
         \n  b: *|c;\
         \n}\n"
            );
        }
        #[test]
        fn and_default() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|*\", \"c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|*\", \"|c\")}\n"),
                "a {\
         \n  b: |c;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|*\", \"c|d\")}\n"),
                "a {\
         \n  b: c|d;\
         \n}\n"
            );
        }
    }
    mod default {
        use super::runner;

        #[test]
        fn and_any() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*\", \"*|c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn and_default() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*\", \"c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*\", \"|c\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*\", \"c|d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
    }
    mod empty {
        use super::runner;

        #[test]
        fn and_any() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"|*\", \"*|c\")}\n"),
                "a {\
         \n  b: |c;\
         \n}\n"
            );
        }
        #[test]
        fn and_default() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"|*\", \"c\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"|*\", \"|c\")}\n"),
                "a {\
         \n  b: |c;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"|*\", \"c|d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
    }
    mod explicit {
        use super::runner;

        #[test]
        fn and_any() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c|*\", \"*|d\")}\n"),
                "a {\
         \n  b: c|d;\
         \n}\n"
            );
        }
        #[test]
        fn and_default() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|*\", \"d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|*\", \"|d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        mod and_explicit {
            use super::runner;

            #[test]
            fn different() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|*\", \"d|e\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
            #[test]
            fn same() {
                assert_eq!(
                    runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c|*\", \"c|d\")}\n"),
                    "a {\
         \n  b: c|d;\
         \n}\n"
                );
            }
        }
    }
}
mod and_universal {
    use super::runner;

    mod any {
        use super::runner;

        #[test]
        fn and_any() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|*\", \"*|*\")}\n"),
                "a {\
         \n  b: *|*;\
         \n}\n"
            );
        }
        #[test]
        fn and_default() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|*\", \"*\")}\n"),
                "a {\
         \n  b: *;\
         \n}\n"
            );
        }
        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|*\", \"|*\")}\n"),
                "a {\
         \n  b: |*;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|*\", \"c|*\")}\n"),
                "a {\
         \n  b: c|*;\
         \n}\n"
            );
        }
    }
    mod default {
        use super::runner;

        #[test]
        fn and_any() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*\", \"*|*\")}\n"),
                "a {\
         \n  b: *;\
         \n}\n"
            );
        }
        #[test]
        fn and_default() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*\", \"*\")}\n"),
                "a {\
         \n  b: *;\
         \n}\n"
            );
        }
        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*\", \"|*\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*\", \"e|*\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
    }
    mod empty {
        use super::runner;

        #[test]
        fn and_any() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"|*\", \"*|*\")}\n"),
                "a {\
         \n  b: |*;\
         \n}\n"
            );
        }
        #[test]
        fn and_default() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"|*\", \"*\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"|*\", \"|*\")}\n"),
                "a {\
         \n  b: |*;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"|*\", \"e|*\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
    }
    mod explicit {
        use super::runner;

        #[test]
        fn and_any() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c|*\", \"*|*\")}\n"),
                "a {\
         \n  b: c|*;\
         \n}\n"
            );
        }
        #[test]
        fn and_default() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|*\", \"*\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn and_empty() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|*\", \"|*\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn and_explicit() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c|*\", \"c|*\")}\n"),
                "a {\
         \n  b: c|*;\
         \n}\n"
            );
        }
    }
}
