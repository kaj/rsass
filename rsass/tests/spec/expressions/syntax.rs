//! Tests auto-converted from "sass-spec/spec/expressions/syntax.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("syntax")
}

mod error {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn single_dot() {
        assert_eq!(
            runner().err("a {a: .}\n"),
            "Error: Expected digit.\
         \n  ,\
         \n1 | a {a: .}\
         \n  |        ^\
         \n  \'\
         \n  input.scss 1:8  root stylesheet",
        );
    }
}
