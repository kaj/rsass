//! Tests auto-converted from "sass-spec/spec/expressions/if/error/semicolon.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("semicolon")
}

#[test]
#[ignore] // missing error
fn comma() {
    assert_eq!(
        runner().err("a {b: if(css(1): c, css(2): d)}\n"),
        "Error: expected \")\".\
         \n  ,\
         \n1 | a {b: if(css(1): c, css(2): d)}\
         \n  |                           ^\
         \n  \'\
         \n  input.scss 1:27  root stylesheet",
    );
}
mod multiple {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn end() {
        assert_eq!(
            runner().err("a {b: if(css(): c;;)}\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: if(css(): c;;)}\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn middle() {
        assert_eq!(
            runner().err("a {b: if(css(1): c;; css(2): d)}\n"),
            "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: if(css(1): c;; css(2): d)}\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
}
