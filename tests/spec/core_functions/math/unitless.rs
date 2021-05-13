//! Tests auto-converted from "sass-spec/spec/core_functions/math/unitless.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn denominator() {
    assert_eq!(
        runner().ok("a {b: unitless(1/1px)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: unitless()}\n"),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n1 | a {b: unitless()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function is-unitless($number) {\
         \n  |           ==================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: unitless(1, 2)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: unitless(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function is-unitless($number) {\
         \n  |           ==================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: unitless(c)}\n"),
            "Error: $number: c is not a number.\
         \n  ,\
         \n1 | a {b: unitless(c)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: unitless($number: 100)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
#[test]
fn numerator() {
    assert_eq!(
        runner().ok("a {b: unitless(1px)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn numerator_and_denominator() {
    assert_eq!(
        runner().ok("a {b: unitless(1px/1em)}\n"),
        "a {\
         \n  b: false;\
         \n}\n"
    );
}
#[test]
fn unitless() {
    assert_eq!(
        runner().ok("a {b: unitless(1)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
