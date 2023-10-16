//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/operator.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("operator")
}

mod divide {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn no_whitespace() {
        assert_eq!(
            runner().ok("a {b: calc(1px/2)}\n"),
            "a {\
         \n  b: 0.5px;\
         \n}\n"
        );
    }
    #[test]
    fn preserved() {
        assert_eq!(
            runner().ok("a {b: calc(1px / var(--c))}\n"),
            "a {\
         \n  b: calc(1px / var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn simplified() {
        assert_eq!(
            runner().ok("a {b: calc(1px / 2)}\n"),
            "a {\
         \n  b: 0.5px;\
         \n}\n"
        );
    }
}
mod minus {
    #[allow(unused)]
    use super::runner;

    mod preserved {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn division() {
            assert_eq!(
                runner().ok("a {b: calc(1px - (2% / var(--c)))}\n"),
                "a {\
         \n  b: calc(1px - 2% / var(--c));\
         \n}\n"
            );
        }
        #[test]
        fn minus() {
            assert_eq!(
                runner().ok("a {b: calc(1px - (2% - var(--c)))}\n"),
                "a {\
         \n  b: calc(1px - (2% - var(--c)));\
         \n}\n"
            );
        }
        #[test]
        fn multiplication() {
            assert_eq!(
                runner().ok("a {b: calc(1px - (2% * var(--c)))}\n"),
                "a {\
         \n  b: calc(1px - 2% * var(--c));\
         \n}\n"
            );
        }
        #[test]
        fn number() {
            assert_eq!(
                runner().ok("a {b: calc(1px - 2%)}\n"),
                "a {\
         \n  b: calc(1px - 2%);\
         \n}\n"
            );
        }
        #[test]
        fn plus() {
            assert_eq!(
                runner().ok("a {b: calc(1px - (2% + var(--c)))}\n"),
                "a {\
         \n  b: calc(1px - (2% + var(--c)));\
         \n}\n"
            );
        }
    }
    #[test]
    fn simplified() {
        assert_eq!(
            runner().ok("a {b: calc(1px - 2px)}\n"),
            "a {\
         \n  b: -1px;\
         \n}\n"
        );
    }
}
mod plus {
    #[allow(unused)]
    use super::runner;

    mod preserved {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn division() {
            assert_eq!(
                runner().ok("a {b: calc(1px + (2% / var(--c)))}\n"),
                "a {\
         \n  b: calc(1px + 2% / var(--c));\
         \n}\n"
            );
        }
        #[test]
        fn minus() {
            assert_eq!(
                runner().ok("a {b: calc(1px + 2% - var(--c))}\n"),
                "a {\
         \n  b: calc(1px + 2% - var(--c));\
         \n}\n"
            );
        }
        #[test]
        fn multiplication() {
            assert_eq!(
                runner().ok("a {b: calc(1px + (2% * var(--c)))}\n"),
                "a {\
         \n  b: calc(1px + 2% * var(--c));\
         \n}\n"
            );
        }
        #[test]
        fn number() {
            assert_eq!(
                runner().ok("a {b: calc(1px + 2%)}\n"),
                "a {\
         \n  b: calc(1px + 2%);\
         \n}\n"
            );
        }
        #[test]
        fn plus() {
            assert_eq!(
                runner().ok("a {b: calc(1px + (2% + var(--c)))}\n"),
                "a {\
         \n  b: calc(1px + 2% + var(--c));\
         \n}\n"
            );
        }
    }
    #[test]
    fn simplified() {
        assert_eq!(
            runner().ok("a {b: calc(1px + 2px)}\n"),
            "a {\
         \n  b: 3px;\
         \n}\n"
        );
    }
}
mod precedence {
    #[allow(unused)]
    use super::runner;

    mod interpolation {
        #[allow(unused)]
        use super::runner;

        mod calculation {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn asterisk() {
                assert_eq!(
                    runner().ok("a {b: calc(calc(#{\"c*\"}))}\n"),
                    "a {\
         \n  b: calc((c*));\
         \n}\n"
                );
            }
            #[test]
            fn plain() {
                assert_eq!(
                    runner().ok("a {b: calc(calc(#{c}))}\n"),
                    "a {\
         \n  b: calc(c);\
         \n}\n"
                );
            }
            #[test]
            fn slash() {
                assert_eq!(
                    runner().ok("a {b: calc(calc(#{\"c/\"}))}\n"),
                    "a {\
         \n  b: calc((c/));\
         \n}\n"
                );
            }
            #[test]
            fn whitespace() {
                assert_eq!(
                    runner().ok("a {b: calc(calc(#{\"c \"}))}\n"),
                    "a {\
         \n  b: calc((c ));\
         \n}\n"
                );
            }
        }
        #[test]
        fn parens() {
            assert_eq!(
                runner().ok("a {b: calc((#{c}))}\n"),
                "a {\
         \n  b: calc((c));\
         \n}\n"
            );
        }
    }
    mod preserved {
        #[allow(unused)]
        use super::runner;

        mod additive {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn calculation() {
                assert_eq!(
                    runner().ok("a {b: calc(1px + calc(2% - 3em))}\n"),
                    "a {\
         \n  b: calc(1px + 2% - 3em);\
         \n}\n"
                );
            }
            #[test]
            fn parens() {
                assert_eq!(
                    runner().ok("a {b: calc(1px + (2% - 3em))}\n"),
                    "a {\
         \n  b: calc(1px + 2% - 3em);\
         \n}\n"
                );
            }
        }
        mod additive_then_multiplicative {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn calculation() {
                assert_eq!(
                    runner().ok("a {b: calc(1px + calc(2px * var(--c)))}\n"),
                    "a {\
         \n  b: calc(1px + 2px * var(--c));\
         \n}\n"
                );
            }
            #[test]
            fn parens() {
                assert_eq!(
                    runner().ok("a {b: calc(1px + (2px * var(--c)))}\n"),
                    "a {\
         \n  b: calc(1px + 2px * var(--c));\
         \n}\n"
                );
            }
        }
        mod multiplicative {
            #[allow(unused)]
            use super::runner;

            mod default {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn calculation() {
                    assert_eq!(
                        runner()
                            .ok("a {b: calc(1px * calc(2 / var(--c)))}\n"),
                        "a {\
         \n  b: calc(1px * 2 / var(--c));\
         \n}\n"
                    );
                }
                #[test]
                fn parens() {
                    assert_eq!(
                        runner().ok("a {b: calc(1px * (2 / var(--c)))}\n"),
                        "a {\
         \n  b: calc(1px * 2 / var(--c));\
         \n}\n"
                    );
                }
            }
            mod needs_parens {
                #[allow(unused)]
                use super::runner;

                #[test]
                fn calculation() {
                    assert_eq!(
                        runner()
                            .ok("a {b: calc(1px / calc(2 * var(--c)))}\n"),
                        "a {\
         \n  b: calc(1px / (2 * var(--c)));\
         \n}\n"
                    );
                }
                #[test]
                fn parens() {
                    assert_eq!(
                        runner().ok("a {b: calc(1px / (2 * var(--c)))}\n"),
                        "a {\
         \n  b: calc(1px / (2 * var(--c)));\
         \n}\n"
                    );
                }
            }
        }
        mod multiplicative_then_additive {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn calculation() {
                assert_eq!(
                    runner().ok("a {b: calc(1px * calc(2 + var(--c)))}\n"),
                    "a {\
         \n  b: calc(1px * (2 + var(--c)));\
         \n}\n"
                );
            }
            #[test]
            fn parens() {
                assert_eq!(
                    runner().ok("a {b: calc(1px * (2 + var(--c)))}\n"),
                    "a {\
         \n  b: calc(1px * (2 + var(--c)));\
         \n}\n"
                );
            }
        }
    }
    mod simplified {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn additive() {
            assert_eq!(
                runner().ok(
                    "a {b: calc(1px + 20px - 300px + 4000px - 50000px)}\n"
                ),
                "a {\
         \n  b: -46279px;\
         \n}\n"
            );
        }
        #[test]
        fn multiplicative() {
            assert_eq!(
                runner().ok("a {b: calc(2 * 3 / 5 * 7 / 11)}\n"),
                "a {\
         \n  b: 0.7636363636;\
         \n}\n"
            );
        }
        #[test]
        fn multiplicative_and_additive() {
            assert_eq!(
                runner().ok("a {b: calc(2 * 3 + 4 / 5 - 6)}\n"),
                "a {\
         \n  b: 0.8;\
         \n}\n"
            );
        }
        mod parens {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn multiplicative() {
                assert_eq!(
                    runner().ok("a {b: calc(1 / (2 * 3))}\n"),
                    "a {\
         \n  b: 0.1666666667;\
         \n}\n"
                );
            }
            #[test]
            fn multiplicative_and_additive() {
                assert_eq!(
                    runner().ok("a {b: calc(2 * (3 + 4) / (5 - 6))}\n"),
                    "a {\
         \n  b: -14;\
         \n}\n"
                );
            }
        }
    }
}
mod sass_script {
    #[allow(unused)]
    use super::runner;

    mod plus_string {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn lhs() {
            assert_eq!(
                runner().ok("a {b: calc(1px + 1%) + \"\"}\n"),
                "a {\
         \n  b: \"calc(1px + 1%)\";\
         \n}\n"
            );
        }
        #[test]
        fn rhs() {
            assert_eq!(
                runner().ok("a {b: \"\" + calc(1px + 1%)}\n"),
                "a {\
         \n  b: \"calc(1px + 1%)\";\
         \n}\n"
            );
        }
    }
}
mod times {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn no_whitespace() {
        assert_eq!(
            runner().ok("a {b: calc(1px*2)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
    #[test]
    fn preserved() {
        assert_eq!(
            runner().ok("a {b: calc(1px * var(--c))}\n"),
            "a {\
         \n  b: calc(1px * var(--c));\
         \n}\n"
        );
    }
    #[test]
    fn simplified() {
        assert_eq!(
            runner().ok("a {b: calc(1px * 2)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
}
mod units {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn denominators() {
        assert_eq!(
            runner().ok("a {b: calc(1/2px + 1/4px) * 1px}\n"),
            "a {\
         \n  b: 0.75;\
         \n}\n"
        );
    }
    #[test]
    fn division() {
        assert_eq!(
            runner().ok("a {b: calc(1px / 2px)}\n"),
            "a {\
         \n  b: 0.5;\
         \n}\n"
        );
    }
    #[test]
    fn multiplication() {
        assert_eq!(
            runner().ok("@use \"sass:math\";\
             \na {b: math.div(calc(2px * 3px), 4px)}\n"),
            "a {\
         \n  b: 1.5px;\
         \n}\n"
        );
    }
    mod percent {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn and_known() {
            assert_eq!(
                runner().ok("a {b: calc(1% + 1px)}\n"),
                "a {\
         \n  b: calc(1% + 1px);\
         \n}\n"
            );
        }
        #[test]
        fn and_unknown() {
            assert_eq!(
                runner().ok("a {b: calc(1% + 1unknown)}\n"),
                "a {\
         \n  b: calc(1% + 1unknown);\
         \n}\n"
            );
        }
    }
    mod unknown {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn and_known() {
            assert_eq!(
                runner().ok("a {b: calc(1unknown + 1px)}\n"),
                "a {\
         \n  b: calc(1unknown + 1px);\
         \n}\n"
            );
        }
        #[test]
        fn and_unknown() {
            assert_eq!(
                runner().ok("a {b: calc(1unknown + 1other)}\n"),
                "a {\
         \n  b: calc(1unknown + 1other);\
         \n}\n"
            );
        }
    }
}
mod var {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn calculation() {
        assert_eq!(
            runner().ok("a {b: calc(1 + calc(var(--c)))}\n"),
            "a {\
         \n  b: calc(1 + (var(--c)));\
         \n}\n"
        );
    }
    #[test]
    fn directly_parenthesized() {
        assert_eq!(
            runner().ok("a {b: calc(1 + (var(--c)))}\n"),
            "a {\
         \n  b: calc(1 + (var(--c)));\
         \n}\n"
        );
    }
    #[test]
    fn indirectly_parenthesized() {
        assert_eq!(
            runner().ok("a {b: calc((1 + var(--c)))}\n"),
            "a {\
         \n  b: calc(1 + var(--c));\
         \n}\n"
        );
    }
}
