//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/invalid/mixin-def.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin a {\
             \n  b: c;\
             \n}\
             \n\
             \n@include a();"
        )
        .unwrap_err(),
        "Error: Declarations may only be used within style rules.\
         \n  ,\
         \n2 |   b: c;\
         \n  |   ^^^^\
         \n  \'\
         \n  input.scss 2:3  a()\
         \n  input.scss 5:1  root stylesheet\
         \n",
    );
}
