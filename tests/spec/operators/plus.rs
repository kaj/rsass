//! Tests auto-converted from "sass-spec/spec/operators/plus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("plus")
}

mod syntax {
    #[allow(unused)]
    use super::runner;

    mod comment {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn both() {
            assert_eq!(
                runner().ok("a {b: c/**/+/**/d}\n"),
                "a {\
         \n  b: cd;\
         \n}\n"
            );
        }
        #[test]
        fn left() {
            assert_eq!(
                runner().ok("a {b: c/**/+d}\n"),
                "a {\
         \n  b: cd;\
         \n}\n"
            );
        }
        #[test]
        fn right() {
            assert_eq!(
                runner().ok("a {b: c+/**/d}\n"),
                "a {\
         \n  b: cd;\
         \n}\n"
            );
        }
    }
    mod whitespace {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn both() {
            assert_eq!(
                runner().ok("a {b: c + d}\n"),
                "a {\
         \n  b: cd;\
         \n}\n"
            );
        }
        mod left {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn newline() {
                assert_eq!(
                    runner().ok("a {b: c\
             \n+d}\n"),
                    "a {\
         \n  b: cd;\
         \n}\n"
                );
            }
            #[test]
            fn space() {
                assert_eq!(
                    runner().ok("a {b: c +d}\n"),
                    "a {\
         \n  b: cd;\
         \n}\n"
                );
            }
            #[test]
            fn tab() {
                assert_eq!(
                    runner().ok("a {b: c\t+d}\n"),
                    "a {\
         \n  b: cd;\
         \n}\n"
                );
            }
        }
        #[test]
        fn neither() {
            assert_eq!(
                runner().ok("a {b: c+d}\n"),
                "a {\
         \n  b: cd;\
         \n}\n"
            );
        }
        #[test]
        fn right() {
            assert_eq!(
                runner().ok("a {b: c+ d}\n"),
                "a {\
         \n  b: cd;\
         \n}\n"
            );
        }
    }
}
