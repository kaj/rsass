//! Tests auto-converted from "sass-spec/spec/core_functions/color/hue.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn above_max() {
    assert_eq!(
        runner().ok("a {b: hue(hsl(540, 100%, 100%))}\n"),
        "a {\
         \n  b: 180deg;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: hue()}\n"),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: hue()}\
         \n  |       ^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function hue($color) {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: hue(red, green)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: hue(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function hue($color) {\
         \n  |           =========== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: hue(1)}\n"),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: hue(1)}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn fraction() {
    assert_eq!(
        runner().ok("a {b: hue(hsl(0.5, 100%, 100%))}\n"),
        "a {\
         \n  b: 0.5deg;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("a {b: hue(hsl(359, 100%, 100%))}\n"),
        "a {\
         \n  b: 359deg;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("a {b: hue(hsl(123, 100%, 100%))}\n"),
        "a {\
         \n  b: 123deg;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: hue(hsl(0, 100%, 100%))}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: hue($color: hsl(234, 100%, 100%))}\n"),
        "a {\
         \n  b: 234deg;\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: hue(hsl(-180, 100%, 100%))}\n"),
        "a {\
         \n  b: 180deg;\
         \n}\n"
    );
}
