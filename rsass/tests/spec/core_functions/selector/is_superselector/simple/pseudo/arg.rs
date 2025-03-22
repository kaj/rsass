//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/arg.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("arg")
}

mod class {
    use super::runner;

    #[test]
    fn equal() {
        assert_eq!(
            runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":c(@#$)\", \":c(@#$)\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod unequal {
        use super::runner;

        #[test]
        fn argument() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":c(@#$)\", \":c(*&^)\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn has_argument() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":c(@#$)\", \":c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn name() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\":c(@#$)\", \":d(@#$)\")}\n"),
                "a {\
         \n  b: false;\
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
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::c(@#$)\", \"::c(@#$)\")}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    mod unequal {
        use super::runner;

        #[test]
        fn argument() {
            assert_eq!(
        runner().ok(
            "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::c(@#$)\", \"::c(*&^)\")}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
        }
        #[test]
        fn has_argument() {
            assert_eq!(
                runner().ok("@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::c(@#$)\", \"::c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        fn name() {
            assert_eq!(
                runner().ok(
                    "@use \"sass:selector\";\
             \na {b: selector.is-superselector(\"::c(@#$)\", \":d(@#$)\")}\n"
                ),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
}
