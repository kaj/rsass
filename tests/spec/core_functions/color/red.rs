//! Tests auto-converted from "sass-spec/spec/core_functions/color/red.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: red()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: red()}\
         \n  |       ^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function red($color) {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: red(red, green)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: red(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function red($color) {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: red(1)}\
             \n"
            )
            .unwrap_err(),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: red(1)}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: red(rgb(255, 0, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 255;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: red(rgb(123, 0, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 123;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: red(rgb(0, 0, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: red($color: rgb(234, 0, 0))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 234;\
        \n}\
        \n"
    );
}
