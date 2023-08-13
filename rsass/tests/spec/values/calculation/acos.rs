//! Tests auto-converted from "sass-spec/spec/values/calculation/acos.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("acos")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: acos(7 % 3)}\n"),
            "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: acos(7 % 3)}\
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
                runner().err("a {b: acos($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: acos($)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: acos()}\n"),
            "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: acos()}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: acos(0, 0)}\n"),
            "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: acos(0, 0)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
            runner().err("a {b: acos(\"0\")}\n"),
            "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: acos(\"0\")}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
        );
    }
    mod unit {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn complex() {
            assert_eq!(
                runner().err("a {b: acos(-7px / 4em)}\n"),
                "Error: Expected -1.75px/em to have no units.\
         \n  ,\
         \n1 | a {b: acos(-7px / 4em)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn known() {
            assert_eq!(
                runner().err("a {b: acos(1px)}\n"),
                "Error: Expected 1px to have no units.\
         \n  ,\
         \n1 | a {b: acos(1px)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn unknown() {
            assert_eq!(
                runner().err("a {b: acos(1%)}\n"),
                "Error: Expected 1% to have no units.\
         \n  ,\
         \n1 | a {b: acos(1%)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
#[test]
#[ignore] // wrong result
fn greater_than_one() {
    assert_eq!(
        runner().ok("a {b: acos(2)}\n"),
        "a {\
         \n  b: calc(NaN * 1deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn less_than_negative_one() {
    assert_eq!(
        runner().ok("a {b: acos(-2)}\n"),
        "a {\
         \n  b: calc(NaN * 1deg);\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn negative_one() {
    assert_eq!(
        runner().ok("a {b: acos(-1)}\n"),
        "a {\
         \n  b: 180deg;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn one() {
    assert_eq!(
        runner().ok("a {b: acos(1)}\n"),
        "a {\
         \n  b: 0deg;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: acos(3px - 1px + var(--c));\
             \n}\n"),
        "a {\
         \n  b: acos(2px + var(--c));\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn zero() {
    assert_eq!(
        runner().ok("a {b: acos(0)}\n"),
        "a {\
         \n  b: 90deg;\
         \n}\n"
    );
}
