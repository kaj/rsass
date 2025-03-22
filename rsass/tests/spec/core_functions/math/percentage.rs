//! Tests auto-converted from "sass-spec/spec/core_functions/math/percentage.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("percentage")
}

mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.percentage()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.percentage()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function percentage($number) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.percentage(1, 2)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.percentage(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function percentage($number) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.percentage(c)}\n"
            ),
            "Error: $number: c is not a number.\
         \n  ,\
         \n2 | a {b: math.percentage(c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.percentage(1%)}\n"
            ),
            "Error: $number: Expected 1% to have no units.\
         \n  ,\
         \n2 | a {b: math.percentage(1%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn integer() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.percentage(42)}\n"),
        "a {\
         \n  b: 4200%;\
         \n}\n"
    );
}
#[test]
fn large() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.percentage(123.456)}\n"),
        "a {\
         \n  b: 12345.6%;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.percentage($number: 1)}\n"),
        "a {\
         \n  b: 100%;\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.percentage(-0.4)}\n"),
        "a {\
         \n  b: -40%;\
         \n}\n"
    );
}
#[test]
fn small() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.percentage(0.246)}\n"),
        "a {\
         \n  b: 24.6%;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.percentage(0)}\n"),
        "a {\
         \n  b: 0%;\
         \n}\n"
    );
}
