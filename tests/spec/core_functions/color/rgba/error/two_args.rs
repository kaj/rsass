//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/error/two_args.hrx"

mod alpha {
    #[test]
    #[ignore] // missing error
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgba(#123, \"foo\");\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $alpha: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgba(#123, \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn unit() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgba(#123, 0.5px);\
             \n}\
             \n"
            )
            .unwrap_err(),
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
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgba(\"foo\", 0.5);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $color: \"foo\" is not a color.\
         \n  ,\
         \n2 |   b: rgba(\"foo\", 0.5);\
         \n  |      ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
