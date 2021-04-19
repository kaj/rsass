//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/invalid-parent/return-in-ruleset.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "ruleset {\r\
             \n  @return 42;\r\
             \n}"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @return 42;\
         \n  |   ^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
