//! Tests auto-converted from "sass-spec/spec/core_functions/color/blue.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("blue")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn non_legacy() {
        assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.blue(color(srgb 1 1 1))}\n"
        ),
        "Error: color.blue() is only supported for legacy colors. Please use color.channel() instead with an explicit $space argument.\
         \n  ,\
         \n2 | a {b: color.blue(color(srgb 1 1 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.blue()}\n"
            ),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.blue()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function blue($color) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.blue(red, green)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.blue(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function blue($color) {\
         \n  |           ============ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.blue(1)}\n"
            ),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.blue(1)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.blue(rgb(0, 0, 255))}\n"),
        "a {\
         \n  b: 255;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.blue(rgb(0, 0, 123))}\n"),
        "a {\
         \n  b: 123;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.blue(rgb(0, 0, 0))}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.blue($color: rgb(0, 0, 234))}\n"),
        "a {\
         \n  b: 234;\
         \n}\n"
    );
}
