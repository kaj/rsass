//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/syntax.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("syntax")
}

#[test]
#[ignore] // missing error
fn dollar() {
    assert_eq!(
        runner().err("a {b: calc($)}\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: calc($)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn double_operator() {
    assert_eq!(
        runner().err("a {b: calc(1px ** 2px)}\n"),
        "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: calc(1px ** 2px)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn empty() {
    assert_eq!(
        runner().err("a {b: calc()}\n"),
        "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: calc()}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn hash() {
    assert_eq!(
        runner().err("a {b: calc(#)}\n"),
        "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: calc(#)}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
mod interpolation {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn in_floating_function() {
        assert_eq!(
            runner().err("a {b: calc(1 c(#{d}))}\n"),
            "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: calc(1 c(#{d}))}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn in_function_arg() {
        assert_eq!(
            runner().err("a {b: calc(c(~#{d}))}\n"),
            "Error: expected \")\".\
         \n  ,\
         \n1 | a {b: calc(c(~#{d}))}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn in_parens() {
        assert_eq!(
            runner().err("a {b: calc(1 (#{c}))}\n"),
            "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: calc(1 (#{c}))}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
}
#[test]
#[ignore] // missing error
fn leading_operator() {
    assert_eq!(
        runner().err("a {b: calc(+ 1px)}\n"),
        "Error: Expected digit.\
         \n  ,\
         \n1 | a {b: calc(+ 1px)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn multiple_args() {
    assert_eq!(
        runner().err("a {b: calc(1px, 2px)}\n"),
        "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: calc(1px, 2px)}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
    );
}
mod no_whitespace {
    #[allow(unused)]
    use super::runner;

    mod minus {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn after() {
            assert_eq!(
        runner().err(
            "a {b: calc(1 -1)}\n"
        ),
        "Error: \"+\" and \"-\" must be surrounded by whitespace in calculations.\
         \n  ,\
         \n1 | a {b: calc(1 -1)}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn before() {
            assert_eq!(
        runner().err(
            "a {b: calc(1- 1)}\n"
        ),
        "Error: \"+\" and \"-\" must be surrounded by whitespace in calculations.\
         \n  ,\
         \n1 | a {b: calc(1- 1)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn both() {
            assert_eq!(
        runner().err(
            "a {b: calc(1-1)}\n"
        ),
        "Error: \"+\" and \"-\" must be surrounded by whitespace in calculations.\
         \n  ,\
         \n1 | a {b: calc(1-1)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
        }
    }
    mod plus {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn after() {
            assert_eq!(
        runner().err(
            "a {b: calc(1 +1)}\n"
        ),
        "Error: \"+\" and \"-\" must be surrounded by whitespace in calculations.\
         \n  ,\
         \n1 | a {b: calc(1 +1)}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn before() {
            assert_eq!(
        runner().err(
            "a {b: calc(1+ 1)}\n"
        ),
        "Error: \"+\" and \"-\" must be surrounded by whitespace in calculations.\
         \n  ,\
         \n1 | a {b: calc(1+ 1)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn both() {
            assert_eq!(
        runner().err(
            "a {b: calc(1+1)}\n"
        ),
        "Error: \"+\" and \"-\" must be surrounded by whitespace in calculations.\
         \n  ,\
         \n1 | a {b: calc(1+1)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
        }
    }
}
#[test]
#[ignore] // missing error
fn trailing_operator() {
    assert_eq!(
        runner().err("a {b: calc(1px *)}\n"),
        "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: calc(1px *)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn unknown_operator() {
    assert_eq!(
        runner().err("a {b: calc(1px % 2px)}\n"),
        "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: calc(1px % 2px)}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
    );
}
