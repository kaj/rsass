//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/no_arg.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_arg")
}

mod class {
    use super::runner;

    #[test]
    fn and_element() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":c\", \"::c\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":c\", \":c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn unequal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":c\", \":d\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
mod element {
    use super::runner;

    #[test]
    fn and_class() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::c\", \":c\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
    #[test]
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::c\", \"::c\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn unequal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::c\", \"::d\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
