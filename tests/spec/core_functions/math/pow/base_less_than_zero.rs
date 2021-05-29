//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow/base_less_than_zero.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod base {
    #[allow(unused)]
    use super::runner;

    mod greater_than_negative_one {
        #[allow(unused)]
        use super::runner;

        mod with_exponent {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-0.5, math.div(1, 0))}\n"),
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
             \na {b: math.pow(-0.5, math.div(-1, 0))}\n"),
                    "a {\
         \n  b: Infinity;\
         \n}\n"
                );
            }
        }
    }
    mod less_than_negative_one {
        #[allow(unused)]
        use super::runner;

        mod with_exponent {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn decimal() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-2, 0.5)}\n"),
                    "a {\
         \n  b: NaN;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-2, math.div(1, 0))}\n"),
                    "a {\
         \n  b: Infinity;\
         \n}\n"
                );
            }
            #[test]
            fn integer() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-2, 2)}\n"),
                    "a {\
         \n  b: 4;\
         \n}\n"
                );
            }
            #[test]
            fn integer_fuzzy() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-2, 2.000000000001)}\n"),
                    "a {\
         \n  b: 4;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-2, math.div(-1, 0))}\n"),
                    "a {\
         \n  b: 0;\
         \n}\n"
                );
            }
            #[test]
            fn zero() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-2, 0)}\n"),
                    "a {\
         \n  b: 1;\
         \n}\n"
                );
            }
            #[test]
            fn zero_fuzzy() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-2, 0.000000000001)}\n"),
                    "a {\
         \n  b: 1;\
         \n}\n"
                );
            }
        }
    }
    mod negative_one {
        #[allow(unused)]
        use super::runner;

        mod with_exponent {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-1, math.div(1, 0))}\n"),
                    "a {\
         \n  b: NaN;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-1, math.div(-1, 0))}\n"),
                    "a {\
         \n  b: NaN;\
         \n}\n"
                );
            }
        }
    }
    mod negative_one_fuzzy {
        #[allow(unused)]
        use super::runner;

        mod with_exponent {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-1.000000000001, math.div(1, 0))}\n"),
                    "a {\
         \n  b: NaN;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\" as math;\
             \na {b: math.pow(-1.000000000001, math.div(-1, 0))}\n"),
                    "a {\
         \n  b: NaN;\
         \n}\n"
                );
            }
        }
    }
}
