//! Tests auto-converted from "sass-spec/spec/css/custom_properties/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod brackets {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // wrong error
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
    #[ignore] // wrong error
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
    #[ignore] // wrong error
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
    #[ignore] // wrong error
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
    #[ignore] // wrong error
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
    #[ignore] // wrong error
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
#[test]
#[ignore] // wrong error
fn empty() {
    assert_eq!(
        runner().err(
            "// CSS requires at least one token in a custom property.\
             \n.empty {\
             \n  --property:;\
             \n}\n"
        ),
        "Error: Expected token.\
         \n  ,\
         \n3 |   --property:;\
         \n  |              ^\
         \n  \'\
         \n  input.scss 3:14  root stylesheet",
    );
}
#[test]
#[ignore] // missing error
fn empty_interpolation() {
    assert_eq!(
        runner().err(
            "// CSS requires at least one token in a custom property.\
             \n.empty {\
             \n  --property:#{\"\"};\
             \n}\n"
        ),
        "Error: Custom property values may not be empty.\
         \n  ,\
         \n3 |   --property:#{\"\"};\
         \n  |              ^^^^^\
         \n  \'\
         \n  input.scss 3:14  root stylesheet",
    );
}
