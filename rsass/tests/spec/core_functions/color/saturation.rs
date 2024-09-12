//! Tests auto-converted from "sass-spec/spec/core_functions/color/saturation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("saturation")
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
             \na {b: color.saturation(color(srgb 1 1 1))}\n"
        ),
        "Error: color.saturation() is only supported for legacy colors. Please use color.channel() instead with an explicit $space argument.\
         \n  ,\
         \n2 | a {b: color.saturation(color(srgb 1 1 1))}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.saturation()}\n"
            ),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.saturation()}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function saturation($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.saturation(red, green)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.saturation(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function saturation($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:color\";\
             \na {b: color.saturation(1)}\n"
            ),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.saturation(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn fraction() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.saturation(hsl(0, 0.5%, 50%))}\n"),
        "a {\
         \n  b: 0.5%;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.saturation(hsl(0, 100%, 50%))}\n"),
        "a {\
         \n  b: 100%;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.saturation(hsl(0, 50%, 50%))}\n"),
        "a {\
         \n  b: 50%;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.saturation(hsl(0, 0%, 50%))}\n"),
        "a {\
         \n  b: 0%;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \"sass:color\";\
             \na {b: color.saturation($color: hsl(0, 42%, 50%))}\n"),
        "a {\
         \n  b: 42%;\
         \n}\n"
    );
}
