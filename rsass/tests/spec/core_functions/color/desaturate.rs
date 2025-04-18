//! Tests auto-converted from "sass-spec/spec/core_functions/color/desaturate.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("desaturate")
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("a {b: desaturate(rgba(plum, 0.3), 100%)}\n"),
        "a {\
         \n  b: rgba(190.5, 190.5, 190.5, 0.3);\
         \n}\n"
    );
}
mod error {
    use super::runner;

    mod bounds {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn too_high() {
            assert_eq!(
        runner().err(
            "a {b: desaturate(plum, 100.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: desaturate(plum, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: desaturate(plum, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn too_low() {
            assert_eq!(
        runner().err(
            "a {b: desaturate(plum, -0.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: desaturate(plum, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: desaturate(plum, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn non_legacy() {
        assert_eq!(
        runner().err(
            "a {b: desaturate(color(srgb 1 1 1), 10%)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: desaturate(color(srgb 1 1 1), 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: desaturate() is only supported for legacy colors. Please use color.adjust() instead with an explicit $space argument.\
         \n  ,\
         \n1 | a {b: desaturate(color(srgb 1 1 1), 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    mod one_arg {
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
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn color() {
            assert_eq!(
        runner().err(
            "a {b: desaturate(1, 2)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: desaturate(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: desaturate(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn lightness() {
            assert_eq!(
        runner().err(
            "a {b: desaturate(plum, blue)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: desaturate(plum, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: blue is not a number.\
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
         \n  b: rgb(190.5, 190.5, 190.5);\
         \n}\n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        runner().ok("a {b: desaturate(plum, 48%)}\n"),
        "a {\
         \n  b: rgb(190.5, 190.5, 190.5);\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("a {b: desaturate(plum, 14%)}\n"),
        "a {\
         \n  b: rgb(211.97, 169.03, 211.97);\
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
         \n  b: rgb(211.97, 169.03, 211.97);\
         \n}\n"
    );
}
