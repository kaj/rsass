//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_592.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%a::-webkit-scrollbar {\
            \n    color: green;\
            \n}\
            \n\
            \n.a {\
            \n    .b {\
            \n        @extend %a;\
            \n    }\
            \n\
            \n    .c .b {\
            \n        @extend %a;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        ".a .c .b::-webkit-scrollbar, .a .b::-webkit-scrollbar {\
        \n  color: green;\
        \n}\
        \n"
    );
}
