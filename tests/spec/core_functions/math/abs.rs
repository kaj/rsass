//! Tests auto-converted from "sass-spec/spec/core_functions/math/abs.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("abs")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: abs()}\n"),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n1 | a {b: abs()}\
         \n  |       ^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function abs($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: abs(1, 2)}\n\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: abs(1, 2)}\
         \n  |       ^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function abs($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: abs(c)}\n"),
            "Error: $number: c is not a number.\
         \n  ,\
         \n1 | a {b: abs(c)}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: abs($number: -3)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
mod negative {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn decimal() {
        assert_eq!(
            runner().ok("a {b: abs(-123.456)}\n"),
            "a {\
         \n  b: 123.456;\
         \n}\n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            runner().ok("a {b: abs(-17)}\n"),
            "a {\
         \n  b: 17;\
         \n}\n"
        );
    }
}
mod positive {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn decimal() {
        assert_eq!(
            runner().ok("a {b: abs(5.6)}\n"),
            "a {\
         \n  b: 5.6;\
         \n}\n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            runner().ok("a {b: abs(1)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
#[test]
fn preserves_units() {
    assert_eq!(
        runner().ok("a {b: abs(-7px / 4em) * 1em}\n"),
        "a {\
         \n  b: 1.75px;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("a {b: abs(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
