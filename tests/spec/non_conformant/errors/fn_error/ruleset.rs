//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/fn-error/ruleset.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\r\
             \n  @error \"error\";\r\
             \n  foo: bar;\r\
             \n}"
        )
        .unwrap_err(),
        "Error: \"error\"\
         \n  ,\
         \n2 |   @error \"error\";\
         \n  |   ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
