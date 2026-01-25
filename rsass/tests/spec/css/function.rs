//! Tests auto-converted from "sass-spec/spec/css/function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

mod error {
    use super::runner;

    mod interpolated {
        use super::runner;

        mod result {
            use super::runner;

            #[test]
            #[ignore] // wrong error
            fn characters() {
                assert_eq!(
                    runner().err(
                        "@#{function} --a() {\
             \n  result: {}#&%^*;\
             \n}\n"
                    ),
                    "Error: expected \"{\".\
         \n  ,\
         \n2 |   result: {}#&%^*;\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 2:18  root stylesheet",
                );
            }
        }
    }
    mod result {
        use super::runner;

        mod interpolated {
            use super::runner;

            #[test]
            #[ignore] // wrong error
            fn characters() {
                assert_eq!(
                    runner().err(
                        "@function --a() {\
             \n  #{result}: {}#&%^*;\
             \n}\n"
                    ),
                    "Error: expected \"{\".\
         \n  ,\
         \n2 |   #{result}: {}#&%^*;\
         \n  |                     ^\
         \n  \'\
         \n  input.scss 2:21  root stylesheet",
                );
            }
        }
        mod style_rule {
            use super::runner;

            #[test]
            #[ignore] // wrong error
            fn characters() {
                assert_eq!(
                    runner().err(
                        ".a {\
             \n  result: {}#&%^*;\
             \n}\n"
                    ),
                    "Error: expected \"{\".\
         \n  ,\
         \n2 |   result: {}#&%^*;\
         \n  |                  ^\
         \n  \'\
         \n  input.scss 2:18  root stylesheet",
                );
            }
        }
    }
}
mod interpolated {
    use super::runner;

    mod result {
        use super::runner;

        #[test]
        fn sass_script() {
            assert_eq!(
                runner().ok("@#{function} --a() {\
             \n  result: 1 + 1;\
             \n}\n"),
                "@function --a() {\
         \n  result: 2;\
         \n}\n"
            );
        }
    }
}
mod lowercase {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn interpolation() {
        assert_eq!(
            runner().ok("@function --#{a}() {result: b}\n"),
            "@function --a() {\
         \n  result: b;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn parameter() {
        assert_eq!(
            runner().ok("@function --a(--b <color>) {result: c}\n"),
            "@function --a(--b <color>) {\
         \n  result: c;\
         \n}\n"
        );
    }
    mod result {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn characters() {
            assert_eq!(
                runner().ok("@function --a() {\
             \n  result: {}#&%^*;\
             \n}\n"),
                "@function --a() {\
         \n  result: {}#&%^*;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn interpolation() {
            assert_eq!(
                runner().ok("@function --a() {\
             \n  result: #{1 + 1};\
             \n}\n"),
                "@function --a() {\
         \n  result: 2;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn sass_script() {
            assert_eq!(
                runner().ok("@function --a() {\
             \n  result: $b;\
             \n}\n"),
                "@function --a() {\
         \n  result: $b;\
         \n}\n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn returns() {
        assert_eq!(
            runner().ok("@function --a() returns <ident> {result: b}\n"),
            "@function --a() returns <ident> {\
         \n  result: b;\
         \n}\n"
        );
    }
}
mod result {
    use super::runner;

    mod interpolated {
        use super::runner;

        #[test]
        #[ignore] // wrong result
        fn sass_script() {
            assert_eq!(
                runner().ok("@function --a() {\
             \n  #{result}: 1 + 1;\
             \n}\n"),
                "@function --a() {\
         \n  result: 2;\
         \n}\n"
            );
        }
    }
    mod style_rule {
        use super::runner;

        #[test]
        fn interpolation() {
            assert_eq!(
                runner().ok(".a {\
             \n  result: #{1 + 1};\
             \n}\n"),
                ".a {\
         \n  result: 2;\
         \n}\n"
            );
        }
        #[test]
        fn sass_script() {
            assert_eq!(
                runner().ok(".a {\
             \n  result: 1 + 1;\
             \n}\n"),
                ".a {\
         \n  result: 2;\
         \n}\n"
            );
        }
    }
    mod uppercase {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn characters() {
            assert_eq!(
                runner().ok("@function --a() {\
             \n  RESULT: {}#&%^*;\
             \n}\n"),
                "@function --a() {\
         \n  RESULT: {}#&%^*;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn interpolation() {
            assert_eq!(
                runner().ok("@function --a() {\
             \n  RESULT: #{1 + 1};\
             \n}\n"),
                "@function --a() {\
         \n  RESULT: 2;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn sass_script() {
            assert_eq!(
                runner().ok("@function --a() {\
             \n  RESULT: $b;\
             \n}\n"),
                "@function --a() {\
         \n  RESULT: $b;\
         \n}\n"
            );
        }
    }
}
mod uppercase {
    use super::runner;

    mod result {
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn characters() {
            assert_eq!(
                runner().ok("@FUNCTION --a() {\
             \n  result: {}#&%^*;\
             \n}\n"),
                "@FUNCTION --a() {\
         \n  result: {}#&%^*;\
         \n}\n"
            );
        }
        #[test]
        fn interpolation() {
            assert_eq!(
                runner().ok("@FUNCTION --a() {\
             \n  result: #{1 + 1};\
             \n}\n"),
                "@FUNCTION --a() {\
         \n  result: 2;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn nesting() {
            assert_eq!(
                runner().ok("@function --a() {\
             \n  RESULT: {b: c};\
             \n}\n"),
                "@function --a() {\
         \n  RESULT: {b: c};\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn sass_script() {
            assert_eq!(
                runner().ok("@FUNCTION --a() {\
             \n  result: $b;\
             \n}\n"),
                "@FUNCTION --a() {\
         \n  result: $b;\
         \n}\n"
            );
        }
    }
}
