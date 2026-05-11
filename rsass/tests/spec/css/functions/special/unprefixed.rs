//! Tests auto-converted from "sass-spec/spec/css/functions/special/unprefixed.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("unprefixed")
}

mod lowercase {
    use super::runner;

    mod test_type {
        use super::runner;

        #[test]
        fn interpolation() {
            assert_eq!(
                runner().ok("a {b: type(#{0})}\n"),
                "a {\
         \n  b: type(0);\
         \n}\n"
            );
        }
        #[test]
        fn number() {
            assert_eq!(
                runner().ok("a {b: type(0)}\n"),
                "a {\
         \n  b: type(0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn punctuation() {
            assert_eq!(
                runner().ok(
                    "a {b: type(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
                ),
                "a {\
         \n  b: type(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
            );
        }
    }
    mod url {
        use super::runner;

        mod exclam {
            use super::runner;

            #[test]
            fn middle() {
                assert_eq!(
                    runner().ok("a {b: url(http://c.d/e!f)}\n"),
                    "a {\
         \n  b: url(http://c.d/e!f);\
         \n}\n"
                );
            }
            #[test]
            fn only() {
                assert_eq!(
                    runner().ok("a {b: url(!)}\n"),
                    "a {\
         \n  b: url(!);\
         \n}\n"
                );
            }
        }
    }
}
mod uppercase {
    use super::runner;

    mod test_type {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn interpolation() {
            assert_eq!(
                runner().ok("a {b: TYPE(#{0})}\n"),
                "a {\
         \n  b: type(0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn number() {
            assert_eq!(
                runner().ok("a {b: TYPE(0)}\n"),
                "a {\
         \n  b: type(0);\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn punctuation() {
            assert_eq!(
                runner().ok(
                    "a {b: TYPE(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/)}\n"
                ),
                "a {\
         \n  b: type(@#$%^&*({[]})_-+=|\\\\:\"\"\'\'<>,.?/);\
         \n}\n"
            );
        }
    }
    mod url {
        use super::runner;

        mod exclam {
            use super::runner;

            #[test]
            #[ignore] // unexepected error
            fn middle() {
                assert_eq!(
                    runner().ok("a {b: URL(http://c.d/e!f)}\n"),
                    "a {\
         \n  b: url(http://c.d/e!f);\
         \n}\n"
                );
            }
            #[test]
            #[ignore] // unexepected error
            fn only() {
                assert_eq!(
                    runner().ok("a {b: URL(!)}\n"),
                    "a {\
         \n  b: url(!);\
         \n}\n"
                );
            }
        }
    }
}
