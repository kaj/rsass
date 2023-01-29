//! Tests auto-converted from "sass-spec/spec/values/numbers/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod trailing_dot {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn digit() {
        assert_eq!(
            runner().err("a {b: 1.}\n"),
            "Error: Expected digit.\
         \n  ,\
         \n1 | a {b: 1.}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn minus() {
        assert_eq!(
            runner().err("a {b: -.}\n"),
            "Error: Expected digit.\
         \n  ,\
         \n1 | a {b: -.}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn minus_digit() {
        assert_eq!(
            runner().err("a {b: -1.}\n"),
            "Error: Expected digit.\
         \n  ,\
         \n1 | a {b: -1.}\
         \n  |          ^\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn plus() {
        assert_eq!(
            runner().err("a {b: +.}\n"),
            "Error: Expected digit.\
         \n  ,\
         \n1 | a {b: +.}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn plus_digit() {
        assert_eq!(
            runner().err("a {b: +1.}\n"),
            "Error: Expected digit.\
         \n  ,\
         \n1 | a {b: +1.}\
         \n  |          ^\
         \n  \'\
         \n  input.scss 1:10  root stylesheet",
        );
    }
}
