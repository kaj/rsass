//! Tests auto-converted from "sass-spec/spec/css/functions/special.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("special")
}

mod comment {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
        use super::runner;

        mod after_open_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: -a-calc(/**/ c)}\n"),
                    "a {\
         \n  b: -a-calc(/**/ c);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("a {\
             \n  b: -a-calc(//\
             \n    c);\
             \n}\n"),
                    "a {\
         \n  b: -a-calc( c);\
         \n}\n"
                );
            }
        }
        mod before_close_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: -a-calc(c /**/)}\n"),
                    "a {\
         \n  b: -a-calc(c /**/);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("a {\
             \n  b: -a-calc(c //\
             \n    );\
             \n}\n"),
                    "a {\
         \n  b: -a-calc(c  );\
         \n}\n"
                );
            }
        }
    }
    mod element {
        #[allow(unused)]
        use super::runner;

        mod after_open_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: element(/**/ c)}\n"),
                    "a {\
         \n  b: element(/**/ c);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("a {\
             \n  b: element(//\
             \n    c);\
             \n}\n"),
                    "a {\
         \n  b: element( c);\
         \n}\n"
                );
            }
        }
        mod before_close_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: element(c /**/)}\n"),
                    "a {\
         \n  b: element(c /**/);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("a {\
             \n  b: element(c //\
             \n    );\
             \n}\n"),
                    "a {\
         \n  b: element(c  );\
         \n}\n"
                );
            }
        }
    }
    mod expression {
        #[allow(unused)]
        use super::runner;

        mod after_open_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: expression(/**/ c)}\n"),
                    "a {\
         \n  b: expression(/**/ c);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("a {\
             \n  b: expression(//\
             \n    c);\
             \n}\n"),
                    "a {\
         \n  b: expression( c);\
         \n}\n"
                );
            }
        }
        mod before_close_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: expression(c /**/)}\n"),
                    "a {\
         \n  b: expression(c /**/);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("a {\
             \n  b: expression(c //\
             \n    );\
             \n}\n"),
                    "a {\
         \n  b: expression(c  );\
         \n}\n"
                );
            }
        }
    }
    mod progid {
        #[allow(unused)]
        use super::runner;

        mod after_open_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: progid:c(/**/ d)}\n"),
                    "a {\
         \n  b: progid:c(/**/ d);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("a {\
             \n  b: progid:c(//\
             \n    d);\
             \n}\n"),
                    "a {\
         \n  b: progid:c( d);\
         \n}\n"
                );
            }
        }
        mod before_close_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("a {b: progid:c(d /**/)}\n"),
                    "a {\
         \n  b: progid:c(d /**/);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("a {\
             \n  b: progid:c(d //\
             \n    );\
             \n}\n"),
                    "a {\
         \n  b: progid:c(d  );\
         \n}\n"
                );
            }
        }
    }
}
mod prefixed {
    #[allow(unused)]
    use super::runner;

    mod calc {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn interpolation() {
            assert_eq!(
                runner().ok("a {b: -a-calc(#{0})}\n"),
                "a {\
         \n  b: -a-calc(0);\
         \n}\n"
            );
        }
        #[test]
        fn number() {
            assert_eq!(
                runner().ok("a {b: -a-calc(0)}\n"),
                "a {\
         \n  b: -a-calc(0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn punctuation() {
            assert_eq!(
                runner().ok(
                    "a {b: -a-calc(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
                ),
                "a {\
         \n  b: -a-calc(@#$%^&*({[]})_-+=|\\\\:\"\"\"\"<>,.?/);\
         \n}\n"
            );
        }
    }
}
