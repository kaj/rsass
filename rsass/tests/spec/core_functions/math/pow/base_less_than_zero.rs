//! Tests auto-converted from "sass-spec/spec/core_functions/math/pow/base_less_than_zero.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("base_less_than_zero")
}

mod base {
    use super::runner;

    mod greater_than_negative_one {
        use super::runner;

        mod with_exponent {
            use super::runner;

            #[test]
            fn infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-0.5, math.div(1, 0))}\n"),
                    "a {\
         \n  b: 0;\
         \n}\n"
                );
            }
            #[test]
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-0.5, math.div(-1, 0))}\n"),
                    "a {\
         \n  b: calc(infinity);\
         \n}\n"
                );
            }
        }
    }
    mod less_than_negative_one {
        use super::runner;

        mod with_exponent {
            use super::runner;

            #[test]
            fn decimal() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-2, 0.5)}\n"),
                    "a {\
         \n  b: calc(NaN);\
         \n}\n"
                );
            }
            #[test]
            fn infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-2, math.div(1, 0))}\n"),
                    "a {\
         \n  b: calc(infinity);\
         \n}\n"
                );
            }
            #[test]
            fn integer() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-2, 2)}\n"),
                    "a {\
         \n  b: 4;\
         \n}\n"
                );
            }
            #[test]
            fn integer_fuzzy() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-2, 2.000000000001)}\n"),
                    "a {\
         \n  b: calc(NaN);\
         \n}\n"
                );
            }
            #[test]
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-2, math.div(-1, 0))}\n"),
                    "a {\
         \n  b: 0;\
         \n}\n"
                );
            }
            #[test]
            fn zero() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-2, 0)}\n"),
                    "a {\
         \n  b: 1;\
         \n}\n"
                );
            }
            #[test]
            fn zero_fuzzy() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-2, 0.000000000001)}\n"),
                    "a {\
         \n  b: calc(NaN);\
         \n}\n"
                );
            }
        }
    }
    mod negative_one {
        use super::runner;

        mod with_exponent {
            use super::runner;

            #[test]
            fn infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-1, math.div(1, 0))}\n"),
                    "a {\
         \n  b: 1;\
         \n}\n"
                );
            }
            #[test]
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-1, math.div(-1, 0))}\n"),
                    "a {\
         \n  b: 1;\
         \n}\n"
                );
            }
        }
    }
    mod negative_one_fuzzy {
        use super::runner;

        mod with_exponent {
            use super::runner;

            #[test]
            fn infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-1.000000000001, math.div(1, 0))}\n"),
                    "a {\
         \n  b: calc(infinity);\
         \n}\n"
                );
            }
            #[test]
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.pow(-1.000000000001, math.div(-1, 0))}\n"),
                    "a {\
         \n  b: 0;\
         \n}\n"
                );
            }
        }
    }
}
