//! Tests auto-converted from "sass-spec/spec/core_functions/math/round.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("round")
}

mod down {
    use super::runner;

    #[test]
    fn low() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.round(2.2)}\n"),
            "a {\
         \n  b: 2;\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.round(-5.6)}\n"),
            "a {\
         \n  b: -6;\
         \n}\n"
        );
    }
    #[test]
    fn to_zero() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.round(0.2)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
    #[test]
    fn within_precision() {
        assert_eq!(
        runner().ok(
            "// This is the largest number that\'s representable as a float and outside the\
             \n// precision range to be considered equal to 1.5.\
             \n@use \"sass:math\";\
             \na {b: math.round(1.4999999999949998)}\n"
        ),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
    }
}
mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.round()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.round()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function round($number) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.round(1, 2)}\n\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.round(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function round($number) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.round(c)}\n"
            ),
            "Error: $number: c is not a number.\
         \n  ,\
         \n2 | a {b: math.round(c)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn integer() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.round(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.round($number: 1.6)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.round(7px / 4em) * 1em}\n"),
        "a {\
         \n  b: 2px;\
         \n}\n"
    );
}
mod up {
    use super::runner;

    #[test]
    fn high() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.round(2.9)}\n"),
            "a {\
         \n  b: 3;\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.round(-5.4)}\n"),
            "a {\
         \n  b: -5;\
         \n}\n"
        );
    }
    #[test]
    fn point_five() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.round(16.5)}\n"),
            "a {\
         \n  b: 17;\
         \n}\n"
        );
    }
    #[test]
    fn to_zero() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.round(-0.2)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
}
