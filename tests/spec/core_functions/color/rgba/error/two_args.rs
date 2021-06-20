//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/error/two_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(#123, \"foo\");\
             \n}\n"
            ),
            "Error: $alpha: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(#123, \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(#123, 0.5px);\
             \n}\n"
            ),
            "Error: $alpha: Expected 0.5px to have no units or \"%\".\
         \n  ,\
         \n2 |   b: rgba(#123, 0.5px);\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod color {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(\"foo\", 0.5);\
             \n}\n"
            ),
            "Error: $color: \"foo\" is not a color.\
         \n  ,\
         \n2 |   b: rgba(\"foo\", 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
