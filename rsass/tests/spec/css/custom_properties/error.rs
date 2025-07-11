//! Tests auto-converted from "sass-spec/spec/css/custom_properties/error.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("error")
}

mod brackets {
    use super::runner;

    #[test]
    fn curly() {
        assert_eq!(
            runner().err(
                ".curly {\
             \n  --prop: };\
             \n}\n"
            ),
            "Error: unmatched \"}\".\
         \n  ,\
         \n3 | }\
         \n  | ^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet",
        );
    }
    #[test]
    fn curly_in_square() {
        assert_eq!(
            runner().err(
                ".curly-in-square {\
             \n  --prop: [{];\
             \n}\n"
            ),
            "Error: expected \"}\".\
         \n  ,\
         \n2 |   --prop: [{];\
         \n  |             ^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet",
        );
    }
    #[test]
    fn paren() {
        assert_eq!(
            runner().err(
                ".paren {\
             \n  --prop: );\
             \n}\n"
            ),
            "Error: expected \";\".\
         \n  ,\
         \n2 |   --prop: );\
         \n  |           ^\
         \n  \'\
         \n  input.scss 2:11  root stylesheet",
        );
    }
    #[test]
    fn paren_in_curly() {
        assert_eq!(
            runner().err(
                ".paren-in-curly {\
             \n  --prop: {(};\
             \n}\n"
            ),
            "Error: expected \")\".\
         \n  ,\
         \n2 |   --prop: {(};\
         \n  |             ^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet",
        );
    }
    #[test]
    fn square() {
        assert_eq!(
            runner().err(
                ".square {\
             \n  --prop: ];\
             \n}\n"
            ),
            "Error: expected \";\".\
         \n  ,\
         \n2 |   --prop: ];\
         \n  |           ^\
         \n  \'\
         \n  input.scss 2:11  root stylesheet",
        );
    }
    #[test]
    fn square_in_paren() {
        assert_eq!(
            runner().err(
                ".square-in-paren {\
             \n  --prop: ([);\
             \n}\n"
            ),
            "Error: expected \"]\".\
         \n  ,\
         \n2 |   --prop: ([);\
         \n  |             ^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet",
        );
    }
}
