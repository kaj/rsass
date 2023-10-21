//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/four_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("four_args")
}

mod alpha {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn unit() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(0, 0, 0, 0.5px);\
             \n}\n"
            ),
            "Error: $alpha: Expected 0.5px to have unit \"%\" or no units.\
         \n  ,\
         \n2 |   b: rgb(0, 0, 0, 0.5px);\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
mod blue {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "a {\
             \n  b: rgb(0, 0, \"foo\", 0.5);\
             \n}\n"
            ),
            "Error: $blue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(0, 0, \"foo\", 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^\
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
             \n  b: rgb(0, \"foo\", 0, 0.5);\
             \n}\n"
            ),
            "Error: $green: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(0, \"foo\", 0, 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^\
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
             \n  b: rgb(\"foo\", 0, 0, 0.5);\
             \n}\n"
            ),
            "Error: $red: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(\"foo\", 0, 0, 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
