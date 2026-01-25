//! Tests auto-converted from "sass-spec/spec/expressions/if/error/missing_whitepsace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("missing_whitepsace")
}

mod after_and {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn full() {
        assert_eq!(
            runner().err("a {b: if(css(1) and(css(2)): d)}\n"),
            "Error: Whitespace is required between \"and\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(css(1) and(css(2)): d)}\
         \n  |                    ^\
         \n  \'\
         \n  input.scss 1:20  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn raw() {
        assert_eq!(
            runner()
                .err("a {b: if(var(--clause-and) css(1) and(css(2)): d)}\n"),
            "Error: Whitespace is required between \"and\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(var(--clause-and) css(1) and(css(2)): d)}\
         \n  |                                      ^\
         \n  \'\
         \n  input.scss 1:38  root stylesheet",
        );
    }
}
#[test]
#[ignore] // missing error
fn after_not() {
    assert_eq!(
        runner().err("a {b: if(not(css()): d)}\n"),
        "Error: Whitespace is required between \"not\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(not(css()): d)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
mod after_or {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn full() {
        assert_eq!(
            runner().err("a {b: if(css(1) or(css(2)): d)}\n"),
            "Error: Whitespace is required between \"and\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(css(1) or(css(2)): d)}\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn raw() {
        assert_eq!(
            runner()
                .err("a {b: if(var(--clause-or) css(1) or(css(2)): d)}\n"),
            "Error: Whitespace is required between \"or\" and \"(\"\
         \n  ,\
         \n1 | a {b: if(var(--clause-or) css(1) or(css(2)): d)}\
         \n  |                                    ^\
         \n  \'\
         \n  input.scss 1:36  root stylesheet",
        );
    }
}
