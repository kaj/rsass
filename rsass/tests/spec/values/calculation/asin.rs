//! Tests auto-converted from "sass-spec/spec/values/calculation/asin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("asin")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: aSiN(1)}\n"),
        "a {\
         \n  b: 90deg;\
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
            runner().err("a {b: asin(7 % 3)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: asin(7 % 3)}\
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
                runner().err("a {b: asin($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: asin($)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: asin()}\n"),
            "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: asin()}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: asin(0, 0)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: asin(0, 0)}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err("a {b: asin(\"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: asin(\"0\")}\
         \n  |            ^^^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    mod unit {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn complex() {
            assert_eq!(
                runner().err("a {b: asin(-7px / 4em)}\n"),
                "Error: Expected calc(-1.75px / 1em) to have no units.\
         \n  ,\
         \n1 | a {b: asin(-7px / 4em)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn known() {
            assert_eq!(
                runner().err("a {b: asin(1px)}\n"),
                "Error: Expected 1px to have no units.\
         \n  ,\
         \n1 | a {b: asin(1px)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn unknown() {
            assert_eq!(
                runner().err("a {b: asin(1%)}\n"),
                "Error: Expected 1% to have no units.\
         \n  ,\
         \n1 | a {b: asin(1%)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
fn greater_than_one() {
    assert_eq!(
        runner().ok("a {b: asin(2)}\n"),
        "a {\
         \n  b: calc(NaN * 1deg);\
         \n}\n"
    );
}
#[test]
fn less_than_negative_one() {
    assert_eq!(
        runner().ok("a {b: asin(-2)}\n"),
        "a {\
         \n  b: calc(NaN * 1deg);\
         \n}\n"
    );
}
#[test]
fn negative_one() {
    assert_eq!(
        runner().ok("a {b: asin(-1)}\n"),
        "a {\
         \n  b: -90deg;\
         \n}\n"
    );
}
#[test]
fn one() {
    assert_eq!(
        runner().ok("a {b: asin(1)}\n"),
        "a {\
         \n  b: 90deg;\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function asin($arg) {@return $arg}\
             \na {b: asin(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: asin(3px - 1px + var(--c));\
             \n}\n"),
        "a {\
         \n  b: asin(2px + var(--c));\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("a {b: asin(0)}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
