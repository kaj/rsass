//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/error/four_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod alpha {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsla(0, 0%, 0%, 0.5px);\
             \n}\n"
            ),
            "Error: $alpha: Expected 0.5px to have no units or \"%\".\
         \n  ,\
         \n2 |   b: hsla(0, 0%, 0%, 0.5px);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod hue {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: hsla(\"foo\", 100%, 50%, 0.5);\
             \n}\n"
            ),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(\"foo\", 100%, 50%, 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \n  b: hsla(0, 100%, \"foo\", 0.5);\
             \n}\n"
            ),
            "Error: $lightness: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(0, 100%, \"foo\", 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^\
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
             \n  b: hsla(0, \"foo\", 50%, 0.5);\
             \n}\n"
            ),
            "Error: $saturation: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(0, \"foo\", 50%, 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
