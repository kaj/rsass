//! Tests auto-converted from "sass-spec/spec/core_functions/math/unitless.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unitless")
}

#[test]
fn denominator() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.is-unitless(1/1px)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.is-unitless()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.is-unitless()}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function is-unitless($number) {\
         \n  |           ==================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.is-unitless(1, 2)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.is-unitless(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function is-unitless($number) {\
         \n  |           ==================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.is-unitless(c)}\n"
            ),
            "Error: $number: c is not a number.\
         \n  ,\
         \n2 | a {b: math.is-unitless(c)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn wrong_name() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.unitless(1)}\n"
            ),
            "Error: Undefined function.\
         \n  ,\
         \n2 | a {b: math.unitless(1)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.is-unitless($number: 100)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn numerator() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.is-unitless(1px)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn numerator_and_denominator() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.is-unitless(1px/1em)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.is-unitless(1)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
