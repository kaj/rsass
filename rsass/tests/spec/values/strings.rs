//! Tests auto-converted from "sass-spec/spec/values/strings.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("strings")
}

mod escape {
    use super::runner;

    #[test]
    fn max_valid() {
        assert_eq!(
        runner().ok(
            "// Regression test. This used to be parsed incorrectly as U+FFFD REPLACEMENT\
             \n// CHARACTER.\
             \na {b: \"\\10FFFF\"}\n"
        ),
        "a {\
         \n  b: \"\\10ffff\";\
         \n}\n"
    );
    }
    #[test]
    fn min_invalid() {
        assert_eq!(
            runner().ok(
                "// This one should be parsed as a replacement character.\
             \na {b: \"\\110000\"}\n"
            ),
            "@charset \"UTF-8\";\
         \na {\
         \n  b: \"�\";\
         \n}\n"
        );
    }
}
mod new_line {
    use super::runner;

    mod scss {
        use super::runner;

        #[test]
        fn cr() {
            assert_eq!(
                runner().err(
                    "@mixin a($b) { b: $b }\
             \na {\
             \n  b: \'line1\
             \n      line2\';\
             \n}\n"
                ),
                "Error: Expected \'.\
         \n  ,\
         \n3 |   b: \'line1\
         \n  |            ^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
            );
        }
        #[test]
        fn escaped() {
            assert_eq!(
                runner().ok("a {\
             \n  b: \'line1 \\\
             \n      line2\';\
             \n}\n"),
                "a {\
         \n  b: \"line1       line2\";\
         \n}\n"
            );
        }
        #[test]
        fn ff() {
            assert_eq!(
                runner().err(
                    "@mixin a($b) { b: $b }\
             \na {\
             \n  b: \'line1\u{c}line2\';\n\n"
                ),
                "Error: Expected \'.\
         \n  ,\
         \n3 |   b: \'line1\u{c}line2\';\
         \n  |            ^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
            );
        }
        #[test]
        fn raw() {
            assert_eq!(
                runner().err(
                    "@mixin a($b) { b: $b }\
             \na {\
             \n  b: \'line1\
             \n      line2\';\
             \n}\n"
                ),
                "Error: Expected \'.\
         \n  ,\
         \n3 |   b: \'line1\
         \n  |            ^\
         \n  \'\
         \n  input.scss 3:12  root stylesheet",
            );
        }
    }
}
