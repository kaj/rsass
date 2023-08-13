//! Tests auto-converted from "sass-spec/spec/values/calculation/pow.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("pow")
}

mod base {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn negative() {
        assert_eq!(
            runner().ok("a {b: pow(-10, 10)}\n"),
            "a {\
         \n  b: 10000000000;\
         \n}\n"
        );
    }
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn base_type() {
        assert_eq!(
            runner().err("a {b: pow(0, \"0\")}\n"),
            "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: pow(0, \"0\")}\
         \n  |              ^\
         \n  \'\
         \n  input.scss 1:14  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn exponent_type() {
        assert_eq!(
            runner().err("a {b: pow(\"0\", 0)}\n"),
            "Error: Expected number, variable, function, or calculation.\
         \n  ,\
         \n1 | a {b: pow(\"0\", 0)}\
         \n  |           ^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: pow(7 % 3, 1)}\n"),
            "Error: expected \"+\", \"-\", \"*\", \"/\", \",\", or \")\".\
         \n  ,\
         \n1 | a {b: pow(7 % 3, 1)}\
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
                runner().err("a {b: pow(10, $)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: pow(10, $)}\
         \n  |                ^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: pow(3)}\n"),
            "Error: 2 arguments required, but only 1 was passed.\
         \n  ,\
         \n1 | a {b: pow(3)}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: pow(3, 2, 1)}\n"),
            "Error: expected \"+\", \"-\", \"*\", \"/\", or \")\".\
         \n  ,\
         \n1 | a {b: pow(3, 2, 1)}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
        );
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn compatible() {
            assert_eq!(
                runner().err("a {b: pow(10px, 10px)}\n"),
                "Error: Expected 10px to have no units.\
         \n  ,\
         \n1 | a {b: pow(10px, 10px)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn real_and_unitless() {
            assert_eq!(
                runner().err("a {b: pow(10px, 10)}\n"),
                "Error: Expected 10px to have no units.\
         \n  ,\
         \n1 | a {b: pow(10px, 10)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn unknown_and_unitless() {
            assert_eq!(
                runner().err("a {b: pow(10%, 10)}\n"),
                "Error: Expected 10% to have no units.\
         \n  ,\
         \n1 | a {b: pow(10%, 10)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod exponent {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn negative() {
        assert_eq!(
            runner().ok("a {b: pow(10, -10)}\n"),
            "a {\
         \n  b: 0.0000000001;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn positive() {
    assert_eq!(
        runner().ok("a {b: pow(10, 10)}\n"),
        "a {\
         \n  b: 10000000000;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {b: pow(3px - 1px + var(--c), 4px + 10px)}\n"),
        "a {\
         \n  b: pow(2px + var(--c), 14px);\
         \n}\n"
    );
}
mod x_infinity {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn positive() {
        assert_eq!(
            runner().ok("a {b: pow(10, infinity)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
}
mod y_infinity {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn positive() {
        assert_eq!(
            runner().ok("a {b: pow(infinity, 10)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn zeros() {
    assert_eq!(
        runner().ok("a {b: pow(0, 0)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
