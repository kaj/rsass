//! Tests auto-converted from "sass-spec/spec/values/calculation/round/strategy/down.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("down")
}

#[test]
fn infinity() {
    assert_eq!(
        runner().ok("a {b: round(down, infinity, infinity)}\n"),
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
            runner().ok("a {b: round(down, 13px, 10px)}\n"),
            "a {\
         \n  b: 10px;\
         \n}\n"
        );
    }
    #[test]
    fn number_is_negative() {
        assert_eq!(
            runner().ok("a {b: round(down, -18px, 10px)}\n"),
            "a {\
         \n  b: -20px;\
         \n}\n"
        );
    }
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: round(down, -101, -25)}\n"),
        "a {\
         \n  b: -125;\
         \n}\n"
    );
}
#[test]
fn negative_and_infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: round(down, -10, infinity)}\n"),
        "a {\
         \n  b: calc(-infinity);\
         \n}\n"
    );
}
#[test]
fn negative_step() {
    assert_eq!(
        runner().ok("a {b: round(down, 12, -7)}\n"),
        "a {\
         \n  b: 7;\
         \n}\n"
    );
}
mod negative_zero {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn positive_infinity() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(down, -1 * 0, infinity))}\n"),
            "a {\
         \n  b: calc(-infinity);\
         \n}\n"
        );
    }
}
#[test]
fn number_is_multiple_of_step() {
    assert_eq!(
        runner().ok("a {b: round(down, 25px, 5px)}\n"),
        "a {\
         \n  b: 25px;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("a {b: round(down, 122px, 25px)}\n"),
        "a {\
         \n  b: 100px;\
         \n}\n"
    );
}
#[test]
fn positive_and_infinity() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(down, 10, infinity))}\n"),
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
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(down, 1, infinity))}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(1, round(down, 0, infinity))}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
}
#[test]
fn step_is_multiple_of_number() {
    assert_eq!(
        runner().ok("a {b: round(down, 5px, 25px)}\n"),
        "a {\
         \n  b: 0px;\
         \n}\n"
    );
}
#[test]
fn step_is_zero() {
    assert_eq!(
        runner().ok("a {b: round(down, 10px, 0px)}\n"),
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
            runner().ok("a {b: round(down, 23px, 10px)}\n"),
            "a {\
         \n  b: 20px;\
         \n}\n"
        );
    }
    #[test]
    fn number_is_half() {
        assert_eq!(
            runner().ok("a {b: round(down, 15px, 10px)}\n"),
            "a {\
         \n  b: 10px;\
         \n}\n"
        );
    }
    #[test]
    fn number_is_negative() {
        assert_eq!(
            runner().ok("a {b: round(down, -13px, 10px)}\n"),
            "a {\
         \n  b: -20px;\
         \n}\n"
        );
    }
    #[test]
    fn number_is_smaller() {
        assert_eq!(
            runner().ok("a {b: round(down, 18px, 10px)}\n"),
            "a {\
         \n  b: 10px;\
         \n}\n"
        );
    }
}
