//! Tests auto-converted from "sass-spec/spec/values/calculation/round/one_argument.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("one_argument")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: rOuNd(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod math {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn slash_as_division() {
        assert_eq!(
            runner().ok("b {\
             \n  a: 2px / round(1.5);\
             \n}\n"),
            "b {\
         \n  a: 1px;\
         \n}\n"
        );
    }
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: round(-5.6)}\n"),
        "a {\
         \n  b: -6;\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function round($arg) {@return $arg}\
             \na {b: round(1.1)}\n"),
        "a {\
         \n  b: 1.1;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("a {b: round(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod preserved {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn variable() {
        assert_eq!(
            runner().ok("a {\
             \n  b: round(var(--c))\
             \n}\n"),
            "a {\
         \n  b: round(var(--c));\
         \n}\n"
        );
    }
}
#[test]
fn preserves_single_unit() {
    assert_eq!(
        runner().ok("a {b: round(1 + 1px)}\n"),
        "a {\
         \n  b: 2px;\
         \n}\n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        runner().ok("a {b: round(-7px / 4em) * 1em}\n"),
        "a {\
         \n  b: -2px;\
         \n}\n"
    );
}
#[test]
fn sass_script() {
    assert_eq!(
        runner().ok("a {b: round($number: -3)}\n"),
        "a {\
         \n  b: -3;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("a {b: round(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
