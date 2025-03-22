//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/three_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("three_args")
}

mod blue {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(0, 0, \"foo\");\
             \n}\n"
            ),
            "Error: $blue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(0, 0, \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod green {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(0, \"foo\", 0);\
             \n}\n"
            ),
            "Error: $green: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(0, \"foo\", 0);\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod red {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(\"foo\", 0, 0);\
             \n}\n"
            ),
            "Error: $red: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(\"foo\", 0, 0);\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
