//! Tests auto-converted from "sass-spec/spec/core_functions/color/saturation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: saturation()}\n"),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: saturation()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function saturation($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: saturation(red, green)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: saturation(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function saturation($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: saturation(1)}\n"),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: saturation(1)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn fraction() {
    assert_eq!(
        runner().ok("a {b: saturation(hsl(0, 0.5%, 100%))}\n"),
        "a {\
         \n  b: 0.5%;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("a {b: saturation(hsl(0, 100%, 100%))}\n"),
        "a {\
         \n  b: 100%;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("a {b: saturation(hsl(0, 50%, 100%))}\n"),
        "a {\
         \n  b: 50%;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: saturation(hsl(0, 0%, 100%))}\n"),
        "a {\
         \n  b: 0%;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: saturation($color: hsl(0, 42%, 100%))}\n"),
        "a {\
         \n  b: 42%;\
         \n}\n"
    );
}
