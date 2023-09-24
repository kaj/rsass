//! Tests auto-converted from "sass-spec/spec/values/calculation/exp.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("exp")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: ExP(5)}\n"),
        "a {\
         \n  b: 148.4131591026;\
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
            runner().err("a {b: exp(7 % 3)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: exp(7 % 3)}\
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
                runner().err("a {b: exp($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: exp($)}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: exp()}\n"),
            "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: exp()}\
         \n  |       ^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: exp(0, 0)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: exp(0, 0)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err("a {b: exp(\"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: exp(\"0\")}\
         \n  |           ^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    mod unit {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn known() {
            assert_eq!(
                runner().err("a {b: exp(1px)}\n"),
                "Error: Expected 1px to have no units.\
         \n  ,\
         \n1 | a {b: exp(1px)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn unknown() {
            assert_eq!(
                runner().err("a {b: exp(1%)}\n"),
                "Error: Expected 1% to have no units.\
         \n  ,\
         \n1 | a {b: exp(1%)}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
fn negative() {
    assert_eq!(
        runner().ok("a {b: exp(-10.5)}\n"),
        "a {\
         \n  b: 0.0000275364;\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function exp($arg) {@return $arg}\
             \na {b: exp(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("a {b: exp(5)}\n"),
        "a {\
         \n  b: 148.4131591026;\
         \n}\n"
    );
}
#[test]
fn result_is_infinity() {
    assert_eq!(
        runner().ok("a {b: exp(1000.65)}\n"),
        "a {\
         \n  b: calc(infinity);\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: exp(1px + 2px - var(--c))\
             \n}\n"),
        "a {\
         \n  b: exp(3px - var(--c));\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("a {b: exp(0)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
