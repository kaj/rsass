//! Tests auto-converted from "sass-spec/spec/core_functions/color/darken.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("darken")
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("a {b: darken(rgba(red, 0.2), 100%)}\n"),
        "a {\
         \n  b: rgba(0, 0, 0, 0.2);\
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
        #[ignore] // wrong error
        fn too_high() {
            assert_eq!(
        runner().err(
            "a {b: darken(red, 100.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: darken(red, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: darken(red, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn too_low() {
            assert_eq!(
        runner().err(
            "a {b: darken(red, -0.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: darken(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: darken(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
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
            "a {b: darken(color(srgb 1 1 1), 10%)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: darken(color(srgb 1 1 1), 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: darken() is only supported for legacy colors. Please use color.adjust() instead with an explicit $space argument.\
         \n  ,\
         \n1 | a {b: darken(color(srgb 1 1 1), 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: darken(red)}\n"),
            "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n1 | a {b: darken(red)}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function darken($color, $amount) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: darken(red, 1%, 2)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: darken(red, 1%, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function darken($color, $amount) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn color() {
            assert_eq!(
        runner().err(
            "a {b: darken(1, 2)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: darken(1, 2)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: darken(1, 2)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn lightness() {
            assert_eq!(
        runner().err(
            "a {b: darken(red, blue)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: darken(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: blue is not a number.\
         \n  ,\
         \n1 | a {b: darken(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
}
#[test]
fn fraction() {
    assert_eq!(
        runner().ok("a {b: darken(red, 0.5%)}\n"),
        "a {\
         \n  b: rgb(252.45, 0, 0);\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("a {b: darken(red, 100%)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        runner().ok("a {b: darken(red, 50%)}\n"),
        "a {\
         \n  b: black;\
         \n}\n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        runner().ok("a {b: darken(red, 14%)}\n"),
        "a {\
         \n  b: rgb(183.6, 0, 0);\
         \n}\n"
    );
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("a {b: darken(red, 0%)}\n"),
        "a {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: darken($color: red, $amount: 14%)}\n"),
        "a {\
         \n  b: rgb(183.6, 0, 0);\
         \n}\n"
    );
}
