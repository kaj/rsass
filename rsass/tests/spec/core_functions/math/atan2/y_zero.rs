//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan2/y_zero.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("y_zero")
}

mod negative {
    #[allow(unused)]
    use super::runner;

    mod with_x {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.0, 1)}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.0, math.div(1, 0))}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.0, -1)}\n"),
                "a {\
         \n  b: -180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.0, math.div(-1, 0))}\n"),
                "a {\
         \n  b: -180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.0, -0.0)}\n"),
                "a {\
         \n  b: -180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.0, -0.000000000001)}\n"),
                "a {\
         \n  b: -180deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.0, 0)}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.0, 0.000000000001)}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
    }
}
mod negative_fuzzy {
    #[allow(unused)]
    use super::runner;

    mod with_x {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.000000000001, 1)}\n"),
                "a {\
         \n  b: -0.0000000001deg;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.000000000001, math.div(1, 0))}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.000000000001, -1)}\n"),
                "a {\
         \n  b: -179.9999999999deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.000000000001, math.div(-1, 0))}\n"),
                "a {\
         \n  b: -180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.000000000001, -0.0)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(-0.000000000001, 0)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
    }
}
mod positive {
    #[allow(unused)]
    use super::runner;

    mod with_x {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0, 1)}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0, math.div(1, 0))}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0, -1)}\n"),
                "a {\
         \n  b: 180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0, math.div(-1, 0))}\n"),
                "a {\
         \n  b: 180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0, -0.0)}\n"),
                "a {\
         \n  b: 180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0, -0.000000000001)}\n"),
                "a {\
         \n  b: 180deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0, 0)}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0, 0.000000000001)}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
    }
}
mod positive_fuzzy {
    #[allow(unused)]
    use super::runner;

    mod with_x {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0.000000000001, 1)}\n"),
                "a {\
         \n  b: 0.0000000001deg;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0.000000000001, math.div(1, 0))}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0.000000000001, -1)}\n"),
                "a {\
         \n  b: 179.9999999999deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0.000000000001, math.div(-1, 0))}\n"),
                "a {\
         \n  b: 180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0.000000000001, -0.0)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(0.000000000001, 0)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
    }
}
