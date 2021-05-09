//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1091.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
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
        ".a, .d > .e, .b .c, .b .d > .e {\
         \n  top: 0;\
         \n}\n"
    );
}
