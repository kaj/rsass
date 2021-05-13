//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan2/y_non_zero_finite.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(-1, 1)}\n"),
                "a {\
         \n  b: -45deg;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(-1, 1 / 0)}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(-1, -1)}\n"),
                "a {\
         \n  b: -135deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(-1, -1 / 0)}\n"),
                "a {\
         \n  b: -180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(-1, -0.0)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(-1, -0.000000000001)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(-1, 0)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(-1, 0.000000000001)}\n"),
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
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(1, 1)}\n"),
                "a {\
         \n  b: 45deg;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(1, 1 / 0)}\n"),
                "a {\
         \n  b: 0deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(1, -1)}\n"),
                "a {\
         \n  b: 135deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(1, -1 / 0)}\n"),
                "a {\
         \n  b: 180deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(1, -0.0)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(1, -0.000000000001)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(1, 0)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\" as math;\
             \na {b: math.atan2(1, 0.000000000001)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
    }
}
