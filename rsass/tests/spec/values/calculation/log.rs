//! Tests auto-converted from "sass-spec/spec/values/calculation/log.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("log")
}

mod base {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn between_zero_and_one() {
        assert_eq!(
            runner().ok("a {b: log(2, 0.5)}\n"),
            "a {\
         \n  b: -1;\
         \n}\n"
        );
    }
    #[test]
    fn negative() {
        assert_eq!(
            runner().ok("a {b: log(2, -1)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    fn one() {
        assert_eq!(
            runner().ok("a {b: log(2, 1)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
    #[test]
    fn positive() {
        assert_eq!(
            runner().ok("a {b: log(2, 10)}\n"),
            "a {\
         \n  b: 0.3010299957;\
         \n}\n"
        );
    }
    #[test]
    fn zero() {
        assert_eq!(
            runner().ok("a {b: log(2, 0)}\n"),
            "a {\
         \n  b: 0;\
         \n}\n"
        );
    }
}
#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: LoG(2)}\n"),
        "a {\
         \n  b: 0.6931471806;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn base_type() {
        assert_eq!(
            runner().err("a {b: log(0, \"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: log(0, \"0\")}\
         \n  |              ^^^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn number_type() {
        assert_eq!(
            runner().err("a {b: log(\"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: log(\"0\")}\
         \n  |           ^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: log(7 % 3)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: log(7 % 3)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: log($, 10)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: log($, 10)}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: log()}\n"),
            "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: log()}\
         \n  |       ^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: log(0, 0, 0)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,\
         \n1 | a {b: log(0, 0, 0)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn complex_and_unknown() {
            assert_eq!(
                runner().err("a {b: log(1px*2px, 10%)}\n"),
                "Error: Expected calc(2px * 1px) to have no units.\
         \n  ,\
         \n1 | a {b: log(1px*2px, 10%)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn known() {
            assert_eq!(
                runner().err("a {b: log(3px)}\n"),
                "Error: Expected 3px to have no units.\
         \n  ,\
         \n1 | a {b: log(3px)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn known_incompatible() {
            assert_eq!(
                runner().err("a {b: log(1deg, 1px)}\n"),
                "Error: Expected 1deg to have no units.\
         \n  ,\
         \n1 | a {b: log(1deg, 1px)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn unitless_and_real() {
            assert_eq!(
                runner().err("a {b: log(1, 1px)}\n"),
                "Error: Expected 1px to have no units.\
         \n  ,\
         \n1 | a {b: log(1, 1px)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
                runner().err("a {b: log(1%)}\n"),
                "Error: Expected 1% to have no units.\
         \n  ,\
         \n1 | a {b: log(1%)}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
fn infinity() {
    assert_eq!(
        runner().ok("a {b: log(infinity)}\n"),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: log(-1)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function log($arg) {@return $arg}\
             \na {b: log(2)}\n"),
        "a {\
         \n  b: 2;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("a {b: log(2)}\n"),
        "a {\
         \n  b: 0.6931471806;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: log(3px - 1px + var(--c), var(--e));\
             \n}\n"),
        "a {\
         \n  b: log(2px + var(--c), var(--e));\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("a {b: log(0)}\n"),
        "a {\
         \n  b: calc(-infinity);\
         \n}\n"
    );
}
