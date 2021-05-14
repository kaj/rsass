//! Tests auto-converted from "sass-spec/spec/expressions/syntax.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
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
