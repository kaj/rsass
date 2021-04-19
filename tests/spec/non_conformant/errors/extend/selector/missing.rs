//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/extend/selector/missing.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            ".baz {\
             \n  @extend .foo;\
             \n  color: green;\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: The target selector was not found.\
         \nUse \"@extend .foo !optional\" to avoid this error.\
         \n  ,\
         \n2 |   @extend .foo;\
         \n  |   ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
