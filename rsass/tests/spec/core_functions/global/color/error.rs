//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod too_low {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn darken() {
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
    #[test]
    #[ignore] // wrong error
    fn desaturate() {
        assert_eq!(
        runner().err(
            "a {b: desaturate(red, -0.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: desaturate(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: desaturate(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn fade_in() {
        assert_eq!(
        runner().err(
            "a {b: fade-in(red, -0.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: fade-in(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: fade-in(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn fade_out() {
        assert_eq!(
        runner().err(
            "a {b: fade-out(red, -0.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: fade-out(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: fade-out(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn lighten() {
        assert_eq!(
        runner().err(
            "a {b: lighten(red, -0.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: lighten(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: lighten(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
    #[test]
    #[ignore] // wrong error
    fn saturate() {
        assert_eq!(
        runner().err(
            "a {b: saturate(red, -0.001)}\n"
        ),
        "DEPRECATION WARNING [global-builtin]: Global built-in functions are deprecated and will be removed in Dart Sass 3.0.0.\
         \nUse color.adjust instead.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/import\n\
         \n  ,\
         \n1 | a {b: saturate(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n    input.scss 1:7  root stylesheet\n\
         \nError: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: saturate(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
    }
}
