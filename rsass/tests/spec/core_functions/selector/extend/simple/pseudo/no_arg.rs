//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/no_arg.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("no_arg")
}

mod class {
    use super::runner;

    #[test]
    fn and_element() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":c\", \"::c\", \"e\")}\n"),
            "a {\
         \n  b: :c;\
         \n}\n"
        );
    }
    #[test]
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":c\", \":c\", \"e\")}\n"),
            "a {\
         \n  b: :c, e;\
         \n}\n"
        );
    }
    #[test]
    fn unequal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":c\", \":d\", \"e\")}\n"),
            "a {\
         \n  b: :c;\
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
             \na {b: selector.extend(\"::c\", \":c\", \"e\")}\n"),
            "a {\
         \n  b: ::c;\
         \n}\n"
        );
    }
    #[test]
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"::c\", \"::c\", \"e\")}\n"),
            "a {\
         \n  b: ::c, e;\
         \n}\n"
        );
    }
    #[test]
    fn unequal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"::c\", \"::d\", \"e\")}\n"),
            "a {\
         \n  b: ::c;\
         \n}\n"
        );
    }
}
