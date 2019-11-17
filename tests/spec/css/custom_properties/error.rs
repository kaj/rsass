//! Tests auto-converted from "sass-spec/spec/css/custom_properties/error.hrx"

mod brackets {
    #[test]
    #[ignore] // wrong error
    fn curly() {
        assert_eq!(
            crate::rsass(
                ".curly {\
             \n  --prop: };\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: unmatched \"}\".\
         \n  ,\
         \n3 | }\
         \n  | ^\
         \n  \'\
         \n  input.scss 3:1  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn curly_in_square() {
        assert_eq!(
            crate::rsass(
                ".curly-in-square {\
             \n  --prop: [{];\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: expected \"}\".\
         \n  ,\
         \n2 |   --prop: [{];\
         \n  |             ^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn paren() {
        assert_eq!(
            crate::rsass(
                ".paren {\
             \n  --prop: );\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: expected \";\".\
         \n  ,\
         \n2 |   --prop: );\
         \n  |           ^\
         \n  \'\
         \n  input.scss 2:11  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn paren_in_curly() {
        assert_eq!(
            crate::rsass(
                ".paren-in-curly {\
             \n  --prop: {(};\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: expected \")\".\
         \n  ,\
         \n2 |   --prop: {(};\
         \n  |             ^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn square() {
        assert_eq!(
            crate::rsass(
                ".square {\
             \n  --prop: ];\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: expected \";\".\
         \n  ,\
         \n2 |   --prop: ];\
         \n  |           ^\
         \n  \'\
         \n  input.scss 2:11  root stylesheet\
         \n",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn square_in_paren() {
        assert_eq!(
            crate::rsass(
                ".square-in-paren {\
             \n  --prop: ([);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: expected \"]\".\
         \n  ,\
         \n2 |   --prop: ([);\
         \n  |             ^\
         \n  \'\
         \n  input.scss 2:13  root stylesheet\
         \n",
        );
    }
}
#[test]
#[ignore] // wrong error
fn empty() {
    assert_eq!(
        crate::rsass(
            "// CSS requires at least one token in a custom property.\
             \n.empty {\
             \n  --property:;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Expected token.\
         \n  ,\
         \n3 |   --property:;\
         \n  |              ^\
         \n  \'\
         \n  input.scss 3:14  root stylesheet\
         \n",
    );
}
#[test]
#[ignore] // missing error
fn empty_interpolation() {
    assert_eq!(
        crate::rsass(
            "// CSS requires at least one token in a custom property.\
             \n.empty {\
             \n  --property:#{\"\"};\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Custom property values may not be empty.\
         \n  ,\
         \n3 |   --property:#{\"\"};\
         \n  |              ^^^^^\
         \n  \'\
         \n  input.scss 3:14  root stylesheet\
         \n",
    );
}
