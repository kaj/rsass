//! Tests auto-converted from "sass-spec/spec/core_functions/color/grayscale.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: grayscale(rgba(#633736, 0.3))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(77, 77, 77, 0.3);\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: grayscale()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: grayscale()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function grayscale($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: grayscale(red, 1)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: grayscale(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function grayscale($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: grayscale(c)}\
             \n"
            )
            .unwrap_err(),
            "Error: $color: c is not a color.\
         \n  ,\
         \n1 | a {b: grayscale(c)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn max_saturation() {
    assert_eq!(
        crate::rsass(
            "a {b: grayscale(red)}\
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
fn mid_saturation() {
    assert_eq!(
        crate::rsass(
            "a {b: grayscale(#633736)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #4d4d4d;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: grayscale($color: white)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: white;\
        \n}\
        \n"
    );
}
mod no_saturation {
    #[test]
    fn black() {
        assert_eq!(
            crate::rsass(
                "a {b: grayscale(black)}\
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
                "a {b: grayscale(#494949)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #494949;\
        \n}\
        \n"
        );
    }
    #[test]
    fn white() {
        assert_eq!(
            crate::rsass(
                "a {b: grayscale(white)}\
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
fn number() {
    assert_eq!(
        crate::rsass(
            "// A number should produce a plain function string, for CSS filter functions.\
            \na {b: grayscale(15%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: grayscale(15%);\
        \n}\
        \n"
    );
}
