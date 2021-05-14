//! Tests auto-converted from "sass-spec/spec/css/functions/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod single_equals {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn no_lhs() {
        assert_eq!(
            runner().err("a {b: c(=d)}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | a {b: c(=d)}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn no_lhs_or_rhs() {
        assert_eq!(
            runner().err("a {b: c(=)}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | a {b: c(=)}\
         \n  |         ^\
         \n  \'\
         \n  input.scss 1:9  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn no_rhs() {
        assert_eq!(
            runner().err("a {b: c(d=)}\n"),
            "Error: Expected expression.\
         \n  ,\
         \n1 | a {b: c(d=)}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
}
