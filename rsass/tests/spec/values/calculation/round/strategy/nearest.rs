//! Tests auto-converted from "sass-spec/spec/values/calculation/round/strategy/nearest.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nearest")
}

mod infinity {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn negative() {
        assert_eq!(
            runner().ok("a {b: round(nearest, -infinity, -infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn negative_and_positive() {
        assert_eq!(
            runner().ok("a {b: round(nearest, -infinity, infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn positive_and_negative() {
        assert_eq!(
            runner().ok("a {b: round(nearest, infinity, -infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn positive_and_positive() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: round(nearest, infinity, infinity)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn infinity_and_negative() {
    assert_eq!(
        runner().ok("a {b: round(nearest, infinity, -5)}\n"),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn infinity_and_positive() {
    assert_eq!(
        runner().ok("a {b: round(nearest, infinity, 5)}\n"),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
}
mod lower_multiple {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn number_is_bigger() {
        assert_eq!(
            runner().ok("a {b: round(nearest, 13px, 10px)}\n"),
            "a {\
         \n  b: 10px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn number_is_negative() {
        assert_eq!(
            runner().ok("a {b: round(nearest, -18px, 10px)}\n"),
            "a {\
         \n  b: -20px;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn negative() {
    assert_eq!(
        runner().ok("a {b: round(nearest, -101, -25)}\n"),
        "a {\
         \n  b: -100;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn negative_and_infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(nearest, -10, infinity))}\n"),
        "a {\
         \n  b: calc(-infinity);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn negative_infinity_and_negative() {
    assert_eq!(
        runner().ok("a {b: round(nearest, -infinity, -5)}\n"),
        "a {\
         \n  b: calc(-infinity);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn negative_infinity_and_positive() {
    assert_eq!(
        runner().ok("a {b: round(nearest, -infinity, 5)}\n"),
        "a {\
         \n  b: calc(-infinity);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn number_is_multiple_of_step() {
    assert_eq!(
        runner().ok("a {b: round(nearest, 25px, 5px)}\n"),
        "a {\
         \n  b: 25px;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn positive() {
    assert_eq!(
        runner().ok("a {b: round(nearest, 117px, 25px)}\n"),
        "a {\
         \n  b: 125px;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn positive_and_infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(nearest, 10, infinity))}\n"),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn simplification() {
    assert_eq!(
        runner().ok(
            "a {\
             \n  b: round(nearest, 3.8px - 1px + var(--test), 1.1px + 4px)}\n"
        ),
        "a {\
         \n  b: round(nearest, 2.8px + var(--test), 5.1px);\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn step_is_multiple_of_number() {
    assert_eq!(
        runner().ok("a {b: round(nearest, 5px, 25px)}\n"),
        "a {\
         \n  b: 0px;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn step_is_zero() {
    assert_eq!(
        runner().ok("a {b: round(nearest, 10px, 0px)}\n"),
        "a {\
         \n  b: calc(NaN * 1px);\
         \n}\n"
    );
}
mod upper_multiple {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn number_is_bigger() {
        assert_eq!(
            runner().ok("a {b: round(nearest, 23px, 10px)}\n"),
            "a {\
         \n  b: 20px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn number_is_half() {
        assert_eq!(
            runner().ok("a {b: round(nearest, 15px, 10px)}\n"),
            "a {\
         \n  b: 20px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn number_is_negative() {
        assert_eq!(
            runner().ok("a {b: round(nearest, -13px, 10px)}\n"),
            "a {\
         \n  b: -10px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn number_is_smaller() {
        assert_eq!(
            runner().ok("a {b: round(nearest, 18px, 10px)}\n"),
            "a {\
         \n  b: 20px;\
         \n}\n"
        );
    }
}
