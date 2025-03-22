//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2053.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_2053")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".foo[disabled] {\
             \n    @extend .foo;\
             \n    background: blue;\
             \n  }\
             \n.bar[disabled] {\
             \n  foo {\
             \n    @extend .bar;\
             \n    background: blue;\
             \n  }\
             \n}\
             \nfoo {\
             \n  .baz[disabled] {\
             \n    @extend .baz;\
             \n    background: blue;\
             \n  }\
             \n}"),
        ".foo[disabled] {\
         \n  background: blue;\
         \n}\
         \n.bar[disabled] foo {\
         \n  background: blue;\
         \n}\
         \nfoo .baz[disabled] {\
         \n  background: blue;\
         \n}\n"
    );
}
