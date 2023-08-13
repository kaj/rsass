//! Tests auto-converted from "sass-spec/spec/values/calculation/sqrt.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("sqrt")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: sqrt(7 % 3)}\n"),
            "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: sqrt(7 % 3)}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: sqrt($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: sqrt($)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn too_few_args() {
            assert_eq!(
                runner().err("a {b: sqrt()}\n"),
                "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: sqrt()}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: sqrt(3, 4)}\n"),
            "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: sqrt(3, 4)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
            runner().err("a {b: sqrt(\"0\")}\n"),
            "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: sqrt(\"0\")}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn real() {
            assert_eq!(
                runner().err("a {b: sqrt(16px)}\n"),
                "Error: Expected 16px to have no units.\
         \n  ,\
         \n1 | a {b: sqrt(16px)}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn unknown() {
            assert_eq!(
                runner().err("a {b: sqrt(1%)}\n"),
                "Error: Expected 1% to have no units.\
         \n  ,\
         \n1 | a {b: sqrt(1%)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
#[ignore] // wrong result
fn negative() {
    assert_eq!(
        runner().ok("a {b: sqrt(-9)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: sqrt(1px + 2px - var(--c))\
             \n}\n"),
        "a {\
         \n  b: sqrt(3px - var(--c));\
         \n}\n"
    );
}
mod units {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: sqrt(2)}\n"),
            "a {\
         \n  b: 1.4142135624;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn zero() {
    assert_eq!(
        runner().ok("a {b: sqrt(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
