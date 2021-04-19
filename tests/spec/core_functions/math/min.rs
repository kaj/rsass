//! Tests auto-converted from "sass-spec/spec/core_functions/math/min.hrx"

mod error {
    #[test]
    fn incompatible_units() {
        assert_eq!(
            crate::rsass(
                "$arg: 1px;\
             \na {b: min($arg, 2s)}\
             \n"
            )
            .unwrap_err(),
            "Error: 1px and 2s have incompatible units.\
         \n  ,\
         \n2 | a {b: min($arg, 2s)}\
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
                "a {b: min()}\
             \n"
            )
            .unwrap_err(),
            "Error: At least one argument must be passed.\
         \n  ,\
         \n1 | a {b: min()}\
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
             \na {b: min($arg)}\
             \n"
                )
                .unwrap_err(),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: min($arg)}\
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
             \na {b: min(1, $arg)}\
             \n"
                )
                .unwrap_err(),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: min(1, $arg)}\
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
             \na {b: min(1, 2, $arg)}\
             \n"
                )
                .unwrap_err(),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: min(1, 2, $arg)}\
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
            \na {b: min($arg)}\
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
            \na {b: min(3, $arg, 2)}\
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
fn two_args() {
    assert_eq!(
        crate::rsass(
            "$arg: 1;\
            \na {b: min($arg, 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
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
            \na {b: min($arg, 1)}\
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
    fn compatible() {
        assert_eq!(
            crate::rsass(
                "$arg: 1px;\
            \na {b: min($arg, 1in, 1cm)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 1px;\
        \n}\
        \n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            crate::rsass(
                "$arg: 6px;\
            \na {b: min($arg, 2px, 10px)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 2px;\
        \n}\
        \n"
        );
    }
}
