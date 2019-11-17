//! Tests auto-converted from "sass-spec/spec/core_functions/math/hypot.hrx"

#[test]
#[ignore] // wrong result
fn compatible_units() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.hypot(3cm, 4mm * 10, 5q * 40, 6in / 2.54, 7px * 96 / 2.54)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 11.6189500386cm;\
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
             \na {b: math.hypot(1turn, 1px, 1s)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[2]: 1px and $numbers[1]: 1turn have incompatible units.\
         \n  ,\
         \n2 | a {b: math.hypot(1turn, 1px, 1s)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn first_and_second() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(1deg, 1px, 1turn)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[2]: 1px and $numbers[1]: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.hypot(1deg, 1px, 1turn)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn first_and_third() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(1deg, 1turn, 1px)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[3]: 1px and $numbers[1]: 1deg have incompatible units.\
         \n  ,\
         \n2 | a {b: math.hypot(1deg, 1turn, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn second_and_third() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(1turn, 1deg, 1px)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[3]: 1px and $numbers[1]: 1turn have incompatible units.\
         \n  ,\
         \n2 | a {b: math.hypot(1turn, 1deg, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
    }
    mod some_unitless {
        #[test]
        fn first() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(0, 1px, 2px)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[2]: 1px and $numbers[1]: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0, 1px, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn first_and_second() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(0, 1, 2px)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[3]: 2px and $numbers[1]: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0, 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn first_and_third() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(0, 1px, 2)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[2]: 1px and $numbers[1]: 0 have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0, 1px, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn second() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(0px, 1, 2px)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[2]: 1 and $numbers[1]: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0px, 1, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn second_and_third() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(0px, 1, 2)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[2]: 1 and $numbers[1]: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0px, 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
        #[test]
        fn third() {
            assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
             \na {b: math.hypot(0px, 1px, 2)}\
             \n"
        ).unwrap_err(),
        "Error: $numbers[3]: 2 and $numbers[1]: 0px have incompatible units (one has units and the other doesn\'t).\
         \n  ,\
         \n2 | a {b: math.hypot(0px, 1px, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
    );
        }
    }
    mod test_type {
        #[test]
        fn first() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.hypot(\"0\", 1px, 1px)}\
             \n"
                )
                .unwrap_err(),
                "Error: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.hypot(\"0\", 1px, 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.hypot(1px, \"0\", 1px)}\
             \n"
                )
                .unwrap_err(),
                "Error: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.hypot(1px, \"0\", 1px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn third() {
            assert_eq!(
                crate::rsass(
                    "@use \"sass:math\" as math;\
             \na {b: math.hypot(1px, 1px, \"0\")}\
             \n"
                )
                .unwrap_err(),
                "Error: \"0\" is not a number.\
         \n  ,\
         \n2 | a {b: math.hypot(1px, 1px, \"0\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \na {b: math.hypot()}\
             \n"
            )
            .unwrap_err(),
            "Error: At least one argument must be passed.\
         \n  ,\
         \n2 | a {b: math.hypot()}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet\
         \n",
        );
    }
}
mod infinity {
    #[test]
    fn first() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.hypot(1/0, 1, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.hypot(1, 1/0, 1)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
    #[test]
    fn third() {
        assert_eq!(
            crate::rsass(
                "@use \"sass:math\" as math;\
            \na {b: math.hypot(1, 1, 1/0)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: Infinity;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn unitless() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:math\" as math;\
            \na {b: math.hypot(3, 4, 5, 6, 7)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 11.6189500386;\
        \n}\
        \n"
    );
}
