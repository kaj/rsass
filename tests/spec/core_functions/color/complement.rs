//! Tests auto-converted from "sass-spec/spec/core_functions/color/complement.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: complement(rgba(turquoise, 0.7))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(224, 64, 80, 0.7);\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: complement()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: complement()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function complement($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: complement(red, 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: complement(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function complement($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: complement(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: complement(1)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
mod grayscale {
    #[test]
    fn black() {
        assert_eq!(
            crate::rsass(
                "a {b: complement(black)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: black;\
        \n}\
        \n"
        );
    }
    #[test]
    fn gray() {
        assert_eq!(
            crate::rsass(
                "a {b: complement(gray)}\
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
    fn white() {
        assert_eq!(
            crate::rsass(
                "a {b: complement(white)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: white;\
        \n}\
        \n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: complement($color: red)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: aqua;\
        \n}\
        \n"
    );
}
#[test]
fn red() {
    assert_eq!(
        crate::rsass(
            "a {b: complement(red)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: aqua;\
        \n}\
        \n"
    );
}
#[test]
fn turquoise() {
    assert_eq!(
        crate::rsass(
            "a {b: complement(turquoise)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #e04050;\
        \n}\
        \n"
    );
}
