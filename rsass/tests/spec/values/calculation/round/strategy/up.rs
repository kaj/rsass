//! Tests auto-converted from "sass-spec/spec/values/calculation/round/strategy/up.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("up")
}

mod strategy {
    #[allow(unused)]
    use super::runner;

    mod up {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: round(up, infinity, infinity)}\n"),
                "a {\
         \n  b: calc(NaN);\
         \n}\n"
            );
        }
        mod lower_multiple {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn number_is_bigger() {
                assert_eq!(
                    runner().ok("a {b: round(up, 13px, 10px)}\n"),
                    "a {\
         \n  b: 20px;\
         \n}\n"
                );
            }
            #[test]
            fn number_is_negative() {
                assert_eq!(
                    runner().ok("a {b: round(up, -18px, 10px)}\n"),
                    "a {\
         \n  b: -10px;\
         \n}\n"
                );
            }
        }
        #[test]
        fn negative() {
            assert_eq!(
                runner().ok("a {b: round(up, -101, -25)}\n"),
                "a {\
         \n  b: -100;\
         \n}\n"
            );
        }
        #[test]
        fn negative_and_infinity() {
            assert_eq!(
                runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(up, -10, infinity))}\n"),
                "a {\
         \n  b: calc(-infinity);\
         \n}\n"
            );
        }
        #[test]
        fn negative_step() {
            assert_eq!(
                runner().ok("a {b: round(up, 12px, -7px)}\n"),
                "a {\
         \n  b: 14px;\
         \n}\n"
            );
        }
        mod negative_zero {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn positive_infinity() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(up, -1 * 0, infinity))}\n"),
                    "a {\
         \n  b: calc(-infinity);\
         \n}\n"
                );
            }
        }
        #[test]
        fn number_is_multiple_of_step() {
            assert_eq!(
                runner().ok("a {b: round(up, 25px, 5px)}\n"),
                "a {\
         \n  b: 25px;\
         \n}\n"
            );
        }
        #[test]
        fn positive() {
            assert_eq!(
                runner().ok("a {b: round(up, 101px, 25px)}\n"),
                "a {\
         \n  b: 125px;\
         \n}\n"
            );
        }
        #[test]
        fn positive_and_infinity() {
            assert_eq!(
                runner().ok("a {b: round(up, 10, infinity)}\n"),
                "a {\
         \n  b: calc(infinity);\
         \n}\n"
            );
        }
        mod positive_zero {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn one() {
                assert_eq!(
                    runner().ok("a {b: round(up, 1, infinity)}\n"),
                    "a {\
         \n  b: calc(infinity);\
         \n}\n"
                );
            }
            #[test]
            fn zero() {
                assert_eq!(
                    runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(up, 0, infinity))}\n"),
                    "a {\
         \n  b: calc(infinity);\
         \n}\n"
                );
            }
        }
        #[test]
        fn step_is_multiple_of_number() {
            assert_eq!(
                runner().ok("a {b: round(up, 5px, 25px)}\n"),
                "a {\
         \n  b: 25px;\
         \n}\n"
            );
        }
        #[test]
        fn step_is_zero() {
            assert_eq!(
                runner().ok("a {b: round(up, 10px, 0px)}\n"),
                "a {\
         \n  b: calc(NaN * 1px);\
         \n}\n"
            );
        }
        mod upper_multiple {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn number_is_bigger() {
                assert_eq!(
                    runner().ok("a {b: round(up, 23px, 10px)}\n"),
                    "a {\
         \n  b: 30px;\
         \n}\n"
                );
            }
            #[test]
            fn number_is_half() {
                assert_eq!(
                    runner().ok("a {b: round(up, 15px, 10px)}\n"),
                    "a {\
         \n  b: 20px;\
         \n}\n"
                );
            }
            #[test]
            fn number_is_negative() {
                assert_eq!(
                    runner().ok("a {b: round(up, -13px, 10px)}\n"),
                    "a {\
         \n  b: -10px;\
         \n}\n"
                );
            }
            #[test]
            fn number_is_smaller() {
                assert_eq!(
                    runner().ok("a {b: round(up, 18px, 10px)}\n"),
                    "a {\
         \n  b: 20px;\
         \n}\n"
                );
            }
        }
    }
}
