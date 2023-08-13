//! Tests auto-converted from "sass-spec/spec/values/calculation/min.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("min")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong error
    fn complex_unit() {
        assert_eq!(
            runner().err("a {b: min(1px*1px, 2%*2%)}\n"),
            "Error: Number 1px*px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: min(1px*1px, 2%*2%)}\
         \n  |           ^^^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    mod known_incompatible {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn first() {
            assert_eq!(
                runner().err("a {b: min(1s, 2px)}\n"),
                "Error: 1s and 2px are incompatible.\
         \n  ,\
         \n1 | a {b: min(1s, 2px)}\
         \n  |           ^^ 1s\
         \n  |               === 2px\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn second() {
            assert_eq!(
                runner().err("a {b: min(1px, 2s)}\n"),
                "Error: 1px and 2s are incompatible.\
         \n  ,\
         \n1 | a {b: min(1px, 2s)}\
         \n  |           ^^^ 1px\
         \n  |                == 2s\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
        #[test]
        #[ignore] // wrong error
        fn third() {
            assert_eq!(
                runner().err("a {b: min(1px, 2px, 3s)}\n"),
                "Error: 1px and 3s are incompatible.\
         \n  ,\
         \n1 | a {b: min(1px, 2px, 3s)}\
         \n  |           ^^^ 1px\
         \n  |                     == 3s\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: min($)}\n"),
                "Error: Expected identifier.\
         \n  ,\
         \n1 | a {b: min($)}\
         \n  |            ^\
         \n  \'\
         \n  input.scss 1:12  root stylesheet",
            );
        }
        #[test]
        fn no_args() {
            assert_eq!(
                runner().err("a {b: min()}\n"),
                "Error: At least one argument must be passed.\
         \n  ,\
         \n1 | a {b: min()}\
         \n  |       ^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // wrong error
    fn unitless_after_potentially_incompatible() {
        assert_eq!(
            runner().err("a {b: min(1c, 2d, 3)}\n"),
            "Error: 1c and 3 are incompatible.\
         \n  ,\
         \n1 | a {b: min(1c, 2d, 3)}\
         \n  |           ^^ 1c\
         \n  |                   = 3\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    mod unitless_and_real {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn in_calc() {
            assert_eq!(
                runner().err("a {b: min(calc(1px + 2))}\n"),
                "Error: 1px and 2 are incompatible.\
         \n  ,\
         \n1 | a {b: min(calc(1px + 2))}\
         \n  |                ^^^^^^^\
         \n  \'\
         \n  input.scss 1:16  root stylesheet",
            );
        }
    }
}
mod extra_whitespace {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn max_in_min() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#1444\
             \na {b: min( max( 1px ) )}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            runner().ok("a {b: min( 1px )}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
}
mod math {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn slash_as_division() {
        assert_eq!(
            runner().ok("b { \
             \n  a: 2px / min(1.5);\
             \n}\n"),
            "b {\
         \n  a: 1.3333333333px;\
         \n}\n"
        );
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
                runner().ok("a {b: min(1% + 1px, 2px)}\n"),
                "a {\
         \n  b: min(1% + 1px, 2px);\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("a {b: min(1px, 1% + 2px)}\n"),
                "a {\
         \n  b: min(1px, 1% + 2px);\
         \n}\n"
            );
        }
        #[test]
        fn third() {
            assert_eq!(
                runner().ok("a {b: min(1px, 2px, 1% + 3px)}\n"),
                "a {\
         \n  b: min(1px, 2px, 1% + 3px);\
         \n}\n"
            );
        }
    }
    mod operation {
        #[allow(unused)]
        use super::runner;

        mod unitless_and_real {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn in_calc() {
                assert_eq!(
                    runner().ok("a {b: calc(min(1%, 2.5 + 0.9px))}\n"),
                    "a {\
         \n  b: min(1%, 3.4px);\
         \n}\n"
                );
            }
            #[test]
            fn minus() {
                assert_eq!(
                    runner().ok("a {b: min(1%, 2.5 - 0.9px)}\n"),
                    "a {\
         \n  b: min(1%, 1.6px);\
         \n}\n"
                );
            }
            #[test]
            fn plus() {
                assert_eq!(
                    runner().ok("a {b: min(1%, 2.5 + 0.9px)}\n"),
                    "a {\
         \n  b: min(1%, 3.4px);\
         \n}\n"
                );
            }
        }
    }
    mod unit {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn first() {
            assert_eq!(
                runner().ok("a {b: min(1%, 2px)}\n"),
                "a {\
         \n  b: min(1%, 2px);\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("a {b: min(1px, 2%)}\n"),
                "a {\
         \n  b: min(1px, 2%);\
         \n}\n"
            );
        }
        #[test]
        fn third() {
            assert_eq!(
                runner().ok("a {b: min(1px, 2px, 3%)}\n"),
                "a {\
         \n  b: min(1px, 2px, 3%);\
         \n}\n"
            );
        }
    }
    #[test]
    fn variable() {
        assert_eq!(
            runner().ok("$a: 1%;\
             \nb {c: min($a, 1px)}\n"),
            "b {\
         \n  c: min(1%, 1px);\
         \n}\n"
        );
    }
}
mod simplified {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn compatible_units() {
        assert_eq!(
            runner().ok("a {b: min(1px, 1in, 1cm)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            runner().ok("a {b: min(0px, 1px)}\n"),
            "a {\
         \n  b: 0px;\
         \n}\n"
        );
    }
    #[test]
    fn only() {
        assert_eq!(
            runner().ok("a {b: min(1px)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    mod operation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn unitless_and_real() {
            assert_eq!(
                runner().ok("a {b: min(1px, 2.5 + 0.9px)}\n"),
                "a {\
         \n  b: 1px;\
         \n}\n"
            );
        }
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("a {b: min(1px, 0.5px)}\n"),
            "a {\
         \n  b: 0.5px;\
         \n}\n"
        );
    }
    #[test]
    fn third() {
        assert_eq!(
            runner().ok("a {b: min(1px, 2.5px, 0.9px)}\n"),
            "a {\
         \n  b: 0.9px;\
         \n}\n"
        );
    }
    #[test]
    fn unitless_and_real() {
        assert_eq!(
            runner().ok("a {b: min(1px, 2.5, 0.9px)}\n"),
            "a {\
         \n  b: 0.9px;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless_between_potentially_incompatible() {
        assert_eq!(
            runner().ok("a {b: min(3d, 2, 1e)}\n"),
            "a {\
         \n  b: 1e;\
         \n}\n"
        );
    }
}
