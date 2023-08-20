//! Tests auto-converted from "sass-spec/spec/values/calculation/sqrt.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("sqrt")
}

#[test]
#[ignore] // wrong result
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: sQrT(2)}\n"),
        "a {\
         \n  b: 1.4142135624;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: sqrt(7 % 3)}\n"),
            "Error: This operation can\'t be used in a calculation.\
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
        #[ignore] // wrong error
        fn too_few_args() {
            assert_eq!(
                runner().err("a {b: sqrt()}\n"),
                "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: sqrt()}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: sqrt(3, 4)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: sqrt(3, 4)}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err("a {b: sqrt(\"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: sqrt(\"0\")}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
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
        #[ignore] // wrong error
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
fn negative() {
    assert_eq!(
        runner().ok("a {b: sqrt(-9)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function sqrt($arg) {@return $arg}\
             \na {b: sqrt(2)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
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
fn zero() {
    assert_eq!(
        runner().ok("a {b: sqrt(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
