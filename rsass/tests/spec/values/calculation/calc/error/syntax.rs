//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/error/syntax.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("syntax")
}

#[test]
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
#[ignore] // wrong error
fn double_operator() {
    assert_eq!(
        runner().err("a {b: calc(1px ** 2px)}\n"),
        "Error: Expected expression.\
         \n  ,\
         \n1 | a {b: calc(1px ** 2px)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
    );
}
#[test]
fn empty() {
    assert_eq!(
        runner().err("a {b: calc()}\n"),
        "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: calc()}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn hash() {
    assert_eq!(
        runner().err("a {b: calc(#)}\n"),
        "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: calc(#)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
}
mod interpolation {
    #[allow(unused)]
    use super::runner;

    #[test]
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
    fn line_noise() {
        assert_eq!(
        runner().err(
            "// Interpolation no longer shifts the parser into a special mode where it allows\
             \n// any interpolated declaration value.\
             \na {b: calc(!{@}#$%^&*#{c}_-[+]=)}\n"
        ),
        "Error: expected \")\".\
         \n  ,\
         \n3 | a {b: calc(!{@}#$%^&*#{c}_-[+]=)}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
    );
    }
}
#[test]
#[ignore] // missing error
fn leading_operator() {
    assert_eq!(
        runner().err("a {b: calc(+ 1px)}\n"),
        "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: calc(+ 1px)}\
         \n  |            ^^^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
    );
}
#[test]
fn multiple_args() {
    assert_eq!(
        runner().err("a {b: calc(1px, 2px)}\n"),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: calc(1px, 2px)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod no_whitespace {
    #[allow(unused)]
    use super::runner;

    mod minus {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
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
        "DEPRECATION WARNING [strict-unary]: This operation is parsed as:\n\
         \n    1 + 1\n\
         \nbut you may have intended it to mean:\n\
         \n    1 (+1)\n\
         \nAdd a space after + to clarify that it\'s meant to be a binary operation, or wrap\
         \nit in parentheses to make it a unary operation. This will be an error in future\
         \nversions of Sass.\n\
         \nMore info and automated migrator: https://sass-lang.com/d/strict-unary\n\
         \n  ,\
         \n1 | a {b: calc(1 +1)}\
         \n  |            ^^^^\
         \n  \'\
         \n    input.scss 1:12  root stylesheet\n\
         \nError: \"+\" and \"-\" must be surrounded by whitespace in calculations.\
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
#[ignore] // wrong error
fn trailing_operator() {
    assert_eq!(
        runner().err("a {b: calc(1px *)}\n"),
        "Error: Expected expression.\
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
        "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: calc(1px % 2px)}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
    );
}
