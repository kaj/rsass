//! Tests auto-converted from "sass-spec/spec/values/calculation/max.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // missing error
    fn complex_unit() {
        assert_eq!(
            runner().err("a {b: max(1px*1px, 2%*2%)}\n"),
            "Error: Number 1px*px isn\'t compatible with CSS calculations.\
         \n  ,\
         \n1 | a {b: max(1px*1px, 2%*2%)}\
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
                runner().err("a {b: max(1s, 2px)}\n"),
                "Error: 1s and 2px are incompatible.\
         \n  ,\
         \n1 | a {b: max(1s, 2px)}\
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
                runner().err("a {b: max(1px, 2s)}\n"),
                "Error: 1px and 2s are incompatible.\
         \n  ,\
         \n1 | a {b: max(1px, 2s)}\
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
                runner().err("a {b: max(1px, 2px, 3s)}\n"),
                "Error: 1px and 3s are incompatible.\
         \n  ,\
         \n1 | a {b: max(1px, 2px, 3s)}\
         \n  |           ^^^ 1px\
         \n  |                     == 3s\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
            );
        }
    }
    #[test]
    #[ignore] // missing error
    fn potentially_incompatible_before_unitless() {
        assert_eq!(
            runner().err("a {b: max(1c, 2d, 3)}\n"),
            "Error: 1c and 3 are incompatible.\
         \n  ,\
         \n1 | a {b: max(1c, 2d, 3)}\
         \n  |           ^^ 1c\
         \n  |                   = 3\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
        );
    }
    mod syntax {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn invalid_arg() {
            assert_eq!(
                runner().err("a {b: max(c)}\n"),
                "Error: c is not a number.\
         \n  ,\
         \n1 | a {b: max(c)}\
         \n  |       ^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn no_args() {
            assert_eq!(
                runner().err("a {b: max()}\n"),
                "Error: At least one argument must be passed.\
         \n  ,\
         \n1 | a {b: max()}\
         \n  |       ^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    mod unitless_and_unit {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn in_calc() {
            assert_eq!(
                runner().err("a {b: max(calc(1px + 2))}\n"),
                "Error: 1px and 2 are incompatible.\
         \n  ,\
         \n1 | a {b: max(calc(1px + 2))}\
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
    fn min_in_max() {
        assert_eq!(
            runner().ok("// Regression test for sass/dart-sass#1444\
             \na {b: max( min( 1px ) )}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn number() {
        assert_eq!(
            runner().ok("a {b: max( 1px )}\n"),
            "a {\
         \n  b: 1px;\
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
                runner().ok("a {b: max(1% + 1px, 2px)}\n"),
                "a {\
         \n  b: max(1% + 1px, 2px);\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("a {b: max(1px, 1% + 2px)}\n"),
                "a {\
         \n  b: max(1px, 1% + 2px);\
         \n}\n"
            );
        }
        #[test]
        fn third() {
            assert_eq!(
                runner().ok("a {b: max(1px, 2px, 1% + 3px)}\n"),
                "a {\
         \n  b: max(1px, 2px, 1% + 3px);\
         \n}\n"
            );
        }
    }
    mod operation {
        #[allow(unused)]
        use super::runner;

        mod unitless_and_unit {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn in_calc() {
                assert_eq!(
                    runner().ok("a {b: calc(max(1%, 2.5 + 0.9px))}\n"),
                    "a {\
         \n  b: max(1%, 3.4px);\
         \n}\n"
                );
            }
            #[test]
            fn minus() {
                assert_eq!(
                    runner().ok("a {b: max(1%, 2.5 - 0.9px)}\n"),
                    "a {\
         \n  b: max(1%, 1.6px);\
         \n}\n"
                );
            }
            #[test]
            fn plus() {
                assert_eq!(
                    runner().ok("a {b: max(1%, 2.5 + 0.9px)}\n"),
                    "a {\
         \n  b: max(1%, 3.4px);\
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
                runner().ok("a {b: max(1%, 2px)}\n"),
                "a {\
         \n  b: max(1%, 2px);\
         \n}\n"
            );
        }
        #[test]
        fn second() {
            assert_eq!(
                runner().ok("a {b: max(1px, 2%)}\n"),
                "a {\
         \n  b: max(1px, 2%);\
         \n}\n"
            );
        }
        #[test]
        fn third() {
            assert_eq!(
                runner().ok("a {b: max(1px, 2px, 3%)}\n"),
                "a {\
         \n  b: max(1px, 2px, 3%);\
         \n}\n"
            );
        }
    }
    #[test]
    fn variable() {
        assert_eq!(
            runner().ok("$a: 1%;\
             \nb {c: max($a, 1px)}\n"),
            "b {\
         \n  c: max(1%, 1px);\
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
            runner().ok("a {b: max(1px, 1in, 1cm)}\n"),
            "a {\
         \n  b: 1in;\
         \n}\n"
        );
    }
    #[test]
    fn first() {
        assert_eq!(
            runner().ok("a {b: max(1px, 0px)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn only() {
        assert_eq!(
            runner().ok("a {b: max(1px)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    mod operation {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn unitless_and_unit() {
            assert_eq!(
                runner().ok("a {b: max(1px, 2.5 + 0.9px)}\n"),
                "a {\
         \n  b: 3.4px;\
         \n}\n"
            );
        }
    }
    #[test]
    fn second() {
        assert_eq!(
            runner().ok("a {b: max(0.5px, 2px)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
    #[test]
    fn third() {
        assert_eq!(
            runner().ok("a {b: max(1px, 2.5px, 2.9px)}\n"),
            "a {\
         \n  b: 2.9px;\
         \n}\n"
        );
    }
    #[test]
    fn unitless_and_unit() {
        assert_eq!(
            runner().ok("a {b: max(1px, 2.5, 0.9px)}\n"),
            "a {\
         \n  b: 2.5;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn unitless_between_potentially_incompatible() {
        assert_eq!(
            runner().ok("a {b: max(1d, 2, 3e)}\n"),
            "a {\
         \n  b: 3e;\
         \n}\n"
        );
    }
}
