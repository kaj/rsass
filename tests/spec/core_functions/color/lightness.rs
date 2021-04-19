//! Tests auto-converted from "sass-spec/spec/core_functions/color/lightness.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: lightness()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: lightness()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function lightness($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: lightness(red, green)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: lightness(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function lightness($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: lightness(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: lightness(1)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn fraction() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness(hsl(0, 100%, 0.5%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.5%;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness(hsl(0, 100%, 100%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 100%;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness(hsl(0, 100%, 50%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 50%;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness(hsl(0, 100%, 0%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0%;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: lightness($color: hsl(0, 100%, 42%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 42%;\
        \n}\
        \n"
    );
}
