//! Tests auto-converted from "sass-spec/spec/core_functions/selector/unify/simple/type/and_universal.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("and_universal")
}

mod any {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn and_any() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|c\", \"*|*\")}\n"),
            "a {\
         \n  b: *|c;\
         \n}\n"
        );
    }
    #[test]
    fn and_default() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|c\", \"*\")}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn and_empty() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|c\", \"|*\")}\n"),
            "a {\
         \n  b: |c;\
         \n}\n"
        );
    }
    #[test]
    fn and_explicit() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"*|c\", \"d|*\")}\n"),
            "a {\
         \n  b: d|c;\
         \n}\n"
        );
    }
}
mod default {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn and_any() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", \"*|*\")}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn and_default() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c\", \"*\")}\n"),
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
             \na {b: meta.inspect(selector.unify(\"c\", \"|*\"))}\n"),
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
             \na {b: meta.inspect(selector.unify(\"c\", \"e|*\"))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
}
mod empty {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn and_any() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"|c\", \"*|*\")}\n"),
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
             \na {b: meta.inspect(selector.unify(\"|c\", \"*\"))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
    #[test]
    fn and_empty() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"|c\", \"|*\")}\n"),
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
             \na {b: meta.inspect(selector.unify(\"|c\", \"e|*\"))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
}
mod explicit {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn and_any() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c|d\", \"*|*\")}\n"),
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
             \na {b: meta.inspect(selector.unify(\"c|d\", \"*\"))}\n"),
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
             \na {b: meta.inspect(selector.unify(\"c|d\", \"|*\"))}\n"),
            "a {\
         \n  b: null;\
         \n}\n"
        );
    }
    mod and_explicit {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn different() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@use \"sass:selector\";\
             \na {b: meta.inspect(selector.unify(\"c|d\", \"e|*\"))}\n"),
                "a {\
         \n  b: null;\
         \n}\n"
            );
        }
        #[test]
        fn same() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.unify(\"c|d\", \"c|*\")}\n"),
                "a {\
         \n  b: c|d;\
         \n}\n"
            );
        }
    }
}
