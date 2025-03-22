//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1248.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1248")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".a.b .c {\
             \n  top: 0;\
             \n}\
             \n.a {\
             \n  @extend .b;\
             \n}\
             \n.a .d {\
             \n  @extend .c;\
             \n}\n"),
        ".a.b .c, .a .c, .a .d {\
         \n  top: 0;\
         \n}\n"
    );
}
