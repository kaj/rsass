//! Tests auto-converted from "sass-spec/spec/expressions/if/error/invalid_function_name.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("invalid_function_name")
}

mod and {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn lowercase() {
        assert_eq!(
            runner().err("a {b: if(not and(): d)}\n"),
            "Error: Whitespace is required between \"and\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(not and(): d)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn uppercase() {
        assert_eq!(
            runner().err("a {b: if(not AND(): d)}\n"),
            "Error: Whitespace is required between \"AND\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(not AND(): d)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
}
mod not {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn lowercase() {
        assert_eq!(
            runner().err("a {b: if(not not(): d)}\n"),
            "Error: Whitespace is required between \"not\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(not not(): d)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn uppercase() {
        assert_eq!(
            runner().err("a {b: if(not NOT(): d)}\n"),
            "Error: Whitespace is required between \"NOT\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(not NOT(): d)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
        );
    }
}
mod or {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn lowercase() {
        assert_eq!(
            runner().err("a {b: if(not or(): d)}\n"),
            "Error: Whitespace is required between \"or\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(not or(): d)}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn uppercase() {
        assert_eq!(
            runner().err("a {b: if(not OR(): d)}\n"),
            "Error: Whitespace is required between \"OR\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(not OR(): d)}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
        );
    }
}
