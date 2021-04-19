//! Tests auto-converted from "sass-spec/spec/core_functions/math/floor.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: floor()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n1 | a {b: floor()}\
         \n  |       ^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function floor($number) {\
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
                "a {b: floor(1, 2)}\
             \n\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: floor(1, 2)}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function floor($number) {\
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
                "a {b: floor(c)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: c is not a number.\
         \n  ,\
         \n1 | a {b: floor(c)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn high() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(2.999999999999999)}\
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
fn integer() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(1)}\
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
fn low() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(6.1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 6;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: floor($number: 1.6)}\
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
fn negative() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(-7.2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -8;\
        \n}\
        \n"
    );
}
#[test]
fn preserves_units() {
    assert_eq!(
        crate::rsass(
            "a {b: floor(7px / 4em) * 1em}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px;\
        \n}\
        \n"
    );
}
