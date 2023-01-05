//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1941/function_mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function_mixin")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@function parent() {\
             \n  @mixin nested {\
             \n    foo: bar;\
             \n  }\n\
             \n  @include nested;\
             \n}\n\n\
             \ntest {\
             \n  foo: parent();\
             \n}\n"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n2 |   @mixin nested {\
         \n  |   ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
