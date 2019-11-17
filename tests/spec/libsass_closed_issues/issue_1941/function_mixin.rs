//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1941/function_mixin.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function parent() {\
             \n  @mixin nested {\
             \n    foo: bar;\
             \n  }\
             \n\
             \n  @include nested;\
             \n}\
             \n\
             \n\
             \ntest {\
             \n  foo: parent();\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @mixin nested {\
         \n  |   ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet\
         \n",
    );
}
