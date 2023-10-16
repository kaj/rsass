//! Tests auto-converted from "sass-spec/spec/values/calculation/round/two_arguments.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("two_arguments")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: RoUnD(117, 25)}\n"),
        "a {\
         \n  b: 125;\
         \n}\n"
    );
}
#[test]
fn equals() {
    assert_eq!(
        runner().ok("a {b: round(10px, 10px)}\n"),
        "a {\
         \n  b: 10px;\
         \n}\n"
    );
}
mod lower_multiple {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn number_is_bigger() {
        assert_eq!(
            runner().ok("a {b: round(13px, 10px)}\n"),
            "a {\
         \n  b: 10px;\
         \n}\n"
        );
    }
    #[test]
    fn number_is_negative() {
        assert_eq!(
            runner().ok("a {b: round(-18px, 10px)}\n"),
            "a {\
         \n  b: -20px;\
         \n}\n"
        );
    }
}
mod math {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn unknown_units() {
        assert_eq!(
            runner().ok("a {\
             \n  b: round(1px + 0%, 1px + 0%);\
             \n}\n"),
            "a {\
         \n  b: round(1px + 0%, 1px + 0%);\
         \n}\n"
        );
    }
}
#[test]
fn nan() {
    assert_eq!(
        runner().ok("a {b: round(NaN, NaN)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
mod negative_step {
    #[allow(unused)]
    use super::runner;

    mod lower_multiple {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn number_is_bigger() {
            assert_eq!(
                runner().ok("a {b: round(13px, -10px)}\n"),
                "a {\
         \n  b: 10px;\
         \n}\n"
            );
        }
        #[test]
        fn number_is_negative() {
            assert_eq!(
                runner().ok("a {b: round(-18px, -10px)}\n"),
                "a {\
         \n  b: -20px;\
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
                runner().ok("a {b: round(23px, -10px)}\n"),
                "a {\
         \n  b: 20px;\
         \n}\n"
            );
        }
        #[test]
        fn number_is_half() {
            assert_eq!(
                runner().ok("a {b: round(15px, -10px)}\n"),
                "a {\
         \n  b: 20px;\
         \n}\n"
            );
        }
        #[test]
        fn number_is_negative() {
            assert_eq!(
                runner().ok("a {b: round(-13px, -10px)}\n"),
                "a {\
         \n  b: -10px;\
         \n}\n"
            );
        }
        #[test]
        fn number_is_smaller() {
            assert_eq!(
                runner().ok("a {b: round(18px, -10px)}\n"),
                "a {\
         \n  b: 20px;\
         \n}\n"
            );
        }
    }
}
mod negative_zero {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn negative_infinity() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(-5, -infinity))}\n"),
            "a {\
         \n  b: calc(-infinity);\
         \n}\n"
        );
    }
    #[test]
    fn positive_infinity() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(-5, infinity))}\n"),
            "a {\
         \n  b: calc(-infinity);\
         \n}\n"
        );
    }
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function round($arg1, $arg2) {@return $arg1}\
             \na {b: round(1, 2)}\n"),
        "a {\
         \n  b: 1;\
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
             \na {b: math.div(1, round(5, -infinity))}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
    #[test]
    fn positive_infinity() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(5, infinity))}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
}
mod preserved {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn interpolation() {
        assert_eq!(
            runner().ok("a {\
             \n  e: round(#{\"5.5px, 1px\"});\
             \n}\n"),
            "a {\
         \n  e: round(5.5px, 1px);\
         \n}\n"
        );
    }
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {b: round(3.4px + 10%, 1px + 4px)}\n"),
        "a {\
         \n  b: round(3.4px + 10%, 5px);\
         \n}\n"
    );
}
#[test]
fn step_is_zero() {
    assert_eq!(
        runner().ok("a {b: round(5px, 0px)}\n"),
        "a {\
         \n  b: calc(NaN * 1px);\
         \n}\n"
    );
}
mod units {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("a {b: round(117cm, 25mm)}\n"),
            "a {\
         \n  b: 117.5cm;\
         \n}\n"
        );
    }
    #[test]
    fn fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: round(1foo, 2bar);\
             \n}\n"),
            "a {\
         \n  b: round(1foo, 2bar);\
         \n}\n"
        );
    }
    #[test]
    fn none() {
        assert_eq!(
            runner().ok("a {b: round(117, 25)}\n"),
            "a {\
         \n  b: 125;\
         \n}\n"
        );
    }
    #[test]
    fn real_and_fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: round(1px, 2bar);\
             \n}\n"),
            "a {\
         \n  b: round(1px, 2bar);\
         \n}\n"
        );
    }
    #[test]
    fn real_and_unknown() {
        assert_eq!(
            runner().ok("a {b: round(1px, 10%)}\n"),
            "a {\
         \n  b: round(1px, 10%);\
         \n}\n"
        );
    }
    #[test]
    fn same_fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: round(1foo, 2foo);\
             \n}\n"),
            "a {\
         \n  b: 2foo;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {\
             \n  b: round(1%, 2%);\
             \n}\n"),
            "a {\
         \n  b: 2%;\
         \n}\n"
        );
    }
}
#[test]
fn unknown_variable() {
    assert_eq!(
        runner().ok("a {\
             \n  c: round(up, var(--c));\
             \n}\n"),
        "a {\
         \n  c: round(up, var(--c));\
         \n}\n"
    );
}
mod upper_multiple {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn number_is_bigger() {
        assert_eq!(
            runner().ok("a {b: round(23px, 10px)}\n"),
            "a {\
         \n  b: 20px;\
         \n}\n"
        );
    }
    #[test]
    fn number_is_half() {
        assert_eq!(
            runner().ok("a {b: round(15px, 10px)}\n"),
            "a {\
         \n  b: 20px;\
         \n}\n"
        );
    }
    #[test]
    fn number_is_negative() {
        assert_eq!(
            runner().ok("a {b: round(-13px, 10px)}\n"),
            "a {\
         \n  b: -10px;\
         \n}\n"
        );
    }
    #[test]
    fn number_is_smaller() {
        assert_eq!(
            runner().ok("a {b: round(18px, 10px)}\n"),
            "a {\
         \n  b: 20px;\
         \n}\n"
        );
    }
}
