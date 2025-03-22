//! Tests auto-converted from "sass-spec/spec/core_functions/math/ceil.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("ceil")
}

mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.ceil()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.ceil()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function ceil($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.ceil(1, 2)}\n\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.ceil(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function ceil($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.ceil(c)}\n"
            ),
            "Error: $number: c is not a number.\
         \n  ,\
         \n2 | a {b: math.ceil(c)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn high() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.ceil(2.9)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn integer() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.ceil(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn low() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.ceil(6.000000000000001)}\n"),
        "a {\
         \n  b: 7;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.ceil($number: 1.6)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.ceil(-7.6)}\n"),
        "a {\
         \n  b: -7;\
         \n}\n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.ceil(7px / 4em) * 1em}\n"),
        "a {\
         \n  b: 2px;\
         \n}\n"
    );
}
