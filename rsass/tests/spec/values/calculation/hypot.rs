//! Tests auto-converted from "sass-spec/spec/values/calculation/hypot.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hypot")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: hYpOt(1, 2)}\n"),
        "a {\
         \n  b: 2.2360679775;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn first_type() {
        assert_eq!(
            runner().err("a {b: hypot(\"0\", 1px, 1px)}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: hypot(\"0\", 1px, 1px)}\
         \n  |             ^^^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn sass_script() {
        assert_eq!(
            runner().err("a {b: hypot(7 % 3, 1)}\n"),
            "Error: This operation can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: hypot(7 % 3, 1)}\
         \n  |               ^\
         \n  \'\
         \n  input.scss 1:15  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn second_type() {
        assert_eq!(
            runner().err("a {b: hypot(1px, \"0\", 1px)}\n"),
            "Error: This expression can\'t be used in a calculation.\
         \n  ,\
         \n1 | a {b: hypot(1px, \"0\", 1px)}\
         \n  |                  ^^^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
        );
    }
    mod syntax {
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: hypot(12, $, 14)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: hypot(12, $, 14)}\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 1:18  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: hypot()}\n"),
            "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: hypot()}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod units {
        use super::runner;

        mod incompatible {
            use super::runner;

            #[test]
            #[ignore] // missing error
            fn first_and_second() {
                assert_eq!(
                    runner().err("a {b: hypot(1deg, 1px, 1turn)}\n"),
                    "Error: 1deg and 1px are incompatible.\
         \n  ,\
         \n1 | a {b: hypot(1deg, 1px, 1turn)}\
         \n  |             ^^^^ 1deg\
         \n  |                   === 1px\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
                );
            }
            #[test]
            #[ignore] // missing error
            fn first_and_third() {
                assert_eq!(
                    runner().err("a {b: hypot(1deg, 1turn, 1px)}\n"),
                    "Error: 1deg and 1px are incompatible.\
         \n  ,\
         \n1 | a {b: hypot(1deg, 1turn, 1px)}\
         \n  |             ^^^^ 1deg\
         \n  |                          === 1px\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
                );
            }
            #[test]
            #[ignore] // missing error
            fn second_and_third() {
                assert_eq!(
                    runner().err("a {b: hypot(1turn, 1deg, 1px)}\n"),
                    "Error: 1turn and 1px are incompatible.\
         \n  ,\
         \n1 | a {b: hypot(1turn, 1deg, 1px)}\
         \n  |             ^^^^^ 1turn\
         \n  |                          === 1px\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
                );
            }
        }
        #[test]
        #[ignore] // missing error
        fn real_and_unitless() {
            assert_eq!(
                runner().err("a {b: hypot(1px, 1)}\n"),
                "Error: 1px and 1 are incompatible.\
         \n  ,\
         \n1 | a {b: hypot(1px, 1)}\
         \n  |             ^^^ 1px\
         \n  |                  = 1\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn unsimplifiable() {
        assert_eq!(
        runner().err(
            "a {b: hypot(-7px / 4em)}\n"
        ),
        "Error: Number calc(-1.75px / 1em) isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: hypot(-7px / 4em)}\
         \n  |             ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
    );
    }
}
mod infinity {
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
            runner().ok("a {b: hypot(infinity, 1, 1)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("a {b: hypot(1, infinity, 1)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
}
#[test]
fn overridden() {
    assert_eq!(
        runner().ok("@function hypot($arg1, $arg2) {@return $arg1}\
             \na {b: hypot(1, 2)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn simplification() {
    assert_eq!(
        runner().ok("a {\
             \n  b: hypot(1px + 2px - var(--c), -7px + 4em)\
             \n}\n"),
        "a {\
         \n  b: hypot(3px - var(--c), -7px + 4em);\
         \n}\n"
    );
}
mod units {
    use super::runner;

    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("a {b: hypot(13cm, 4mm, 5q, 6in, 7px)}\n"),
            "a {\
         \n  b: 20.0366545892cm;\
         \n}\n"
        );
    }
    #[test]
    fn fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: hypot(1foo, 2bar);\
             \n}\n"),
            "a {\
         \n  b: hypot(1foo, 2bar);\
         \n}\n"
        );
    }
    #[test]
    fn none() {
        assert_eq!(
            runner().ok("a {b: hypot(3, 4, 5, 6, 7)}\n"),
            "a {\
         \n  b: 11.6189500386;\
         \n}\n"
        );
    }
    #[test]
    fn real_and_fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: hypot(1px, 2bar);\
             \n}\n"),
            "a {\
         \n  b: hypot(1px, 2bar);\
         \n}\n"
        );
    }
    #[test]
    fn real_and_unknown() {
        assert_eq!(
            runner().ok("a {b: hypot(13cm, 4%)}\n"),
            "a {\
         \n  b: hypot(13cm, 4%);\
         \n}\n"
        );
    }
    #[test]
    fn same_fake() {
        assert_eq!(
            runner().ok("a {\
             \n  b: hypot(1foo, 2foo);\
             \n}\n"),
            "a {\
         \n  b: 2.2360679775foo;\
         \n}\n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            runner().ok("a {\
             \n  b: hypot(1%, 2%);\
             \n}\n"),
            "a {\
         \n  b: hypot(1%, 2%);\
         \n}\n"
        );
    }
}
