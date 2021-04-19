//! Tests auto-converted from "sass-spec/spec/core_functions/math/comparable.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number2.\
         \n  ,--> input.scss\
         \n1 | a {b: comparable(1)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function compatible($number1, $number2) {\
         \n  |           ============================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1, 2, 3)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: comparable(1, 2, 3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function compatible($number1, $number2) {\
         \n  |           ============================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "a {b: comparable(c, 1)}\
             \n"
                )
                .unwrap_err(),
                "Error: $number1: c is not a number.\
         \n  ,\
         \n1 | a {b: comparable(c, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "a {b: comparable(1, c)}\
             \n"
                )
                .unwrap_err(),
                "Error: $number2: c is not a number.\
         \n  ,\
         \n1 | a {b: comparable(1, c)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: comparable($number1: 1, $number2: 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}
mod unit {
    #[test]
    fn to_compatible() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1px, 2in)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    #[test]
    fn to_different() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1px, 2em)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn to_inverse() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1px, 1/1px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: false;\
        \n}\
        \n"
        );
    }
    #[test]
    fn to_same() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1px, 2px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
}
mod unitless {
    #[test]
    fn to_unit() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1, 2px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
    #[test]
    fn to_unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: comparable(1, 2)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: true;\
        \n}\
        \n"
        );
    }
}
