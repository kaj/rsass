//! Tests auto-converted from "sass-spec/spec/core_functions/color/invert.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(rgba(turquoise, 0.4))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(191, 31, 47, 0.4);\
        \n}\
        \n"
    );
}
#[test]
fn black() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(black)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: white;\
        \n}\
        \n"
    );
}
mod error {
    mod bounds {
        #[test]
        fn too_high() {
            assert_eq!(
                crate::rsass(
                    "a {b: invert(red, 100.001)}\
             \n"
                )
                .unwrap_err(),
                "Error: $weight: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: invert(red, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn too_low() {
            assert_eq!(
                crate::rsass(
                    "a {b: invert(red, -0.001)}\
             \n"
                )
                .unwrap_err(),
                "Error: $weight: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: invert(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
    #[test]
    fn number_with_weight() {
        assert_eq!(
        crate::rsass(
            "a {b: invert(1, 50%)}\
             \n"
        ).unwrap_err(),
        "Error: Only one argument may be passed to the plain-CSS invert() function.\
         \n  ,\
         \n1 | a {b: invert(1, 50%)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: invert()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: invert()}\
         \n  |       ^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function invert($color, $weight: 100%) {\
         \n  |           ============================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 0%, 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: invert(turquoise, 0%, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function invert($color, $weight: 100%) {\
         \n  |           ============================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    mod test_type {
        #[test]
        #[ignore] // missing error
        fn color() {
            assert_eq!(
                crate::rsass(
                    "a {b: invert(c)}\
             \n"
                )
                .unwrap_err(),
                "Error: $color: c is not a color.\
         \n  ,\
         \n1 | a {b: invert(c)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn weight() {
            assert_eq!(
                crate::rsass(
                    "a {b: invert(red, c)}\
             \n"
                )
                .unwrap_err(),
                "Error: $weight: c is not a number.\
         \n  ,\
         \n1 | a {b: invert(red, c)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
}
#[test]
fn gray() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(gray)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #7f7f7f;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: invert($color: turquoise, $weight: 0%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: turquoise;\
        \n}\
        \n"
    );
}
#[test]
fn number() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(10%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: invert(10%);\
        \n}\
        \n"
    );
}
#[test]
fn turquoise() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(turquoise)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #bf1f2f;\
        \n}\
        \n"
    );
}
mod weighted {
    #[test]
    fn high() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 92%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #b52e3c;\
        \n}\
        \n"
        );
    }
    #[test]
    fn low() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 23%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #5db4ab;\
        \n}\
        \n"
        );
    }
    #[test]
    fn max() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 100%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #bf1f2f;\
        \n}\
        \n"
        );
    }
    #[test]
    fn middle() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 50%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: gray;\
        \n}\
        \n"
        );
    }
    #[test]
    fn min() {
        assert_eq!(
            crate::rsass(
                "a {b: invert(turquoise, 0%)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: turquoise;\
        \n}\
        \n"
        );
    }
}
#[test]
fn white() {
    assert_eq!(
        crate::rsass(
            "a {b: invert(white)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: black;\
        \n}\
        \n"
    );
}
