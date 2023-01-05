//! Tests auto-converted from "sass-spec/spec/core_functions/color/lightness.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("lightness")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: lightness()}\n"),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: lightness()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function lightness($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: lightness(red, green)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: lightness(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function lightness($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: lightness(1)}\n"),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: lightness(1)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn fraction() {
    assert_eq!(
        runner().ok("a {b: lightness(hsl(0, 100%, 0.5%))}\n"),
        "a {\
         \n  b: 0.5%;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("a {b: lightness(hsl(0, 100%, 100%))}\n"),
        "a {\
         \n  b: 100%;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("a {b: lightness(hsl(0, 100%, 50%))}\n"),
        "a {\
         \n  b: 50%;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: lightness(hsl(0, 100%, 0%))}\n"),
        "a {\
         \n  b: 0%;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: lightness($color: hsl(0, 100%, 42%))}\n"),
        "a {\
         \n  b: 42%;\
         \n}\n"
    );
}
