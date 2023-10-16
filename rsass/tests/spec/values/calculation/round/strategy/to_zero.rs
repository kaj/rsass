//! Tests auto-converted from "sass-spec/spec/values/calculation/round/strategy/to-zero.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("to-zero")
}

mod strategy {
    #[allow(unused)]
    use super::runner;

    mod to_zero {
        #[allow(unused)]
        use super::runner;

        mod lower_multiple {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn number_is_bigger() {
                assert_eq!(
                    runner().ok("a {b: round(to-zero, 13px, 10px)}\n"),
                    "a {\
         \n  b: 10px;\
         \n}\n"
                );
            }
            #[test]
            fn number_is_negative() {
                assert_eq!(
                    runner().ok("a {b: round(to-zero, -18px, 10px)}\n"),
                    "a {\
         \n  b: -10px;\
         \n}\n"
                );
            }
        }
        #[test]
        fn negative() {
            assert_eq!(
                runner().ok("a {b: round(to-zero, -120px, -25px)}\n"),
                "a {\
         \n  b: -125px;\
         \n}\n"
            );
        }
        mod negative_zero {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(to-zero, -5, -infinity))}\n"),
                    "a {\
         \n  b: calc(-infinity);\
         \n}\n"
                );
            }
            #[test]
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(to-zero, -5, infinity))}\n"),
                    "a {\
         \n  b: calc(-infinity);\
         \n}\n"
                );
            }
        }
        #[test]
        fn positive() {
            assert_eq!(
                runner().ok("a {b: round(to-zero, 120px, 25px)}\n"),
                "a {\
         \n  b: 100px;\
         \n}\n"
            );
        }
        mod positive_zero {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn negative_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(to-zero, 5, -infinity))}\n"),
                    "a {\
         \n  b: calc(infinity);\
         \n}\n"
                );
            }
            #[test]
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(to-zero, 5, infinity))}\n"),
                    "a {\
         \n  b: calc(infinity);\
         \n}\n"
                );
            }
        }
        mod upper_multiple {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn number_is_bigger() {
                assert_eq!(
                    runner().ok("a {b: round(to-zero, 23px, 10px)}\n"),
                    "a {\
         \n  b: 20px;\
         \n}\n"
                );
            }
            #[test]
            fn number_is_half() {
                assert_eq!(
                    runner().ok("a {b: round(to-zero, 15px, 10px)}\n"),
                    "a {\
         \n  b: 10px;\
         \n}\n"
                );
            }
            #[test]
            fn number_is_negative() {
                assert_eq!(
                    runner().ok("a {b: round(to-zero, -13px, 10px)}\n"),
                    "a {\
         \n  b: -10px;\
         \n}\n"
                );
            }
            #[test]
            fn number_is_smaller() {
                assert_eq!(
                    runner().ok("a {b: round(to-zero, 18px, 10px)}\n"),
                    "a {\
         \n  b: 10px;\
         \n}\n"
                );
            }
        }
    }
}
