//! Tests auto-converted from "sass-spec/spec/core_functions/math/clamp.hrx"

#[test]
fn chooses_max() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.clamp(0, 2, 1)}\
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
fn chooses_min() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.clamp(1, 0, 2)}\
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
fn chooses_number() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.clamp(0, 1, 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
mod error {
    mod incompatible_units {
        #[test]
        fn all() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1deg, 1px, 1s)}\
             \n"
                )
                .unwrap_err(),
                "Error: $number: 1px and $min: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.clamp(1deg, 1px, 1s)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn min_and_max() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1deg, 1turn, 1px)}\
             \n"
                )
                .unwrap_err(),
                "Error: $max: 1px and $min: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.clamp(1deg, 1turn, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn min_and_number() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1deg, 1px, 1turn)}\
             \n"
                )
                .unwrap_err(),
                "Error: $number: 1px and $min: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.clamp(1deg, 1px, 1turn)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn number_and_max() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1turn, 1deg, 1px)}\
             \n"
                )
                .unwrap_err(),
                "Error: $max: 1px and $min: 1turn have incompatible units.\
         \n  ,\
         \n2 | a {b: math.clamp(1turn, 1deg, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
    }
    #[test]
    fn one_arg() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.clamp(0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n2 | a {b: math.clamp(0)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function clamp($min, $number, $max) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    mod some_unitless {
        #[test]
        fn max() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0px, 1px, 2)}\
             \n"
        ).unwrap_err(),
        "Error: $max: 2 and $min: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0px, 1px, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn min() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 1px, 2px)}\
             \n"
        ).unwrap_err(),
        "Error: $number: 1px and $min: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0, 1px, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn min_and_max() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 1px, 2)}\
             \n"
        ).unwrap_err(),
        "Error: $number: 1px and $min: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0, 1px, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn min_and_number() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 1, 2px)}\
             \n"
        ).unwrap_err(),
        "Error: $max: 2px and $min: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0, 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn number() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0px, 1, 2px)}\
             \n"
        ).unwrap_err(),
        "Error: $number: 1 and $min: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0px, 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn number_and_max() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.clamp(0px, 1, 2)}\
             \n"
        ).unwrap_err(),
        "Error: $number: 1 and $min: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.clamp(0px, 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 0, 0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: math.clamp(0, 0, 0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function clamp($min, $number, $max) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn two_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.clamp(0, 0)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $max.\
         \n  ,--> input.scss\
         \n2 | a {b: math.clamp(0, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function clamp($min, $number, $max) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
    mod test_type {
        #[test]
        fn max() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1, 2, \"0\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $max: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.clamp(1, 2, \"0\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(\"0\", 1, 2)}\
             \n"
                )
                .unwrap_err(),
                "Error: $min: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.clamp(\"0\", 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn number() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.clamp(1, \"0\", 2)}\
             \n"
                )
                .unwrap_err(),
                "Error: $number: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.clamp(1, \"0\", 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
    }
    #[test]
    fn zero_args() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
             \na {b: math.clamp()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $min.\
         \n  ,--> input.scss\
         \n2 | a {b: math.clamp()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function clamp($min, $number, $max) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn min_equals_max() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {\
            \n  b: math.clamp(1, 2, 1);\
            \n}\
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
fn min_greater_than_max() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {\
            \n  b: math.clamp(1, 2, 0);\
            \n}\
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
fn named_args() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.clamp($min: 0, $number: 1, $max: 2)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
mod preserves_units {
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 1turn, 360deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 360deg;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 0.5turn, 360deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 180deg;\
        \n}\
        \n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.clamp(180deg, 0.75turn, 360deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 0.75turn;\
        \n}\
        \n"
        );
    }
}
