//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/error/three_args.hrx"

mod blue {
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgba(0, 0, \"foo\");\
             \n}\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgba(0, \"foo\", 0);\
             \n}\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgba(\"foo\", 0, 0);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $red: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(\"foo\", 0, 0);\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
