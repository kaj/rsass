//! Tests auto-converted from "sass-spec/spec/values/calculation/sin.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("sin")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: SiN(1deg)}\n"),
        "a {\
         \n  b: 0.0174524064;\
         \n}\n"
    );
}
#[test]
fn deg() {
    assert_eq!(
        runner().ok("a {b: sin(1deg)}\n"),
        "a {\
         \n  b: 0.0174524064;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: sin(7 % 3)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: sin(7 % 3)}\
         \n  |             ^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    mod syntax {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: sin($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: sin($)}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: sin()}\n"),
            "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: sin()}\
         \n  |       ^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: sin(0, 0)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,\
         \n1 | a {b: sin(0, 0)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            runner().err("a {b: sin(\"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: sin(\"0\")}\
         \n  |           ^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    mod units {
        use super::runner;

        #[test]
        fn complex() {
            assert_eq!(
        runner().err(
            "a {b: sin(-7px / 4em)}\n"
        ),
        "Error: $number: Expected calc(-1.75px / 1em) to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n1 | a {b: sin(-7px / 4em)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        fn known() {
            assert_eq!(
        runner().err(
            "a {b: sin(1px)}\n"
        ),
        "Error: $number: Expected 1px to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n1 | a {b: sin(1px)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
        #[test]
        fn unknown() {
            assert_eq!(
        runner().err(
            "a {b: sin(1%)}\n"
        ),
        "Error: $number: Expected 1% to have an angle unit (deg, grad, rad, turn).\
         \n  ,\
         \n1 | a {b: sin(1%)}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
        }
    }
}
#[test]
fn grad() {
    assert_eq!(
        runner().ok("a {b: sin(1grad)}\n"),
        "a {\
         \n  b: 0.0157073173;\
         \n}\n"
    );
}
#[test]
fn infinity() {
    assert_eq!(
        runner().ok("a {b: sin(infinity)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
#[test]
fn negative_infinity() {
    assert_eq!(
        runner().ok("a {b: sin(-infinity)}\n"),
        "a {\
         \n  b: calc(NaN);\
         \n}\n"
    );
}
#[test]
fn negative_one() {
    assert_eq!(
        runner().ok("a {b: sin(-1)}\n"),
        "a {\
         \n  b: -0.8414709848;\
         \n}\n"
    );
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function sin($arg) {@return $arg}\
             \na {b: sin(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn rad() {
    assert_eq!(
        runner().ok("a {b: sin(1rad)}\n"),
        "a {\
         \n  b: 0.8414709848;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: sin(3px - 1px + var(--c));\
             \n}\n"),
        "a {\
         \n  b: sin(2px + var(--c));\
         \n}\n"
    );
}
#[test]
fn turn() {
    assert_eq!(
        runner().ok("a {b: sin(1turn)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        runner().ok("a {b: sin(0)}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
