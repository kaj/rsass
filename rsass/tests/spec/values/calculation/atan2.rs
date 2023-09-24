//! Tests auto-converted from "sass-spec/spec/values/calculation/atan2.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("atan2")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: aTaN2(1, -10)}\n"),
        "a {\
         \n  b: 174.2894068625deg;\
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
            runner().err("a {b: atan2(7 % 3, 1)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: atan2(7 % 3, 1)}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
        );
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: atan2(1, $)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: atan2(1, $)}\
         \n  |                 ^\
         \n  \'\
         \n  input.scss 1:17  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: atan2(0)}\n"),
            "Error: 2 arguments required, but only 1 was passed.\
         \n  ,\
         \n1 | a {b: atan2(0)}\
         \n  |       ^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: atan2(0, 0, 0)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,\
         \n1 | a {b: atan2(0, 0, 0)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod units {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn complex_and_unknown() {
            assert_eq!(
        runner().err(
            "a {b: atan2(1px*2px, 10%)}\n"
        ),
        "Error: Number 2px*px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: atan2(1px*2px, 10%)}\
         \n  |             ^^^^^^^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
        }
        #[test]
        #[ignore] // missing error
        fn known_incompatible() {
            assert_eq!(
                runner().err("a {b: atan2(1deg, 1px)}\n"),
                "Error: 1deg and 1px are incompatible.\
         \n  ,\
         \n1 | a {b: atan2(1deg, 1px)}\
         \n  |             ^^^^ 1deg\
         \n  |                   === 1px\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn unitless_and_real() {
            assert_eq!(
                runner().err("a {b: atan2(1, 1px)}\n"),
                "Error: 1 and 1px are incompatible.\
         \n  ,\
         \n1 | a {b: atan2(1, 1px)}\
         \n  |             ^ 1\
         \n  |                === 1px\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn x_type() {
        assert_eq!(
            runner().err("a {b: atan2(0, \"0\")}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: atan2(0, \"0\")}\
         \n  |                ^^^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn y_type() {
        assert_eq!(
            runner().err("a {b: atan2(\"0\", 0)}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: atan2(\"0\", 0)}\
         \n  |             ^^^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function atan2($arg) {@return $arg}\
             \na {b: atan2(1)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: atan2(3px - 1px + var(--c), -7px / 4em * 1em);\
             \n}\n"),
        "a {\
         \n  b: atan2(2px + var(--c), -1.75px);\
         \n}\n"
    );
}
mod units {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("a {b: atan2(1cm, -10mm)}\n"),
            "a {\
         \n  b: 135deg;\
         \n}\n"
        );
    }
    #[test]
    fn fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: atan2(1foo, 2bar);\
             \n}\n"),
            "a {\
         \n  b: atan2(1foo, 2bar);\
         \n}\n"
        );
    }
    #[test]
    fn none() {
        assert_eq!(
            runner().ok("a {b: atan2(1, -10)}\n"),
            "a {\
         \n  b: 174.2894068625deg;\
         \n}\n"
        );
    }
    #[test]
    fn real_and_fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: atan2(1px, 2bar);\
             \n}\n"),
            "a {\
         \n  b: atan2(1px, 2bar);\
         \n}\n"
        );
    }
    #[test]
    fn real_and_unknown() {
        assert_eq!(
            runner().ok("a {b: atan2(1px, 10%)}\n"),
            "a {\
         \n  b: atan2(1px, 10%);\
         \n}\n"
        );
    }
    #[test]
    fn same_fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: atan2(1foo, 2foo);\
             \n}\n"),
            "a {\
         \n  b: 26.5650511771deg;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {\
             \n  b: atan2(1%, 2%);\
             \n}\n"),
            "a {\
         \n  b: atan2(1%, 2%);\
         \n}\n"
        );
    }
}
