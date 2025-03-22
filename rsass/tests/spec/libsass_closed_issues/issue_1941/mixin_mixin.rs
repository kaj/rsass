//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1941/mixin_mixin.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin_mixin")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@mixin parent {\
             \n  @mixin nested {\
             \n    foo: bar;\
             \n  }\n\
             \n  @include nested;\
             \n}\n\n\
             \ntest {\
             \n  @include parent;\
             \n}\n"
        ),
        "Error: Mixins may not contain mixin declarations.\
         \n  ,\
         \n2 |   @mixin nested {\
         \n  |   ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:3  root stylesheet",
    );
}
