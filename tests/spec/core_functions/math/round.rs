//! Tests auto-converted from "sass-spec/spec/core_functions/math/round.hrx"

mod down {
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: round(2.2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2;\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            crate::rsass(
                "a {b: round(-5.6)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -6;\
        \n}\
        \n"
        );
    }
    #[test]
    fn to_zero() {
        assert_eq!(
            crate::rsass(
                "a {b: round(0.2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    fn within_precision() {
        assert_eq!(
        crate::rsass(
            "// This is the largest number that\'s representable as a float and outside the\
            \n// precision range to be considered equal to 5.\
            \na {b: round(1.49999999999)}\
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
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: round()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n1 | a {b: round()}\
         \n  |       ^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function round($number) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: round(1, 2)}\
             \n\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: round(1, 2)}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function round($number) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: round(c)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: c is not a number.\
         \n  ,\
         \n1 | a {b: round(c)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn integer() {
    assert_eq!(
        crate::rsass(
            "a {b: round(1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: round($number: 1.6)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        crate::rsass(
            "a {b: round(7px / 4em) * 1em}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2px;\
        \n}\
        \n"
    );
}
mod up {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: round(2.9)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 3;\
        \n}\
        \n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            crate::rsass(
                "a {b: round(-5.4)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: -5;\
        \n}\
        \n"
        );
    }
    #[test]
    fn point_five() {
        assert_eq!(
            crate::rsass(
                "a {b: round(16.5)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 17;\
        \n}\
        \n"
        );
    }
    #[test]
    fn to_zero() {
        assert_eq!(
            crate::rsass(
                "a {b: round(-0.2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn within_precision() {
        assert_eq!(
        crate::rsass(
            "// This is the smallest number that\'s representable as a float and in the\
            \n// precision range to be considered equal to 5.\
            \na {b: round(0.4999999999900001)}\
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
