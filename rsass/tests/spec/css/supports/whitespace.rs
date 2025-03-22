//! Tests auto-converted from "sass-spec/spec/css/supports/whitespace.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("whitespace")
}

mod after_query {
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@supports (a: b)\
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
    use super::runner;

    mod after_ident {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn scss() {
            assert_eq!(
                runner().ok("@supports (a\
             \n  b) {c {d: e}}\n"),
                "@supports (a\
         \n  b) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
    }
    mod after_open_paren {
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@supports (\
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
        use super::runner;

        #[test]
        fn scss() {
            assert_eq!(
                runner().ok("@supports (a b\
             \n  ) {c {d: e}}\n"),
                "@supports (a b\
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
    use super::runner;

    #[test]
    fn scss() {
        assert_eq!(
            runner().ok("@supports\
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
    use super::runner;

    mod normal_prop {
        use super::runner;

        mod after_colon {
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@supports (a:\
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
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@supports (\
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
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@supports (a: b\
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
            use super::runner;

            #[test]
            fn scss() {
                assert_eq!(
                    runner().ok("@supports (a \
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
    use super::runner;

    mod after_open_paren {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn scss() {
            assert_eq!(
                runner().ok("@supports a(\
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
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn scss() {
            assert_eq!(
                runner().ok("@supports a(b \
             \n  ) {c {d: e}}\n"),
                "@supports a(b\
         \n  ) {\
         \n  c {\
         \n    d: e;\
         \n  }\
         \n}\n"
            );
        }
    }
}
