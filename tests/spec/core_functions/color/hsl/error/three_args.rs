//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/three_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod hue {
    #[allow(unused)]
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
    #[allow(unused)]
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
    #[allow(unused)]
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
