//! Tests auto-converted from "sass-spec/spec/core_functions/color/hue.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hue")
}

#[test]
fn above_max() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue(hsl(540, 50%, 50%))}\n"),
        "a {\
         \n  b: 180deg;\
         \n}\n"
    );
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
             \na {b: color.hue(lch(0% 0 0deg))}\n"
        ),
        "Error: color.hue() is only supported for legacy colors. Please use color.channel() instead with an explicit $space argument.\
         \n  ,\
         \n2 | a {b: color.hue(lch(0% 0 0deg))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hue()}\n"
            ),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.hue()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function hue($color) {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hue(red, green)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.hue(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function hue($color) {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.hue(1)}\n"
            ),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.hue(1)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn fraction() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue(hsl(0.5, 50%, 50%))}\n"),
        "a {\
         \n  b: 0.5deg;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue(hsl(359, 50%, 50%))}\n"),
        "a {\
         \n  b: 359deg;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue(hsl(123, 50%, 50%))}\n"),
        "a {\
         \n  b: 123deg;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue(hsl(0, 50%, 50%))}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue($color: hsl(234, 50%, 50%))}\n"),
        "a {\
         \n  b: 234deg;\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.hue(hsl(-180, 50%, 50%))}\n"),
        "a {\
         \n  b: 180deg;\
         \n}\n"
    );
}
