//! Tests auto-converted from "sass-spec/spec/core_functions/math/unit.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unit")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:math\";\
             \na {b: math.unit()}\n"
            ),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.unit()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function unit($number) {\
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
             \na {b: math.unit(1, 2)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.unit(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function unit($number) {\
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
             \na {b: math.unit(c)}\n"
            ),
            "Error: $number: c is not a number.\
         \n  ,\
         \n2 | a {b: math.unit(c)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn multiple_denominators() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.unit(1 / 1px / 3em / 4rad)}\n"),
        "a {\
         \n  b: \"(px*em*rad)^-1\";\
         \n}\n"
    );
}
#[test]
fn multiple_numerators() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.unit(1px * 1em * 1rad)}\n"),
        "a {\
         \n  b: \"px*em*rad\";\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.unit($number: 1)}\n"),
        "a {\
         \n  b: \"\";\
         \n}\n"
    );
}
#[test]
fn none() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.unit(1)}\n"),
        "a {\
         \n  b: \"\";\
         \n}\n"
    );
}
mod numerator_and_denominator {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn multiple() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.unit(1px * 1em / 1rad / 1s)}\n"),
            "a {\
         \n  b: \"px*em/rad*s\";\
         \n}\n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.unit(1px / 1em)}\n"),
            "a {\
         \n  b: \"px/em\";\
         \n}\n"
        );
    }
}
#[test]
fn one_denominator() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.unit(1/1px)}\n"),
        "a {\
         \n  b: \"px^-1\";\
         \n}\n"
    );
}
#[test]
fn one_numerator() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.unit(1px)}\n"),
        "a {\
         \n  b: \"px\";\
         \n}\n"
    );
}
