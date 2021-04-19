//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/one_arg.hrx"

mod list {
    #[test]
    #[ignore] // missing error
    fn bracketed() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb([1 2 3]);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $channels must be an unbracketed list.\
         \n  ,\
         \n2 |   b: rgb([1 2 3]);\
         \n  |      ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn comma_separated() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb((1, 2, 3));\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $channels must be a space-separated list.\
         \n  ,\
         \n2 |   b: rgb((1, 2, 3));\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn empty() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb(());\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $red.\
         \n  ,\
         \n2 |   b: rgb(());\
         \n  |      ^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn four_elements() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb(1 2 3 0.4);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 3 elements allowed, but 4 were passed.\
         \n  ,\
         \n2 |   b: rgb(1 2 3 0.4);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn one_element() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb(1);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $green.\
         \n  ,\
         \n2 |   b: rgb(1);\
         \n  |      ^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // missing error
    fn two_elements() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb(1 2);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing element $blue.\
         \n  ,\
         \n2 |   b: rgb(1 2);\
         \n  |      ^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
}
#[test]
fn quoted_var_slash() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  b: rgb(1 2 \"var(--foo) / 0.4\");\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: $blue: \"var(--foo) / 0.4\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(1 2 \"var(--foo) / 0.4\");\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
    );
}
mod test_type {
    #[test]
    fn blue() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb(1 2 \"foo\");\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $blue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(1 2 \"foo\");\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
    #[test]
    fn green() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb(1 \"foo\" 3);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $green: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(1 \"foo\" 3);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
    #[test]
    fn red() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: rgb(\"foo\" 2 3);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $red: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: rgb(\"foo\" 2 3);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
}
