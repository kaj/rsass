//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/simple/type/and_type.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("and_type")
}

mod any {
    #[allow(unused)]
    use super::runner;

    mod and_any {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*|c\", \"*|d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|c\", \"*|c\")}\n"),
                "a {\
         \n  b: *|c;\
         \n}\n"
            );
        }
    }
    mod and_default {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different_type() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*|c\", \"d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same_type() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|c\", \"c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
    mod and_empty {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different_type() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*|c\", \"|d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same_type() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|c\", \"|c\")}\n"),
                "a {\
         \n  b: |c;\
         \n}\n"
            );
        }
    }
    mod and_explicit {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different_type() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"*|c\", \"d|e\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same_type() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|c\", \"d|c\")}\n"),
                "a {\
         \n  b: d|c;\
         \n}\n"
            );
        }
    }
}
mod default {
    #[allow(unused)]
    use super::runner;

    mod and_any {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different_type() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c\", \"*|d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same_type() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", \"*|c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
    mod and_default {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c\", \"d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", \"c\")}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
    }
    #[test]
    fn and_empty() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c\", \"|c\"))}\n"),
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
             \na {b: meta.inspect(selector.unify(\"c\", \"d|c\"))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
}
mod empty {
    #[allow(unused)]
    use super::runner;

    mod and_any {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different_type() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"|c\", \"*|d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same_type() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"|c\", \"*|c\")}\n"),
                "a {\
         \n  b: |c;\
         \n}\n"
            );
        }
    }
    #[test]
    fn and_default() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"|c\", \"c\"))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
    mod and_empty {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"|c\", \"|d\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"|c\", \"|c\")}\n"),
                "a {\
         \n  b: |c;\
         \n}\n"
            );
        }
    }
    #[test]
    fn and_explicit() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"|c\", \"e|c\"))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
}
mod explicit {
    #[allow(unused)]
    use super::runner;

    mod and_any {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different_type() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|d\", \"*|e\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same_type() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c|d\", \"*|d\")}\n"),
                "a {\
         \n  b: c|d;\
         \n}\n"
            );
        }
    }
    #[test]
    fn and_default() {
        assert_eq!(
            runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|d\", \"d\"))}\n"),
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
             \na {b: meta.inspect(selector.unify(\"c|d\", \"|d\"))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
    mod and_explicit {
        #[allow(unused)]
        use super::runner;

        mod different {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn namespace() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|d\", \"e|d\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
            #[test]
            fn test_type() {
                assert_eq!(
                    runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|d\", \"c|e\"))}\n"),
                    "a {\
         \n  b: null;\
         \n}\n"
                );
            }
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c|d\", \"c|d\")}\n"),
                "a {\
         \n  b: c|d;\
         \n}\n"
            );
        }
    }
}
