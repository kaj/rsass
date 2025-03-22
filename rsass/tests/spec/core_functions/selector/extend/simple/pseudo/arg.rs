//! Tests auto-converted from "sass-spec/spec/core_functions/selector/extend/simple/pseudo/arg.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("arg")
}

mod class {
    use super::runner;

    #[test]
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":c(@#$)\", \":c(@#$)\", \"e\")}\n"),
            "a {\
         \n  b: :c(@#$), e;\
         \n}\n"
        );
    }
    mod unequal {
        use super::runner;

        #[test]
        fn argument() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":c(@#$)\", \":c(*&^)\", \"e\")}\n"),
                "a {\
         \n  b: :c(@#$);\
         \n}\n"
            );
        }
        #[test]
        fn has_argument() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":c(@#$)\", \":c\", \"e\")}\n"),
                "a {\
         \n  b: :c(@#$);\
         \n}\n"
            );
        }
        #[test]
        fn name() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\":c(@#$)\", \":d(@#$)\", \"e\")}\n"),
                "a {\
         \n  b: :c(@#$);\
         \n}\n"
            );
        }
    }
}
mod element {
    use super::runner;

    #[test]
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"::c(@#$)\", \"::c(@#$)\", \"e\")}\n"),
            "a {\
         \n  b: ::c(@#$), e;\
         \n}\n"
        );
    }
    mod unequal {
        use super::runner;

        #[test]
        fn argument() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"::c(@#$)\", \"::c(*&^)\", \"e\")}\n"),
                "a {\
         \n  b: ::c(@#$);\
         \n}\n"
            );
        }
        #[test]
        fn has_argument() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"::c(@#$)\", \"::c\", \"e\")}\n"),
                "a {\
         \n  b: ::c(@#$);\
         \n}\n"
            );
        }
        #[test]
        fn name() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.extend(\"::c(@#$)\", \":d(@#$)\", \"e\")}\n"),
                "a {\
         \n  b: ::c(@#$);\
         \n}\n"
            );
        }
    }
}
