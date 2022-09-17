//! Tests auto-converted from "sass-spec/spec/operators/minus.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("minus")
}

mod syntax {
    #[allow(unused)]
    use super::runner;

    mod comment {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn both() {
            assert_eq!(
                runner().ok("a {b: c/**/-/**/d}\n"),
                "a {\
         \n  b: c-d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn left() {
            assert_eq!(
                runner().ok("a {b: c/**/-(d)}\n"),
                "a {\
         \n  b: c-d;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn right() {
            assert_eq!(
                runner().ok("a {b: (c)-/**/d}\n"),
                "a {\
         \n  b: c-d;\
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
                runner().ok("a {b: c - d}\n"),
                "a {\
         \n  b: c-d;\
         \n}\n"
            );
        }
        mod left {
            #[allow(unused)]
            use super::runner;

            #[test]
            #[ignore] // wrong result
            fn newline() {
                assert_eq!(
                    runner().ok("a {b: c\
             \n-(d)}\n"),
                    "a {\
         \n  b: c-d;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn space() {
                assert_eq!(
                    runner().ok("a {b: c -(d)}\n"),
                    "a {\
         \n  b: c-d;\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // wrong result
            fn tab() {
                assert_eq!(
                    runner().ok("a {b: c\t-(d)}\n"),
                    "a {\
         \n  b: c-d;\
         \n}\n"
                );
            }
        }
        #[test]
        fn neither() {
            assert_eq!(
                runner().ok("a {b: (c)-(d)}\n"),
                "a {\
         \n  b: c-d;\
         \n}\n"
            );
        }
        #[test]
        fn right() {
            assert_eq!(
                runner().ok("a {b: (c)- d}\n"),
                "a {\
         \n  b: c-d;\
         \n}\n"
            );
        }
    }
}
