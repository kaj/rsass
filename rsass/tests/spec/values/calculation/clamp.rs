//! Tests auto-converted from "sass-spec/spec/values/calculation/clamp.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("clamp")
}

#[test]
fn case_insensitive() {
    assert_eq!(
        runner().ok("a {b: ClAmP(1px, 0px, 3px)}\n"),
        "a {\
         \n  b: 1px;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn complex_unit() {
        assert_eq!(
            runner().err("a {b: clamp(1px*1px, 2%*2%, 3px*3px)}\n"),
            "Error: Number 1px*px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: clamp(1px*1px, 2%*2%, 3px*3px)}\
         \n  |             ^^^^^^^\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
        );
    }
    mod known_incompatible {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn first() {
            assert_eq!(
                runner().err("a {b: clamp(1s, 2px, 3px)}\n"),
                "Error: 1s and 2px are incompatible.\
         \n  ,\
         \n1 | a {b: clamp(1s, 2px, 3px)}\
         \n  |             ^^ 1s\
         \n  |                 === 2px\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn second() {
            assert_eq!(
                runner().err("a {b: clamp(1px, 2s, 3px)}\n"),
                "Error: 1px and 2s are incompatible.\
         \n  ,\
         \n1 | a {b: clamp(1px, 2s, 3px)}\
         \n  |             ^^^ 1px\
         \n  |                  == 2s\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn third() {
            assert_eq!(
                runner().err("a {b: clamp(1px, 2px, 3s)}\n"),
                "Error: 1px and 3s are incompatible.\
         \n  ,\
         \n1 | a {b: clamp(1px, 2px, 3s)}\
         \n  |             ^^^ 1px\
         \n  |                       == 3s\
         \n  \'\
         \n  input.scss 1:13  root stylesheet",
            );
        }
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn four_args() {
            assert_eq!(
                runner().err("a {b: clamp(1px, 2px, 3px, 4px)}\n"),
                "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,\
         \n1 | a {b: clamp(1px, 2px, 3px, 4px)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: clamp(1px, $, 2px)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: clamp(1px, $, 2px)}\
         \n  |                   ^\
         \n  \'\
         \n  input.scss 1:19  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn no_args() {
            assert_eq!(
                runner().err("a {b: clamp()}\n"),
                "Error: Missing argument.\
         \n  ,\
         \n1 | a {b: clamp()}\
         \n  |       ^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn one_arg() {
            assert_eq!(
                runner().err("a {b: clamp(1px)}\n"),
                "Error: 3 arguments required, but only 1 was passed.\
         \n  ,\
         \n1 | a {b: clamp(1px)}\
         \n  |       ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn rest() {
            assert_eq!(
                runner().err("a {b: clamp(1px 2px 3px...)}\n"),
                "Error: Rest arguments can\'t be used with calculations.\
         \n  ,\
         \n1 | a {b: clamp(1px 2px 3px...)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn two_args() {
            assert_eq!(
                runner().err("a {b: clamp(1px, 2px)}\n"),
                "Error: 3 arguments required, but only 2 were passed.\
         \n  ,\
         \n1 | a {b: clamp(1px, 2px)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
}
mod preserved {
    #[allow(unused)]
    use super::runner;

    mod math {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("a {b: clamp(1% + 1px, 2px, 3px)}\n"),
                "a {\
         \n  b: clamp(1% + 1px, 2px, 3px);\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("a {b: clamp(1px, 1% + 2px, 3px)}\n"),
                "a {\
         \n  b: clamp(1px, 1% + 2px, 3px);\
         \n}\n"
            );
        }
        #[test]
        fn third() {
            assert_eq!(
                runner().ok("a {b: clamp(1px, 2px, 1% + 3px)}\n"),
                "a {\
         \n  b: clamp(1px, 2px, 1% + 3px);\
         \n}\n"
            );
        }
    }
    mod single_arg {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn interpolation() {
            assert_eq!(
                runner().ok("a {b: clamp(#{c})}\n"),
                "a {\
         \n  b: clamp(c);\
         \n}\n"
            );
        }
        #[test]
        fn unquoted_string() {
            assert_eq!(
                runner().ok("$a: b;\
             \nc {d: clamp($a)}\n"),
                "c {\
         \n  d: clamp(b);\
         \n}\n"
            );
        }
        #[test]
        fn var() {
            assert_eq!(
                runner().ok("a {b: clamp(var(--c))}\n"),
                "a {\
         \n  b: clamp(var(--c));\
         \n}\n"
            );
        }
    }
    mod unit {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("a {b: clamp(1%, 2px, 3px)}\n"),
                "a {\
         \n  b: clamp(1%, 2px, 3px);\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("a {b: clamp(1px, 2%, 3px)}\n"),
                "a {\
         \n  b: clamp(1px, 2%, 3px);\
         \n}\n"
            );
        }
        #[test]
        fn third() {
            assert_eq!(
                runner().ok("a {b: clamp(1px, 2px, 3%)}\n"),
                "a {\
         \n  b: clamp(1px, 2px, 3%);\
         \n}\n"
            );
        }
    }
}
mod simplified {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn between() {
        assert_eq!(
            runner().ok("a {b: clamp(1px, 2.5px, 3px)}\n"),
            "a {\
         \n  b: 2.5px;\
         \n}\n"
        );
    }
    #[test]
    fn compatible_units() {
        assert_eq!(
            runner().ok("a {b: clamp(1px, 1in, 1cm)}\n"),
            "a {\
         \n  b: 1cm;\
         \n}\n"
        );
    }
    mod lower_bound {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn far_below() {
            assert_eq!(
                runner().ok("a {b: clamp(1px, 0px, 3px)}\n"),
                "a {\
         \n  b: 1px;\
         \n}\n"
            );
        }
        #[test]
        fn fuzzy_equal() {
            assert_eq!(
                runner().ok("a {b: clamp(1px, 1.00000000001px, 3px)}\n"),
                "a {\
         \n  b: 1px;\
         \n}\n"
            );
        }
    }
    mod upper_bound {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn far_above() {
            assert_eq!(
                runner().ok("a {b: clamp(1px, 4px, 3px)}\n"),
                "a {\
         \n  b: 3px;\
         \n}\n"
            );
        }
        #[test]
        fn fuzzy_equal() {
            assert_eq!(
                runner().ok("a {b: clamp(1px, 2.99999999999px, 3px)}\n"),
                "a {\
         \n  b: 3px;\
         \n}\n"
            );
        }
    }
}
