//! Tests auto-converted from "sass-spec/spec/values/numbers/degenerate.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("degenerate")
}

mod infinity {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn denominator_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(1, 0px)}\n"),
            "a {\
         \n  b: calc(infinity / 1px);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn multiple_numerator_units() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(1px * 1em, 0)}\n"),
            "a {\
         \n  b: calc(infinity * 1px * 1em);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn numerator_and_denominator_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(1px, 0em)}\n"),
            "a {\
         \n  b: calc(infinity * 1px / 1em);\
         \n}\n"
        );
    }
    #[test]
    fn single_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(1px, 0)}\n"),
            "a {\
         \n  b: calc(infinity * 1px);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(1, 0)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
}
mod minus_infinity {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn denominator_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(-1, 0px)}\n"),
            "a {\
         \n  b: calc(-infinity / 1px);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn multiple_numerator_units() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(-1px * 1em, 0)}\n"),
            "a {\
         \n  b: calc(-infinity * 1px * 1em);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn numerator_and_denominator_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(-1px, 0em)}\n"),
            "a {\
         \n  b: calc(-infinity * 1px / 1em);\
         \n}\n"
        );
    }
    #[test]
    fn single_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(-1px, 0)}\n"),
            "a {\
         \n  b: calc(-infinity * 1px);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(-1, 0)}\n"),
            "a {\
         \n  b: calc(-infinity);\
         \n}\n"
        );
    }
}
mod nan {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn denominator_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(0, 0px)}\n"),
            "a {\
         \n  b: calc(NaN / 1px);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn multiple_numerator_units() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(0px * 0em, 0)}\n"),
            "a {\
         \n  b: calc(NaN * 1px * 1em);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn numerator_and_denominator_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(0px, 0em)}\n"),
            "a {\
         \n  b: calc(NaN * 1px / 1em);\
         \n}\n"
        );
    }
    #[test]
    fn single_unit() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(0px, 0)}\n"),
            "a {\
         \n  b: calc(NaN * 1px);\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("@use \'sass:math\';\
             \na {b: math.div(0, 0)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
}
