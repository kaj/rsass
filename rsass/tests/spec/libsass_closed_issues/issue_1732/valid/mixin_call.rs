//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1732/valid/mixin-call.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("mixin-call")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin bar() {\
             \n  @content;\
             \n}\n\
             \nfoo {\
             \n  @include bar {\
             \n    color: blue;\
             \n  }\
             \n}"),
        "foo {\
         \n  color: blue;\
         \n}\n"
    );
}
