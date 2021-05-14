//! Tests auto-converted from "sass-spec/spec/core_functions/color/lighten.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("a {b: lighten(rgba(red, 0.4), 100%)}\n"),
        "a {\
         \n  b: rgba(255, 255, 255, 0.4);\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod bounds {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn too_high() {
            assert_eq!(
                runner().err("a {b: lighten(red, 100.001)}\n"),
                "Error: $amount: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: lighten(red, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_low() {
            assert_eq!(
                runner().err("a {b: lighten(red, -0.001)}\n"),
                "Error: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: lighten(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: lighten(red)}\n"),
            "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n1 | a {b: lighten(red)}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function lighten($color, $amount) {\
         \n  |           ======================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: lighten(red, 1%, 2)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: lighten(red, 1%, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function lighten($color, $amount) {\
         \n  |           ======================== declaration\
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
                runner().err("a {b: lighten(1, 2)}\n"),
                "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: lighten(1, 2)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn lightness() {
            assert_eq!(
                runner().err("a {b: lighten(red, blue)}\n"),
                "Error: $amount: blue is not a number.\
         \n  ,\
         \n1 | a {b: lighten(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
fn fraction() {
    assert_eq!(
        runner().ok("a {b: lighten(red, 0.5%)}\n"),
        "a {\
         \n  b: #ff0303;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("a {b: lighten(red, 100%)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        runner().ok("a {b: lighten(red, 50%)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("a {b: lighten(red, 14%)}\n"),
        "a {\
         \n  b: #ff4747;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: lighten(red, 0%)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: lighten($color: red, $amount: 14%)}\n"),
        "a {\
         \n  b: #ff4747;\
         \n}\n"
    );
}
