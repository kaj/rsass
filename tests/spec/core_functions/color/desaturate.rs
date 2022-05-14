//! Tests auto-converted from "sass-spec/spec/core_functions/color/desaturate.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("desaturate")
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("a {b: desaturate(rgba(plum, 0.3), 100%)}\n"),
        "a {\
         \n  b: rgba(191, 191, 191, 0.3);\
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
                runner().err("a {b: desaturate(plum, 100.001)}\n"),
                "Error: $amount: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: desaturate(plum, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_low() {
            assert_eq!(
                runner().err("a {b: desaturate(plum, -0.001)}\n"),
                "Error: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: desaturate(plum, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    mod one_arg {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn test_type() {
            assert_eq!(
                runner().err("a {b: desaturate(red)}\n"),
                "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n1 | a {b: desaturate(red)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function desaturate($color, $amount) {\
         \n  |           =========================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: desaturate(plum)}\n"),
            "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n1 | a {b: desaturate(plum)}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function desaturate($color, $amount) {\
         \n  |           =========================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: desaturate(plum, 1%, 2)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: desaturate(plum, 1%, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function desaturate($color, $amount) {\
         \n  |           =========================== declaration\
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
                runner().err("a {b: desaturate(1, 2)}\n"),
                "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: desaturate(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn lightness() {
            assert_eq!(
                runner().err("a {b: desaturate(plum, blue)}\n"),
                "Error: $amount: blue is not a number.\
         \n  ,\
         \n1 | a {b: desaturate(plum, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("a {b: desaturate(plum, 100%)}\n"),
        "a {\
         \n  b: #bfbfbf;\
         \n}\n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        runner().ok("a {b: desaturate(plum, 48%)}\n"),
        "a {\
         \n  b: #bfbfbf;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("a {b: desaturate(plum, 14%)}\n"),
        "a {\
         \n  b: #d4a9d4;\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: desaturate(plum, 0%)}\n"),
        "a {\
         \n  b: plum;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: desaturate($color: plum, $amount: 14%)}\n"),
        "a {\
         \n  b: #d4a9d4;\
         \n}\n"
    );
}
