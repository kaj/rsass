//! Tests auto-converted from "sass-spec/spec/core_functions/math/max.hrx"

mod error {
    #[test]
    fn incompatible_units() {
        assert_eq!(
            crate::rsass(
                "$arg: 1px;\
             \na {b: max($arg, 2s)}\
             \n\
             \n"
            )
            .unwrap_err(),
            "Error: 1px and 2s have incompatible units.\
         \n  ,\
         \n2 | a {b: max($arg, 2s)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: max()}\
             \n"
            )
            .unwrap_err(),
            "Error: At least one argument must be passed.\
         \n  ,\
         \n1 | a {b: max()}\
         \n  |       ^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    mod test_type {
        #[test]
        fn arg_1() {
            assert_eq!(
                crate::rsass(
                    "$arg: c;\
             \na {b: max($arg)}\
             \n"
                )
                .unwrap_err(),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: max($arg)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                crate::rsass(
                    "$arg: c;\
             \na {b: max(1, $arg)}\
             \n"
                )
                .unwrap_err(),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: max(1, $arg)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                crate::rsass(
                    "$arg: c;\
             \na {b: max(1, 2, $arg)}\
             \n"
                )
                .unwrap_err(),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: max(1, 2, $arg)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
    }
}
#[test]
fn one_arg() {
    assert_eq!(
        crate::rsass(
            "$arg: 1;\
            \na {b: max($arg)}\
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
fn three_args() {
    assert_eq!(
        crate::rsass(
            "$arg: 1;\
            \na {b: max(3, $arg, 2)}\
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
fn two_args() {
    assert_eq!(
        crate::rsass(
            "$arg: 1;\
            \na {b: max($arg, 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
}
mod units {
    #[test]
    fn and_unitless() {
        assert_eq!(
            crate::rsass(
                "$arg: 2px;\
            \na {b: max($arg, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2px;\
        \n}\
        \n"
        );
    }
    #[test]
    fn compatible() {
        assert_eq!(
            crate::rsass(
                "$arg: 1px;\
            \na {b: max($arg, 1in, 1cm)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1in;\
        \n}\
        \n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            crate::rsass(
                "$arg: 6px;\
            \na {b: max($arg, 2px, 10px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 10px;\
        \n}\
        \n"
        );
    }
}
