//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1752.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1752")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin some-fn($args...) {\
             \n  @each $item in $args {\
             \n    @debug($item);\
             \n  }\
             \n}\n\
             \nfoo {\
             \n  @include some-fn(());\
             \n}"),
        ""
    );
}
