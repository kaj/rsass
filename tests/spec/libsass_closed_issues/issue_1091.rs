//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1091.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".a {\
            \n  top: 0;\
            \n}\
            \n\
            \n.b .c {\
            \n  @extend .a;\
            \n}\
            \n\
            \n.d > .e {\
            \n  @extend .a;\
            \n  @extend .c;\
            \n}\
            \n"
        )
        .unwrap(),
        ".a, .d > .e, .b .c, .b .d > .e {\
        \n  top: 0;\
        \n}\
        \n"
    );
}
