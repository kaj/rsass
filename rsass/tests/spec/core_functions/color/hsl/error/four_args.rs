//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/four_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("four_args")
}

mod alpha {
    use super::runner;

    #[test]
    fn unit() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(0, 0%, 0%, 0.5px);\
             \n}\n"
            ),
            "Error: $alpha: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 |   b: hsl(0, 0%, 0%, 0.5px);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod hue {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(\"foo\", 100%, 50%, 0.5);\
             \n}\n"
            ),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(\"foo\", 100%, 50%, 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod lightness {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(0, 100%, \"foo\", 0.5);\
             \n}\n"
            ),
            "Error: $lightness: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(0, 100%, \"foo\", 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod saturation {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(0, \"foo\", 50%, 0.5);\
             \n}\n"
            ),
            "Error: $saturation: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(0, \"foo\", 50%, 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
