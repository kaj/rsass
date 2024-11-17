//! Tests auto-converted from "sass-spec/spec/values/strings.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("strings")
}

mod new_line {
    #[allow(unused)]
    use super::runner;

    mod sass {
        #[allow(unused)]
        use super::runner;
    }
    mod scss {
        #[allow(unused)]
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
