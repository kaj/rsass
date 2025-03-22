//! Tests auto-converted from "sass-spec/spec/core_functions/math/abs.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("abs")
}

mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.abs()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.abs()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function abs($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.abs(1, 2)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.abs(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function abs($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.abs(c)}\n"
            ),
            "Error: $number: c is not a number.\
         \n  ,\
         \n2 | a {b: math.abs(c)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na { b: math.abs($number: 3)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
mod negative {
    use super::runner;

    #[test]
    fn decimal() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.abs(-123.456)}\n"),
            "a {\
         \n  b: 123.456;\
         \n}\n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.abs(-17)}\n"),
            "a {\
         \n  b: 17;\
         \n}\n"
        );
    }
}
mod positive {
    use super::runner;

    #[test]
    fn decimal() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.abs(5.6)}\n"),
            "a {\
         \n  b: 5.6;\
         \n}\n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.abs(1)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
}
#[test]
fn preserves_units() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.abs(-7px / 4em) * 1em}\n"),
        "a {\
         \n  b: 1.75px;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.abs(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
