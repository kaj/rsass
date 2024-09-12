//! Tests auto-converted from "sass-spec/spec/core_functions/global/color/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod too_low {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn darken() {
        assert_eq!(
            runner().err("a {b: darken(red, -0.001)}\n"),
            "Error: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: darken(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn desaturate() {
        assert_eq!(
            runner().err("a {b: desaturate(red, -0.001)}\n"),
            "Error: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: desaturate(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn fade_in() {
        assert_eq!(
            runner().err("a {b: fade-in(red, -0.001)}\n"),
            "Error: $amount: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: fade-in(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn fade_out() {
        assert_eq!(
            runner().err("a {b: fade-out(red, -0.001)}\n"),
            "Error: $amount: Expected -0.001 to be within 0 and 1.\
         \n  ,\
         \n1 | a {b: fade-out(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn lighten() {
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
    #[test]
    fn saturate() {
        assert_eq!(
            runner().err("a {b: saturate(red, -0.001)}\n"),
            "Error: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: saturate(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
