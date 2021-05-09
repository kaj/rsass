//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1752.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
