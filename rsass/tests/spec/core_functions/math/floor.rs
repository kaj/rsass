//! Tests auto-converted from "sass-spec/spec/core_functions/math/floor.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("floor")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.floor()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.floor()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function floor($number) {\
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
             \na {b: math.floor(1, 2)}\n\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.floor(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function floor($number) {\
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
             \na {b: math.floor(c)}\n"
            ),
            "Error: $number: c is not a number.\
         \n  ,\
         \n2 | a {b: math.floor(c)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn high() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.floor(2.999999999999999)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn integer() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.floor(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn low() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.floor(6.1)}\n"),
        "a {\
         \n  b: 6;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.floor($number: 1.6)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.floor(-7.2)}\n"),
        "a {\
         \n  b: -8;\
         \n}\n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.floor(7px / 4em) * 1em}\n"),
        "a {\
         \n  b: 1px;\
         \n}\n"
    );
}
