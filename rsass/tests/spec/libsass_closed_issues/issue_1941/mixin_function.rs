//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1941/mixin_function.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin_function")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@mixin parent {\
             \n  @function nested() {\
             \n    @return foo;\
             \n  }\n\
             \n  foo: nested();\
             \n}\n\n\
             \ntest {\
             \n  @include parent;\
             \n}\n"
        ),
        "Error: Mixins may not contain function declarations.\
         \n  ,\
         \n2 |   @function nested() {\
         \n  |   ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
