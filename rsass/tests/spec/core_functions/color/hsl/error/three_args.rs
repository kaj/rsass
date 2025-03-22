//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/three_args.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("three_args")
}

mod hue {
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsl(\"foo\", 100%, 50%);\
             \n}\n"
            ),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(\"foo\", 100%, 50%);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^\
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
             \n  b: hsl(0, 100%, \"foo\");\
             \n}\n"
            ),
            "Error: $lightness: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(0, 100%, \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
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
             \n  b: hsl(0, \"foo\", 50%);\
             \n}\n"
            ),
            "Error: $saturation: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsl(0, \"foo\", 50%);\
         \n  |      ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
