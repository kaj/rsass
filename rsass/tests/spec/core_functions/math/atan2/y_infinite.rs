//! Tests auto-converted from "sass-spec/spec/core_functions/math/atan2/y_infinite.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("y_infinite")
}

mod negative {
    use super::runner;

    mod with_x {
        use super::runner;

        #[test]
        fn finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(-1, 0), 1)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(-1, 0), math.div(1, 0))}\n"),
                "a {\
         \n  b: -45deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(-1, 0), -1)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(-1, 0), math.div(-1, 0))}\n"),
                "a {\
         \n  b: -135deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(-1, 0), -0.0)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(-1, 0), 0)}\n"),
                "a {\
         \n  b: -90deg;\
         \n}\n"
            );
        }
    }
}
mod positive {
    use super::runner;

    mod with_x {
        use super::runner;

        #[test]
        fn finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(1, 0), 1)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(1, 0), math.div(1, 0))}\n"),
                "a {\
         \n  b: 45deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_finite() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(1, 0), -1)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(1, 0), math.div(-1, 0))}\n"),
                "a {\
         \n  b: 135deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(1, 0), -0.0)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn negative_zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(1, 0), -0.000000000001)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(1, 0), 0)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
        #[test]
        fn zero_fuzzy() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.atan2(math.div(1, 0), 0.000000000001)}\n"),
                "a {\
         \n  b: 90deg;\
         \n}\n"
            );
        }
    }
}
