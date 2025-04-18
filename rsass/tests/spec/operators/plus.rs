//! Tests auto-converted from "sass-spec/spec/operators/plus.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("plus")
}

mod syntax {
    use super::runner;

    mod comment {
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
