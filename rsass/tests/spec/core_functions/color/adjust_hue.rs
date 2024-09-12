//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("adjust_hue")
}

#[test]
fn above_max() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 540)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn alpha() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(rgba(red, 0.1), 359)}\n"),
        "a {\
         \n  b: rgba(255, 0, 4.25, 0.1);\
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
            "a {b: adjust-hue(lch(0% 0 0deg), 10deg)}\n"
        ),
        "Error: adjust-hue() is only supported for legacy colors. Please use color.adjust() instead with an explicit $space argument.\
         \n  ,\
         \n1 | a {b: adjust-hue(lch(0% 0 0deg), 10deg)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: adjust-hue(red)}\n"),
            "Error: Missing argument $degrees.\
         \n  ,--> input.scss\
         \n1 | a {b: adjust-hue(red)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function adjust-hue($color, $degrees) {\
         \n  |           ============================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: adjust-hue(red, 1, 2)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: adjust-hue(red, 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function adjust-hue($color, $degrees) {\
         \n  |           ============================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn color() {
            assert_eq!(
                runner().err("a {b: adjust-hue(1, 2)}\n"),
                "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: adjust-hue(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn hue() {
            assert_eq!(
                runner().err("a {b: adjust-hue(red, blue)}\n"),
                "Error: $degrees: blue is not a number.\
         \n  ,\
         \n1 | a {b: adjust-hue(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
#[ignore] // wrong result
fn fraction() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 0.5)}\n"),
        "a {\
         \n  b: rgb(255, 2.125, 0);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn max() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 359)}\n"),
        "a {\
         \n  b: rgb(255, 0, 4.25);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn middle() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, 123)}\n"),
        "a {\
         \n  b: rgb(0, 255, 12.75);\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(blue, 0)}\n"),
        "a {\
         \n  b: blue;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        runner().ok("a {b: adjust-hue($color: red, $degrees: 123)}\n"),
        "a {\
         \n  b: rgb(0, 255, 12.75);\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: adjust-hue(red, -180)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
mod units {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn angle() {
        assert_eq!(
            runner().ok("a {b: adjust-hue(red, 60rad)}\n"),
            "a {\
         \n  b: rgb(0, 179.576224164, 255);\
         \n}\n"
        );
    }
    #[test]
    fn deg() {
        assert_eq!(
            runner().ok("a {b: adjust-hue(red, 60deg)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: adjust-hue(red, 60)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {b: adjust-hue(red, 60in)}\n"),
            "a {\
         \n  b: yellow;\
         \n}\n"
        );
    }
}
