//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1091.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1091")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a {\
             \n  top: 0;\
             \n}\n\
             \n.b .c {\
             \n  @extend .a;\
             \n}\n\
             \n.d > .e {\
             \n  @extend .a;\
             \n  @extend .c;\
             \n}\n"),
        ".a, .d > .e, .b .c {\
         \n  top: 0;\
         \n}\n"
    );
}
