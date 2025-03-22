//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/two_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("two_args")
}

mod alpha {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(#123, \"foo\");\
             \n}\n"
            ),
            "Error: $alpha: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(#123, \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(#123, 0.5px);\
             \n}\n"
            ),
            "Error: $alpha: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 |   b: rgb(#123, 0.5px);\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod color {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(\"foo\", 0.5);\
             \n}\n"
            ),
            "Error: $color: \"foo\" is not a color.\
         \n  ,\
         \n2 |   b: rgb(\"foo\", 0.5);\
         \n  |      ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
