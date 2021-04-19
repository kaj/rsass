//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/error/one_arg.hrx"

mod list {
    #[test]
    #[ignore] // missing error
    fn bracketed() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla([0 100% 50%]);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 |   b: hsla([0 100% 50%]);\
         \n  |      ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn comma_separated() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla((0, 100%, 50%));\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 |   b: hsla((0, 100%, 50%));\
         \n  |      ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(());\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $hue.\
         \n  ,\
         \n2 |   b: hsla(());\
         \n  |      ^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn four_elements() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(0 100% 50% 0.4);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 |   b: hsla(0 100% 50% 0.4);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn one_element() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(0);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $saturation.\
         \n  ,\
         \n2 |   b: hsla(0);\
         \n  |      ^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    #[ignore] // missing error
    fn two_elements() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(0 100%);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $lightness.\
         \n  ,\
         \n2 |   b: hsla(0 100%);\
         \n  |      ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
#[test]
fn quoted_var_slash() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  b: hsla(0 100% \"var(--foo) / 0.4\");\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: $lightness: \"var(--foo) / 0.4\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(0 100% \"var(--foo) / 0.4\");\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
mod test_type {
    #[test]
    fn hue() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(\"foo\" 100% 50%);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(\"foo\" 100% 50%);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn lightness() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(0 100% \"foo\");\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $lightness: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(0 100% \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
    #[test]
    fn saturation() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(0 \"foo\" 50%);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $saturation: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(0 \"foo\" 50%);\
         \n  |      ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
        );
    }
}
