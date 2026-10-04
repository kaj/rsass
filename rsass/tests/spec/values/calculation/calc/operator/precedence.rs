//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/operator/precedence.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("precedence")
}

mod interpolation {
    use super::runner;

    mod calculation {
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
    use super::runner;

    mod additive {
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
        mod degenerate {
            use super::runner;

            #[test]
            fn no_unit() {
                assert_eq!(
                    runner().ok("a {b: calc(var(--c) + infinity)}\n"),
                    "a {\
         \n  b: calc(var(--c) + infinity);\
         \n}\n"
                );
            }
            #[test]
            fn unit() {
                assert_eq!(
                    runner().ok("a {b: calc(var(--c) + infinity * 1%)}\n"),
                    "a {\
         \n  b: calc(var(--c) + infinity * 1%);\
         \n}\n"
                );
            }
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
        use super::runner;

        mod default {
            use super::runner;

            #[test]
            fn calculation() {
                assert_eq!(
                    runner().ok("a {b: calc(1px * calc(2 / var(--c)))}\n"),
                    "a {\
         \n  b: calc(1px * 2 / var(--c));\
         \n}\n"
                );
            }
            #[test]
            fn degenerate() {
                assert_eq!(
                    runner().ok("a {b: calc(var(--c) / infinity)}\n"),
                    "a {\
         \n  b: calc(var(--c) / infinity);\
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
            use super::runner;

            #[test]
            fn calculation() {
                assert_eq!(
                    runner().ok("a {b: calc(1px / calc(2 * var(--c)))}\n"),
                    "a {\
         \n  b: calc(1px / (2 * var(--c)));\
         \n}\n"
                );
            }
            #[test]
            fn degenerate() {
                assert_eq!(
                    runner().ok("a {b: calc(var(--c) / (infinity * 1px))}\n"),
                    "a {\
         \n  b: calc(var(--c) / (infinity * 1px));\
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
    use super::runner;

    #[test]
    fn additive() {
        assert_eq!(
            runner()
                .ok("a {b: calc(1px + 20px - 300px + 4000px - 50000px)}\n"),
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
