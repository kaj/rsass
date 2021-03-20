//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1248.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".a.b .c {\
            \n  top: 0;\
            \n}\
            \n.a {\
            \n  @extend .b;\
            \n}\
            \n.a .d {\
            \n  @extend .c;\
            \n}\
            \n"
        )
        .unwrap(),
        ".a.b .c, .a .c, .a .d {\
        \n  top: 0;\
        \n}\
        \n"
    );
}
