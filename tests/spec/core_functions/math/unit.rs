//! Tests auto-converted from "sass-spec/spec/core_functions/math/unit.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: unit()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n1 | a {b: unit()}\
         \n  |       ^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function unit($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: unit(1, 2)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: unit(1, 2)}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function unit($number) {\
         \n  |           ============= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: unit(c)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: c is not a number.\
         \n  ,\
         \n1 | a {b: unit(c)}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn multiple_denominators() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1 / 1px / 3em / 4rad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"(px*em*rad)^-1\";\
        \n}\
        \n"
    );
}
#[test]
fn multiple_numerators() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1px * 1em * 1rad)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"px*em*rad\";\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: unit($number: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"\";\
        \n}\
        \n"
    );
}
#[test]
fn none() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"\";\
        \n}\
        \n"
    );
}
mod numerator_and_denominator {
    #[test]
    fn multiple() {
        assert_eq!(
            crate::rsass(
                "a {b: unit(1px * 1em / 1rad / 1s)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"px*em/rad*s\";\
        \n}\
        \n"
        );
    }
    #[test]
    fn single() {
        assert_eq!(
            crate::rsass(
                "a {b: unit(1px / 1em)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: \"px/em\";\
        \n}\
        \n"
        );
    }
}
#[test]
fn one_denominator() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1/1px)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"px^-1\";\
        \n}\
        \n"
    );
}
#[test]
fn one_numerator() {
    assert_eq!(
        crate::rsass(
            "a {b: unit(1px)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"px\";\
        \n}\
        \n"
    );
}
