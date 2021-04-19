//! Tests auto-converted from "sass-spec/spec/core_functions/color/saturate.hrx"

mod error {
    mod one_arg {
        #[test]
        #[ignore] // missing error
        fn test_type() {
            assert_eq!(
                crate::rsass(
                    "a {b: saturate(red)}\
             \n"
                )
                .unwrap_err(),
                "Error: $amount: red is not a number.\
         \n  ,\
         \n1 | a {b: saturate(red)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n1 | a {b: saturate()}\
         \n  |       ^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function saturate($amount) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 1%, 2)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: saturate(plum, 1%, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function saturate($color, $amount) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    mod two_args {
        mod bounds {
            #[test]
            fn too_high() {
                assert_eq!(
        crate::rsass(
            "a {b: saturate(plum, 100.001)}\
             \n"
        ).unwrap_err(),
        "Error: $amount: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: saturate(plum, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
            }
            #[test]
            fn too_low() {
                assert_eq!(
                    crate::rsass(
                        "a {b: saturate(plum, -0.001)}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: saturate(plum, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
                );
            }
        }
        mod test_type {
            #[test]
            #[ignore] // missing error
            fn color() {
                assert_eq!(
                    crate::rsass(
                        "a {b: saturate(1, 2)}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: saturate(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
                );
            }
            #[test]
            fn lightness() {
                assert_eq!(
                    crate::rsass(
                        "a {b: saturate(plum, blue)}\
             \n"
                    )
                    .unwrap_err(),
                    "Error: $amount: blue is not a number.\
         \n  ,\
         \n1 | a {b: saturate(plum, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
                );
            }
        }
    }
}
mod one_arg {
    #[test]
    fn named() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate($amount: 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: saturate(50%);\
        \n}\
        \n"
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: saturate(50%);\
        \n}\
        \n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: saturate(1);\
        \n}\
        \n"
        );
    }
}
mod two_args {
    #[test]
    fn alpha() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(rgba(plum, 0.5), 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: rgba(255, 126, 255, 0.5);\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff7eff;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max_remaining() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 53%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #ff7eff;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #e697e6;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate(plum, 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: plum;\
        \n}\
        \n"
        );
    }
    #[test]
    fn named() {
        assert_eq!(
            crate::rsass(
                "a {b: saturate($color: plum, $amount: 14%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #e697e6;\
        \n}\
        \n"
        );
    }
}
