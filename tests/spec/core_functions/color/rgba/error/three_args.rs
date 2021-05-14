//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/error/three_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod blue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(0, 0, \"foo\");\
             \n}\n"
            ),
            "Error: $blue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(0, 0, \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod green {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(0, \"foo\", 0);\
             \n}\n"
            ),
            "Error: $green: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(0, \"foo\", 0);\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod red {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgba(\"foo\", 0, 0);\
             \n}\n"
            ),
            "Error: $red: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(\"foo\", 0, 0);\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
