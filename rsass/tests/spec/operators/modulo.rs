//! Tests auto-converted from "sass-spec/spec/operators/modulo.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("modulo")
}

mod degenerate {
    #[allow(unused)]
    use super::runner;

    mod modulus {
        #[allow(unused)]
        use super::runner;

        mod infinity {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn negative_and_negative() {
                assert_eq!(
                    runner().ok("a {b: -1px % calc(-infinity * 1px)}\n"),
                    "a {\
         \n  b: -1px;\
         \n}\n"
                );
            }
            #[test]
            fn negative_and_positive() {
                assert_eq!(
                    runner().ok("a {b: -1px % calc(infinity * 1px)}\n"),
                    "a {\
         \n  b: calc(NaN * 1px);\
         \n}\n"
                );
            }
            #[test]
            fn positive_and_negative() {
                assert_eq!(
                    runner().ok("a {b: 1px % calc(-infinity * 1px)}\n"),
                    "a {\
         \n  b: calc(NaN * 1px);\
         \n}\n"
                );
            }
            #[test]
            fn positive_and_positive() {
                assert_eq!(
                    runner().ok("a {b: 1px % calc(infinity * 1px)}\n"),
                    "a {\
         \n  b: 1px;\
         \n}\n"
                );
            }
        }
    }
}
