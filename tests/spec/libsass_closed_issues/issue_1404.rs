//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1404.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".test {\r\
            \n    color: #aaabbb--1-2-a;\r\
            \n    color: type-of(#aaabbb--1-2-a);\r\
            \n    color: type-of(#aaabbb--1-2);\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  color: #aaabbb--1-2-a;\
        \n  color: string;\
        \n  color: string;\
        \n}\
        \n"
    );
}
