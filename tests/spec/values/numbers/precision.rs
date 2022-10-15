//! Tests auto-converted from "sass-spec/spec/values/numbers/precision.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("precision")
}

mod very_close_to_int {
    #[allow(unused)]
    use super::runner;

    mod negative {
        #[allow(unused)]
        use super::runner;

        mod above {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn at_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 + math.pow(10, -11)}\n"),
                    "a {\
         \n  b: -10;\
         \n}\n"
                );
            }
            #[test]
            fn bigger_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 + math.pow(10, -10)}\n"),
                    "a {\
         \n  b: -9.9999999999;\
         \n}\n"
                );
            }
            #[test]
            fn slightly_bigger_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 + 2 * math.pow(10, -11)}\n"),
                    "a {\
         \n  b: -10;\
         \n}\n"
                );
            }
            #[test]
            fn slightly_smaller_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 + 0.5 * math.pow(10, -11)}\n"),
                    "a {\
         \n  b: -10;\
         \n}\n"
                );
            }
            #[test]
            fn smaller_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 + math.pow(10, -12)}\n"),
                    "a {\
         \n  b: -10;\
         \n}\n"
                );
            }
        }
        mod below {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn at_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 - math.pow(10, -11)}\n"),
                    "a {\
         \n  b: -10;\
         \n}\n"
                );
            }
            #[test]
            fn bigger_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 - math.pow(10, -10)}\n"),
                    "a {\
         \n  b: -10.0000000001;\
         \n}\n"
                );
            }
            #[test]
            fn slightly_bigger_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 - 2 * math.pow(10, -11)}\n"),
                    "a {\
         \n  b: -10;\
         \n}\n"
                );
            }
            #[test]
            fn slightly_smaller_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 - 0.5 * math.pow(10, -11)}\n"),
                    "a {\
         \n  b: -10;\
         \n}\n"
                );
            }
            #[test]
            fn smaller_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: -10 - math.pow(10, -12)}\n"),
                    "a {\
         \n  b: -10;\
         \n}\n"
                );
            }
        }
    }
    mod positive {
        #[allow(unused)]
        use super::runner;

        mod above {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn at_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 + math.pow(10, -11)}\n"),
                    "a {\
         \n  b: 10;\
         \n}\n"
                );
            }
            #[test]
            fn bigger_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 + math.pow(10, -10)}\n"),
                    "a {\
         \n  b: 10.0000000001;\
         \n}\n"
                );
            }
            #[test]
            fn slightly_bigger_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 + 2 * math.pow(10, -11)}\n"),
                    "a {\
         \n  b: 10;\
         \n}\n"
                );
            }
            #[test]
            fn slightly_smaller_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 + 0.5 * math.pow(10, -11)}\n"),
                    "a {\
         \n  b: 10;\
         \n}\n"
                );
            }
            #[test]
            fn smaller_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 + math.pow(10, -12)}\n"),
                    "a {\
         \n  b: 10;\
         \n}\n"
                );
            }
        }
        mod below {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn at_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 - math.pow(10, -11)}\n"),
                    "a {\
         \n  b: 10;\
         \n}\n"
                );
            }
            #[test]
            fn bigger_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 - math.pow(10, -10)}\n"),
                    "a {\
         \n  b: 9.9999999999;\
         \n}\n"
                );
            }
            #[test]
            fn slightly_bigger_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 - 2 * math.pow(10, -11)}\n"),
                    "a {\
         \n  b: 10;\
         \n}\n"
                );
            }
            #[test]
            fn slightly_smaller_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 - 0.5 * math.pow(10, -11)}\n"),
                    "a {\
         \n  b: 10;\
         \n}\n"
                );
            }
            #[test]
            fn smaller_than_boundary() {
                assert_eq!(
                    runner().ok("@use \'sass:math\';\
             \na {b: 10 - math.pow(10, -12)}\n"),
                    "a {\
         \n  b: 10;\
         \n}\n"
                );
            }
        }
    }
}
mod very_small {
    #[allow(unused)]
    use super::runner;

    mod negative {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn at_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: -(math.pow(10, -11))}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn bigger_than_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: -(math.pow(10, -10))}\n"),
                "a {\
         \n  b: -0.0000000001;\
         \n}\n"
            );
        }
        #[test]
        fn slightly_bigger_than_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: -2 * math.pow(10, -11)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn slightly_smaller_than_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: -0.5 * math.pow(10, -11)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn smaller_than_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: -(math.pow(10, -12))}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
    }
    mod positive {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn at_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: math.pow(10, -11)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn bigger_than_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: math.pow(10, -10)}\n"),
                "a {\
         \n  b: 0.0000000001;\
         \n}\n"
            );
        }
        #[test]
        fn slightly_bigger_than_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: 2 * math.pow(10, -11)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn slightly_smaller_than_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: 0.5 * math.pow(10, -11)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
        #[test]
        fn smaller_than_boundary() {
            assert_eq!(
                runner().ok("@use \'sass:math\';\
             \na {b: 0.5 * math.pow(10, -12)}\n"),
                "a {\
         \n  b: 0;\
         \n}\n"
            );
        }
    }
}
