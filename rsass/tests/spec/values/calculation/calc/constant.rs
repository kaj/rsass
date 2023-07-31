//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/constant.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("constant")
}

mod e {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a {b: calc(e)}\n"),
            "a {\
         \n  b: 2.7182818285;\
         \n}\n"
        );
    }
    #[test]
    fn case_insensitive() {
        assert_eq!(
            runner().ok("a {b: calc(E)}\n"),
            "a {\
         \n  b: 2.7182818285;\
         \n}\n"
        );
    }
    #[test]
    fn equals_max_precision() {
        assert_eq!(
        runner().ok(
            "// Verify that e is equal to the maximum precision representable in a double.\
             \na {b: calc(e) == 2.718281828459045}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    mod math {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(e * 2)}\n"),
                "a {\
         \n  b: 5.4365636569;\
         \n}\n"
            );
        }
        #[test]
        fn unsimplified() {
            assert_eq!(
                runner().ok("a {b: calc(e * (1% + 1px))}\n"),
                "a {\
         \n  b: calc(2.7182818285 * (1% + 1px));\
         \n}\n"
            );
        }
    }
}
mod infinity {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a {b: calc(infinity)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
    #[test]
    fn case_insensitive() {
        assert_eq!(
            runner().ok("a {b: calc(InFiNiTy)}\n"),
            "a {\
         \n  b: calc(infinity);\
         \n}\n"
        );
    }
    mod math {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(infinity * 2)}\n"),
                "a {\
         \n  b: calc(infinity);\
         \n}\n"
            );
        }
        mod unsimplified {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn computed() {
                assert_eq!(
                    runner().ok("a {b: calc((1/0) * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(infinity * (1% + 1px));\
         \n}\n"
                );
            }
            #[test]
            fn from_variable() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \n$var: math.div(1, 0);\
             \na {b: calc($var * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(infinity * (1% + 1px));\
         \n}\n"
                );
            }
            #[test]
            fn literal() {
                assert_eq!(
                    runner().ok("a {b: calc(infinity * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(infinity * (1% + 1px));\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().ok("@use \'sass:meta\';\
             \na {b: meta.type-of(calc(infinity))}\n"),
            "a {\
         \n  b: number;\
         \n}\n"
        );
    }
}
mod minus_infinity {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a {b: calc(-infinity)}\n"),
            "a {\
         \n  b: calc(-infinity);\
         \n}\n"
        );
    }
    #[test]
    fn case_insensitive() {
        assert_eq!(
            runner().ok("a {b: calc(-iNfInItY)}\n"),
            "a {\
         \n  b: calc(-infinity);\
         \n}\n"
        );
    }
    mod math {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(-infinity * 2)}\n"),
                "a {\
         \n  b: calc(-infinity);\
         \n}\n"
            );
        }
        mod unsimplified {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn computed() {
                assert_eq!(
                    runner().ok("a {b: calc((-1/0) * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(-infinity * (1% + 1px));\
         \n}\n"
                );
            }
            #[test]
            fn from_variable() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \n$var: math.div(-1, 0);\
             \na {b: calc($var * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(-infinity * (1% + 1px));\
         \n}\n"
                );
            }
            #[test]
            fn literal() {
                assert_eq!(
                    runner().ok("a {b: calc(-infinity * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(-infinity * (1% + 1px));\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().ok("@use \'sass:meta\';\
             \na {b: meta.type-of(calc(-infinity))}\n"),
            "a {\
         \n  b: number;\
         \n}\n"
        );
    }
}
mod nan {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a {b: calc(NaN)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    #[test]
    fn case_insensitive() {
        assert_eq!(
            runner().ok("a {b: calc(nan)}\n"),
            "a {\
         \n  b: calc(NaN);\
         \n}\n"
        );
    }
    mod math {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(NaN * 2)}\n"),
                "a {\
         \n  b: calc(NaN);\
         \n}\n"
            );
        }
        mod unsimplified {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn computed() {
                assert_eq!(
                    runner().ok("a {b: calc((0/0) * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(NaN * (1% + 1px));\
         \n}\n"
                );
            }
            #[test]
            fn from_variable() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \n$var: math.div(0, 0);\
             \na {b: calc($var * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(NaN * (1% + 1px));\
         \n}\n"
                );
            }
            #[test]
            fn literal() {
                assert_eq!(
                    runner().ok("a {b: calc(NaN * (1% + 1px))}\n"),
                    "a {\
         \n  b: calc(NaN * (1% + 1px));\
         \n}\n"
                );
            }
        }
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().ok("@use \'sass:meta\';\
             \na {b: meta.type-of(calc(NaN))}\n"),
            "a {\
         \n  b: number;\
         \n}\n"
        );
    }
}
mod pi {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a {b: calc(pi)}\n"),
            "a {\
         \n  b: 3.1415926536;\
         \n}\n"
        );
    }
    #[test]
    fn case_insensitive() {
        assert_eq!(
            runner().ok("a {b: calc(pI)}\n"),
            "a {\
         \n  b: 3.1415926536;\
         \n}\n"
        );
    }
    #[test]
    fn equals_max_precision() {
        assert_eq!(
        runner().ok(
            "// Verify that pi is equal to the maximum precision representable in a double.\
             \na {b: calc(pi) == 3.141592653589793}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    mod math {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn simplified() {
            assert_eq!(
                runner().ok("a {b: calc(pi * 2)}\n"),
                "a {\
         \n  b: 6.2831853072;\
         \n}\n"
            );
        }
        #[test]
        fn unsimplified() {
            assert_eq!(
                runner().ok("a {b: calc(pi * (1% + 1px))}\n"),
                "a {\
         \n  b: calc(3.1415926536 * (1% + 1px));\
         \n}\n"
            );
        }
    }
}
mod precedence {
    #[allow(unused)]
    use super::runner;

    mod after_divide {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn no_unit() {
            assert_eq!(
                runner().ok("a {b: calc(var(--c) / infinity)}\n"),
                "a {\
         \n  b: calc(var(--c) / infinity);\
         \n}\n"
            );
        }
        #[test]
        fn unit() {
            assert_eq!(
                runner().ok("a {b: calc(var(--c) / (infinity * 1px))}\n"),
                "a {\
         \n  b: calc(var(--c) / (infinity * 1px));\
         \n}\n"
            );
        }
    }
    #[test]
    fn after_minus() {
        assert_eq!(
            runner().ok("a {b: calc(1% - (infinity * 1px))}\n"),
            "a {\
         \n  b: calc(1% - infinity * 1px);\
         \n}\n"
        );
    }
    #[test]
    fn after_plus() {
        assert_eq!(
            runner().ok("a {b: calc(1% + (infinity * 1px))}\n"),
            "a {\
         \n  b: calc(1% + infinity * 1px);\
         \n}\n"
        );
    }
    #[test]
    fn after_times() {
        assert_eq!(
            runner().ok("a {b: calc(var(--c) * (infinity * 1px))}\n"),
            "a {\
         \n  b: calc(var(--c) * infinity * 1px);\
         \n}\n"
        );
    }
}
mod undefined {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a {b: calc(c)}\n"),
            "a {\
         \n  b: calc(c);\
         \n}\n"
        );
    }
    #[test]
    fn math() {
        assert_eq!(
            runner().ok("a {b: calc(c * 2)}\n"),
            "a {\
         \n  b: calc(c * 2);\
         \n}\n"
        );
    }
}
