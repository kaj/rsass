//! Tests auto-converted from "sass-spec/spec/directives/function/name.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("name")
}

mod custom_ident {
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn call() {
        assert_eq!(
            runner().ok("@function __a() {@return 1}\
             \nb {c: --a()}\n"),
            "b {\
         \n  c: --a();\
         \n}\n"
        );
    }
}
#[test]
fn double_underscore() {
    assert_eq!(
        runner().ok("@function __a() {@return 1}\
             \nb {c: __a()}\n"),
        "b {\
         \n  c: 1;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    mod special {
        use super::runner;

        mod and {
            use super::runner;

            #[test]
            #[ignore] // missing error
            fn lowercase() {
                assert_eq!(
                    runner().err("@function and() {@return 1}\n"),
                    "Error: Invalid function name.\
         \n  ,\
         \n1 | @function and() {@return 1}\
         \n  |           ^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                );
            }
        }
        mod element {
            use super::runner;

            mod no_prefix {
                use super::runner;

                #[test]
                #[ignore] // wrong error
                fn lowercase() {
                    assert_eq!(
                        runner().err("@function element() {@return 1}\n"),
                        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function element() {@return 1}\
         \n  |           ^^^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                    );
                }
            }
            mod prefix {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn lowercase() {
                    assert_eq!(
                        runner().err("@function -a-element() {@return 1}\n"),
                        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function -a-element() {@return 1}\
         \n  |           ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                    );
                }
            }
        }
        mod expression {
            use super::runner;

            mod no_prefix {
                use super::runner;

                #[test]
                #[ignore] // wrong error
                fn lowercase() {
                    assert_eq!(
                        runner().err("@function expression() {@return 1}\n"),
                        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function expression() {@return 1}\
         \n  |           ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                    );
                }
            }
        }
        mod not {
            use super::runner;

            #[test]
            #[ignore] // missing error
            fn lowercase() {
                assert_eq!(
                    runner().err("@function not() {@return 1}\n"),
                    "Error: Invalid function name.\
         \n  ,\
         \n1 | @function not() {@return 1}\
         \n  |           ^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                );
            }
        }
        mod or {
            use super::runner;

            #[test]
            #[ignore] // missing error
            fn lowercase() {
                assert_eq!(
                    runner().err("@function or() {@return 1}\n"),
                    "Error: Invalid function name.\
         \n  ,\
         \n1 | @function or() {@return 1}\
         \n  |           ^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                );
            }
        }
        mod test_type {
            use super::runner;

            mod no_prefix {
                use super::runner;

                #[test]
                #[ignore] // missing error
                fn lowercase() {
                    assert_eq!(
                        runner().err(
                            "@function type() {@return 1}\
             \na {b: type()}\n"
                        ),
                        "Error: This name is reserved for the plain-CSS function.\
         \n  ,\
         \n1 | @function type() {@return 1}\
         \n  |           ^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                    );
                }
                #[test]
                #[ignore] // missing error
                fn uppercase() {
                    assert_eq!(
                        runner().err(
                            "@function TYPE() {@return 1}\
             \na {b: TYPE()}\n"
                        ),
                        "Error: This name is reserved for the plain-CSS function.\
         \n  ,\
         \n1 | @function TYPE() {@return 1}\
         \n  |           ^^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                    );
                }
            }
        }
        mod url {
            use super::runner;

            mod no_prefix {
                use super::runner;

                #[test]
                #[ignore] // wrong error
                fn lowercase() {
                    assert_eq!(
                        runner().err("@function url() {@return 1}\n"),
                        "Error: Invalid function name.\
         \n  ,\
         \n1 | @function url() {@return 1}\
         \n  |           ^^^\
         \n  \'\
         \n  input.scss 1:11  root stylesheet",
                    );
                }
            }
        }
    }
}
mod special {
    use super::runner;

    mod and {
        use super::runner;

        #[test]
        fn prefix() {
            assert_eq!(
                runner().ok("@function -a-and() {@return 1}\
             \nb {c: -a-and()}\n"),
                "b {\
         \n  c: 1;\
         \n}\n"
            );
        }
        #[test]
        fn uppercase() {
            assert_eq!(
                runner().ok("@function AND() {@return 1}\
             \na {b: AND()}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn calc() {
        assert_eq!(
            runner().ok("@function calc() {@return 1}\
             \na {b: calc()}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn clamp() {
        assert_eq!(
            runner().ok("@function clamp() {@return 1}\
             \na {b: clamp()}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    mod element {
        use super::runner;

        mod no_prefix {
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn uppercase() {
                assert_eq!(
                    runner().ok("@function ELEMENT() {@return 1}\
             \na {\
             \n  b: ELEMENT();\
             \n}\n"),
                    "a {\
         \n  b: element();\
         \n}\n"
                );
            }
        }
        mod prefix {
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn uppercase() {
                assert_eq!(
                    runner().ok("@function -A-ELEMENT() {@return 1}\
             \na {\
             \n  b: -A-ELEMENT();\
             \n}\n"),
                    "a {\
         \n  b: -a-element();\
         \n}\n"
                );
            }
        }
    }
    mod expression {
        use super::runner;

        #[test]
        fn prefix() {
            assert_eq!(
                runner().ok("@function -a-expression() {@return 1}\
             \nb {\
             \n  c: -a-expression();\
             \n}\n"),
                "b {\
         \n  c: -a-expression();\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn uppercase() {
            assert_eq!(
                runner().ok("@function EXPRESSION() {@return 1}\
             \na {\
             \n  b: EXPRESSION();\
             \n}\n"),
                "a {\
         \n  b: expression();\
         \n}\n"
            );
        }
    }
    mod not {
        use super::runner;

        #[test]
        fn prefix() {
            assert_eq!(
                runner().ok("@function -a-not() {@return 1}\
             \nb {c: -a-not()}\n"),
                "b {\
         \n  c: 1;\
         \n}\n"
            );
        }
        #[test]
        fn uppercase() {
            assert_eq!(
                runner().ok("@function NOT() {@return 1}\
             \na {b: NOT()}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
    }
    mod or {
        use super::runner;

        #[test]
        fn prefix() {
            assert_eq!(
                runner().ok("@function -a-or() {@return 1}\
             \nb {c: -a-or()}\n"),
                "b {\
         \n  c: 1;\
         \n}\n"
            );
        }
        #[test]
        fn uppercase() {
            assert_eq!(
                runner().ok("@function OR() {@return 1}\
             \na {b: OR()}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
    }
    mod test_type {
        use super::runner;

        #[test]
        fn prefix() {
            assert_eq!(
                runner().ok("@function -a-type() {@return 1}\
             \nb {c: -a-type()}\n"),
                "b {\
         \n  c: 1;\
         \n}\n"
            );
        }
    }
    mod url {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn prefix() {
            assert_eq!(
                runner().ok("@function -a-url() {@return 1}\
             \nb {\
             \n  c: -a-url();\
             \n}\n"),
                "b {\
         \n  c: url();\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn uppercase() {
            assert_eq!(
                runner().ok("@function URL() {@return 1}\
             \na {\
             \n  b: URL();\
             \n}\n"),
                "a {\
         \n  b: url();\
         \n}\n"
            );
        }
    }
}
mod vendor_like_underscore {
    use super::runner;

    #[test]
    fn middle() {
        assert_eq!(
            runner().ok("@function -moz_element() {@return 1}\
             \nb {c: -moz_element()}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
    #[test]
    fn start() {
        assert_eq!(
            runner().ok("@function _moz-element() {@return 1}\
             \nb {c: _moz-element()}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
}
