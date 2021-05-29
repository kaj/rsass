//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow/base_negative_infinity.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod with_exponent {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn decimal() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), 2.5)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn even_integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), 2)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn even_integer_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), 2.000000000001)}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn infinity() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), math.div(1, 0))}\n"),
            "a {\
         \n  b: Infinity;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative_decimal() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), -2.5)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative_even_integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), -2)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative_even_integer_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), -2.000000000001)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative_infinity() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), math.div(-1, 0))}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative_odd_integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), -3)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative_odd_integer_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), -3.000000000001)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn odd_integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), 3)}\n"),
            "a {\
         \n  b: -Infinity;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn odd_integer_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), 3.000000000001)}\n"),
            "a {\
         \n  b: -Infinity;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), 0)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn zero_fuzzy() {
        assert_eq!(
            runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(math.div(-1, 0), 0.000000000001)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
