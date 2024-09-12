//! Tests auto-converted from "sass-spec/spec/css/supports/comment.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("comment")
}

mod after_query {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn loud() {
        assert_eq!(
            runner().ok("@supports (a: b) /**/ {c {d: e}}\n"),
            "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn silent() {
        assert_eq!(
            runner().ok("@supports (a: b) //\
             \n  {c {d: e}}\n"),
            "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
        );
    }
}
mod anything {
    #[allow(unused)]
    use super::runner;

    mod after_ident {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn loud() {
            assert_eq!(
                runner().ok("@supports (a /**/ b) {c {d: e}}\n"),
                "@supports (a /**/ b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn silent() {
            assert_eq!(
                runner().ok("@supports (a //\
             \n  b) {c {d: e}}\n"),
                "@supports (a \
         \n  b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod after_open_paren {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(
                runner().ok("@supports (/**/ a b) {c {d: e}}\n"),
                "@supports (a b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        fn silent() {
            assert_eq!(
                runner().ok("@supports (//\
             \n  a b) {c {d: e}}\n"),
                "@supports (a b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
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
                runner().ok("@supports (a b /**/) {c {d: e}}\n"),
                "@supports (a b /**/) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn silent() {
            assert_eq!(
                runner().ok("@supports (a b //\
             \n  ) {c {d: e}}\n"),
                "@supports (a b \
         \n  ) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
    }
}
mod before_query {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn loud() {
        assert_eq!(
            runner().ok("@supports /**/ (a: b) {c {d: e}}\n"),
            "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
        );
    }
    #[test]
    fn silent() {
        assert_eq!(
            runner().ok("@supports //\
             \n  (a: b) {c {d: e}}\n"),
            "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
        );
    }
}
mod declaration {
    #[allow(unused)]
    use super::runner;

    mod custom_prop {
        #[allow(unused)]
        use super::runner;

        mod after_colon {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@supports (--a: /**/ b) {c {d: e}}\n"),
                    "@supports (--a: /**/ b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("@supports (--a: //\
             \n  b) {c {d: e}}\n"),
                    "@supports (--a:  b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
        }
        mod after_open_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn loud() {
                assert_eq!(
                    runner().ok("@supports (/**/ --a: b) {c {d: e}}\n"),
                    "@supports (--a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@supports (//\
             \n  --a: b) {c {d: e}}\n"),
                    "@supports (--a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
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
                    runner().ok("@supports (--a: b /**/) {c {d: e}}\n"),
                    "@supports (--a: b /**/) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn silent() {
                assert_eq!(
                    runner().ok("@supports (--a: b //\
             \n  ) {c {d: e}}\n"),
                    "@supports (--a: b  ) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
        }
        mod before_colon {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@supports (--a /**/: b) {c {d: e}}\n"),
                    "@supports (--a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@supports (--a //\
             \n  : b) {c {d: e}}\n"),
                    "@supports (--a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
        }
    }
    mod normal_prop {
        #[allow(unused)]
        use super::runner;

        mod after_colon {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn loud() {
                assert_eq!(
                    runner().ok("@supports (a: /**/ b) {c {d: e}}\n"),
                    "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@supports (a: //\
             \n  b) {c {d: e}}\n"),
                    "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
        }
        mod after_open_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn loud() {
                assert_eq!(
                    runner().ok("@supports (/**/ a: b) {c {d: e}}\n"),
                    "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@supports (//\
             \n  a: b) {c {d: e}}\n"),
                    "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
        }
        mod before_close_paren {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn loud() {
                assert_eq!(
                    runner().ok("@supports (a: b /**/) {c {d: e}}\n"),
                    "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@supports (a: b //\
             \n  ) {c {d: e}}\n"),
                    "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
        }
        mod before_colon {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@supports (a /**/: b) {c {d: e}}\n"),
                    "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@supports (a //\
             \n  : b) {c {d: e}}\n"),
                    "@supports (a: b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
                );
            }
        }
    }
}
mod function {
    #[allow(unused)]
    use super::runner;

    mod after_open_paren {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn loud() {
            assert_eq!(
                runner().ok("@supports a(/**/ b) {c {d: e}}\n"),
                "@supports a(/**/ b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn silent() {
            assert_eq!(
                runner().ok("@supports a(//\
             \n  b) {c {d: e}}\n"),
                "@supports a(\
         \n  b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
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
                runner().ok("@supports a(b /**/) {c {d: e}}\n"),
                "@supports a(b /**/) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn silent() {
            assert_eq!(
                runner().ok("@supports a(b //\
             \n  ) {c {d: e}}\n"),
                "@supports a(b \
         \n  ) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
    }
}
