//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_469.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "/*!\
            \n*/\
            \n\
            \n@charset \"utf-8\";\
            \n\
            \na {\
            \n  color: red;\
            \n}\
            \n\
            \n@import url(\"x\");\
            \n"
        )
        .unwrap(),
        "/*!\
        \n*/\
        \n@import url(\"x\");\
        \na {\
        \n  color: red;\
        \n}\
        \n"
    );
}
