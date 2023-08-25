//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/arg.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("arg")
}

mod class {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn equal() {
        assert_eq!(
            runner()
                .ok("a {b: is-superselector(\":c(@#$)\", \":c(@#$)\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod unequal {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn argument() {
            assert_eq!(
                runner().ok(
                    "a {b: is-superselector(\":c(@#$)\", \":c(*&^)\")}\n"
                ),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn has_argument() {
            assert_eq!(
                runner().ok("a {b: is-superselector(\":c(@#$)\", \":c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn name() {
            assert_eq!(
                runner().ok(
                    "a {b: is-superselector(\":c(@#$)\", \":d(@#$)\")}\n"
                ),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
}
mod element {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn equal() {
        assert_eq!(
            runner()
                .ok("a {b: is-superselector(\"::c(@#$)\", \"::c(@#$)\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    mod unequal {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn argument() {
            assert_eq!(
                runner().ok(
                    "a {b: is-superselector(\"::c(@#$)\", \"::c(*&^)\")}\n"
                ),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn has_argument() {
            assert_eq!(
                runner()
                    .ok("a {b: is-superselector(\"::c(@#$)\", \"::c\")}\n"),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn name() {
            assert_eq!(
                runner().ok(
                    "a {b: is-superselector(\"::c(@#$)\", \":d(@#$)\")}\n"
                ),
                "a {\
         \n  b: false;\
         \n}\n"
            );
        }
    }
}
