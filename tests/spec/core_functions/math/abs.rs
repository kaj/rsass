//! Tests auto-converted from "sass-spec/spec/core_functions/math/abs.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: abs()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n1 | a {b: abs()}\
         \n  |       ^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function abs($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(1, 2)}\
             \n\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: abs(1, 2)}\
         \n  |       ^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function abs($number) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(c)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: c is not a number.\
         \n  ,\
         \n1 | a {b: abs(c)}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: abs($number: -3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}
mod negative {
    #[test]
    fn decimal() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(-123.456)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 123.456;\
        \n}\
        \n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(-17)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 17;\
        \n}\
        \n"
        );
    }
}
mod positive {
    #[test]
    fn decimal() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(5.6)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 5.6;\
        \n}\
        \n"
        );
    }
    #[test]
    fn integer() {
        assert_eq!(
            crate::rsass(
                "a {b: abs(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1;\
        \n}\
        \n"
        );
    }
}
#[test]
fn preserves_units() {
    assert_eq!(
        crate::rsass(
            "a {b: abs(-7px / 4em) * 1em}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1.75px;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "a {b: abs(0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
