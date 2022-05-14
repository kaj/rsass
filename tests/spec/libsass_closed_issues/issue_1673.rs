//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1673.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1673")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%foo {\
             \n  test: outer;\n\
             \n  &-inner {\
             \n    test: inner;\
             \n  }\
             \n}\n\
             \n.foo {\
             \n  @extend %foo;\n\
             \n  &.inner { @extend %foo-inner; }\
             \n}"),
        ".foo {\
         \n  test: outer;\
         \n}\
         \n.foo.inner {\
         \n  test: inner;\
         \n}\n"
    );
}
